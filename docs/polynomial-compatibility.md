# Polynomial evaluation and root compatibility

`polynomial::evaluate` and `evaluate_real` cover `cig_polyval`, `polyval` and
`polyvalr`. Coefficients are ordered from highest degree to constant. Evaluation
retains the source's ascending-power accumulation and scalar complex products.
Horner evaluation is not substituted because it would change rounding. The
real-coefficient evaluator avoids the source's temporary complex allocation.

`roots` and `roots_real` cover `cig_roots`, `roots` and `rootsr`. They retain
coefficient normalization, simultaneous Durand-Kerner updates, returned root
order, the source radius clamp, 50 iterations and the 1e-6 stopping threshold.
Initial points use the source level-one binary32 sine/cosine, including when
the surrounding algorithm uses binary64. These approximations are also public
as `approximation::sine_fast` and `cosine_fast`, covering `sin_1` and `cos_1`.
Fastapprox attribution is retained in `LICENSES/fastapprox-BSD.txt`.

The returned `RootEstimate` exposes root values, iteration count and the
maximum residual of the returned roots in the normalized polynomial.
`converged` means that this final residual is below 1e-6. The original stopping
test examines the previous iteration before applying its new values, so final
convergence is checked separately without changing the returned C-compatible
estimates. Reaching the iteration limit can produce useful finite estimates
with `converged == false`; it does not silently report successful convergence.

Nonzero constant polynomials return no roots. Empty input, a zero leading
coefficient, nonfinite coefficients/arguments and singular or overflowing root
arithmetic return errors instead of invalid memory reads or NaN roots. Exact
roots require no correction, including when the denominator would be zero.
This implementation retains the source algorithm's limitations for repeated,
ill-conditioned and high-degree polynomials. It does not guarantee convergence
for arbitrary inputs or improve the source's 50-step estimates.

The independent oracle links unmodified ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206` with `-O2`. Thirty-two cases per
precision cover degrees 1 through 8, real and complex coefficients, scalar
and complex normalization, and three complex evaluation points per case.
Circle inputs are fixed constants in `tests/reference/polynomial_points.h`.
Generating them with the host C math library initially changed degree-five
binary64 coefficients by at most 2.43e-16 between Windows and Linux. Fixing
the inputs permits comparison of the actual solver with identical coefficients.
An additional binary64 polynomial with roots 1 through 8 verifies that a
50-step unconverged estimate still matches C. Separate tests check its residual
using a product of linear factors, independently of coefficient evaluation.
Sixty-one inputs per precision directly check both level-one approximations.
`tests/generate_polynomial_reference.py --upstream PATH --check` regenerates
the fixtures; `--cc zig --zig` selects Zig on Windows.

The complex value interface uses the documented
[num_complex::Complex](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html)
type already provided by RustFFT. Reverse coefficient traversal follows Rust's
[slice iterator](https://doc.rust-lang.org/std/slice/struct.Iter.html) and
[double-ended iterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
interfaces.
