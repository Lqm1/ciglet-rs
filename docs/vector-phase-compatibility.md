# Vector and phase compatibility

The `vector` and `phase` modules support f32 and f64 through the sealed
`statistics::Scalar` trait. Scalar interpolation is exposed as
`interpolation::between`. First differences and cumulative sums retain the
first input sample, including its zero sign. Linear grids include both bounds
and retain source multiply-before-divide rounding. Array reversal works with
any Clone element. Integer progressions use checked i32 results and i64
intermediates, avoiding undefined signed multiplication in the C source.

Phase wrapping retains the C double-precision expression and ties-away-from-zero
rounding. Unwrapping retains the source's single-turn correction for each jump
beyond pi, rather than replacing it with a different multi-turn algorithm.
Directed phase differences preserve integer shift rounding and the source
remainder expression. An unrepresentable shift returns an error rather than
performing an invalid float-to-integer conversion.

Complex magnitude preserves scalar squares and addition followed by a double
square root. A double hypot fallback handles intermediate overflow or complete
underflow when the final magnitude is representable. Complex argument uses
double atan2 followed by conversion to the selected precision, matching c_arg,
c_arg_2, c_arg_3 and argcplx. The c_arg_1 variant is implemented through
`complex::argument` with `Precision::Fast`; its independent comparison and
numerical boundary evidence are recorded in complex-compatibility.md.
Polar conversion retains the upstream level-two f32 sine/cosine approximation,
including in the double build. No exact-trigonometry substitution is made.

Spectrum completion mirrors the upper half and keeps supplied DC/Nyquist values
by default, consistent with the existing inverse STFT policy. Explicit Legacy
mode reproduces the C overwrite of an even-length Nyquist bin by its preceding
sample, including the positive substitution for antisymmetric completion.
Only input bins that are actually read require finite values; overwritten
upper bins may be uninitialized placeholders such as NaN.

Empty vectors are safe. A zero-point grid is empty and a one-point grid contains
the starting value. Nonfinite samples, invalid shapes, unrepresentable results,
excessive lengths and allocation failures return errors where applicable.
These policies correct C accesses to element zero or minus one of empty arrays.
The bounds oracle reproduces those invalid accesses for diff, cumsum, unwrap,
complete_symm and complete_asymm, and the signed overflow in iota, using GCC's
address/undefined-behavior sanitizers. The Rust boundary test verifies the
corrected results without linking C into the crate.

The unmodified source is ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Each precision fixture contains
24 vector cases covering lengths 1, 2, 3, 4, 7, 8, 16 and 65, with irregular
Cartesian components, decimal ramps, signed zeros and axis points. Twelve
output arrays per case cover vector operations, phase operations, Cartesian/
polar conversions and Legacy completion. Eighteen grid cases and nine integer
progressions follow. Each precision compares 3954 scalar outputs and 45 integer
outputs. The oracle also checks scalar complex helper aliases against array
results. Regenerate with `tests/generate_vector_phase_reference.py --upstream
PATH --check`, adding `--cc zig --zig` on Windows. GCC and Zig produce identical
fixture bytes. Corrected edge cases are independent Rust checks.

Implementation follows the official Rust
[round](https://doc.rust-lang.org/std/primitive.f64.html#method.round),
[hypot](https://doc.rust-lang.org/std/primitive.f64.html#method.hypot),
[atan2](https://doc.rust-lang.org/std/primitive.f64.html#method.atan2) and
[remainder](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)
interfaces. The target-specific numeric comparison results are recorded in the
project progress record; exact agreement on this finite corpus is not a claim
of universal bitwise transcendental compatibility.

All 100 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. Every compared scalar and integer output in
this corpus matches the C fixture exactly on all five targets. The measured
atan2 difference was zero in both precisions, so its final test requires bit
equality as well, including signed zero. Those strengthened three tests are
repeated on all five targets after the full suite. Formatting, Clippy and
Rustdoc with warnings denied also pass. This chapter does not establish full
dependency coverage, all real-audio acceptance requirements or binding support.
