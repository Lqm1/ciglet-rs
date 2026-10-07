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

Statistics bindings still need selection/partition mutation and stable sorting
with original indices. Array2 ownership/dimensions/reshape/transpose also remain
required. These additions do not complete ciglet or the three-package bindings.

## Paired statistics checkpoint

Twelve additional functions expose dot product, default and explicit-policy
covariance/correlation, and cross correlation, each in both precisions. There
are now 44 exports including the ABI version. Both input lengths are explicit
and must match; mismatch returns status two before reading either input.
Input ranges may overlap. Scalar output or the cross-correlation output may
overlap either input because all input borrows end before output commit.
Cross-correlation output length selects the number of nonnegative lags and
retains the native zero-filled suffix when lags exceed the input length.
It intentionally retains the core's acceptance of nonfinite arithmetic; this
adapter does not add finite-value validation to dot/cross correlation.

Windows MSVC x86_64/i686 Debug pass eight boundary integration tests and actual
C callers exercising all 44 exports. Windows x86_64 Debug/Release also pass
Python calls, and Release passes the same eight Rust tests and optimized C
caller. Extreme binary64 input regression verifies Stable correlation remains
one when standalone covariance is unrepresentable and Legacy correlation
returns its existing error. Failure keeps the previous output. Additional
tests cover constant-vector correlation errors, unequal lengths, null inputs,
invalid policy values, identical input/output overlap, empty dot product,
empty cross correlation and zero lag count.
Linux GNU x86_64/i686 Debug subsequently pass the same eight integration tests
and real GCC-compiled C callers linked to the actual shared libraries. Linux
x86_64 also passes Python ctypes calls for all 44 exports. Linux 32-bit Python
and Linux Release binding callers remain unverified.
Windows GNU x86_64 Debug also passes eight integration tests and independent
C/Python callers for all exports. Together with Windows MSVC x86_64/i686, all
five agreed native targets now execute these current C bindings successfully.
This is binding-boundary verification, not a rerun of the full native test matrix
or proof of complete binding coverage. Cbindgen header verification, Clippy and
Rustdoc with warnings denied, formatting and whitespace checks pass.

`binding-inventory.csv` retains every row from the two native inventories.
It maps 20 upstream symbols, including aliases, to these verified C endpoints;
312 other public/configuration rows remain pending for C and all 332 public rows
remain pending for WASM. Two macro formal-parameter rows are explicitly nonpublic.
The native status column preserves the existing Rust implementation audit;
binding status does not replace or infer that audit. Export count and upstream
row count differ because precision/policy variants and aliases are explicit.
