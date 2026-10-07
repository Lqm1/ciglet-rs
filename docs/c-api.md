# C interface implementation status

The optional `c-api` feature exports a C interface from this existing package.
It does not create another crate. The native Rust implementation remains the
calculation backend; unsafe code is allowed only in `src/c_api.rs`.

This is the first binding checkpoint, not complete binding coverage. Currently
the ABI version and binary32/binary64 minimum/maximum operations are exported.
Binary32/binary64 first difference, cumulative sum and reverse are also exported.
Statistics and array generation exports are listed in the checkpoint below.
All remaining public ciglet operations, including the bundled approximation
functions, still require bindings. Liblrhsmm and SHIRO bindings and all browser/
Node WebAssembly bindings remain required work.

Build with `cargo build --locked --features c-api --lib`. Cargo emits the Rust
library, shared library and static library. Regenerate the checked-in header
with cbindgen 0.29.4:

```text
cbindgen --config cbindgen.toml --crate ciglet-rs --output include/ciglet_rs.h
```

The generated header is specifically for a `c-api` build. Cbindgen currently
reports unmapped feature conditions and unrelated private/generic Rust items;
the generated declarations are checked by compiling an independent C caller.
The header intentionally exports functions only, without unrelated Rust constants.

## Pointer and numerical contract

Scalar comparison functions return an unsigned 32-bit status: zero means success and one means
null or misaligned output. Rejected outputs are not written. Otherwise the caller
must supply writable, exclusively accessed storage for the declared output type.
Checking alignment cannot prove allocation validity, lifetime or exclusive access.
Output storage may be uninitialized. These scalar operations neither allocate
nor invoke callbacks and contain no panic-producing operations. Later allocating
or fallible interfaces will need explicit error and unwind handling.

Float results use output pointers rather than float return values. This avoids
the documented i686 C ABI x87 return path's potential NaN payload changes.
Second-operand ties, signed zeros and unordered comparisons follow the native
Rust scalar implementation. The interface uses C calling convention on Windows
32-bit too; Python consumers use `ctypes.CDLL`, not `WinDLL`.

## Verified checkpoint

Windows x86_64 MSVC Debug passes two Rust boundary tests, an independently
compiled C caller linked to the actual generated import library, and Python
ctypes calls against the actual DLL. The C caller checks all four operations,
signed zeros, binary32 NaN payload bits and null rejection. Rust additionally
checks binary64 NaN payload bits and misaligned output rejection without writes.
Python checks both precisions, both operations, signed zeros and null rejection.
Other targets and Release binding verification remain pending. Clippy with
warnings denied, formatting and whitespace checks pass at this checkpoint.

Primary references:

- https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html
- https://doc.rust-lang.org/reference/abi.html
- https://doc.rust-lang.org/std/primitive.pointer.html#method.is_aligned
- https://doc.rust-lang.org/std/ptr/fn.write.html
- https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- https://doc.rust-lang.org/beta/rustc/platform-support.html
- https://github.com/mozilla/cbindgen/blob/main/docs.md

## Vector boundary checkpoint

Vector operations take input pointer/count and output pointer/count. Counts must
match and fit the native vector bound, `isize` byte limit and address arithmetic.
Status two reports invalid lengths; status three reports calculation/allocation
errors returned by the Rust core; status four reports a caught unwinding panic.
Zero-length buffers accept null pointers. Nonempty buffers require initialized
single-allocation input and writable output. Computation finishes into a separate
owned buffer before any output write, supporting identical or partially overlapping
input/output ranges. Failure leaves caller output unchanged. Reverse retains the
native operation's acceptance of nonfinite values; difference/cumulative sum keep
their existing finite-value and overflow validation.

The unwind boundary does not catch aborting panics or allocator aborts, and the
existing panic hook runs before an unwind is caught. On the exceptional panic path
the opaque payload is intentionally retained, because its destructor could panic
again. This leaks that payload only on an unexpected panic. A unit test supplies
a payload with a panicking destructor and verifies the status without dropping it.

Windows MSVC x86_64 and i686 Debug pass all four Rust boundary integration tests
and actual independently compiled C callers. Windows x86_64 additionally passes
Python ctypes vector calls in both precisions. The Rust tests include partial
overlap, in-place writes, empty null buffers, length mismatch, oversized lengths,
nonfinite rejection and arithmetic overflow with unchanged output. Other targets
and Release remain pending; Windows 32-bit Python has not been tested.

Additional primary references:

- https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
- https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html
- https://doc.rust-lang.org/std/panic/fn.catch_unwind.html

## Statistics and generation checkpoint

The interface now has 32 functions including the ABI version. New functions
cover sum, sum of squares, mean, population variance, median and array extrema
in both precisions, explicit mean/variance policies in both precisions, inclusive
linear grids in both precisions and checked i32 arithmetic progressions.
Policy arguments are checked u32 integers: zero means Stable, one Legacy, and
other values return status three. Default mean/variance select Stable, matching
the Rust API. Empty sums return positive zero; empty mean/variance/median/extrema
return status three. Unordered extrema, total-order median, legacy nonfinite
arithmetic and stable validation retain their distinct native behavior.

Scalar reductions finish before committing one output value, allowing it to
overlap the input. Generation also computes into separate owned storage before
committing. Integer overflow and invalid grid values leave output unchanged.
No caller allocation is adopted or freed by Rust.

Windows MSVC x86_64/i686 Debug pass six integration boundary tests and actual
C callers exercising all 32 exports. Windows x86_64 Python ctypes also exercises
all exports, including both valid policies and invalid-policy rejection. Tests
demonstrate stable mean of two f32::MAX values remains finite while Legacy
retains infinity, and verify a scalar output overlapping its input. Ordinary
nonexact Python mean/variance expectations use explicit small rounding bounds;
no native numerical comparison tolerance was changed.
Windows x86_64 Release subsequently passes the same six integration tests and
all 32 exports through independent C and Python callers. Windows i686 Release,
Linux and Windows GNU binding validation remain pending. Cbindgen's `--verify`
confirms the checked-in header matches generation. All-target Clippy and Rustdoc
with warnings denied, formatting and whitespace checks pass.

Statistics bindings still need selection/partition mutation, stable sorting
with original indices, dot product, covariance/correlation with both policies
and cross correlation. Array2 ownership/dimensions/reshape/transpose also remain
required. These additions do not complete ciglet or the three-package bindings.
