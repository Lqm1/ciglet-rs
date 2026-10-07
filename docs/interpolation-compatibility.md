# Interpolation compatibility

`interpolation::linear` implements `cig_interp` and `interp1` with strictly
increasing source coordinates and constant endpoint extension. It preserves
source scalar arithmetic and chooses the preceding interval at exact knots.
Queries can arrive in any order. The C implementation carries its source index
forward and returns wrong values when queries decrease despite advertising
arbitrary queries. Rust uses the standard slice `partition_point` operation
with a monotonic predicate, following the official documentation:
https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point.

`linear_uniform` implements `cig_interpu` and `interp1u`. Source coordinates
occupy `[start, end)`, with index `(position - start) / (end - start) * count`.
The upper coordinate is exclusive, not the coordinate of the last sample.
The original clamp at fractional index `count - 1.01` is retained. Empty and
unordered queries and singleton sources are safe. Original C's single-query
endpoint accesses beyond the input; AddressSanitizer reproduces this at
`ciglet.c:810` using `tests/reference/interpolation-bounds_oracle.c`.
Rust returns the last value at that boundary. Source arrays must be nonempty;
invalid coordinates, nonfinite values and arithmetic overflow return errors.

`sinc_uniform` and `sinc_uniform_with` implement `cig_sincinterpu` and
`sincinterp1u`, using the same four-lobe Lanczos kernel as the resampler.
The default `BoundaryPolicy::IncludeFirst` corrects the omission of sample zero.
`LegacySkipFirst` preserves that omission for numerical comparison. Both policies
start the output accumulator at zero. Original C uses an uninitialized malloc
buffer, so its direct output has no deterministic reference value. Queries
outside kernel support return zero. No normalization or endpoint replication
is added to the source sinc kernel.

`fill_missing` implements `interp_in_blank`. Exact sentinel values are replaced
by linear interpolation, with nearest known values at leading and trailing
gaps. A NaN sentinel matches NaNs explicitly. An all-missing series becomes
zero, retaining C's finite-sentinel fallback. Known values must be finite.
Rust handles empty input and avoids temporary duplicate padding coordinates.
The original NaN condition admits every NaN because `NaN != NaN`. Its source
count also includes one extra padding entry and exceeds its allocation when
every input sample is known. The corrected C comparison fixes
both lines and retains the remaining operations.

## Reference coverage

`tests/generate_interpolation_reference.py --upstream /path/to/ciglet --check`
rebuilds all six fixtures with the pinned original source and GCC `-O2`.
The linear references are unchanged C. Each precision has 24 cases and 504
output values over irregular/uniform grids, four source lengths, three signal
patterns and sorted queries inside and outside the sample range.

The extended references have 28 cases and 436 output values per precision and
boundary policy. They cover twelve sinc cases and sixteen missing-value cases,
including NaN sentinels, all missing, alternating gaps, one known value and no
gaps. The three necessary deterministic/safety corrections are recorded in
`tests/reference/interpolation-corrections.patch`. The optional inclusion of
sample zero is the separate `interpolation-include-first.patch`. Reference
patches retain source arithmetic and are verified by the generator.

Independent Rust regressions exercise unordered queries, single-query endpoints,
singleton sources, empty queries, invalid coordinates, NaN gaps, all-missing
input, an impulse at sample zero and distant sinc queries. Rust owns returned
vectors and never mutates borrowed input. All 72 integration tests pass on Windows MSVC64/32, Windows GNU64 and
Linux GNU64/32, including these eight interpolation tests. Formatting and
Clippy with warnings denied pass on the primary Windows target.

The later near-zero conditioning correction uses KernelPolicy independently
of sample-zero handling. Default sinc_uniform selects Stable, while the existing
sinc_uniform_with keeps source kernel arithmetic. sinc_uniform_with_kernel
exposes both policies explicitly. Original/disclosed-corrected fixture comparisons
are unchanged; tests/resampling_conditioning.rs adds independent mathematical
limit checks. See dsp-optimization-audit.md for measurements and current validation.
