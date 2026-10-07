# Recursive filter compatibility

`filtering::filter` and `forward_backward` implement cig_filter/filter and
filtfilt for f32 and f64. `filter_with` and `forward_backward_with` accept an
explicit `Startup` policy. The default `ZeroState` begins at input sample zero
with zero history. `LegacyZeroPrefix` preserves the original skipped leading
samples when numerator and denominator lengths are equal and between two and
six. Other shapes use the original zero-history recurrence.

The denominator's first element remains ignored, as in the original API.
No coefficient normalization is added. An empty denominator supports FIR
filtering. An empty numerator produces zeros. Outputs contain exactly the
input length rather than exposing the source allocation's unused zero tail.
Forward/backward filtering reverses between passes and after the second pass;
it does not add endpoint padding or steady-state initialization.

The original unrolled paths group feedback products and feedforward products
before adding each sum. Other shapes subtract/add coefficients sequentially.
Rust preserves this distinction because it affects floating-point rounding.
Inputs are borrowed without the padded copies used in the C reference adapter.
Nonfinite inputs, excessive lengths, allocation failure and output overflow
return errors. Empty and short inputs are safe.

## Source bugs and reference scope

Upstream ciglet commit: `895ba9b1c0eabee83d3544208bbc82420efa3206`.
The fast paths in ciglet.c begin their loops at coefficient count minus one,
leaving those output samples zero and discarding their input contributions.
For input `[1, 0, 0]`, numerator `[0.5, 0]`, denominator `[1, -0.5]`, the
original result is all zeros; zero-state filtering gives `[0.5, 0.25, 0.125]`.
The default corrects this while explicit legacy startup remains available.

The generic startup loop iterates to max(na, nb), even for shorter input.
AddressSanitizer on unchanged C reproduced a heap-buffer-overflow at
ciglet.c:723 for one input sample, one numerator coefficient and seven
denominator coefficients. Rust only visits actual input samples.

`tests/reference/recursive_filter_oracle.c` calls unchanged cig_filter.
Short inputs receive trailing zeros to bound the unsafe generic startup;
causal outputs for the original input interval are unaffected. Corrected
startup for the unrolled paths is obtained by prepending coefficient count
minus one zeros and discarding the corresponding leading outputs. The C
algorithm itself is not modified. Both policies are recorded for causal and
forward/backward filtering. No unsafe source call is made on an unpadded short
input. The bounded empty case records an empty result, not original-C empty
input safety.

The IIR1 binary format contains a four-byte signature, little-endian uint32
scalar width and record count (210). Each record stores uint32 numerator,
denominator and input lengths, then corresponding scalar arrays. It ends
with input-length arrays for legacy causal, legacy two-pass, corrected causal
and corrected two-pass outputs. Fourteen coefficient shapes cover equal
lengths one through eight and asymmetric lengths up to eight. Input lengths
are 0/1/2/7/32; signals are silence, a leading impulse, and a signed ramp.
Each precision compares 7,056 output values.

Reproduce fixtures with:

```text
python3 tests/generate_recursive_filter_reference.py --upstream /path/to/ciglet --check
```

Initial Windows MSVC 64bit tests match every f32 and f64 reference value
exactly for both policies and both passes. Independent tests verify a causal
impulse recurrence, short FIR inputs, a hand-computed two-pass FIR example,
empty operands, a[0] handling, nonfinite rejection and overflow errors.
Other native targets and original -Ofast builds require separate validation.

Rust primary API references: https://doc.rust-lang.org/std/primitive.slice.html,
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact,
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take.

All 60 current ciglet integration tests pass on Windows MSVC x86_64/i686,
Windows GNU x86_64 and Linux GNU x86_64/i686. All 7,056 outputs per precision
match the C reference exactly on every target. Formatting and Clippy pass.
The reference generator reproduced both fixture files with --check.
