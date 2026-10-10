# Complex arithmetic and numerical precision

The public `complex::Complex` type is the existing RustFFT re-export of
num-complex 0.4.6. `Complex::new`, `conj`, `+`, `-` and `*` cover the source
construction, conjugation, addition, subtraction and multiplication operations.
The dependency's operator grouping is checked against unmodified C, including
signed zero. No new dependency is introduced. These ordinary operators retain
IEEE behavior; they do not turn nonfinite arithmetic into Result errors.

`complex::divide` validates finite operands and a nonzero divisor, then retains
the source scalar products, denominator, addition/subtraction and division.
When intermediate squares/products overflow or completely underflow, or a
result is nonfinite, it tries scaled double arithmetic and validates conversion
to the selected precision. Scaling the divisor into a unit-sized range avoids
squaring an extreme value. Division precedes the final weighted sums to avoid
an overflowing sum for a representable result. If scaling the numerator itself
overflows, division by the bounded normalized denominator happens first.
Ordinary finite results retain their original arithmetic. This fallback is
not a universal compensated-arithmetic algorithm for partially subnormal
intermediates; the final numerical-stability audit remains outstanding.

`Precision::Fast`, `Approximate` and `Exact` correspond to source levels one,
two and three. `complex::exponential` retains each level's magnitude rounding
before its trigonometric products. Levels one/two always use f32 fastapprox
internally, including for f64 operands. Level three computes exp in double,
rounds magnitude to the selected precision, then multiplies in double by
double sin/cos before converting the components. Computing a standard complex
exp or rounding the trig values early would change the f32 source result.
The default source alias c_exp corresponds to Approximate explicitly.

`complex::argument` uses the source rational atan2 approximation in Fast mode.
The other two levels are identical double atan2 followed by scalar conversion,
as in the source. Fast mode preserves its own signed-zero conventions, including
positive pi for (-1,-0). It retains selected-precision rational arithmetic and
the source coefficient rounded from 0.28f. A reciprocal form avoids an
overflowing ratio square; an overflowing ratio uses the correctly signed
half-pi limit. Nonfinite operands return an error.

Level-one logarithm/exponential helpers retain the source bit approximations
and the -126 base-two exponential clamp. Natural log uses its own coefficients,
not ln(2) times the base-two approximation. Positive finite log operands,
finite bounded exponents and valid bit conversions are checked before use.
`exponential_with` and `logarithm_with` expose all three levels for f32/f64.
Unsupported or unrepresentable results return errors. Exact means host libm,
not mathematical exactness or guaranteed bit equality across hosts.

The cross-platform fixture test permits at most one ULP for host-libm complex
exponentials, arguments, logarithms and scalar exponentials, with normalized
error bounded by the selected scalar's epsilon. It reports both maxima for each
operation. Zero results, including their signs, still require exact equality.
Ordinary complex arithmetic and the Fast/Approximate kernels remain bit-exact.
This bound includes the one-ULP complex exponential difference observed on
macOS ARM. ARM CI checks the full core and C ABI suites on Linux, macOS and
Windows instead of extrapolating from x86 results.

The reference uses unmodified ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206` headers. Each precision has 64 complex
cases (eight real values by eight angles), five arithmetic/conjugation results,
three complex exponential levels and two argument levels. Fifteen scalar cases
cover all logarithm/exponential levels and the faster base-two helpers,
including subnormal inputs, signed zero and the underflow clamp. There are
1272 scalar outputs per precision. `tests/reference/complex_bounds_oracle.c`
independently confirms source NaNs from maximum/tiny self-division, a lost
finite quotient after product underflow and a NaN from overflowing fast atan2.
Rust tests verify finite corrected results for those cases and invalid inputs.

Regenerate with `tests/generate_complex_reference.py --upstream PATH --check`,
adding `--cc zig --zig` on Windows. The canonical fixture comes from Windows
Zig 0.16.0. GCC/Linux regeneration checks all inputs, layouts and all ordinary/
approximate results exactly. The only permitted host-C difference is scalar
double exp, using the measured normalized bound documented in the test.
The generator checks signed zero separately and runs the original boundary
driver in both precisions. C is never linked into the Rust library.

Primary references are
[num-complex](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html),
[Rust bit conversion](https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits),
[exp](https://doc.rust-lang.org/std/primitive.f64.html#method.exp),
[ln](https://doc.rust-lang.org/std/primitive.f64.html#method.ln) and
[atan2](https://doc.rust-lang.org/std/primitive.f64.html#method.atan2).
The stable dependency type is used; the experimental std complex API is not
needed by the MSRV 1.94 implementation.

All 103 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. In this corpus, every f32 output and every
f64 output except scalar Exact exp matches the C bits exactly. Scalar double
exp differs by at most one ULP, with maximum normalized difference
`1.1943398331215387e-16`. Both the Rust test and GCC regeneration require at
most one ULP and normalized error no greater than `f64::EPSILON`; a zero result
must match exactly. This keeps tiny exponentials from silently disappearing
under an absolute-error bound. Complex Exact exp, logarithms and arguments
require bit equality on the measured corpus. The strengthened three tests
are repeated on all five targets after measurement. These scoped results do
not prove universal libm bit equality, complete dependency coverage or bindings.
