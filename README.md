# ciglet-rs

A Rust reimplementation of the complete public ciglet signal-processing functionality. Upstream: https://github.com/Sleepwalking/ciglet.

Itakura-Saito distance exposes corrected mean/log-mean divergence and an explicit
source-compatible approximate log mean. Stable evaluation retains near-identity
distances and handles extreme ratios in the log domain. See
[distance compatibility](docs/distance-compatibility.md).

Aliased DTFT sinc and its angular derivative support f32/f64, source Legacy
thresholds and corrected small-angle and removable-pole evaluation. See
[sinc compatibility](docs/sinc-compatibility.md).

Scalar sine/cosine/atan2 expose all source precision levels. Scalar min/max
retain the original second-operand behavior for ties and NaNs. See
[scalar math compatibility](docs/scalar-math-compatibility.md).

Owned generic row-major arrays support initialization, copying, flattening,
reshape and transpose. In-place transpose moves non-Clone elements safely;
floating-point storage preserves every bit pattern. These arrays are distinct
from the column-major numerical matrices. See
[array compatibility](docs/array-compatibility.md).

Real cepstra, inverse reconstruction, minimum-phase angles and frame-major
magnitude/cepstrum conversion support f32/f64 with reusable FFT workspaces.
Explicit Legacy policies retain original Nyquist and lifter behavior; defaults
preserve Nyquist and use the causal minimum-phase lifter. See
[real cepstrum compatibility](docs/real-cepstrum-compatibility.md).

Quadratic spectral peak interpolation returns fractional bin positions and
capped magnitudes for f32/f64 with source arithmetic rounding. Missing neighbors
and invalid participating samples return errors. See
[spectral peak compatibility](docs/spectral-peak-compatibility.md).

Linear prediction supports waveform and magnitude inputs, f32/f64 Levinson
recursion, owned autocorrelation/coefficient results and gain estimation.
Reusable spectrum plans support source precision and an explicit Legacy
Nyquist policy. Resonance estimates pair sorted frequencies with poles and
report root convergence. Order zero and silence are safe. See
[estimation compatibility](docs/prediction-compatibility.md) and
[spectrum compatibility](docs/prediction-spectrum-compatibility.md).

Complex construction, conjugation and arithmetic use the existing RustFFT
`Complex` type. Checked division repairs intermediate range loss. Complex
exponentials and arguments expose all three source precision levels; scalar
logarithm/exponential helpers include the original level-one approximations.
See [complex compatibility](docs/complex-compatibility.md).

Vector differences, cumulative sums, reversal, inclusive grids, checked integer
progressions and scalar interpolation support the original workflows. Phase
wrapping/unwrapping, directed differences, Cartesian magnitude/argument, polar
conversion and symmetric/antisymmetric spectrum completion support f32/f64.
Explicit Legacy completion reproduces the source Nyquist overwrite; the default
preserves it. See [compatibility details](docs/vector-phase-compatibility.md).

The port is in progress. Statistics, f32/f64 precise and level-two window functions, centered zero-padded frame extraction, and reusable f32/f64 complex FFTs are implemented. FftPlan preserves unnormalized forward and length-normalized inverse transforms, optional zero input components, and in-place operation. FFT frequency shifting supports odd and even lengths. A direct normalized inverse DFT supports arbitrary positive lengths, preserving upstream scalar phase precision and accumulation order. RustFFT provides reusable plans, preallocated scratch storage, and CPU-selected SIMD with scalar fallback. The original approximate-cosine DCT-II and level-two sine/cosine approximations are also implemented, including the original small deviation at cos(0). Reusable chirp-z plans support arbitrary positive lengths and frequency steps, including the original inverse convention. Centered single/multiple sinusoid synthesis uses the original recurrence, applies amplitude correctly, and safely handles zero/one sample lengths. Bounded WAV reading supports PCM and IEEE float32/float64, mono/stereo left-channel selection, and extensible formats, with documented decoding corrections. Mono PCM8/16/24/32 writing provides corrected defaults and explicit legacy byte-compatible options. Reusable forward STFT supports arbitrary centered frames, variable window lengths, optional magnitude/phase outputs, mean removal, and explicit overlap normalization. Odd windows and variable-window factors have safe corrected behavior. Inverse STFT retains approximate polar conversion, normalized inverse transforms, endpoint fades, offsets, and overlap-add. The default preserves Nyquist energy; explicit Legacy policy reproduces the original overwrite. FrameGrid provides the original regular-grid Blackman analysis and 32-sample-fade synthesis workflows with checked dimensions. Mel/Bark conversions, mel-spaced frequency points, equal-loudness weights, and the original level-two logarithm/exponential/power approximations are implemented with retained scalar rounding. Owned mel/PLP filterbanks support construction, custom response/support configuration, spectrum and spectrogram application, logarithmic or PLP compression, and corrected small-grid boundaries. Spectrogram output uses one flat allocation. Band-feature cepstral conversion retains the source approximate DCT, order clamping and final DC placement, with requested-coefficient and complete legacy buffer APIs plus flattened matrix conversion. Typed windowed FIR design, full linear convolution without a padded input copy, and antialiased Lanczos resampling are implemented for f32/f64. Resampling defaults to corrected first-sample inclusion and offers an explicit original boundary policy. See docs/filter-resample-compatibility.md for cutoffs, measured errors and reference reproduction. Recursive causal and forward/backward filters now retain the original monic convention and arithmetic grouping, correct short-input bounds and default startup, and provide explicit legacy startup. See docs/recursive-filter-compatibility.md. The remaining DSP algorithms, other approximation variants, and plotting integrations are pending.

The scalar Kalman API supports time-varying process/observation noise, explicit or first-measurement initialization, posterior variances, source-compatible likelihood and backward smoothing. Empty inputs and deterministic smoothing have checked behavior. See docs/kalman-compatibility.md.

Vector statistics support f32 and f64 with source arithmetic order, positive-zero reduction identities, deterministic IEEE selection/sorting and source cross-correlation lags. Median averaging avoids finite-value overflow, and numerically undefined correlations return errors. See docs/statistics-compatibility.md for sorting corrections and the source variance formula's cancellation limits.

Owned column-major matrices support zero/identity construction, rectangular products and ordered row swaps. Source-compatible preliminary pivoting, packed LU factorization and substitution are available alongside elimination-stage partial pivoting for complete solves. Rectangular stride/wrapper defects are corrected, and singular or nonfinite arithmetic returns errors. See docs/matrix-compatibility.md for the C comparisons and independent residual checks.

```powershell
cargo test
cargo test --target i686-pc-windows-msvc
cargo clippy --all-targets -- -D warnings
```

This Rust project uses GPL-3.0-or-later as requested for the three-project port. The original ciglet BSD 3-Clause attribution and license are retained in `LICENSES/ciglet-BSD-3-Clause.txt`. No original C implementation is linked into the Rust library.

FFT API references: https://docs.rs/rustfft/6.4.1/rustfft/struct.FftPlanner.html and https://docs.rs/rustfft/6.4.1/rustfft/trait.Fft.html. The native FFT API retains the positive power-of-two size domain documented by upstream external/fftsg_h.c. See tests/fixtures/README.md for measured comparison tolerances and docs/dependencies.md for retained dependency notices.
`instantaneous_frequency::Detector` owns the original configurable four-filter detector, with explicit Legacy arithmetic and scaled Stable estimation. See docs/instantaneous-frequency-compatibility.md.

Framewise correlograms support ACF, AMDF, squared difference and YIN, with period-to-frequency mapping, selected-scalar Legacy arithmetic and Stable centered ACF. See docs/correlogram-compatibility.md.

Reusable spectral-envelope plans return the complete log-envelope and one-sided cepstrum, with explicit harmonic counts and Nyquist compatibility policies. See docs/spectral-envelope-compatibility.md.

LF glottal models support Rd construction, direct public timing/amplitude configuration, reusable derived parameters, analytic magnitude/phase response and sampled periods. Signed excitation and source approximation stages are retained. See docs/glottal-compatibility.md.
