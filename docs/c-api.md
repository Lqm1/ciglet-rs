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

## DCT and phase checkpoint

Eighteen binary32/binary64 functions expose the native orthonormal DCT-II,
phase wrap/unwrap/directed difference, Cartesian magnitude/argument,
polar real/imaginary components and spectrum completion. The generated header
now declares 110 functions. Complex values use equally sized real/imaginary
planes rather than a Rust complex struct layout. Scalar complex aliases use
one-element planes. Polar functions retain the native f32 sine/cosine
approximations in both precisions. DCT retains its original cosine convention
and scaling; it rejects empty inputs. Empty phase arrays accept null pointers.

Spectrum completion accepts checked symmetry codes zero symmetric and one
antisymmetric, plus Nyquist codes zero Preserve and one Legacy. Other codes
return status three. Legacy keeps the original positive substitution of the
preceding bin even for antisymmetric completion. Preserve retains the supplied
DC/Nyquist values. The adapter stages a complete owned result before writing
caller storage. All vector outputs may overlap input, including in-place DCT
and spectrum completion. Matching component/output lengths are explicit;
length errors return status two. Native finite-value and integer-range checks
remain in force. Scalar and vector outputs remain unchanged on failure.

Six new integration tests compare all 24 original-C phase records per precision
and all 32 original-C DCT records per precision through actual extern functions.
Phase comparisons retain exact bits; DCT retains the established normalized
2e-7 binary32 and 1e-15 binary64 bounds. Additional checks cover both Nyquist
policies, odd/even fixture lengths, overlapping input/output, invalid option
codes, NaN/infinite values, directed phase integer-range rejection, empty
inputs and unchanged outputs after a later component fails validation.
No numerical core, C reference fixture or tolerance changed.

Windows MSVC x86_64/i686 and Windows GNU x86_64 Debug pass all 23 boundary
integration tests across four suites and independently compiled C callers
invoking all 110 exports. Both Windows x86_64 toolchains additionally pass
Python ctypes callers. The initial MSVC C caller compile failed because the
new NaN test lacked math.h; adding the standard header fixes compilation,
without a Rust change or weakened assertion. Warning-denied all-target Clippy
and Rustdoc, formatting, whitespace and generated-header verification pass.

Linux GNU x86_64/i686 Debug also pass all 23 tests and independently compiled
GCC C callers. Linux x86_64 passes Python ctypes calls. The runner completed
with exit zero; each target log independently has exact ten, four, three and
six passing summaries with zero failed/ignored tests. This verifies current
Debug boundaries on all five required targets. Linux/GNU Release and static
link callers remain pending.

The binding ledger maps 51 upstream symbol rows including fifteen DCT/phase
aliases. Every mapped export was checked against the generated header.
There are 281 pending C public/configuration rows, two nonpublic macro formal
parameters and 332 pending WASM rows. Remaining configuration/structure/ownership
audits, ciglet interfaces, both other packages' C interfaces and all three
WASM interfaces remain required. Current DCT/phase Release and Python32 caller
execution has not been verified. This checkpoint does not refresh the complete
native regression matrix or complete any package's bindings.

## Window and custom-callback checkpoint

Six functions expose all seven built-in windows and custom trigonometric
generation. There are 116 C exports. Built-in window codes are zero Boxcar,
one Hann, two Hamming, three Blackman, four Blackman-Harris, five Nuttall98
and six MLT sine; precision codes zero Accurate and one Approximate retain
native staging. Unknown codes return status three. Blackman retains accurate
cosine in both built-in modes, matching the source alias. Empty windows are
empty and singleton windows are unity, retaining documented native corrections.

Four custom functions cover every f32/f64 output and callback-result pairing.
The callback accepts a double angle, opaque void context and a pointer to its
f32/f64 result, returning a u32 status. Returning zero requires writing a finite
result; nonzero returns calculation status three. Pointer results avoid i686
floating-point return conventions. Option of an extern C function pointer
uses Rust's guaranteed nullable representation; the generated header directly
declares the matching C function pointer without a Rust struct or transmute.
No callback/context is stored beyond generation. Calls run serially on the
invoking thread. Null callbacks are accepted for empty, singleton and Boxcar
windows, which make no calls; other tapered windows require a callback.

Callbacks must return normally and must not unwind, make nonlocal jumps, retain
the temporary result pointer or access final output while generation is running.
Context may be null if the callback supports it. Any context mutation must meet
the callback's lifetime/exclusivity requirements. Context effects are not rolled
back on subsequent failure, while the adapter commits final output only after
complete success. Callback errors, nonfinite results, invalid options or rejected
lengths leave final output unchanged under these contracts. The outer Rust unwind
boundary does not catch a foreign exception or a panic crossing the callback's
non-unwinding C boundary. It remains the callback's responsibility to convert
errors to status codes. Python examples retain their CFUNCTYPE instances and
convert callback exceptions to nonzero status.

Custom Blackman honors the supplied callback, retaining the native correction
of the source macro's ignored argument. Custom C fixtures document their explicit
Blackman macro instantiation adapter; the upstream header remains unchanged.
Six integration tests compare all 112 original built-in records per precision
with the existing 1e-7/1e-15 absolute bounds, and all 60 custom records per output
precision with exact bits and callback counts. They cover all four mixed-precision
signatures, callback order, state retained after failure, nonfinite result rejection,
Blackman correction, empty/singleton cases and checked options/lengths. Native
algorithms, reference data and acceptance thresholds are unchanged.

Windows MSVC x86_64 Debug passes all 29 boundary tests across five suites and
actual optimized C/Python callers invoking all 116 exports. Python's initial
caller run failed on a missing math import; adding the standard-library import
fixes the caller without a Rust change or weakened test. Warning-denied Clippy
and Rustdoc, formatting, whitespace and generated-header verification pass.

Windows MSVC i686, Windows GNU x86_64 and Linux GNU x86_64/i686 Debug also
pass all 29 integration tests and independently compiled C callers invoking
every export, including all four callback/output precision pairings. Windows
GNU x86_64 and Linux x86_64 additionally pass Python callbacks. Every runner
completed with exit zero. Each Linux log independently contains the exact
ten, four, three, six and six passing summaries with zero failures/ignored
tests. This proves current boundary execution on all five required targets.

The ledger maps 70 upstream symbol rows, including all thirteen built-in aliases
and six custom macros. There are 262 pending public/configuration C rows, two
nonpublic macro formals and 332 pending WASM rows. Current window Release,
Python32 and static-link callers remain unverified. Remaining ciglet interfaces,
both other packages' C interfaces and all three WASM interfaces remain required;
symbol counts do not replace configuration/structure/ownership auditing. This
checkpoint does not refresh the entire native regression matrix.

Callback primary references:

- https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization
- https://doc.rust-lang.org/reference/items/functions.html#unwinding
- https://doc.rust-lang.org/std/primitive.fn.html
- https://docs.python.org/3/library/ctypes.html#callback-functions

## Chirp-Z checkpoint

Ten binary32/binary64 functions add one-shot chirp-Z transforms and reusable
plan construction/length/transform/release. The header now declares 126 functions
and six opaque types. Positive non-power-of-two lengths are supported within
the native padded-buffer bounds. Angular step is finite and has the selected
scalar precision. Direction zero is Forward; one negates the angular step and
divides the result by length. Other codes return status three. This inverse
direction inverts Forward on orthogonal DFT grids. On other grids it retains
the native normalized inverse-direction transform, rather than promising a
general matrix inverse. Grid and direction remain fixed for a reusable plan.

The shared Fourier planar adapter validates buffer ranges and mutually disjoint
outputs, represents omitted input planes by zeros, permits discarded output
planes and computes a complete owned result before committing output. Either
output may overlap input. No new input finite-value guard or numerical kernel
is introduced. Plans cache native chirps, convolution kernels, FFT plans and
work storage. A mutex serializes that mutable storage; calls still allocate
the owned result used for atomic output staging. The native transform_into
remains available to Rust callers without that output allocation.

Correctly typed live handles and library provenance are caller requirements.
All caller storage must be independent of private plan memory. Constructors
transfer ownership only on success and must not overwrite an unreleased owner.
Length is immutable. Concurrent calls require stable inputs and independent
writable outputs; release excludes all outstanding calls, clears the pointer
slot and invalidates every alias. A cleared-slot release succeeds. Observed
mutex poison returns status three without writing output; the poisoned object
can still be released. Poisoning is not a raw-pointer safety assumption. The
existing clearing Box release adapter now accepts any RefUnwindSafe plan type,
with its body and previous FFT behavior unchanged. Shared planar validation
and direction conversion gain private sibling-module visibility only.

Five new integration tests compare all 1,200 original-C cases per precision
through one-shot and twice-reused plans, retaining the existing normalized
5e-7 binary32 and 2e-15 binary64 bounds. The corpus includes ten lengths,
three signal patterns, five angular steps, all four component masks and both
directions. Additional checks cover non-power-of-two round trips on orthogonal
grids, overlapping input/output, output-plane intersection rejection, invalid
directions/nonfinite steps/phase overflow, retained constructor ownership,
failed-length output retention, immutable length, clearing release and four
scoped concurrent calls. An induced-poison unit test passes on Windows MSVC
x86_64 Debug; other-target poison-unit execution is not claimed.

Windows MSVC x86_64/i686 and Windows GNU x86_64 Debug pass all 34 boundary
integration tests across six suites and independently compiled optimized C
callers invoking all 126 exports. Both Windows x86_64 toolchains also pass
Python ctypes calls. Warning-denied Clippy/Rustdoc, formatting, whitespace
and generated-header verification pass. One test-only redundant binding
reported by Clippy was removed; no fixture, tolerance or assertion changed.
Native numerical implementations and dependencies are unchanged.

Linux GNU x86_64/i686 Debug also pass all 34 tests and independently compiled
GCC C callers; Linux x86_64 additionally passes Python ctypes calls. The Linux
runner completed with exit zero. Both target logs independently contain the
exact ten, four, five, three, six and six passing summaries with zero failed
and zero ignored tests. These results verify the current C boundaries on all
five required native targets.

The ledger maps 73 upstream rows including cig_czt/czt/iczt. Every mapped
export was checked against the generated declarations. There are 259 pending
public/configuration C rows, two nonpublic macro formal parameters and 332
pending WASM rows. Current chirp-Z Release, Python32 and static-link callers
remain unverified. Remaining ciglet interfaces, all liblrhsmm/SHIRO C interfaces
and all three browser/Node WASM interfaces remain required. Symbol counts do
not replace structure/configuration/ownership audits. This is current boundary
execution, not a refreshed complete native regression matrix or finished bindings.

## Frequency and spectral-distance checkpoint

Eighteen f32/f64 exports add frequency-to/from-Mel and Bark, equal-loudness
weighting, Mel-spaced grids, stable Itakura-Saito mean divergence and natural
log divergence with default or explicit Stable/Legacy policy. The header now
declares 144 functions. Scalar output pointers retain the existing i686 return
convention protection. Native conversion domains, scalar interpolation and
approximate Mel logarithm/exponential calculations remain unchanged; the adapter
does not substitute exact conversions or force mathematically exact endpoints.
Grids contain intervals+1 scalars. Counts must match explicitly; overflow or
count mismatch returns status two before writing. Native invalid bounds/domains
or zero intervals return status three. Output remains unchanged on failure.

Distance inputs must be matching nonempty finite nonnegative spectra. Stable
identical positive spectra return zero mean or negative-infinite log divergence.
A zero versus a positive bin returns positive infinity successfully; paired zeros
are undefined in Stable mode and return status three. Finite mean divergence
outside scalar range may also return positive infinity. Stable log divergence
retains finite extreme-ratio results. Policy code zero is Stable, one Legacy;
other codes return status three. Legacy retains the original selected-scalar
accumulation and approximate logarithm bits, including its source edge behavior.
The upstream itakura_saito symbol returns log divergence; its compatible binding
is log_itakura_saito_with with Legacy policy. The separately named mean function
follows the native Rust API. No output finite-value filter changes these semantics.

Distance buffers use the existing paired-scalar adapter. Input/output overlap
is permitted after input borrowing ends; shape, pointer and calculation errors
retain output. The private scalar-value adapter previously used by phase moves
to the shared C boundary without behavior changes. Existing phase callers use
that helper, avoiding a dependency from frequency conversion to phase internals.

Six new tests compare all twelve original C frequency records and five spacing
records per precision. Conversion bounds remain normalized 1e-7/1e-15, and grid
bits remain exact. All 53 original C distance records per precision retain exact
Legacy bits. Independent Stable checks cover analytic ratios, successful zero/
infinite outcomes, paired-zero rejection, extreme finite log divergence and
input/output overlap. Additional tests cover invalid modes/domains/lengths,
interval overflow, empty distance buffers, and retained output after rejection.
No native numerical source, reference fixture or acceptance bound changed.

Windows MSVC x86_64 Debug passes all forty boundary tests across seven suites
and independently compiled optimized C/Python callers invoking all 144 exports.
Warning-denied Clippy/Rustdoc, formatting, whitespace and generated-header
verification pass. The ledger has eighty implemented upstream rows, 252 pending
public/configuration C rows, two nonpublic macro formals and 332 pending WASM rows.
Every implemented mapping was checked against the generated declarations.
Current family Release, Python32 and static-link callers remain unverified.
Remaining ciglet interfaces, all liblrhsmm/SHIRO C interfaces and all three
browser/Node WASM interfaces remain required; symbol counts do not replace
structure/configuration/ownership audits. This checkpoint is current boundary
execution rather than a refreshed complete native regression matrix.

Windows MSVC i686, Windows GNU x86_64 and Linux GNU x86_64/i686 Debug also
pass all forty integration tests and actual independently compiled C callers
invoking every export. Windows GNU x86_64 and Linux x86_64 additionally pass
Python ctypes calls. All runners completed with exit zero. Each Linux log
independently contains exact ten, four, five, three, six, six and six passing
summaries with zero failed/ignored tests. Current C boundaries therefore pass
on all five required native targets.

## Frames and extrema checkpoint

Six f32/f64 functions add centered zero-padded frames, first global extrema
and strict local peaks/valleys. The generated header now declares 150 functions.
Frame centers use signed pointer-sized integers; output length determines the
frame size independently of input length. Empty inputs produce zeros, while
empty outputs accept null. Complete owned results are staged before output
commit, so input/output overlap is supported. Checked center arithmetic and
buffer-range errors retain output. No finite-value filter changes copied samples.

The frame audit found the native Rust function was still f32-only although the
original FP_TYPE header supports both precisions. fetch_frame is now generic
over the sealed Scalar trait, retaining its original f32 arithmetic and adding
f64 copies. Existing ordinary f32 calls infer their type; an untyped empty slice
needs an explicit scalar type as with other generic functions. It uses the existing
fallible vector reservation helper and rejects lengths beyond the legacy range
instead of using infallible Vec::with_capacity. Samples retain exact bits;
padding is positive zero. No resampling, STFT or other numerical kernel changes.

Search ranges are inclusive; direction code zero Maximum and one Minimum.
Unknown codes, invalid ranges and nonfinite inspected samples return status three.
Global searches preserve the first tie. Peak searches retain strict two-neighbor
comparisons and the start-index fallback when no candidate beats its value.
Endpoints are skipped as already corrected in the native API. Neighbor samples
outside the requested range can still be inspected; unrelated nonfinite samples
outside the inspected region remain permitted. Outputs are pointer-sized indices
and remain unchanged on error. Input borrowing ends before the index is written.

Four new boundary tests compare all 180 existing original C extremum records
per precision and 25 new original-header frame records per precision. The new
oracle only calls unchanged fetch_frame and covers odd/even/empty frames with
negative, internal and out-of-signal centers. Windows Zig generates both fixtures;
Linux GCC independently regenerates and verifies identical bytes. Three native
frame tests additionally cover f64 NaN payloads, signed zero/infinity, checked
arithmetic and rejected oversized allocation. The original three DSP regressions
remain unchanged except for an explicit f32 empty-slice type annotation. Extra
boundary cases prove supported overlap, atomic errors, fallback/ties and precise
nonfinite inspection behavior. No old fixture or acceptance threshold changed.

Windows MSVC x86_64 Debug passes 44 boundary tests across eight suites, six focused
native frame/DSP tests under default features, and actual optimized C/Python
callers invoking all 150 exports. Windows MSVC i686 and Windows GNU x86_64 Debug
pass the same 44 boundary tests plus six focused native tests with c-api enabled
and actual optimized C callers; GNU x86_64 also passes Python. Warning-denied
Clippy/Rustdoc, formatting, whitespace and cbindgen verification pass. The
reservation pattern follows stable Vec::try_reserve_exact documentation.

Linux GNU x86_64 and i686 Debug each pass the same 44 boundary tests and six
focused native tests with c-api enabled, followed by a library build and an
independently compiled optimized C caller invoking all 150 exports. Linux
x86_64 also passes the Python ctypes caller. The Linux runner completed with
exit zero; both logs independently contain exact 10,4,5,3,4,6,6,6,3,3 passing
counts, zero failed/ignored tests and no error diagnostic. The independent GCC
fixture regeneration also verifies both original-header frame fixtures.

The ledger maps 87 upstream rows including fetch_frame and all six search aliases.
There are 245 pending public/configuration C rows, two nonpublic macro formals and
332 pending WASM rows. Every mapped export was checked against the generated header.
Current family Release, Python32 and static-link callers remain unverified.
Remaining ciglet interfaces, all liblrhsmm/SHIRO C interfaces and all three WASM
interfaces remain required. The older complete native matrix predates this frame
precision/allocation correction; this focused checkpoint does not replace a final
complete native regression matrix. Native and binding inventories now record both
frame precisions. Symbol counts do not replace configuration/ownership auditing.

Reservation reference:

- https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.try_reserve_exact
