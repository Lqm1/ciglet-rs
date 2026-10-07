# Vector statistics compatibility

All thirteen `statistics` operations support binary32 and binary64 through the
sealed `Scalar` trait. The public names retain their existing Rust meanings.
Unconstrained empty arrays require a scalar annotation such as `mean::<f32>(&[])`.

Sum, squared sum, dot product and cross correlation accumulate sequentially from
positive zero. This corrects the earlier Rust implementation's negative-zero
identity from floating-point `Iterator::sum`. Mean divides by the count in the
selected precision. Variance retains the source expression
`sum_squares / count - mean * mean`, including cancellation and possible negative
variance at large offsets. Covariance retains centered products. Correlation
multiplies the two variances in the selected precision before a binary64 square
root and division, then rounds back to the selected precision.

Correlation returns an error for nonpositive/nonfinite variances, a zero or
nonfinite normalization, or a nonfinite result. Source constant vectors produce
0/0, and cancelled variances can otherwise yield an invalid correlation. These
cases are explicitly classified by the C oracle; their payload is an unused
zero placeholder. Valid source correlations are compared exactly.

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
Source arithmetic is unchanged; only sort's comparator is corrected. Generator
commands are `tests/generate_statistics_reference.py --upstream PATH --check`,
with `--cc zig --zig` on Windows.

Implementation follows the official Rust
[f64::total_cmp](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp),
[stable sort_by](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by)
and [select_nth_unstable_by](https://doc.rust-lang.org/std/primitive.slice.html#method.select_nth_unstable_by)
interfaces. Sum identities and operation order are explicit rather than delegated
to the generic floating-point Sum implementation.
