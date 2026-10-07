# Linear prediction estimation core

`prediction::levinson` estimates autoregressive coefficients from nonnegative
autocorrelation lags. The leading coefficient is one and the remaining length
defines the order. `from_samples` computes unnormalized nonnegative-lag
autocorrelation and returns owned coefficients and autocorrelation in
`Prediction`. `gain` returns the square root of their compensated scaled dot product.
All operations support f32 and f64 through the existing sealed Scalar trait.

The explicit `ArithmeticPolicy::Legacy` recursion preserves source operation order, including its 1.000001
regularizer and mixed-precision expressions. The initial reflection coefficient
uses a double regularized denominator before scalar conversion. Initial residual
energy multiplies in double after forming the scalar reflection square and
scalar subtraction. Later reflection coefficients and coefficient updates use
selected-precision arithmetic; residual updates use a scalar square followed by
double subtraction/multiplication and scalar conversion. Replacing all of these
with uniform f32 or f64 arithmetic would change the source result.

One owned scratch buffer is reused throughout the recursion. Updating an equal
length coefficient prefix with copy_from_slice preserves the simultaneous
update; it never overwrites a coefficient still needed by the current stage.
Allocation failures, nonfinite data, excessive lengths, negative energy,
inconsistent zero-energy lags and nonpositive/nonfinite recursion residuals
return errors. Waveform order must be smaller than its sample count. These
checks do not claim a solver for every ill-conditioned autocorrelation. The
original waveform correlation and magnitude-squaring stages still require
separate range handling; Stable recursion cannot recover information already
lost or rejected by those stages.

The source unconditionally reads lag one and writes coefficient one, even for
an empty/singleton autocorrelation or order-zero prediction. Rust rejects empty
input and returns [1] for order zero. Zero autocorrelation returns [1,0,...]
and zero gain, making silence a valid result instead of source NaN coefficients.
An impossible correlation matrix is rejected instead of silently returning
unstable coefficients. Invalid gain shapes, negative residual energy and
unrepresentable/nonfinite gain results also return errors.

The reference uses unmodified ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Its generator extracts the unchanged
cig_xcorr and cig_levinson function bodies and uses unchanged headers; no C
algorithm is patched or linked into Rust. Sixty-four waveform cases per
precision cover lengths 2, 3, 4, 7, 8, 16, 32 and 65, irregular inputs, a centered
impulse, a constant waveform and alternating levels. Both order one and the
maximum valid order are tested. Each precision compares 612 correlation values,
612 coefficients and 64 gains, totaling 1288 scalar outputs. Regenerate with
`tests/generate_prediction_reference.py --upstream PATH --check`, adding
`--cc zig --zig` on Windows. Zig/Windows and GCC/Linux generate identical bytes.

An independent test substitutes coefficients into the regularized Toeplitz
equations rather than repeating the recursion. Boundary tests cover order zero,
silence, invalid correlation/energy, mismatched shapes and excessive order.
The dedicated original-C bounds oracle demonstrates empty/singleton accesses
under AddressSanitizer and nonfinite silence coefficients in both precisions.
Frequency-domain estimation, prediction spectra and resonance extraction are
covered separately in prediction-spectrum-compatibility.md. This chapter's
fixture covers waveform estimation only.

Primary Rust references are
[slice::copy_from_slice](https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice)
and [Vec::try_reserve_exact](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact).
The prefix lengths are equal by construction, and allocation errors propagate
before an output model is returned.

All 106 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. Every correlation, coefficient and gain in
this corpus matches the C bit patterns on all five targets. Formatting, Clippy
and Rustdoc with warnings denied pass. AddressSanitizer reproduces both short
length errors independently for float and double; the silence driver confirms
nonfinite coefficients in both precisions. These checks cover the estimation
core and do not replace separate frequency-domain or full-port acceptance.

## Stable recursion and gain

`levinson` and `gain` now default to `ArithmeticPolicy::Stable`. `levinson_with`
and `gain_with` retain explicitly selectable Legacy arithmetic. `from_samples_with`,
`from_magnitude_with_arithmetic` and `SpectrumPlan::estimate_with_arithmetic`
select recursion arithmetic independently of the spectrum Nyquist policy.
Original-C tests explicitly select Legacy; their bit comparisons and spectrum
error bounds are unchanged.

Stable recursion divides autocorrelation lags by positive lag-zero energy and
solves the same Toeplitz system with its 1.000001 diagonal regularizer. Binary64
coefficients, compensated reflection numerators, FMA simultaneous coefficient
updates and residual factors avoid an overflowing regularized input energy or
an underflowing scaled residual. Coefficients are converted to the selected
precision after recursion. Impossible matrices, nonfinite/zero residuals and
unrepresentable output coefficients return errors. This is not a universal
convergence or forward-accuracy claim for ill-conditioned systems.

Gain accumulates coefficient/correlation products with the shared exponent
buckets, then takes a scaled square root before conversion. Finite gains survive
overflowing/cancelling products and below-subnormal squared energy. A genuinely
negative sum remains an error. Independent float/double tests cover maximum
energy, subnormal recursion energy, huge gain cancellation and tiny squared
gain, with explicit Legacy failure preservation.

`tests/generate_stable_prediction_reference.py --check` solves systems by
exact-rational Gaussian elimination rather than repeating Levinson recursion.
Each precision contains 100 systems and 400 coefficient outputs, with orders
0/1/2/4/8, four energy scales and five positive/negative lag decays, including
near-constant strongly correlated systems. The binary64 1.000001 literal is
represented exactly in the reference diagonal. On primary Windows x86_64,
maximum normalized coefficient differences are 9.436895704622536e-16 for float
and 1.926515763819101e-11 for double. Windows GNU double reaches
5.39424561899087e-11 in the strongly correlated order-eight case.
Measured bounds are 2e-15 and 6e-11,
respectively; this corpus is not a universal error bound.

All 168 integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64
and Linux GNU x86_64/i686, with required Gnuplot rendering. After strengthening
the normal-waveform equation checks and default magnitude/gain/response paths,
all ten prediction tests pass again on all five targets. Original-C LPC
and exact-system reference regeneration pass on Windows and Linux. Formatting,
Clippy and warning-as-error Rustdoc pass. Initial Windows GNU failures exposed
a too-tight single-platform bound; all 100 outputs are now measured before the
maximum bound is checked. The final bound follows the observed five-target data.

Primary Rust arithmetic reference:
[f64::mul_add](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add).
