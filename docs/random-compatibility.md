# Random sampling compatibility

The original `randu`, `randn` and `white_noise` functions are covered by
`random::uniform_from_draw`, `normal_from_draws`, `white_from_draw` and the owned
`Sampler` methods. Both f32 and f64 retain selected-scalar division of each
integer draw by the inclusive maximum before double arithmetic. White noise
multiplies amplitude before two, avoiding an extra intermediate scalar rounding.
Signed finite amplitudes and means are supported. Gaussian parameter two is
variance, including the degenerate zero-variance distribution.

Legacy Gaussian transformation retains binary32 fast-log and fast-cosine calls,
double square root and final selected-scalar conversion. A zero first draw
retains the original finite bit-log artifact. Nonfinite source results are
reported as arithmetic errors instead of returning a NaN. Stable uses true
double logarithm/cosine and separately scaled square roots, avoiding overflow
in variance times the radial factor. Its positive-variance raw transformation
requires a nonzero first draw. No seed can make platform-specific libc `rand`
streams portable, so the library does not claim to emulate those streams.

`Generator` names ChaCha8 explicitly and accepts a 32-byte seed. `Sampler`
also accepts a caller-provided Rand RNG and a grid maximum in 1..=i32::MAX.
Unbiased inclusive range sampling can consume multiple underlying RNG words.
Legacy normals consume two grid draws; white noise consumes one per output,
including zero amplitude. Empty white noise consumes none. Stable normals
redraw a zero first draw for positive variance, failing after 128 consecutive
zeros to bound work with a pathological custom RNG. Zero variance consumes
two draws and returns the mean in Stable mode. Validation and white-noise
allocation precede drawing; subsequent arithmetic errors can advance state.
There is no global RNG, OS entropy request or hidden reseeding.

The unchanged-header oracle replaces only the libc random source and RAND_MAX
with controlled integers. For each precision its 128 records cover both 32767
and 2147483647 maxima, endpoint/near-endpoint/interior draws, signed/zero means
and amplitudes, and several variances. Each record compares four returned
scalars (512 output slots per precision). The oracle also checks one/two/zero
source-draw consumption for uniform/normal/white-noise calls. Regenerate with
`tests/generate_random_reference.py`; `--check` requires identical fixture bytes.
Independent tests verify analytic Box-Muller values, extreme variance, invalid
parameters, stream cloning/consumption, the zero-key ChaCha8 block prefix,
bounded zero-source rejection and 100,000-sample uniform/Gaussian moments.
The standalone unchanged-header random_bounds.c probe independently reproduces
the zero-input artifact (about 13.2686 in both precisions) and double-variance
overflow to infinity; separate square roots yield finite 2.7343e154 instead.

All 143 integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64
and Linux GNU x86_64/i686. Every finite C-reference random output matches bits
on all five targets, including signed zero. Windows Zig and Linux GCC regenerate
identical float/double reference files. Formatting, Clippy with warnings denied,
and warning-as-error Rustdoc pass. These tests cover this sampling family, not
full port acceptance or the still-pending C ABI/WASM bindings.

Primary references:

- https://docs.rs/rand/0.10.3/rand/trait.RngExt.html#method.random_range
- https://docs.rs/rand/0.10.3/rand/trait.SeedableRng.html
- https://docs.rs/rand_chacha/0.10.0/rand_chacha/struct.ChaCha8Rng.html
- https://rust-random.github.io/book/crate-reprod.html
