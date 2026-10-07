# Column-major matrix compatibility

`matrix::Matrix` owns finite binary32 or binary64 values, retaining the source
column-major layout `row + column * rows`. Construction validates dimensions
and buffer lengths. Zero matrices and identities cover `zeros` and `eye`;
matrix and vector products cover `cig_matmul`/`matmul` and
`cig_mvecmul`/`mvecmul`. Products retain C's scalar accumulation order.
Zero-sized dimensions produce correctly shaped empty or zero-filled results.

`pivot_rows` covers `cig_ppivot`/`ppivot`: each column chooses the first row
with the largest absolute entry before any elimination. Its return value is
an ordered list of swaps `(i, swaps[i])`, not a final row-index permutation.
`apply_row_swaps` covers `cig_permm`/`permm`, and `matrix::apply_swaps` covers
`permv`. Invalid swap plans are fully checked before any input is modified.

Two rectangular defects are corrected. `cig_permm` used the number of columns
as its column stride; the documented column-major layout requires the number
of rows. The `mvecmul` wrapper passed the number of columns for both dimensions,
which changed output size and could read/write beyond rectangular buffers.
Both corrections are limited to these dimensions. The independent generator
patches exactly those expressions in a temporary copy of the source/header.
The patch is retained in `tests/reference/matrix-corrections.patch`.
AddressSanitizer reproduces the original heap buffer overflow in 2x3 row
swapping and stack buffer overflow in the 2x3 vector wrapper; both operations
complete without diagnostics against the corrected copy.

`Lu::factor_unpivoted` covers `cig_lu`/`lu` with the original row/column and
dot-product arithmetic order. It consumes an already-pivoted matrix and
rejects zero pivots or nonfinite factors. `Lu::solve` covers
`cig_lusolve`/`lusolve` with the original forward/backward substitution order.
It returns an owned solution and retains the caller's right-hand side.

`Lu::factor` is the preferred complete factorization. It pivots after each
elimination step, exchanges previously computed lower factors with their rows,
and stores the swap plan so `solve` applies it automatically. Source preliminary
pivoting alone can miss a zero pivot arising during elimination of an invertible
matrix. The independent 3x3 regression uses rows `[4,4,1]`, `[2,2,2]`, `[1,0,3]`.
Source-style preliminary pivoting retains row order and unpivoted factorization
fails; elimination-stage pivoting solves `[15,12,10]` as `[1,2,3]`. Tests also
reconstruct `L*U` and compare it to the row-swapped input. The two earlier lower
factors differ, so this comparison detects failure to exchange them in a later
row pivot.

The pinned original source is ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. Each legacy precision fixture
contains 54 matrix/vector products, five square swap cases and twenty
pivot/LU/substitution cases: 1578 scalar outputs and 95 swap indices. Each
corrected precision fixture contains the same 54 products using the corrected
vector wrapper and twelve square/rectangular swap cases: 660 scalar outputs
and 40 swap indices. Comparisons are exact. Elimination-stage factorization is
separately checked through the original matrix/right-hand-side residual, with
normalized bounds 2e-6 for binary32 and 1e-13 for binary64 on these inputs.
These bounds do not guarantee conditioning or accuracy for arbitrary matrices.
Regenerate using `tests/generate_matrix_reference.py --upstream PATH --check`;
on Windows add `--cc zig --zig`.

Checked allocation and row exchange follow Rust's standard
[Vec::try_reserve_exact](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact)
and [slice::swap](https://doc.rust-lang.org/std/primitive.slice.html#method.swap)
interfaces. All core code remains safe Rust.
