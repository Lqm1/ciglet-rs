# Real cepstrum and minimum-phase processing

`real_cepstrum::Plan` implements `rceps`, `irceps`, `minphase`, `spgm2cegm`
and `cegm2spgm` from ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. One forward/inverse FFT pair and
one complex workspace are reused across calls and frames. Returned arrays are
owned. Matrix interfaces use contiguous frame-major storage and checked sizes.
No C code is linked into the Rust implementation.

## Input conventions and precision

`analyze` takes an already logarithmic one-sided spectrum and returns all FFT
length real cepstral samples. It does not apply another logarithm. `reconstruct`
zero-pads a cepstral prefix, mirrors the nonnegative quefrencies and returns the
full-grid real log spectrum. Empty coefficients mean zero. Prefixes through the
full FFT length are supported, including the source's overwritten mirrored
suffix. Only samples participating in the transform require finite values.

`spectrogram_to_cepstra` first applies upstream level-two
`log(magnitude + 1e-15)` and returns each requested coefficient prefix. Its
inverse applies the matching level-two exponential and returns one-sided
magnitudes. The inverse takes an explicit frame count so zero coefficients and
empty frame matrices are unambiguous. The approximate exponential at zero is
retained; zero cepstra therefore reconstruct its source value rather than an
artificial exact one. Silence uses the original epsilon and remains finite.

FFT arithmetic uses the selected f32/f64 precision. The approximate log/exp
helpers use binary32 internally in both source configurations. The epsilon
addition is performed in double before conversion to the approximation input,
as in C. Inverse transforms are normalized by the FFT length; forward
transforms are unnormalized. These conventions follow the reusable FFT module.
RustFFT's primary [planner documentation](https://docs.rs/rustfft/latest/rustfft/struct.FftPlanner.html),
[scratch interface](https://docs.rs/rustfft/latest/rustfft/trait.Fft.html#tymethod.process_with_scratch)
and [normalization documentation](https://docs.rs/rustfft/latest/rustfft/#normalization)
describe reusable plans and explicitly supplied scratch and normalization.

## Nyquist and minimum-phase policies

`NyquistPolicy::Preserve` retains supplied Nyquist values.
`NyquistPolicy::Legacy` substitutes the preceding sample for even transforms,
matching C's `complete_symm`. Discarded Nyquist inputs do not affect results or
finite-value validation. This policy also applies to cepstral reconstruction.

The `minimum_phase` convenience function uses retained Nyquist data and a
causal one-sided lifter. It doubles positive quefrencies strictly below the
Nyquist quefrency, leaves DC/Nyquist unchanged and clears negative quefrencies.
The returned full-grid array contains phase angles, not complex spectra.

`Plan::minimum_phase_with` permits independent Nyquist and lifter policies.
`Lifter::Legacy` exactly follows the source loops: it doubles Nyquist and leaves
index `nfft/2 + 1` uncleared. That negative-quefrency sample changes the returned
phase. The independent analytic test isolates its contribution. The Nyquist
doubling itself does not contribute an imaginary component on the FFT grid.

## Bounds and validation

Positive power-of-two FFT lengths follow the existing FFT plan's range. The
owned plan also safely supports length one. The standalone one-sided convenience
functions require at least two bins; use a length-one plan for singleton data.
Incorrect bin counts, excessive prefixes, invalid matrix dimensions, nonfinite
participating samples, undefined logarithm inputs and overflowing transform
outputs return errors. An error does not prevent reuse of the same plan.
General extreme-scale transform rescaling remains under numerical audit.

## Reproducible reference corpus

`tests/generate_real_cepstrum_reference.py` compiles the unmodified source and
header with Ooura FFT and median dependencies at `-O2`, without `-Ofast`.
The RCP1 fixtures contain 72 cases per precision: FFT sizes 2/4/8/16/32/64,
four signed log-spectrum shapes and coefficient counts one, one-sided bin count
and full FFT length. Every case includes single-frame cepstrum, reconstruction,
Legacy phase, and three-frame magnitude/cepstrum/reconstruction matrices.
The matrices include silence, a positive ramp and a narrow spectral peak.
There are 9,432 stored C output scalars per precision. Tests additionally check
complete Rust forward/inverse chains, totaling 13,428 scalar comparisons.

Run `python tests/generate_real_cepstrum_reference.py --upstream PATH` to
generate fixtures; add `--check` to verify existing bytes. Windows supports
`--cc zig --zig`; Linux uses GCC. The Windows and Linux generators produce
identical float and double fixture bytes.

Independent tests check a cosine log spectrum with two analytic cepstral
coefficients, its round trip and negative-sine causal phase. Tests also cover
retained Nyquist, source lifter residue, discarded nonfinite suffixes, silence,
zero-coefficient frames, invalid shapes, singleton transforms and plan reuse.

The following bounds were selected after measurements on Windows MSVC 64/32bit,
Windows GNU 64bit and Linux GNU 64/32bit. Error is
`abs(actual - reference) / (1 + abs(reference))`. Bounds describe this corpus,
not a universal guarantee for arbitrary input or FFT sizes.

| Operation | Worst f32 error | f32 bound | Worst f64 error | f64 bound |
| --- | ---: | ---: | ---: | ---: |
| Real cepstrum | 2.228768064499478e-8 | 3e-8 | 4.364540199625453e-17 | 5e-17 |
| Inverse from C coefficients | 1.0104661339460338e-7 | 1.2e-7 | 2.688771486842021e-16 | 3e-16 |
| Complete log-spectrum round trip | 1.5878753533437675e-7 | 2e-7 | 2.805738260205458e-16 | 3e-16 |
| Legacy minimum phase | 1.411484763920191e-7 | 1.6e-7 | 2.5460427212166587e-16 | 3e-16 |
| Magnitude matrix to cepstra | 5.559972055005921e-9 | 7e-9 | 2.4212402950875138e-17 | 3e-17 |
| Magnitude matrix from C cepstra | 0 | 0 | 0 | 0 |
| Complete magnitude matrix round trip | 2.199170033229459e-6 | 2.5e-6 | 0 | 0 |

The larger f32 matrix round-trip error includes the source's binary32
approximate exponential applied after a different FFT rounding path. Isolating
the exponential with C coefficients gives identical outputs in this corpus.

`tests/reference/real_cepstrum_bounds_oracle.c` exposes the source's zero-length
`rceps` and 32-coefficient `irceps` with an eight-sample transform. Compiling
this driver and the unmodified library/dependencies with GCC
`-O1 -g -fsanitize=address -DFP_TYPE=float` and then `double` confirms a
heap-buffer-overflow for each mode, four failures total. Run with `zero` or
`prefix`; these failures are reference diagnostics, not Rust tests or API uses.
Rust rejects both inputs before workspace writes.
