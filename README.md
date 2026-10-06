# ciglet-rs

A Rust reimplementation of the complete public ciglet signal-processing functionality. Upstream: https://github.com/Sleepwalking/ciglet.

The port is in progress. Statistics, precise window functions, centered zero-padded frame extraction, and reusable f32/f64 complex FFTs are implemented. FftPlan preserves unnormalized forward and length-normalized inverse transforms, optional zero input components, and in-place operation. FFT frequency shifting supports odd and even lengths. A direct normalized inverse DFT supports arbitrary positive lengths, preserving upstream scalar phase precision and accumulation order. RustFFT provides reusable plans, preallocated scratch storage, and CPU-selected SIMD with scalar fallback. The original approximate-cosine DCT-II and level-two sine/cosine approximations are also implemented, including the original small deviation at cos(0). Reusable chirp-z plans support arbitrary positive lengths and frequency steps, including the original inverse convention. The remaining DSP algorithms, other approximation variants, audio I/O, synthesis, and plotting integrations are pending.

```powershell
cargo test
cargo test --target i686-pc-windows-msvc
cargo clippy --all-targets -- -D warnings
```

This Rust project uses GPL-3.0-or-later as requested for the three-project port. The original ciglet BSD 3-Clause attribution and license are retained in `LICENSES/ciglet-BSD-3-Clause.txt`. No original C implementation is linked into the Rust library.

FFT API references: https://docs.rs/rustfft/6.4.1/rustfft/struct.FftPlanner.html and https://docs.rs/rustfft/6.4.1/rustfft/trait.Fft.html. The native FFT API retains the positive power-of-two size domain documented by upstream external/fftsg_h.c. See tests/fixtures/README.md for measured comparison tolerances and docs/dependencies.md for retained dependency notices.
