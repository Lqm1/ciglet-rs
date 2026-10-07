# Linear prediction estimation core

`prediction::levinson` estimates autoregressive coefficients from nonnegative
autocorrelation lags. The leading coefficient is one and the remaining length
defines the order. `from_samples` computes unnormalized nonnegative-lag
autocorrelation and returns owned coefficients and autocorrelation in
`Prediction`. `gain` returns the square root of their sequential dot product.
All operations support f32 and f64 through the existing sealed Scalar trait.

The recursion preserves source operation order, including its 1.000001
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
checks do not claim an overflow-free or compensated solver for every finite
extreme-scale autocorrelation; that numerical-stability audit is still pending.

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
