# Itakura-Saito spectral distance

`distance::log_itakura_saito_with(..., Policy::Legacy)` implements the source
`itakura_saito` from ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Despite its name, the source returns
an approximate natural logarithm of the mean divergence, not the raw divergence.
`log_itakura_saito` defaults to corrected evaluation of that log quantity.
`itakura_saito` additionally exposes the corrected nonnegative mean itself.

For each bin with positive source `x` and reference `y`, the divergence is
`x/y - ln(x/y) - 1`. The mean is over all bins. The operation is directional,
so swapping the spectra generally changes the result. No epsilon floor is
added to the caller's input. Inputs must be equal-length, nonempty, finite and
nonnegative. Negative samples, nonfinite samples and invalid shapes return
errors in both policies. No temporary difference array is allocated.

## Source precision and compatibility

Legacy forms each ratio in the selected f32/f64 precision, applies the binary32
level-two approximate log, then retains source subtraction and accumulation
stages. The `-1.0` literal performs double subtraction before conversion back
to the selected scalar. Sequential accumulation begins at positive zero. The
mean is formed in that scalar, and the final approximate log again uses f32
internally. Source aliases and arithmetic order are retained without C linkage.

The source's bit approximation produces finite values even when its input is
zero, negative, infinity or NaN. Such values can arise internally from a rounded
mean, a zero denominator or an overflowing/underflowing ratio. A private
bit-approximation helper is shared with the validated public logarithm code to
preserve these Legacy artifacts. It does not weaken the public logarithm input
checks. In particular, Legacy accepts paired zero bins and reproduces the
source's implementation-dependent finite result; this is not a mathematical
divergence guarantee. Other platforms' NaN propagation remains unaudited.

For identical singleton spectra `[0.5]`, the pinned C fixture returns about
`-13.30342` in float and `-13.3132562637329` in double. These finite values arise
from the inner approximate log's error, not a nonzero mathematical distance.
Legacy reproduces them, while corrected evaluation returns zero mean and
negative-infinity log mean.

## Corrected evaluation

Stable evaluation uses double internally. Near identity, it obtains the relative
change from `(x-y)/y`, avoiding a ratio rounded prematurely to one. The factored
series for `delta - ln(1+delta)` retains the positive quadratic term for adjacent
floating-point inputs. It is used for relative changes through magnitude 0.5,
with up to 64 terms and early termination when a term no longer changes the
coefficient. Ordinary ratios use the double expression directly.

When a ratio overflows or underflows, a difference of input logarithms retains
the information lost by direct division. An overflowing ratio is evaluated in
the log domain using `ln_1p`; an underflowing positive ratio uses its logarithmic
limit. A scaled online log-sum-exp with compensated accumulation computes the
mean without an overflowing linear sum. This also keeps finite log-distance
results when the linear mean cannot fit in f64.

| Input/result case | Corrected mean | Corrected log mean |
| --- | --- | --- |
| Identical positive spectra | 0 | Negative infinity |
| A zero bin against a positive bin | Positive infinity | Positive infinity |
| A paired zero bin | Error | Error |
| Finite mean exceeding the selected scalar range | Positive infinity | Finite |

Every paired zero is checked before a single-zero infinity result is returned.
Applications requiring a silence floor can apply one explicitly before calling
the distance API. An implicit floor would change the mathematical input and is
not introduced here.

Primary Rust references include
[natural logarithms](https://doc.rust-lang.org/std/primitive.f64.html#method.ln),
[ln_1p](https://doc.rust-lang.org/std/primitive.f64.html#method.ln_1p),
[exponentials](https://doc.rust-lang.org/std/primitive.f64.html#method.exp)
and [float bit access](https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits).
Host transcendental precision and cross-platform NaN behavior are not universal
bit-exact promises.

## Reference corpus and independent tests

`tests/generate_distance_reference.py` compiles the unchanged header and
fastapprox definitions at `-O2`, without `-Ofast`. ISD1 contains 53 records per
scalar precision. Forty-eight use six bin counts and eight shapes covering
identity, fixed/differing ratios, near identity, large/small ratios and zero
source spectra. Five edge records include maximum/subnormal inputs, zero
denominators and paired zeros. Each record stores a uint32 bin count, source
array, reference array and source approximate log mean. Tests require exact
Legacy output bits, including artifacts outside the inner log's domain.

Regenerate with `python tests/generate_distance_reference.py --upstream PATH`,
add `--check` to verify existing bytes, or use `--cc zig --zig` on Windows.
The Linux GCC and Windows Zig fixtures are checked against the same bytes.

Independent corrected tests verify analytic fixed ratios, scale invariance,
mixed-bin means, adjacent f32/f64 inputs, exact identity, overflowing and
underflowing ratios, zero semantics and invalid inputs. Tests also assert that
public approximate logarithms still reject invalid inputs. More extensive
accuracy evaluation across large heterogeneous spectra and additional Tier 1
platforms remains part of the numerical stability audit.

All 126 integration tests passed on Windows MSVC 64/32bit, Windows GNU 64bit
and Linux GNU 64/32bit. All 53 Legacy outputs per precision match C bits on every
target, including the tested intermediate artifacts. Linux GCC and Windows Zig
regenerate identical fixture bytes. Formatting, Clippy and warning-as-error
Rustdoc passed. The public approximation refactor also passes the existing
precision and downstream DSP reference suites.
