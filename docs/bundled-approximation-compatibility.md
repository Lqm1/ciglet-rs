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
The remaining fourteen scalar functions now map to `approximation::special`,
leaving all 44 four-lane functions pending. Existing mappings do not establish
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

## Remaining interfaces

The four-lane functions require their own source arithmetic audit; mapping
each to a scalar call must not be presumed bit-equivalent. Portable owned
four-element arrays can replace platform vector types without requiring C/SSE
layout in the Rust API. Conversion/index/splat/absolute-value helper macros
and union representations also require explicit safe ownership/bit-operation
mappings. They are not removed from the audit because they are macros/types.

The header's include guards, compiler conditional selection and the single-file
packaging mode have no independent runtime algorithm. The single Cargo library
replaces that packaging configuration. Fastapprox's BSD attribution is retained
in `LICENSES/fastapprox-BSD.txt` for existing and new adapted formulas.
