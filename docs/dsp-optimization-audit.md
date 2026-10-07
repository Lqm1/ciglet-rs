# DSP optimization and near-zero conditioning

The baseline is ciglet `895ba9b1c0eabee83d3544208bbc82420efa3206`.
`tests/compare_dsp_optimization.py` independently compiles unchanged original
sources and the existing Kalman and filter/resample drivers at both precisions.
Profiles are `-O2`, `-Ofast`, and explicit `-O3 -ffast-math`. All disable
contraction. The report records compiler identity, source SHA-256 and effective
fast-math macros. Inputs and discrete dimensions must remain identical.

Each precision has 3,672 Kalman results, 4,560 FIR coefficients, 1,830 convolution
samples and 2,298 resampled samples. Strict Kalman results must equal existing
fixtures numerically. Binary32 filter/resample results must also equal their
fixtures. Binary64 filter/resample results retain the existing 2e-15 normalized
fixture bound for compiler/libm variation. Optimization differences are measured
against the same compiler's strict output, separately from fixture comparisons.
Existing Rust fixtures and acceptance thresholds are unchanged.

Reports are `dsp-optimization-windows.json` and `dsp-optimization-linux.json`.
Clang 21.1.0 through Windows Zig leaves fast-math inactive for `-Ofast` alone;
explicit fast-math is active and verified. GCC 13.3.0 enables it for both flags.
The first Windows linking attempt omitted the unused FFT/median implementation
objects needed by the COFF linker. Final diagnostic builds include those unchanged
objects. They do not introduce C code into a Rust library.

The official GCC optimization documentation, consulted through Context7 and
a Mintlify query, distinguishes finite-math/reassociation permissions from
contraction. `-ffp-contract=off` disables fused contraction; it does not undo
the other fast-math assumptions. Primary reference:
https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html.
Mintlify returned no matching excerpt; Context7 returned the official reference.

Under explicit fast-math, convolution preserves all 1,830 results at both
precisions on both compilers. FIR differences are small on this corpus. For
binary32 both compilers reach 5.960464477539063e-8 normalized error. Binary64
maxima are 5.20014139276041e-15 on Clang and 1.6653345369377348e-16 on GCC.
Kalman means, variances and smoothing change by ordinary rounding amounts;
likelihood additionally uses the original binary32 approximate logarithm.
These are corpus measurements, not universal error bounds.

## Reproduced resampling defect

Resampling is substantially different. Both compilers have maximum binary32
absolute difference 0.03439971059560776. For binary64, the strict source produces
-561584025553959 at one sample, while the optimized source produces -0.125.
The absolute difference is 561584025553958.875 and normalized difference is
0.9999999999999998. These results cannot justify relaxing a general tolerance.

Direct evaluation of the original private `interp_kernel` at distance 2^-50,
with GCC `-O2 -ffp-contract=off` and binary64 storage, gives
71882755270906752. Both original `sin_2` factors round to
-3.7405240504995163e-7. The true four-lobe Lanczos kernel approaches one as
distance approaches zero. Its exact-zero branch returns one, but the nearby
nonzero branch divides the residual approximate-sine error by distance squared.
Tiny nonzero coordinates are produced by ordinary fractional-rate division.
An optimizer may round a coordinate to zero and avoid the faulty branch; this
does not make strict or optimized source output a stable numerical definition.

## Corrected and legacy APIs

`KernelPolicy::Stable` uses the product of two normalized sinc expansions
through degree eight when absolute distance is at most 1e-3. The omitted Taylor
term is below 2.4e-29 for each factor in this interval; floating-point evaluation
still rounds. Outside this interval, the original kernel arithmetic is retained.
`KernelPolicy::Legacy` preserves the source branch, including its defect.
Kernel policy and `BoundaryPolicy` are independently selectable.

`resample` and `interpolation::sinc_uniform` use Stable by default. Existing
`resample_with` and `sinc_uniform_with` retain legacy kernel arithmetic and their
explicit boundary selection. New `resample_with_kernel` and
`sinc_uniform_with_kernel` expose both choices. The default changes only the
reproduced near-zero numerical defect and the already disclosed sample-zero fix.
No model, audio or feature format changes.

`tests/resampling_conditioning.rs` compares impulse interpolation near integer
coordinates to an independently evaluated sinc product, checks both signs,
binary32/binary64, fractional-rate bounded output and every boundary/kernel
combination. Legacy fixture suites remain unchanged. SHIRO selects the corrected
kernel by default and its `--legacy-resample` mode selects the legacy kernel.
All five native targets pass the focused suites in Debug and Release.
Ciglet has thirteen passes across three suites and SHIRO has nine passes
across four suites in every configuration, with zero failures and ignored tests.
Both packages pass warning-denied Clippy/Rustdoc, formatting and whitespace
checks. Final combined native acceptance must account for these subsequent
production changes.

Reproduce the diagnostic with:

```text
python tests/compare_dsp_optimization.py --upstream /path/to/ciglet --cc gcc --report /path/to/report.json
python tests/compare_dsp_optimization.py --upstream C:/path/to/ciglet --cc zig cc --target x86_64-windows-gnu --report C:/path/to/report.json
```
