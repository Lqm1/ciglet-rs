# Aliased sinc and angular derivative

`sinc::aliased_with` and `derivative_with` implement `safe_aliased_sinc` and
`safe_aliased_dsinc` from ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Length and angular frequency accept
finite f32/f64, including signed and fractional lengths. The sinc is
`sin(length * angle / 2) / sin(angle / 2)`. Its derivative is with respect to
angle in radians, not frequency in hertz or length. No allocation is needed.

## Source-compatible policy

`Policy::Legacy` retains C's arithmetic stages. The sinc denominator is rounded
to the selected scalar, while the numerator sine and final division use double.
The numerator argument first multiplies length and angle in the selected scalar.
The derivative rounds its four trigonometric terms to the selected scalar and
forms the subtraction in that precision before double multiplication/division.
The source's `10e-5` sinc threshold is 1e-4; the derivative threshold is 1e-5.
Below these absolute denominator thresholds the source returns length and zero,
respectively. Discarded numerator calculations do not cause overflow errors.

## Corrected default

`aliased` and `derivative` select `Policy::Stable`. They evaluate internally in
double before converting the result to the caller's scalar. Length zero and
signed unit lengths use their analytic constants. Small-angle evaluation uses
the ratio of normalized sinc series through eighth order. The derivative uses
its factored series with a fourth-order correction to the leading slope. This
avoids the source's artificial zero-derivative plateau and cancellation between
nearly identical cosine terms.

For integer lengths near a representable multiple of 2*pi, reduction to a local
angle retains the parity sign. An even length at angle 2*pi has limit `-length`,
while the original plateau returns `length`. Negative integer lengths follow
the same sign rule. Fractional lengths are reduced only near zero; their other
denominator zeros are not generally removable and must not be assigned an
integer-length limit. Ordinary angles use the double trigonometric quotient.

Local reduction is limited to 2^52 turns. Beyond that range the direct
trigonometric expression is used. The series branch requires both the local
angle and length-times-angle to have magnitude at most 1e-3. Finite input alone
does not guarantee representable phase products or output; overflow and
nonfinite results return errors. General accuracy for extreme angles,
subnormal slopes or lengths arbitrarily close to a signed unit length remains
part of the numerical stability audit.

The implementation uses standard
[double trigonometry](https://doc.rust-lang.org/std/primitive.f64.html#method.sin),
[rounding](https://doc.rust-lang.org/std/primitive.f64.html#method.round) and
[fractional parts](https://doc.rust-lang.org/std/primitive.f64.html#method.fract).
Host sine/cosine precision is not a universal cross-platform exactness promise.

## Reference and independent tests

`tests/generate_sinc_reference.py` compiles the unchanged header/fastapprox
definitions at `-O2`, without `-Ofast`. SNC1 contains 192 cases per precision:
eight signed/integer/fractional lengths and 24 angular frequencies, including
zeros, both threshold edges, small angles, ordinary angles and signed 2*pi.
Every case stores length, angle, source sinc and source derivative, giving 384
reference outputs per precision. Windows Zig and Linux GCC regenerate identical
fixture bytes. Use `--upstream PATH`, optionally `--cc zig --zig` or `--check`.

Legacy comparisons use normalized error
`abs(actual - reference) / (1 + abs(reference))`. All float outputs match C
exactly on the five required targets. Maximum double sinc error is
1.1080159538408965e-16 with bound 2e-16; derivative error is
3.648531197375612e-16 with bound 5e-16. These bounds were chosen after measuring
Windows MSVC 64/32bit, Windows GNU 64bit and Linux GNU 64/32bit. They describe
this corpus, not all possible inputs.

Independent analytic tests use length two's `2*cos(angle/2)` and derivative
`-sin(angle/2)`, and length three's `1+2*cos(angle)` and derivative
`-2*sin(angle)`. They check positive/negative poles, small angles, both scalar
precisions, fractional near-zero slope, removable zero/unit lengths, finite
maximum-length pole values, invalid inputs and overflowing ordinary phase
products. Corrected and Legacy results intentionally differ at the disclosed
source plateaus and even-length poles.

All 123 integration tests passed on all five required native targets. The final
strengthened sinc tests additionally require exact float output bits and exact
zero signs in both precisions; all three sinc tests passed again on all five
targets after those assertions were added. Formatting, Clippy and
warning-as-error Rustdoc passed. Other Tier 1 targets remain unverified.
