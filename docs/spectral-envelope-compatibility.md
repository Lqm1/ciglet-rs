# Spectral envelope estimation

`spectral_envelope::Plan` and the convenience `estimate` function cover
`cig_spec2env` and `spec2env`. Inputs are the nonnegative one-sided spectrum,
power-of-two FFT length >= 2, fundamental/sampling-rate ratio in (0,0.5], and
an optional harmonic count. None reproduces the original wrapper's count;
explicit counts must be between two and i32::MAX. Outputs include the entire
full-grid natural-log envelope and the original optional one-sided cepstrum.
No exponential or magnitude conversion is introduced. FFT plans are reused.

Generation retains the original low-frequency reflection, selected-scalar
reflection/cutoff arithmetic, mixed-precision smoothing width, moving average,
1e-15 log floor and binary32 approximate logarithm/trigonometry. Cepstral
lifters retain source multiplication staging. The default preserves Nyquist;
explicit Legacy replaces it with its preceding bin at both transforms and
in the returned coefficient prefix, matching the source `complete_symm` calls.

At zero cutoff the source reads `smoothed[-1]`; Rust retains the first smoothed
bin throughout instead. Reflection is bounded by the valid fundamental range.
Zero/nonfinite/out-of-band fundamental, negative/nonfinite spectrum, mismatched
input length, unsupported FFT size/count, allocation failures and nonfinite
arithmetic return errors. Counts computed beyond the C int range use the
mathematical cutoff without converting out of range: for FFT length >= 4 it
exceeds the one-sided spectrum; the two-point case retains the first bin and
flattens the following bin. A zero phase uses the
removable sinc limit. Broader tiny-input/conditioning accuracy remains under audit.

The original fractional-width moving average's endpoint conventions can
perturb an otherwise constant spectrum. They remain covered by the source
comparison and the documented smoothing compatibility policy; they are not
silently replaced here. The independent constant-spectrum test uses an integer
smoothing width. Other independent tests cover zero/positive constant spectra,
coefficient shapes, full-grid symmetry, repeatable plan reuse, Nyquist policies,
corrected two-harmonic cutoff, the two-point wrapper, large implicit count and
invalid inputs.

The unchanged C corpus has 144 records: six FFT sizes 8..256, four spectra
(zero/constant/ramp/peak), three fundamentals and explicit/default counts.
Every full envelope and coefficient prefix is compared, totaling 18,288 source
output scalars per precision. Windows MSVC x86_64 measured normalized maxima
2.0599021608270673e-7 / 2.5898624986317432e-8 for
float envelope/coefficients and 2.5059969251072237e-16 /
4.3183865659335516e-17 for double. Bounds are 3e-7 and 5e-16 respectively.

GCC AddressSanitizer reproduces the negative cutoff index in unmodified float
and double C. Copies changing only zero cutoff to one pass for both precisions;
this leaves the first bin untouched and replicates it over all following bins,
equivalent to Rust's safe first-bin fill. The tracked bounds driver also probes
the wrapper's out-of-range harmonic conversion. See
`tests/reference/spectral_envelope_bounds_oracle.c` and
`tests/generate_spectral_envelope_reference.py`.

GCC float-cast-overflow instrumentation also confirms the wrapper's invalid
harmonic-count conversion at a fundamental of 1e-12 for both float and double.
Rust regressions cover the oversized implicit count for both a normal FFT and
the two-point boundary.

Implementation follows existing FFT ownership and reusable scratch conventions;
primary references are [RustFFT process_with_scratch](https://docs.rs/rustfft/6.4.1/rustfft/trait.Fft.html#tymethod.process_with_scratch)
and [Rust Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

All 135 integration tests passed on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. After the final two-point oversized-count
correction, all three envelope tests passed again on all five targets.
Formatting, Clippy and warning-as-error Rustdoc pass. GCC/Linux and Zig/Windows
regenerate byte-identical source fixtures. The fractional smoothing endpoint
and tiny-width arithmetic remain explicitly pending in the numerical audit.
