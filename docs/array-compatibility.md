# Owned two-dimensional arrays

`array::Array2<T>` replaces the raw row-pointer helpers from ciglet commit
`895ba9b1c0eabee83d3544208bbc82420efa3206`. It owns one contiguous row-major
buffer. The separate numerical `matrix::Matrix` remains column-major for the
source LU operations. Data conversion must respect these different layouts.

| Source function and macro | Rust operation |
| --- | --- |
| `malloc2d_`, `malloc2d` | `Array2::zeros` for numeric elements; `filled` for other initial values |
| `copy2d_`, `copy2d` | `from_rows`, `try_clone` |
| `free2d_`, `free2d` | Automatic `Drop` of the owned buffer and its elements |
| `flatten_`, `flatten` | `flatten` for a copy; `into_row_major` to transfer ownership |
| `reshape_`, `reshape` | `from_slice` for an independent copy; `from_row_major` to take ownership |
| `cig_transpose`, `transpose` | `transposed` for an independent copy; `transpose` to rearrange owned elements |

`get`, `get_mut`, `row` and `row_mut` provide checked element and row access.
The row-major element index is `row * columns + column`. Mutable slice access
cannot alter the vector length or invalidate dimensions. `reshape` changes
dimensions while retaining the flat element sequence and original allocation.
Both old and new shapes must have the same checked element count.

## Ownership and supported elements

Borrowed constructors and copy methods require `Clone`; buffer storage is
independent. As with any Rust `Clone`, the element's own implementation decides
whether references or shared pointees remain shared. Numeric default values
match the original zero-initialized allocations. For other types, `zeros` uses
their defined Rust `Default`, without constructing potentially invalid all-zero
objects. Destruction follows normal Rust ownership rather than a caller-supplied
row count. No unsafe code is used.

The C transpose dispatch supports element widths 1/2/4/8 bytes. The Rust type
also supports other sizes, zero-sized types and types containing owned resources.
In-place transpose requires no `Copy` or `Clone`. Permutation cycles rearrange
values with slice swaps and an auxiliary byte-per-element visitation array.
Allocation of this array completes before mutation, so allocation failure
leaves the original array intact. One-row, one-column and empty arrays require
only a dimension swap. The copying transpose reads values in their destination
order and creates one new contiguous element buffer.

Floating-point values are storage elements, not arithmetic inputs. NaNs,
infinities, signed zeros and NaN payload bits are retained. The numerical matrix
type's finite-input checks therefore do not apply to these generic arrays.

## Shapes, bounds and allocation

Dimension products use checked `usize` multiplication. Allocation requests use
`Vec::try_reserve_exact`; capacity overflow and allocation failure return errors.
Shape mismatches and ragged row slices are rejected. Invalid reshapes leave both
dimensions and values unchanged. Empty shapes preserve their separate row and
column counts, including a zero dimension paired with `usize::MAX`, without
iterating through those empty logical rows or columns. Out-of-bounds access
returns `None`. Generic `Clone` or `Default` implementations can still panic
according to their own contracts; fallible buffer allocation does not change
those contracts.

Primary Rust documentation for the implementation includes
[fallible vector reservation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact),
[slice swaps](https://doc.rust-lang.org/std/primitive.slice.html#method.swap),
[checked slice access](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
and [ownership and destruction](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

## C reference and independent verification

`tests/generate_array_reference.py` builds the unchanged header and library with
their FFT/median dependencies at `-O2`. The scalar configuration is float, but
the array functions manipulate raw bytes and do not perform scalar arithmetic.
The ARR1 fixture contains 240 cases: row counts 1/2/3/4/7/9, column counts
1/2/3/5/8, element widths 1/2/4/8 and two distinct byte patterns. Each case stores
original bytes, zero-allocation bytes, copied/flattened bytes,
reshaped/flattened bytes and transposed/flattened bytes. Tests compare all
59,280 stored output bytes exactly, and check that two in-place transposes
restore the original array.

Independent tests cover row-major indexing, independent copies, mutation,
buffer-preserving reshape, overflow, ragged rows, empty dimensions, owned
non-Clone values with exactly-once destruction, three-byte elements, zero-sized
elements and bit-preserving f32/f64 transposition. The source's zero initialization
is tested with integer elements rather than invalid arbitrary zero bit patterns.

Regenerate with `python tests/generate_array_reference.py --upstream PATH` and
verify existing bytes with `--check`. Windows supports `--cc zig --zig`; Linux
uses GCC. These generators must produce identical fixture bytes.
