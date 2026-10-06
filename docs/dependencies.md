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

The compatible trigonometric approximation is adapted from the pinned bundled fastapprox code, copyright 2011 Paul Mineiro. Its BSD notice is retained in LICENSES/fastapprox-BSD.txt. This adds no Cargo dependency. Sign-bit operations use the safe f32::to_bits/from_bits API documented at https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits and https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits.

Hound was evaluated for WAVE I/O but its documented Sample implementations omit f64, and floating-point decoding supports 32-bit samples. Adopting it alone would remove original IEEE float64 support. The reader therefore implements the small uncompressed RIFF/WAVE subset from primary format documentation without copying the original wavfile.c implementation, whose inspected header lacks a license grant. No new Cargo dependency is added. Mono PCM writing is now implemented with explicit legacy compatibility and corrected defaults.

Primary references:
- https://docs.rs/hound/3.5.1/hound/trait.Sample.html
- https://learn.microsoft.com/en-us/windows/win32/xaudio2/resource-interchange-file-format--riff-
- https://learn.microsoft.com/en-us/windows/win32/multimedia/devices-and-data-types
- https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible
- https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
- https://doc.rust-lang.org/std/io/trait.Seek.html

Output follows standard Write::write_all error propagation and explicit BufWriter flushing. Primary references: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all and https://doc.rust-lang.org/std/io/struct.BufWriter.html.
