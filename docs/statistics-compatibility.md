# Vector statistics compatibility

All thirteen `statistics` operations support binary32 and binary64 through the
sealed `Scalar` trait. The public names retain their existing Rust meanings.
Unconstrained empty arrays require a scalar annotation such as `mean::<f32>(&[])`.

Sum, squared sum, dot product and cross correlation accumulate sequentially from
positive zero. This corrects the earlier Rust implementation's negative-zero
identity from floating-point `Iterator::sum`. Explicit `Policy::Legacy` mean
divides by the count in the selected precision. Legacy variance retains the expression
`sum_squares / count - mean * mean`, including cancellation and possible negative
variance at large offsets. Legacy covariance retains centered products. Legacy correlation
multiplies the two variances in the selected precision before a binary64 square
root and division, then rounds back to the selected precision.

Legacy correlation returns an error for nonpositive/nonfinite variances, a zero or
nonfinite normalization, or a nonfinite result. Source constant vectors produce
0/0, and cancelled variances can otherwise yield an invalid correlation. These
cases are explicitly classified by the C oracle; their payload is an unused
zero placeholder. Valid source correlations are compared exactly. The existing
32-case source corpus and early DSP reference test explicitly select Legacy for
these four operations. All original functionality remains available.

The default `mean`, `variance`, `covariance` and `correlation` select
`Policy::Stable`; their corresponding `_with` functions accept either policy.
Stable validates nonempty finite arrays and the original signed-int length
domain. It accumulates binary64 significands in compensated exponent buckets.
Products include their fused-multiply-add residuals. Two-part subtraction retains
small offsets subtracted from large samples; differences that exceed binary64
range are represented by halved components and a separate exponent.

For rounded centering values, residual means are corrected using
`(sum(dx * dy) - sum(dx) * sum(dy) / count) / count`. This also gives population
variance when both inputs are the same. Compensated final division avoids
premature rounding of a large sum, including repeated maximum values. Unlike
independent whole-vector normalization, product buckets retain the covariance
of `[A, -A, t, -t]` and `[t, -t, A, -A]` even when `t / A` underflows.

Each temporary accumulator has 4,353 two-component buckets, about 68 KiB, and
uses no heap allocation or new dependency. The exponent bounds cover binary64
subnormals, centered differences and products, plus count/residual corrections
for the checked length domain. Only the active exponent span is reduced, though
initialization still has a fixed cost. Legacy is available when exact source
grouping or its simpler reduction cost is required.

Standalone mean/variance/covariance return errors when a result exceeds the
selected scalar range; subnormal rounding to zero is allowed. Correlation uses
scaled variances before this rounding and separate square roots, so finite
extreme vectors remain correlatable even when variance would overflow or round
to zero. Constant vectors remain errors. Final rounding is clamped to [-1, 1].
No mathematical exact-rounding guarantee for every ill-conditioned input is
claimed; independently generated cases and measured errors are the evidence.

An additional unchanged-header C corpus has 48 records per precision, with
legacy output classifications and four moment slots per record. Its generator
derives independent exact-rational mean/variance/covariance from the captured
IEEE input values and computes correlation with 220-digit Decimal precision.
Rational values are rounded directly to binary32/binary64, ties to even, avoiding
double rounding. Both C compilers regenerate byte-identical references. Tests
also cover repeated extrema/subnormals at lengths through 1,024, affine negative
correlation, reversal, exact known variance, mixed-scale covariance and invalid
arrays. See tests/generate_stable_statistics_reference.py and tests/statistics.rs.

All 161 integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64
and Linux GNU x86_64/i686, with required Gnuplot rendering. Across these targets,
the 48-case stable corpus has zero measured float error. Double mean, variance
and covariance also match their rounded references exactly; maximum correlation
relative error is 3.3306690738754696e-16. Tests allow 2e-7/4e-15 relative error
for binary32/binary64. Formatting, Clippy and warning-as-error Rustdoc pass.
The rational-rounding generator independently checks ties to even, signed
rounding, zero/subnormal/normal boundaries and maximum-value overflow.

Minimum and maximum preserve the source strict-comparison fold, including its
NaN and signed-zero behavior. Selection and sorting instead use IEEE total
ordering, distinguishing negative and positive zero and ordering NaNs. Sort
retains original indices for equal values. The C comparator returned -1 even
when comparing an item with itself, violating the qsort comparator contract.
The reference's only source correction compares finite values correctly and
breaks ties by original index. The exact patch is retained beside the oracle.
NaN/infinity ordering is independently checked against Rust's documented total
order; it is not claimed compatible with the invalid source NaN comparator.

Median partitions once and selects the largest element of the lower partition
for an even count, avoiding the original second recursive selection. Ordinary
averaging retains source addition and binary64 halving. If two finite middle
values would overflow on addition, they are halved before addition, preserving
a finite median for repeated maximum values. Empty means/medians are errors;
empty extrema return None. Empty sums/dots and cross-correlation padding are
positive zero. Cross-correlation allocation and lag counts are checked.

The pinned source is ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Thirty-two cases per precision cover
lengths 1, 2, 3, 4, 7, 8, 16 and 65 with irregular, monotonic, duplicate and
large-offset inputs. Each precision compares 1688 scalar output slots and 424
sort indices. This includes all selection ranks and three excess zero lags.
Legacy arithmetic is unchanged; only sort's comparator is corrected. Generator
commands are `tests/generate_statistics_reference.py --upstream PATH --check`,
with `--cc zig --zig` on Windows.

Implementation follows the official Rust
[f64::total_cmp](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp),
[stable sort_by](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by)
and [select_nth_unstable_by](https://doc.rust-lang.org/std/primitive.slice.html#method.select_nth_unstable_by)
interfaces. Sum identities and operation order are explicit rather than delegated
to the generic floating-point Sum implementation.

The stable implementation also follows official
[f64::mul_add](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add),
[to_bits](https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits) and
[from_bits](https://doc.rust-lang.org/std/primitive.f64.html#method.from_bits)
documentation, consulted through Context7/Mintlify and primary pages. Numerical
background on sum-of-powers instability is available from
[Sandia's centered-moment report](https://www.sandia.gov/research/publications/details/formulas-for-robust-one-pass-parallel-computation-of-covariances-and-arbitr-2008-09-01/).
The report's online/parallel formulas are not described as this implementation.
