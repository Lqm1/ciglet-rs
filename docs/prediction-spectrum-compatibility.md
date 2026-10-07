# Magnitude prediction, spectra and resonances

`prediction::SpectrumPlan` owns one reusable FFT plan and a complex workspace.
Its estimate method squares the supplied real magnitudes, zero-pads the lower
half, mirrors the power spectrum, applies a normalized inverse FFT and estimates
coefficients from the requested autocorrelation prefix. Signed input magnitudes
remain supported because the source squares them. Plans may be reused across
estimation and spectrum evaluation, including after a rejected input.

`from_magnitude` sizes the transform to the next power of two at least
`2 * (bin_count - 1)`. This retains the source zero-padding grid; it does not
resample the supplied bins across the new transform. At least two bins are
required. Order must be smaller than the transform length. Checked integer
arithmetic and the existing bounded FFT constructor prevent invalid sizing.
The default preserves the supplied Nyquist value. `from_magnitude_with` and
the plan's explicit policy allow Legacy replacement by the preceding bin,
matching unmodified flpc. Missing lower bins are zero padding in both modes.
Only bins actually used by the chosen policy are validated and squared. When
Legacy overwrites a supplied Nyquist bin, its original value may be NaN or too
large to square; both are discarded as in C. A shortened spectrum's final bin
is still validated because it is not the transform's Nyquist bin.

`SpectrumPlan::spectrum` zero-pads coefficients and returns every full-grid
FFT magnitude scaled by the supplied gain. The source scalar squares/addition
precede a double square root and division before output conversion; the square
root is not rounded to f32 early. Negative finite gains retain negative output.
A hypot fallback repairs complete underflow/overflow of the squares. Nonempty,
nonzero coefficients must fit the transform. Zero gain returns signed zeros,
including at a grid pole; nonzero gain at a zero divisor returns an error.
Unrepresentable/nonfinite results and squared input power overflow return errors.
General extreme-scale FFT/correlation stability remains part of the final audit.

`prediction::resonances` uses the existing source-compatible polynomial solver,
computes double atan2 divided by pi before scalar conversion, and retains only
frequencies strictly greater than 0.001 and less than 0.999 relative to Nyquist.
The comparisons promote the scalar result to double, retaining C's boundary
constants. Stable total-order sorting keeps each frequency paired with its pole.
`ResonanceEstimate` exposes iterations, convergence and maximum residual for all
roots, so the source iteration cap is visible even if only some poles survive.
Finite unconverged estimates remain available, as in the polynomial API.
The source's invalid sort comparator is not copied into Rust. The reference
corpus uses distinct retained frequencies; equal-frequency ordering is stable.

Pinned source is ciglet commit `895ba9b1c0eabee83d3544208bbc82420efa3206`.
The unchanged C fixture contains 48 magnitude cases: bin counts 3, 5, 7, 9,
17 and 33, four positive spectrum families, and two orders. It records
autocorrelation, coefficients, gain and the complete source prediction spectrum.
Eleven resonance cases cover positive/negative real poles, conjugate pairs,
multiple resonances, coefficient scaling and both sides of each exclusion edge.
Each precision has 1560 scalar output slots. Zig/Windows and GCC/Linux regenerate
identical bytes with `tests/generate_prediction_spectrum_reference.py --upstream
PATH --check` (add `--cc zig --zig` on Windows). No source is patched or linked
into the Rust crate.

The tests compare isolated spectrum evaluation with C coefficients/gain and the
complete Rust estimation/gain/spectrum pipeline separately. The worst normalized
differences across the five required targets and scoped final bounds are:

| Output | f32 maximum | f32 bound | f64 maximum | f64 bound |
| --- | ---: | ---: | ---: | ---: |
| Correlation | 1.3775297214090778e-8 | 2e-8 | 4.0543402972642954e-17 | 5e-17 |
| Coefficients | 2.050684709611527e-8 | 3e-8 | 1.6386607847033336e-15 | 2e-15 |
| Gain | 0 | exact | 3.5389629442130536e-16 | 4e-16 |
| Isolated spectrum | 5.8660281299512945e-8 | 7e-8 | 1.2248633730662573e-15 | 2e-15 |
| Complete spectrum | 6.274567283879118e-8 | 8e-8 | 1.92478530053269e-15 | 3e-15 |

Normalization is absolute difference divided by one plus absolute C output.
Every selected resonance frequency and pole component matches C bits. Counts
also match, including the new exclusion-edge cases. Bounds are chosen after
measurement, and do not claim universal accuracy or libm bit equality.

Independent tests check isolated Nyquist preservation/loss, silence/order zero,
an analytic first-order transfer spectrum, signed gain, minimum/maximum constant
polynomial scaling, a one-point transform, invalid shapes and reuse after errors.
The original-C bounds driver reproduces singleton flpc and oversized lpspec
heap overflows under AddressSanitizer in both precisions. It also confirms lost
maximum/tiny scaling results and zero-gain NaN at a pole. Rust tests verify the
corrected finite results and errors. Formatting, Clippy and warning-as-error
Rustdoc pass. Remaining dependency coverage and bindings are not established here.

All 109 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. After the initial measurement run, comparison
bounds and exclusion-edge cases were added. All five targets pass those checks
and the final complete suite, including policy-aware discarded-bin validation.
The original-C driver independently confirms a discarded Nyquist NaN yields
finite correlation and coefficients in both precisions.

Primary Rust interfaces are
[checked_next_power_of_two](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_next_power_of_two)
and [stable sort_by](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by).
