# Scalar Kalman compatibility

`kalman::filter_with_initial` implements cig_kalmanf1d. `kalman::filter`
implements the first-measurement initialization of kalmanf1d. The owned
`Estimate<T>` contains posterior means, posterior variances and total log
likelihood. `kalman::smooth` implements cig_kalmans1d/kalmans1d using borrowed
posterior means/variances and time-varying process variances. Both f32 and f64
are supported without linking C.

The initial predicted mean is caller-supplied or the first measurement.
Initial predicted variance is Q[0]. Later steps predict the previous
posterior mean and add Q[t] to its variance. Updates retain source arithmetic
order, including `Ppred - gain * Ppred`. Backward smoothing preserves the
final posterior mean and uses P[t-1] / (P[t-1] + Q[t]).

Likelihood uses the source level-two f32 natural-log approximation, even in
the f64 build. Its argument `2*pi*innovation_variance` is evaluated in f64
then narrowed for that approximation. The squared-error likelihood term is
evaluated in f64 because the original expression begins with a double
constant. The complete increment is added to the previous accumulated
likelihood, then rounded into the selected scalar precision every step.

## Checked limits and corrections

Input arrays must have equal lengths, finite means and finite nonnegative
variances within the original signed-32bit sample-count range. Positive
innovation variance is required. Invalid arrays, nonfinite initial means,
allocation failure and calculation overflow return errors.

Empty filtering returns empty mean/variance vectors and zero likelihood;
empty smoothing returns an empty vector. The original filter reads Q[0]
before checking the count, the wrapper reads z[0], and the smoother reads
y[ny-1] and writes x[0], making empty inputs unsafe.

When posterior and process variances are both zero, smoothing uses zero gain
and retains the deterministic previous mean. This avoids the original 0/0
and propagation of NaN. Filtering still requires positive innovation variance;
a zero-variance measurement of a zero-variance prediction is a degenerate
likelihood, reported as an error.

For positive f64 innovation variances whose logarithm argument narrows to
zero or infinity in f32, likelihood uses `ln(2*pi) + ln(variance)` in f64.
The original float bit approximation does not provide a meaningful log in
those cases. The normal source-compatible domain keeps the approximation.

## Original-C references

Pinned ciglet revision: `895ba9b1c0eabee83d3544208bbc82420efa3206`.
`tests/reference/kalman_oracle.c` calls unchanged filter/smoother functions
in 108 cases per precision. Lengths are 1/2/9/32; initial means are zero,
the first measurement or -0.5; measurements are silence, a leading impulse,
or a signed repeating ramp. Noise cases include constant positive variances,
time-varying positive variances and exact measurements with zero observation
noise. All coefficients/measurements use binary-exact fractions.

The KLM1 format begins with a four-byte signature, little-endian uint32 scalar
width and case count. Each case stores uint32 sample count, scalar initial
mean, arrays of observations/process variances/observation variances, then
posterior means, posterior variances, smoothed means and scalar likelihood.
There are 3,672 reference outputs per precision. Fixtures can be reproduced:

```text
python3 tests/generate_kalman_reference.py --upstream /path/to/ciglet --check
```

Initial Windows MSVC 64bit tests match every mean, variance, smoothed mean
and likelihood exactly for both precisions. Independent tests cover a
hand-computed posterior and backward update, deterministic zero uncertainty,
empty input, validation errors, overflow and extreme f64 variance likelihood.
Other native targets and original -Ofast comparisons are separately pending.

Rust primary references:
https://doc.rust-lang.org/std/convert/trait.Into.html,
https://doc.rust-lang.org/std/primitive.slice.html,
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact.

All 64 current ciglet integration tests pass on Windows MSVC x86_64/i686,
Windows GNU x86_64 and Linux GNU x86_64/i686. Every f32/f64 reference output
including likelihood matches exactly on all five targets. Formatting and
Clippy pass. The reference generator verifies both fixture files with --check.
