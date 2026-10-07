# Scalar root finding compatibility

`root::find` covers `cig_fzero` and its `fzero` inline wrapper for binary32 and
binary64. A mutable closure captures caller state instead of accepting an
untyped environment pointer. The algorithm retains the C inverse quadratic
interpolation, secant steps, bisection decisions, scalar arithmetic order,
absolute residual/interval tolerance of 1e-8, and representable-midpoint stop.
It also retains the original bound test after swapping interval endpoints.

Intervals without a sign change return their midpoint, as in the source. The
absolute residual tolerance is scale dependent; callers must scale their
function appropriately. This is not a relative-error convergence guarantee.

The source returns the midpoint when either endpoint is an exact zero. Rust
returns that endpoint. Source product-based sign checks can underflow or
overflow; Rust compares signs directly. Bounds must be finite and ordered,
and inspected function values must be finite. Invalid interpolation falls
back to bisection, and midpoint overflow is avoided. A 4096-iteration limit
returns an error instead of allowing an unbounded callback loop.

The independent C oracle evaluates five functions over five brackets in both
precisions: quadratic, cubic, cosine fixed point, exponential, and linear.
`tests/generate_root_reference.py` compiles the pinned upstream source with
`-O2`, generates 25 roots per precision, and supports `--check` regeneration.
Use `--upstream PATH` with GCC, or `--cc zig --zig` on Windows. Separate Rust
tests cover endpoint corrections, captured mutable state, nonfinite callbacks,
tiny residuals, invalid bounds and overflow-safe midpoint fallback.

The closure interface follows the official Rust
[FnMut documentation](https://doc.rust-lang.org/std/ops/trait.FnMut.html) and
[closure call traits](https://doc.rust-lang.org/reference/types/closure.html#call-traits-and-coercions).
Endpoint exchanges use the standard
[mem::swap](https://doc.rust-lang.org/std/mem/fn.swap.html) operation.
