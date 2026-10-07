# Correlogram and frequency axis

`correlogram::compute/compute_with` covers `cig_correlogram` and all four
`CIG_CORR_*` constants through the `Method` enum: autocorrelation, absolute
difference, squared difference and cumulative-mean normalized YIN difference.
Rows use caller-selected centers and window lengths. Negative/outside centers
clamp as upstream; lags are `0..periods`. The original strict trailing test
`start + window + lag < samples.len()` is retained. Unavailable lags yield zero,
or one for YIN. YIN always sets lag zero to one and retains its 1e-10 floor.

Legacy ACF uses double prefix sums followed by selected-scalar conversions,
products and subtraction, a double square root and scalar conversion. The
original max/min behavior maps NaN correlation from constant windows to zero.
Absolute difference retains scalar subtraction followed by double absolute
value/addition; squared difference and YIN reductions retain scalar staging.
Unused source squared-sample storage is eliminated.

Stable ACF scales paired samples and subtracts local origins before computing
centered covariance/energies in double. Constant windows give zero; negative
correlation clamps to zero. This fixes the tested false-zero result for a ramp
with DC offset 1e12, and handles finite maximum-amplitude alternating samples.
Difference methods retain their native scalar arithmetic; unrepresentable
results return errors. Broader conditioning and extreme-value accuracy remain
part of the final numerical audit.

`to_frequency` covers `cig_invcrgm`: sampling rate is a positive integer in hertz,
and query positions are rate/frequency. Original source linear-grid and scalar
interpolation stages are retained. Positive zero clamps directly to the last
period sample; negative zero/negative frequencies clamp to the first. This
distinction prevents a one-ULP artifact from interpolating the final knot.
Finite queries may arrive in any order, removing the original ascending-query
assumption without changing ordered-input output. Singleton period axes are
defined as constant. Empty output axes/zero frames are supported. Nonfinite
input, zero period count, nonpositive windows/rate, inconsistent dimensions,
allocation failure and arithmetic overflow return errors.

The unchanged C corpus contains 32 records: eight waveform shapes times four
methods. Each uses 64 samples, eight negative/interior/trailing/outside centers,
odd/even windows, 24 lags and eight frequency queries. There are 8,192 compared
output scalars per precision: 6,144 lag values and 2,048 frequency values.
Float outputs require identical bits; double normalized tolerance is 3e-15.
Initial Windows MSVC x86_64 comparisons are exact for both precisions. The
generator links the unchanged pinned C library for reference generation only.

Independent tests verify ramp lag differences, exact YIN values, positive and
negative correlation, DC-offset correction, maximum amplitudes, silence,
strict trailing boundaries, fractional/knot/end frequency queries, signed
zero, unsorted queries, singleton axes, zero frames and invalid inputs.

The source `cig_invcrgm` allocates each result row using `sizeof(FP_TYPE*)`.
On 32bit with double scalars this allocates four bytes for each eight-byte
value. A GCC i686 AddressSanitizer probe reproduces heap-buffer-overflow with
eight frequencies. The original float probe passes; disclosed corrected copies
using `sizeof(FP_TYPE)` pass for float and double. Rust owned typed storage
eliminates the mismatch. See `tests/reference/correlogram_bounds_oracle.c`.

Implementation references: official [Vec fallible reservation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact)
and [f64 clamp](https://doc.rust-lang.org/std/primitive.f64.html#method.clamp).
Prefix lengths use the requested element count rather than allocator capacity.
Stable clamp bounds are finite constants; nonfinite results are checked.
Legacy uses explicit comparisons to preserve the source's NaN-to-zero behavior.

All 132 integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64
and Linux GNU x86_64/i686. Every compared corpus output is exact on every target
in both precisions. Windows Zig and Linux GCC regenerate byte-identical fixtures.
Formatting, Clippy and warning-as-error Rustdoc pass.
