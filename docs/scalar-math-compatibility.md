# Scalar comparisons and trigonometric precision

`scalar::{minimum,maximum}` and `approximation::{sine_with,cosine_with,atan2_with}`
complete the comparison macros and explicit trigonometric precision interfaces
from ciglet commit `895ba9b1c0eabee83d3544208bbc82420efa3206`.

## Comparison semantics

The source `min` and `max` macros use strict comparison and select their second
operand when the comparison is false. The Rust functions preserve this rule.
They accept any `PartialOrd` type, taking ownership of both inputs and returning
the chosen value without requiring `Copy` or `Clone`. Each Rust argument is
evaluated once; repeated macro argument evaluation is not reproduced.

For floating-point operands, equal values and unordered NaN comparisons retain
the second operand. Signed zero and quiet NaN payload bits therefore depend on
argument order. This differs from Rust's floating-point `min`/`max` methods,
which ignore a single NaN. The source behavior is retained rather than replaced
with those methods or a total ordering.

## Trigonometric precision

`Precision::Fast` and `Approximate` select the existing source level-one and
level-two binary32 sine/cosine approximations. The input is converted to f32
before approximation, and the resulting f32 is converted to the caller's scalar
type. This also applies when the surrounding calculation uses f64.

`Precision::Exact` maps to the highest source level, host double `sin` and `cos`,
before conversion to f32/f64 output. The enum name describes that source level;
it does not promise a correctly rounded result on every platform.
`atan2_with(y, x, precision)` uses the existing selected-scalar fast rational
formula at level one, while levels two and three both use double `atan2`.
Complex arguments now share this scalar implementation.

Finite inputs and representable outputs are required. Nonfinite inputs return
errors. Approximate sine/cosine reject range reductions outside the source's
defined integer-conversion domain. The exact level accepts larger finite angles.
Signed zeros are preserved by host trigonometric functions; the fast atan2 axis
rule still returns positive zero when both values compare equal to zero.

Primary Rust references include
[sin](https://doc.rust-lang.org/std/primitive.f64.html#method.sin),
[cos](https://doc.rust-lang.org/std/primitive.f64.html#method.cos),
[atan2](https://doc.rust-lang.org/std/primitive.f64.html#method.atan2),
[floating-point min](https://doc.rust-lang.org/std/primitive.f64.html#method.min)
and [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
The host transcendental methods document unspecified precision. Compatibility
tests therefore record actual differences for the tested corpus and platforms.

## Reproducible corpus

`tests/generate_scalar_math_reference.py` compiles the unchanged header and
fastapprox definitions at `-O2`, without `-Ofast`. It generates 64 finite cases
covering signed-zero axes, quadrants, positive/negative angles and every source
trigonometric precision level. Sixteen additional comparison cases include ties,
signed zeros, infinities and quiet NaNs with distinct signs/payloads. Each scalar
configuration contains 608 source outputs.

Comparison and approximate-trigonometry outputs require identical bits. Host
double transcendental comparisons require identical zero signs and at most one
ULP, with normalized error `abs(actual - reference) / (1 + abs(reference))`
bounded by `f64::EPSILON`. This corpus limit is not a general host-libm guarantee.
The same constraints apply to Linux/Windows C regeneration of the four double
libm fields. All other reference bytes must remain identical; float fixture
regeneration is byte-exact. The check permits only these four specific libm
output fields to vary, preserving the input and comparison payloads.

Run `python tests/generate_scalar_math_reference.py --upstream PATH` to regenerate,
or add `--check` to verify existing data. Windows supports `--cc zig --zig`;
Linux uses GCC. The independent tests check analytic axis values, signed zeros,
ownership of non-Copy comparisons, invalid input and large-angle precision
boundaries. Sinc/derivative and spectral-distance helpers remain separate port
items.

All 120 integration tests passed on Windows MSVC 64/32bit, Windows GNU 64bit
and Linux GNU 64/32bit. Every compared float output, approximation output and
scalar comparison payload matched source bits. The worst host double sine
normalized error was 7.360859413802367e-17, one ULP; cosine and both accurate
atan2 levels matched exactly in this corpus. Formatting, Clippy and
warning-as-error Rustdoc passed. Other Tier 1 platforms remain unverified.
