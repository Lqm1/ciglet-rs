# Dependency choices

RustFFT 6.4.1 provides reusable f32/f64 complex transform plans and explicit
scratch-buffer processing. Default features select available AVX/SSE/Neon
acceleration and retain a scalar fallback. The WASM SIMD feature is not enabled
at this native stage. The public API preserves ciglet's documented transform
size domain, signs, frequency order, and inverse normalization.

The decision is validated against pinned C reference transforms, including
impulses, complex deterministic signals, large offsets, missing input components,
and lengths from 1 to 1024. Numerical thresholds are scoped to this evidence;
complete SHIRO feature/inference compatibility is still pending.

All currently resolved dependencies offer MIT or Apache-2.0. This project uses
their MIT option and retains the corresponding copyright/license text in
LICENSES. Cargo.lock records exact resolved versions.

| Package | Version | Retained notice |
| --- | --- | --- |
| rustfft | 6.4.1 | rustfft-6.4.1-MIT.txt |
| num-complex | 0.4.6 | num-complex-0.4.6-MIT.txt |
| num-integer | 0.1.47 | num-integer-0.1.47-MIT.txt |
| num-traits | 0.2.19 | num-traits-0.2.19-MIT.txt |
| primal-check | 0.3.4 | primal-check-0.3.4-MIT.txt |
| strength_reduce | 0.2.4 | strength_reduce-0.2.4-MIT.txt |
| transpose | 0.2.3 | transpose-0.2.3-MIT.txt |
| autocfg | 1.5.1 | autocfg-1.5.1-MIT.txt |

Primary API references:

- https://docs.rs/rustfft/6.4.1/rustfft/struct.FftPlanner.html
- https://docs.rs/rustfft/6.4.1/rustfft/trait.Fft.html
