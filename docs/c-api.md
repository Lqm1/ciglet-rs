# C interface implementation status

The optional `c-api` feature exports a C interface from this existing package.
It does not create another crate. The native Rust implementation remains the
calculation backend; unsafe code is allowed only in `src/c_api.rs`.

This is the first binding checkpoint, not complete binding coverage. Currently
the ABI version and binary32/binary64 minimum/maximum operations are exported.
Binary32/binary64 first difference, cumulative sum and reverse are also exported.
Statistics and array generation exports are listed in the checkpoint below.
Selection/partition and stable sort with original indices are also exported.
Typed opaque binary32/binary64 Array2 objects are now available as described below.
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
The header exports functions and referenced opaque object types, without unrelated
Rust constants or the private Rust object layout.

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

Statistics selection/partition and stable sorting with original indices are now
bound, as documented below. The following checkpoint implements Array2 ownership,
dimensions, reshape and transpose. These additions do not complete ciglet or the
three-package bindings.

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

## Ordering checkpoint

Four more functions expose in-place rank selection and stable sorting with
original indices in both precisions. The interface now has 48 functions.
Selection stages the native partition in a separate allocation before copying
it back and writing the selected value. Its scalar output must not overlap the
partition output. Sorting takes separate explicit counts for input, sorted values
and size_t original indices. All counts must match. Either output may overlap
input; the two output regions must be disjoint. Overlapping output byte ranges
return status two before reading or writing, including ranges with different
element types. Empty sort accepts three null pointers; empty selection returns
an invalid-index error when its scalar output is otherwise valid.

Total ordering retains negative/positive NaNs, payload bits and signed zeros.
Stable sorting retains the original indices of duplicate values. Selection
guarantees the native total-order partition, not a particular order inside its
lower/upper partitions. Invalid indices, lengths, output overlap and allocation/
calculation failures leave every caller output unchanged. No new algorithm or
native numerical policy is introduced.

Windows MSVC x86_64/i686 and Windows GNU x86_64 Debug pass ten boundary tests
and independently compiled C callers for all 48 functions. Windows x86_64
MSVC Debug/Release and GNU Debug pass Python ctypes calls; MSVC Release passes
the same ten Rust tests and optimized C caller. C tests check both precisions'
signed zeros and exact positive/negative NaN payloads in sorting and selection.
Rust additionally checks permutation preservation, partial-order partitions,
overlapping outputs of different types and atomic failure behavior.
Linux GNU x86_64/i686 Debug also passes all ten boundary tests and independently
compiled GCC callers for all 48 exports; Linux x86_64 passes Python ctypes too.
Thus the updated boundary tests and real C callers pass on all five agreed
native targets. Cbindgen header verification, Clippy/Rustdoc with warnings denied,
formatting and whitespace checks pass. This does not replace the earlier full
native matrix or prove complete bindings. Other targets' Release callers and
32-bit Python remain unverified.

The binding inventory now maps 24 upstream rows, including all four ordering
aliases, to implemented C endpoints; 308 other public/configuration rows remain
pending for C. All 332 public rows remain pending for WASM. This completes the
current native statistics module's calculation endpoints through C, while the
rest of ciglet and the three-package binding objective remain unfinished.

## Owned array checkpoint

Thirty new functions, fifteen per precision, expose filled/zero construction,
row-major construction, checked row-table construction, independent cloning,
independent transposed copies, release, shape, flattened/row copies, element
reads/writes, bulk/row replacement, reshape and committed transpose. There are
now 78 C functions and two forward-declared opaque types. Row-table construction
accepts an explicit pointer and element count for each row, checks rectangular
shape before allocation, and retains native empty-table shape zero by zero.
Shape writes exactly three size_t values: rows, columns and total element count.
Array data retain all floating-point bit patterns without a DSP finite-value
restriction. Construction copies caller storage; it never adopts the caller's
allocator. Read/copy functions do not expose internal pointers. Bulk replacement
and row access cover native slice mutation without keeping a borrow across FFI.

Constructors return ownership through a pointer slot only after successful
construction. A caller must release any existing owner before overwriting its
slot with another successful constructor. Clone/transposed results have separate
allocations. Handles must come from this library and the correct precision's
constructor, remain alive throughout each call and never be accessed through
their private layout. A null or misaligned handle is rejected unless a null case
is expressly allowed; alignment checks cannot establish liveness or provenance.
Mutations/release require exclusive object access; reads permit no concurrent
mutation. Caller buffers and pointer slots must be independent of private object
storage. Release takes a pointer-to-handle, transfers sole ownership back to Rust,
and clears that slot. Releasing the same cleared slot again succeeds. Other
aliases become invalid and must not be used or freed. A null outer slot is an
invalid pointer; a null handle in a valid initialized slot is a successful no-op.

Core reshape validates before changing metadata. In-place transpose builds a
complete transposed array before replacing the old storage, retaining the
original on allocation/calculation errors or caught unwinds during preparation.
Copies and constructor output slots are committed only on success. Native
fallible array/vector allocation errors return status three; Box allocation
failure may abort under the standard allocator and is not caught. Trivial
float-array destruction invokes no user destructor or callback. Expected errors
leave existing handles, metadata, values and caller outputs unchanged.

All five agreed targets pass fourteen integration boundary tests across two
suites and actual C callers exercising all 78 exports. Windows MSVC x86_64/i686
Release also passes both suites and optimized C callers. Python ctypes calls
all exports and performs owned-array lifecycle checks on Windows MSVC x86_64
Debug/Release, Windows GNU x86_64 Debug and Linux GNU x86_64 Debug. The examples
use CDLL/C calling convention, c_void_p handle slots, byref outputs, typed row
pointer/count tables and finally cleanup. Python 32-bit and Linux/GNU Release
binding callers remain unverified. Tests cover independent copies, all row/bulk
operations, invalid lengths/coordinates/shapes, failed constructor slot retention,
ragged rows, zero-column arrays and exact special-value bits. Empty-array tests
also exercise usize::MAX by zero shape and zero-length null element buffers.

The ledger now maps 30 original symbol rows to implemented C endpoints, including
all six original array allocation/copy/flatten/free/reshape/transpose entries.
Other public/configuration rows (302) and all 332 public WASM rows remain pending.
That ledger must be supplemented by structure/configuration/ownership audits; it
is not an exhaustive binding-completion percentage. Core Rust algorithms and
historical model formats are unchanged by these wrappers. Final completed-binding
verification and the remaining three-package work are still required.

Additional primary references consulted for this checkpoint:

- https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw
- https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
- https://doc.rust-lang.org/nomicon/ffi.html
- https://docs.python.org/3/library/ctypes.html
- https://github.com/mozilla/cbindgen/blob/v0.29.4/docs.md

## Fourier checkpoint

Fourteen more endpoints provide binary32/binary64 one-shot FFT, direct inverse
DFT, fftshift and reusable FFT plan creation/length/transform/release. There are
92 functions and four opaque types. Direction is a checked u32 code: zero
Forward, one normalized Inverse, other values status three. FFT plans require
positive power-of-two lengths; direct inverse DFT retains any positive supported
length and its quadratic cost. Real/imaginary null inputs mean all zeros;
null output components are discarded. Present planes have the stated element
count, and outputs must be disjoint. Either output can overlap either input:
the complete owned complex result is computed before any caller output commit.
Odd fftshift retains the native right rotation by length/2 and supports in-place
buffers. No finite-value guard or new numerical kernel is introduced.

Plan handles reuse the native forward/inverse plans and scratch allocation.
Their mutex serializes scratch access; concurrent calls must still have stable
inputs and independently writable outputs. The mutex guard is acquired within
the unwind boundary. Observed poison returns status three rather than reusing
scratch state; poisoning is advisory and not used to justify raw-pointer safety.
An induced-poison unit test verifies unchanged output and successful release.
Release requires exclusive ownership and no outstanding plan calls, clears its
slot, and consumes the object even if destruction produces a caught unwind.
It must not be reused through other aliases. Metadata remains immutable; length
queries do not acquire the scratch lock. Computation failures retain caller
output; plan scratch is private and is not exposed as a caller-controlled buffer.

Windows MSVC x86_64/i686 and Windows GNU x86_64 Debug pass seventeen boundary
integration tests across three suites and actual C callers invoking all 92
exports. Windows MSVC x86_64 Release passes the same suites and optimized C/
Python callers. Windows x86_64 MSVC/GNU Debug also pass Python calls. Rust tests
cover exact four-point forward/inverse results, normalized non-power-of-two
inverse DFT, absent components, discarded outputs, odd shift, reused plans,
invalid directions/lengths, overlapping-output rejection, failed constructor
slot retention and four scoped concurrent transforms using one live plan.
The poison unit passes on Windows x86_64 MSVC Debug; other-target poison-unit
execution has not been claimed. Current i686 Release Fourier bindings are not
yet verified; the previous Release array checkpoint predates these additions.

Linux GNU x86_64/i686 Debug also pass all seventeen tests and independently
compiled GCC C callers invoking all 92 exports. Linux x86_64 additionally passes
the Python caller. The Linux runner completed with exit zero; both target logs
contain the exact ten, four and three passing test summaries with zero failures
and zero ignored tests. Linux Release and Python i686 remain unverified.

The ledger now maps 36 upstream rows, including all six Fourier aliases;
296 public/configuration rows remain pending C and all 332 remain pending WASM.
Counts do not replace the remaining configuration/structure/ownership audit.
This does not complete ciglet or either other package's interfaces, and it is
not a refreshed full native regression matrix. C/Python checked outputs retain
native normalization and ordering; historical model formats are unchanged.

Additional primary references:

- https://doc.rust-lang.org/std/sync/struct.Mutex.html
- https://doc.rust-lang.org/std/panic/trait.RefUnwindSafe.html
- https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html
