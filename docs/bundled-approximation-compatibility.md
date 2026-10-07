# Installed bundled approximation interfaces

The pinned ciglet Makefile installs `external/fastapprox-all.h` alongside
`ciglet.h`, which includes it by default. Its public approximation functions
are therefore tracked separately from the original own-header inventory.
They remain part of the complete accepted functionality scope.

The installed header defines 44 scalar functions and 44 SSE2 four-lane
functions. `bundled-approximation-inventory.csv` records each one. Thirteen
scalar functions already map to existing approximation APIs. Eight further
scalar functions map to `approximation::hyperbolic`. Nine additional scalar
functions map to restricted trigonometry, full tangent and faster power.
The remaining fourteen scalar functions now map to `approximation::special`.
Ten four-lane exponential/logarithm/power functions now map to
`approximation::lanes`. Eight hyperbolic/sigmoid functions now map to its
`hyperbolic` module. Twelve trigonometric functions now map to
`lanes::trigonometric`. Fourteen special functions now map to `lanes::special`.
All 88 installed scalar/four-lane functions have Rust mappings.
Existing mappings do not establish
complete bundled-specific verification by themselves.

## Hyperbolic and sigmoid functions

| Source | Rust call |
| --- | --- |
| fastsinh / fastersinh | hyperbolic::sine with Approximate / Fast |
| fastcosh / fastercosh | hyperbolic::cosine with Approximate / Fast |
| fasttanh / fastertanh | hyperbolic::tangent with Approximate / Fast |
| fastsigmoid / fastersigmoid | hyperbolic::sigmoid with Approximate / Fast |

The APIs share the existing selected exponential kernels rather than adding
eight duplicated entry points. Formula stages remain binary32, including
subtraction/addition, multiplication by one half and the tangent/logistic
quotients. `Precision::Exact` additionally selects the host exponential before
binary32 conversion; it does not replace the surrounding formulas with a
different hyperbolic solver or a numerically saturated rewrite.

Nonfinite inputs and unrepresentable exponential/results return errors.
Domain limits follow the selected exponential inputs: for example tangent
evaluates at `-2 * value`. An unrepresentable exponential is rejected even if
a differently arranged mathematical formula could yield a finite answer.
Ordinary valid source values preserve operation order and rounding.

The independent oracle includes the installed original header unchanged.
Sixteen values per function cover -40 through 40, small magnitudes, both
signs and both zero signs. All 128 results match Rust binary32 bits. Windows
Zig and Linux GCC regenerate byte-identical fixtures with `-O2` and contraction
disabled. Additional tests verify nonfinite/range errors and zero values in
Exact mode. No Cargo dependency or original source modification is introduced.

## Restricted trigonometry, full tangent and faster power

| Source | Rust call |
| --- | --- |
| fastsin / fastersin | trigonometric::sine with Approximate / Fast |
| fastcos / fastercos | trigonometric::cosine with Approximate / Fast |
| fasttan / fastertan | trigonometric::tangent with Approximate / Fast |
| fasttanfull / fastertanfull | trigonometric::tangent_full with Approximate / Fast |
| fasterpow | approximation::power_fast |

Restricted kernels do not reduce angles modulo a period. They retain the
source polynomial even outside its calibration interval. The faster cosine
uses its own even polynomial. Approximate restricted tangent divides two sine
polynomials, whereas Fast divides sine by the even cosine polynomial. Full
tangent retains the source reduction direction, which differs from full sine.
The existing full sine kernels now share the restricted polynomial without
changing their range reduction or validation.

The unchanged bundled C header supplies 456 bit-encoded records: twelve trig
functions at 32 inputs and both power functions at 36 base/exponent pairs.
The inputs include both zero signs, tiny values, adjacent representable values
around half-pi, pi and two-pi, and angles out to +/-100. Finite outputs match
binary32 bits. Nonfinite pole outputs become explicit Rust errors. The full
tangent rejects turn counts outside signed 32-bit conversion range, and faster
power uses the existing positive logarithm and representable exponential checks.
Windows Zig and Linux GCC regenerate identical fixture bytes with contraction
disabled. These measurements do not establish accuracy outside calibration
intervals or at every representable angle.

All five native targets pass the six affected integration suites in Debug and
Release, eighteen tests per configuration. The suites cover the new APIs,
existing scalar and complex math, polynomial coefficients, windows and glottal
synthesis. The fixture has 446 finite bit comparisons and ten nonfinite pole
results; all ten are explicitly checked as errors. Clippy and Rustdoc pass
with warnings denied, and formatting and whitespace checks pass.

Sign manipulation uses safe [f32::to_bits/from_bits](https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits).
Exact mode uses host double sine/cosine/tangent before conversion. Their
[Rust standard-library precision contract](https://doc.rust-lang.org/std/primitive.f64.html#method.tan)
does not guarantee identical bits across platforms. Approximate and Fast
retain explicit binary32 arithmetic without fused multiply-add calls.

## Error, gamma and Lambert W functions

`approximation::special` exposes fourteen checked scalar functions. The `_fast`
suffix selects the source's faster variant. These functions have no host Exact
mode because the installed source exposes two approximation algorithms here.

| Source | Rust function |
| --- | --- |
| fasterfc / fastererfc | complementary_error / complementary_error_fast |
| fasterf / fastererf | error / error_fast |
| fastinverseerf / fasterinverseerf | inverse_error / inverse_error_fast |
| fastlgamma / fasterlgamma | log_gamma / log_gamma_fast |
| fastdigamma / fasterdigamma | digamma / digamma_fast |
| fastlambertw / fasterlambertw | lambert_w / lambert_w_fast |
| fastlambertwexpx / fasterlambertwexpx | lambert_w_exp / lambert_w_exp_fast |

The unusual `fasterfc` spelling means fast erfc; it is the more accurate variant,
distinct from `fastererfc`. Rust names express that distinction without preserving
the ambiguous spelling. The algorithms retain binary32 stages and source
coefficients, including natural-log coefficients, the inverse-error rational
correction, branch-selected Lambert initialization and distinct refinements.
W(exp(value)) is evaluated without forming exp(value). Both such variants use
the faster base-two exponential in their initialization, as in the source.

Inputs and outputs must be finite. Inverse-error arguments must be strictly
between -1 and 1. Gamma/digamma domains follow their actual intermediate
logarithm arguments and poles, rather than a blanket positive-input restriction.
The more accurate log-gamma accepts source-defined negative inputs between -2
and -1; digamma also accepts its valid negative intervals. Faster log-gamma's
log(value) still requires positive values, and faster digamma's log(1 + value)
requires value > -1 while excluding zero. Principal real
Lambert W requires value >= -1/e; this comparison widens the binary32 input
before applying the double constant. Intermediate logarithms and exponentials
use existing checked APIs. Defined ordinary source values match; out-of-domain
or unrepresentable stages return errors rather than unchecked bit conversions.
Ordinary valid refinements retain the source approximations. Invalid Lambert
exponential refinements use the numerical correction described below.

The unchanged original header supplies 232 bit-encoded records: eighty for
error/complementary error, twenty-eight for inverse error, sixty for
gamma/digamma, twenty-four for W(value), and forty for W(exp(value)). The inputs
include zero signs, tiny values, inverse-error values next to +/-1, positive
gamma inputs from 0.0001 through 1e6 and twelve valid negative gamma/digamma
cases, negative and positive principal-branch
arguments, and adjacent floats around both Lambert branch thresholds.
The initial port matched all 232 binary32 results; after the disclosed numerical
correction, 226 still match source bits and six negative results are repaired.
Windows Zig/Linux GCC generate identical
fixture bytes with contraction disabled. Domain and nonfinite tests are
separate from this source comparison. Approximation accuracy for unmeasured
extreme conditioning is not established by bit compatibility.

## Lambert exponential correction

Both original W(exp(value)) refinements can produce negative values, including
at -8, -40 and -100. Large positive arguments can also overflow intermediate
products even though the actual result is finite. These are reproduced source
defects, corrected under the accepted bug-fix policy. The original header and
fixtures remain unchanged so the intentional differences remain reviewable.

Positive finite source outputs retain their original bits. Nonpositive or
nonfinite refinements use eight Newton steps in double precision. Negative
arguments solve W * exp(W) = exp(value), where the exponential is bounded by
one. Positive arguments solve W + ln(W) = value without forming exp(value).
This avoids both the invalid negative tail and positive exponential overflow.
Binary32 underflow rounds to positive zero, and representable subnormals are
retained. There is no new dependency or change to the shared source exponential
or logarithm kernels.

The new independent reference computes real roots at 80 and 120 decimal digits,
checks convergence residuals and nearest binary32 rounding intervals, and
requires identical rounded bits at both precisions. Its 481 inputs cover
-120 through -4 at quarter steps, neighboring floats around underflow/normal
boundaries, -200/-1000, extreme negative binary32 input and large positive
inputs through f32::MAX. The separate C oracle includes the unmodified bundled
header and canonicalizes only nonfinite output classification. Among 962
function/input pairs, 941 require correction and 21 retain original bits.
Corrected results are checked within one binary32 ULP of the independent real
reference; zero results additionally require positive-zero bits. Original
positive finite results are checked bit-for-bit. Windows Zig/Linux GCC
regenerate identical mathematical and source fixture bytes.

The correction uses standard [f64 exponential](https://doc.rust-lang.org/std/primitive.f64.html#method.exp)
and [natural logarithm](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
APIs. Their precision is unspecified by Rust's platform contract, so the
one-ULP corpus tolerance is a measured acceptance check, not a universal
correct-rounding guarantee. Positive source approximations are deliberately
preserved and are not claimed to have one-ULP mathematical accuracy.

All five native targets pass the Lambert correction, special-function,
scalar-math, hyperbolic and trigonometric integration suites in Debug and
Release, eleven tests per configuration. The final ten logs include the
negative-domain expansion and numerical correction.
Clippy and Rustdoc pass with warnings denied, as do formatting and whitespace
checks. This is focused verification; final combined native acceptance must
also include the corrected refinement and four-lane implementation.

## Portable four-lane exponential, logarithm and power

`approximation::lanes` accepts owned `[f32; 4]` arrays and returns a checked
four-element result. It introduces no platform-specific alignment requirement,
unsafe operation or dependency. One invalid lane aborts the entire calculation;
no partly computed output escapes. The same algorithm is available on targets
without SSE2. This is portable numerical functionality, not a promise that the
compiler emits native SIMD instructions.

| Source | Rust function |
| --- | --- |
| vfastpow2 / vfasterpow2 | exponential2 / exponential2_fast |
| vfastexp / vfasterexp | exponential / exponential_fast |
| vfastlog2 / vfasterlog2 | logarithm2 / logarithm2_fast |
| vfastlog / vfasterlog | logarithm / logarithm_fast |
| vfastpow / vfasterpow | power / power_fast |

Source inspection establishes matching lane expression order for these ten
functions on their checked domains. Positive finite log inputs have a clear
sign bit, so the original signed vector bit-to-float conversion agrees with
the scalar unsigned bit conversion. Representable exponential result bits also
fit signed 32-bit conversion. This permits sharing the verified scalar kernels
here; it does not establish equivalence for other vector functions.

| Original helper/type | Safe Rust representation |
| --- | --- |
| v4sf / v4si | Floats = [f32; 4] / Integers = [i32; 4] |
| v4sfl / v4sil | ordinary [value; 4] array construction |
| v2dil | from_integer64_lanes([value; 2]) |
| v4sfindexer / v4siindexer, indexing macros | owned arrays and checked indexing |
| v4sfv4sipun | to_bits / from_bits |
| v4sf_fabs | absolute |
| v4sf_to_v4si | to_integers |
| v4si_to_v4sf | from_integers |

Float-to-integer conversion explicitly returns the SSE2 indefinite value
i32::MIN for NaN, infinity and out-of-range values. A plain Rust float cast
would instead saturate or return zero, so it is not used unchecked here.
Integer-to-float conversion follows Rust's documented
[nearest-even numeric-cast rules](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast).
The APIs assume the ordinary nearest-even arithmetic environment. Floating-point
exception flags and alternate native rounding modes are not exposed by these
portable value APIs. Array construction/indexing follow the standard
[Rust array API](https://doc.rust-lang.org/std/primitive.array.html).

The original integer splat macro has a signed-promotion defect: evaluating
v4sil(-2) produces [-2, -1, -2, -1]. An isolated unchanged-header C probe
reproduces this result. Ordinary Rust [-2; 4] correctly fills all four lanes;
the source macro's accidental high-word sign extension is not retained.

The unchanged original SSE2 header provides eighty four-lane reference records:
64 function evaluations and sixteen conversion/absolute-value records. Mixed
lanes cover sign and zero differences, exponential clamp neighbors and upper
range, subnormal/normal log inputs, neighboring mantissas, wide magnitudes,
distinct power exponents, integer rounding ties and invalid conversion lanes.
Seventy-nine complete arrays match original bits, 316 lane comparisons. The
remaining array has an infinite source exponential result at the last float
below 128; the checked Rust calculation returns an error for the whole array.
The scalar exponential2 API now applies the same finite-result guard, correcting
its previously unchecked Ok(infinity) at this reproduced rounding boundary.
Finite original outputs and the distinct faster exponential approximation are
unchanged. Scalar and lane boundary tests verify both rejection and nearby
representable outputs.
Windows Zig and Linux GCC regenerate identical fixtures with contraction
disabled. Separate tests cover error position in every lane and safe owned
bit/splat/index/64-bit-word representations.

All five native targets pass the six affected integration suites in Debug and
Release, fourteen tests per configuration. The suites include the new lanes,
scalar math, special functions, Lambert correction, hyperbolic and trigonometric
operations. Clippy and Rustdoc with warnings denied, formatting and whitespace
checks pass. This focused result still precedes final combined native acceptance.

## Four-lane hyperbolic and sigmoid functions

`lanes::hyperbolic::{sine,cosine,tangent,sigmoid}` accepts four independent
binary32 lanes and a Precision selection. Approximate/Fast map to the eight
original vfast/vfaster sinh/cosh/tanh/sigmoid functions. Source inspection
confirms the same lane expression order as the scalar algorithms, including
half-times-sum/difference, the tangent exponential at -2*value and the sigmoid
exponential at -value. The checked mapper now accepts a closure so these APIs
share the same source kernels without duplicating formulas or exposing partial
results. Existing four-lane APIs retain their behavior.

The independent unchanged-header SSE2 oracle records six mixed input arrays
per function, 48 arrays and 192 exact binary32 output comparisons. Inputs
include zero signs, tiny values, positive/negative magnitudes, fractional
arguments and values through +/-40. Windows Zig and Linux GCC regenerate
identical reference bytes with contraction disabled. Every lane position is
tested for nonfinite inputs and unsupported exponential ranges. Exact mode
additionally follows the existing scalar host-exponential selection and retains
the binary32 surrounding formulas; zero outputs have explicit tests. Exact
mode is additional Rust functionality and has no bundled C counterpart.

All five native targets pass seven affected integration suites in Debug and
Release, sixteen tests per configuration, including the previous four-lane
kernels and Lambert correction. The ten final logs have no failed or ignored
tests. Clippy and Rustdoc pass with warnings denied; formatting and whitespace
checks pass. This checkpoint does not replace final combined native acceptance.

## Four-lane trigonometric functions

`lanes::trigonometric` exposes sine, cosine and tangent, each with a separate
full-range variant. Approximate/Fast cover the twelve original vector functions;
Exact additionally selects host double trigonometry before binary32 conversion.
Restricted variants retain their source polynomials without period reduction.
Full sine reduces as center-angle; full tangent reduces as angle-center. Full
cosine retains the binary32 half-pi addition before source reduction. Full
approximation turn counts must fit signed 32-bit conversion; invalid inputs or
nonfinite stages/results reject the whole owned array.

The Approximate sine evaluates its unsigned correction polynomial before
XORing the input sign into the correction, matching vector source order.
Approximate cosine selects its negative offset at angle >= half-pi, while
scalar cosine uses strictly greater. The isolated original C probe gives scalar
b4c8d15f versus vector 00000000 at binary32 half-pi; the Rust vector API retains
00000000. Faster restricted kernels share scalar code only where source order
is identical. Full tangent uses the vector cosine branch in its denominator.

The unchanged-header SSE2 oracle supplies 144 mixed arrays, twelve per original
function. Forty-eight input values include signed zeros, tiny magnitudes,
adjacent floats around half-pi/pi/two-pi, both signs, fractions, and magnitudes
through 1e9. All 128 finite arrays match original bits, 512 lane comparisons.
Sixteen arrays include source infinities or NaNs and are checked as Rust errors;
NaN reference bits are canonicalized because only classification is compared.
Windows Zig and Linux GCC generate identical fixtures with contraction disabled.
Separate tests cover nonfinite/error position in every lane, signed-conversion
bounds, the cosine equality branch and Exact signed-zero/zero results.

All five native targets pass eight affected integration suites in Debug and
Release, eighteen tests per configuration. Each of the ten final logs has no
failed or ignored tests. Clippy and Rustdoc with warnings denied, formatting
and whitespace checks pass. Final combined native acceptance remains pending.

## Four-lane special functions

`lanes::special` exposes the fourteen error/complementary-error/inverse-error,
log-gamma/digamma, principal Lambert W and W(exp(value)) APIs. Each has a
descriptive name matching the scalar API, with `_fast` for the faster variant.
Source inspection confirms matching per-lane expression order and threshold
selection. Valid logarithm intermediates have positive bits, so the audited
scalar/vector conversion equivalence applies. The checked mapper shares the
scalar kernels while retaining supported negative gamma/digamma intervals.
An invalid input or pole in any lane aborts the entire owned result.

The independent original SSE2 oracle supplies 67 mixed arrays, 268 lane
comparisons: 255 preserve original bits and thirteen receive independently
verified Lambert corrections. Inputs cover zero signs, tiny values, error/inverse-error domains,
inverse endpoint neighbors, negative gamma/digamma intervals, the principal
Lambert lower boundary and neighboring branch thresholds. W(exp(value)) mixes
ordinary values with the reproduced negative-tail and overflowing-positive
source defects. Each positive finite Lambert output retains its source bits;
invalid refinements use the existing scalar correction independently per lane.
The corrected lanes are checked within one binary32 ULP of the independently
rounded 80/120-digit mathematical corpus. Expected zero also requires positive
zero bits. Other outputs match the original bits directly. Windows Zig and
Linux GCC regenerate identical fixtures with contraction disabled, canonicalizing
only NaN classification. Additional tests check every invalid lane position,
inverse endpoints, poles, principal-real-domain errors and supported negatives.

All five native targets pass nine affected integration suites in Debug and
Release, twenty tests per configuration. All ten final logs have no failed or
ignored tests. Clippy and Rustdoc with warnings denied, formatting and
whitespace checks pass. The complete three-package native matrix still needs
refreshing before the final acceptance decision.

## Remaining acceptance audit

The completed four-lane functions have source arithmetic audits; mapping
each to a scalar call was not presumed bit-equivalent. Portable owned
four-element arrays can replace platform vector types without requiring C/SSE
layout in the Rust API. Conversion/index/splat/absolute-value helper macros
and union representations now have explicit safe mappings documented above.
They remain part of the audit rather than being excluded as macros/types.

All function rows are implemented, but that inventory alone does not establish
final acceptance. Public-definition and numerical evidence still require a
complete requirement audit, and the three-package native execution matrix must
be refreshed at the completed bundled-function revision before bindings start.

The header's include guards, compiler conditional selection and the single-file
packaging mode have no independent runtime algorithm. The single Cargo library
replaces that packaging configuration. Fastapprox's BSD attribution is retained
in `LICENSES/fastapprox-BSD.txt` for existing and new adapted formulas.
