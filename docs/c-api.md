# C interface implementation status

The optional `c-api` feature exports a C interface from this existing package.
It does not create another crate. The native Rust implementation remains the
calculation backend; unsafe code is confined to `src/c_api.rs` and its binding
modules under `src/c_api/`.

Binding coverage is tracked in `docs/binding-inventory.csv`. The current header
exports 565 status-returning functions, 26 opaque types, 24 concrete descriptors
and seventeen constants. The checkpoints below record implementation and verification
evidence. Of 332 public upstream mappings, 311 have verified C bindings and 21
remain pending. All bundled approximation functions have verified C bindings. Liblrhsmm and SHIRO
bindings and all browser/Node WebAssembly bindings remain required work.

Build with `cargo build --locked --features c-api --lib`. Cargo emits the Rust
library, shared library and static library. Regenerate the checked-in header
with cbindgen 0.29.4:

```text
cbindgen --config cbindgen.toml --crate ciglet-rs --output include/ciglet_rs.h
```

The generated header is specifically for a `c-api` build. Cbindgen currently
reports unmapped feature conditions and unrelated private/generic Rust items;
the generated declarations are checked by compiling an independent C caller.
The header exports functions, referenced opaque objects, C-layout descriptors and
named binding constants without exposing private Rust object layouts.

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

## Refreshed native regression after frame precision correction

The complete default-feature Debug matrix at ciglet 70ac0fc, liblrhsmm f9bf303
and SHIRO 842e479 passes on all five agreed targets. Each target has 198, 120
and 58 passing tests respectively, with zero failed or ignored tests and all
ignored tests explicitly included. All three runner processes exit zero.
Both compile-fail liblrhsmm doctests succeed with expected E0502 diagnostics;
these intentionally printed errors are not build failures. Exact package
totals and summary counts are checked in all fifteen logs. This refresh includes
the generic/fallible native frame change, while C boundary execution remains
the separate feature-enabled evidence above. It does not prove later bindings
or Release execution of the frame correction.

## Band cepstrum interfaces

Six f32/f64 functions expose selected band-to-cepstrum coefficients, the complete
legacy band-sized buffer and flattened frame-major band spectrograms. The header
now declares 156 functions and retains six opaque types. Inputs are already
band features, including signed log-band values; these calls apply no extra log.
Order clamps to band count minus one. include_dc accepts only zero or one;
requested non-DC coefficients precede the appended DC coefficient.

The selected output count is min(order, bands-1)+include_dc. Legacy output count
equals the full band count, preserving the original untouched DCT suffix. Matrix
output count is rows times selected count; input must divide exactly into positive
bounded bands. Empty matrices accept null buffers but still require valid bands.
A zero selected count still executes native validation, rejecting nonfinite
features rather than silently succeeding. Unknown options, invalid native shapes
or nonfinite data return status three. Output count mismatch returns status two.
Complete staged results preserve output after every failure, including errors
in a later matrix frame, and allow supported input/output overlap.

The adapter calls the existing generic native cepstrum functions without changing
their numerical kernels. Four new boundary tests compare all 48 original CEP1
records per precision, each with three frames, and consume the complete duplicated
original matrix. Full buffers, selected prefixes and matrix results retain the
existing normalized bounds 1e-7 and 1e-15. Extra cases cover in-place output,
clamped order, zero coefficients, invalid selectors/shapes, empty matrices and
nonfinite rejection with atomic output retention. MSVC x86_64 Debug passes these
four tests. Warning-denied all-target Clippy and Rustdoc, formatting, whitespace
and generated-header verification pass. All five required targets pass 48 C
boundary integration tests across nine suites and six existing frame/DSP tests
with c-api enabled. Actual optimized, assert-enabled C callers invoke all 156
exports on each target; MSVC x86_64, Windows GNU x86_64 and Linux x86_64 also pass
Python ctypes calls. All corrected runners terminate with exit zero. Each log
independently has exact 10,4,4,5,3,4,6,6,6,3,3 passing counts and zero failed or
ignored tests. No old fixture or tolerance changed.

The initial C/Python caller incorrectly expected exact DC value five for bands
1,2,3,4. Strict unchanged original cos_2(0), with contraction disabled as in
the reference fixtures, is 0.99998641014099121; the binary64 DC value is
4.999932050704956. Only that caller expectation was corrected. The four original
fixture boundary tests and their numerical bounds already passed unchanged.

The investigation also found optimized Zig defines NDEBUG. Earlier optimized
Windows C smoke runs therefore did not establish their asserted function calls
or checks, despite successful process exits. Those historical Windows C caller
coverage claims must not be counted as execution evidence. Rust boundary tests
and Python ctypes evidence are unaffected. The C caller now undefines NDEBUG
before including assert.h, so optimized builds execute all checks. Current
five-target caller reruns pass and cover all 156 exports, including earlier
families, establishing current execution proof with assertions enabled.

The ledger retains 334 rows: 89 implemented C mappings, 243 pending public or
configuration C rows, two nonpublic macro formals and all 332 public WASM rows
pending. Both be2cc variants and be2ccgm now map the typed interfaces. Every
implemented mapping was checked against the generated header. Current family
Release, Python32 and static-link callers remain unverified. Remaining ciglet
C interfaces, all liblrhsmm/SHIRO C interfaces and all three WASM interfaces
remain required. The complete native matrix above predates only these opt-in
boundary changes, which modify no native numerical source. Symbol counts do
not replace configuration, ownership or full functionality auditing.

## Filterbank interfaces

Thirty f32/f64 functions expose empty, explicit Mel, default Mel and PLP
constructors, cloning, clearing release, shape, Nyquist metadata, response/support
copies, exclusive channel/metadata mutation and single/inferred/explicit-frame
application. The header declares 186 functions and eight opaque types. Handles
own native FilterBank values; no native numerical kernel or dependency changes.

Shapes are copied as bins/channels; supports are half-open start/end ranges.
Both require exactly two pointer-sized outputs. Responses require exactly bins
scalars. Channels retain complete responses, including signed weights and
nonfinite values outside active supports. Active response values must be finite.
Nyquist metadata can retain nonfinite values and does not change application.
Only inspected spectrum bins require finite values. Compression zero is native
Logarithmic, one native PLP; unknown codes return status three. The original
approximate log and PLP exponent 0.33 remain unchanged.

Single applications require bins input samples and channels outputs. Inferred
spectrograms require positive bins and whole flattened frames. Explicit frames
support zero-bin frames and validate the supplied frame count. Output count
mismatches return status two; invalid native shapes/supports/channels or data
return three. Empty buffers accept null. Complete owned results stage before
commit, so applications permit input/output overlap and retain caller output
after a later-frame error. Failed constructors retain their old output slot.
Release clears sole ownership and accepts a null stored handle repeatedly.

Live correctly typed handles must remain immutable during reads, cloning or
application; mutations and release require exclusive access. Buffers/owner slots
must be independent of the handle and all its Rust-owned storage. Pointer checks
cannot prove these lifetime, allocation or exclusion contracts. Clone uses the
native Clone implementation; allocator-abort limitations remain as documented.

Six new host boundary tests compare every original generated-bank record, 72
per precision, and every original custom-bank record, 33 per precision, through
all typed functions. Responses/supports, compressed single frames and repeated
spectrogram records are consumed completely. Generated response/output bounds
remain 2e-7/2e-14 and 2e-6/2e-13; custom response equality and 2e-7/2e-15 output
bounds remain unchanged. Inert metadata and independently cloned banks are
verified against the same outputs. Additional cases cover NaN payload copies,
inactive nonfinite bins, supported overlap, retained output after later-frame
failure, invalid option/count/support errors, failed owner replacement, zero-bin
explicit frames, zero-channel applications and idempotent clearing release.

The first custom comparator treated identical infinities as a numerical
difference; exact equality now handles them before finite difference testing.
No implementation, fixture or numerical threshold was changed for that test
failure. Host tests and warning-denied all-target Clippy/Rustdoc, formatting,
whitespace and cbindgen verification pass. All five required targets pass 54
boundary integration tests across ten suites and six focused frame/DSP tests
with c-api enabled. Each log independently contains exact
10,4,4,5,6,3,4,6,6,6,3,3 passing counts with zero failed or ignored tests and no
error diagnostic. All runners complete with exit zero. Independently compiled
optimized, assert-enabled C callers invoke all 186 exports on all five targets;
MSVC x86_64, Windows GNU x86_64 and Linux x86_64 also pass Python ctypes calls.
The expanded optimized C caller references every generated declaration.

The ledger retains 334 rows: 101 implemented C source mappings, 231 pending
public/configuration C rows, two nonpublic macro formals and 332 public WASM rows
pending. All twelve filterbank source functions/aliases now map typed entry
points, and every implemented mapping was checked against the header. Current
family Release, Python32 and static-link callers remain unverified. Remaining
ciglet C interfaces, all liblrhsmm/SHIRO C interfaces and all three WASM
interfaces remain required. No native numerical source or original fixture was
changed. This boundary checkpoint does not replace final combined acceptance;
symbol counts do not substitute for configuration/ownership auditing.

## Real cepstrum interfaces

Twenty-two f32/f64 functions expose owned reusable plans, clearing release,
length/bin shape, log analysis/reconstruction, minimum-phase angles, magnitude
spectrogram/cepstral matrices and three modern one-shot conveniences. The header
declares 208 functions and ten opaque types. Plan lengths remain positive powers
of two within the native legacy bound. Shape contains length and bins; positive
length implies the native is_empty result is always false.

Nyquist code zero Preserve retains supplied data, one Legacy substitutes the
source neighboring bin. Lifter code zero Causal selects the corrected lifter,
one Legacy retains doubled Nyquist and the first uncleared negative-quefrency
sample. Unknown codes return status three. Analysis and minimum-phase inputs
are already logarithmic; neither applies a log. Reconstruction mirrors the
native used prefix, ignores the unused suffix and returns full-grid logs.
One-shots from_log_spectrum and minimum_phase_default infer length=2*(bins-1)
and need at least two bins. Their defaults Preserve/Causal match native Rust.
to_log_spectrum uses the explicit output count as the transform length.

Magnitude matrices use original approximate log(magnitude+1e-15); reverse
matrices use original approximate exp. Explicit reverse frame counts represent
zero-coefficient rows. Zero selected coefficients still validate used magnitude
bins. Native policies inspect only used values, including ignored Legacy
Nyquist/suffix samples. Empty matrices accept null buffers. Invalid native
input shapes return status three; explicit output mismatches or checked output
count overflow return two. Complete owned results retain all output on error,
including a later frame, and permit supported input/output overlap.

Typed handles own Mutex-protected Plan scratch. Calls serialize computation;
caller inputs must remain stable while waiting. Immutable length/bin queries
do not lock. Expected native errors release the guard and permit reuse.
Observed poisoning returns three, without treating poisoning as a memory
safety condition. The guard is inside the unwind boundary; release must exclude
all outstanding calls and transfer sole ownership exactly once. Buffers and
owner slots must remain independent of handles and Rust-owned storage.

Six new host boundary tests consume all 72 original RCP1 records per precision,
reusing each plan across every operation and all seven original comparisons.
Their unchanged f32 bounds are 3e-8,1.2e-7,2e-7,1.6e-7,7e-9,0,2.5e-6; f64 bounds
are 5e-17,3e-16,3e-16,3e-16,3e-17,0,0. Independent analytic round-trip and phase
tests retain 1e-15 accuracy. Both Nyquist policies and both lifters are exercised,
including the independent uncleared-sample difference. Additional cases prove
ignored NaNs, supported overlap, late-frame atomic failure, overflow/recovery,
invalid options/shapes/counts, owner retention after failed creation, singleton
plans, empty matrices, zero coefficients and four concurrent shared-plan calls.
The induced-poison unit test passes on MSVC x86_64 Debug and confirms unchanged
output plus clearing release. No native numerical implementation, old fixture,
dependency or acceptance threshold changed.

Host checks and warning-denied all-target Clippy/Rustdoc, formatting, whitespace
and generated-header verification pass. All five required targets pass 60
boundary integration tests across eleven suites and six focused frame/DSP tests
with c-api enabled. Every log independently has exact
10,4,4,5,6,3,4,6,6,6,6,3,3 passing counts, zero failed/ignored and no error
diagnostics. All runners terminate with exit zero. Independently compiled
optimized, assert-enabled C callers invoke all 208 exports on every target;
MSVC x86_64, Windows GNU x86_64 and Linux x86_64 also pass Python ctypes calls.
The expanded optimized C caller references every generated declaration.

The 334-row ledger has 106 implemented C mappings, 226 pending public or
configuration C rows, two nonpublic macro formals and 332 public WASM rows
pending. All five real-cepstrum source symbols now map typed operations, including
modern conveniences and explicit legacy selections. Every implemented mapping
was checked against the header. Current family Release, Python32 and static-link
callers remain unverified. Remaining ciglet C interfaces, all liblrhsmm/SHIRO C
interfaces and all three WASM interfaces remain required. This opt-in boundary
checkpoint changes no native numerical implementation, original fixture or
threshold and does not replace final combined acceptance. Symbol counts do not
substitute for configuration, scratch, ownership or full functionality audits.

## Polynomial interfaces

Eight f32/f64 functions expose descending-order complex/real polynomial
evaluation and complex/real root estimates. The header declares 216 functions;
the ten existing opaque types are unchanged. Complex values use separate scalar
planes, so no Rust Complex layout crosses the ABI. Missing complex coefficient
planes represent zero components; nonempty real-only inputs require a readable
pointer. Both evaluation scalar outputs are required and disjoint.

Roots require exactly coefficient_count-1 elements in each output plane. Constant
nonzero polynomials succeed with zero roots, accepting null root planes, and
report iterations zero, converged one and residual zero. Diagnostics use separate
required usize/u32/f64 pointers, avoiding a new structure-layout dependency.
Nonconvergence is a successful native estimate with converged zero, not an
error that discards useful roots. All original initialization/iteration order,
radius clamp, 50-step limit, approximate trigonometry and residual threshold
remain unchanged. Evaluation retains original power accumulation and does not
regroup arithmetic into Horner form.

All five root output ranges, including diagnostics, must be pairwise disjoint;
overlap and output count errors return status two. Invalid native coefficients,
arguments, zero leading coefficients or numerical failures return three.
Input/output overlap is permitted only after all input borrowing ends. Complete
owned estimates stage every output before committing, retaining root planes
and diagnostics together on failure. Pointer checks cannot prove allocation,
initialization, stable input or exclusive writable storage.

Six new host boundary tests consume all original POL1 records, 32 for f32 and
33 for f64, including complete trailing trigonometric data. Each contains three
complex evaluations and roots; eligible real-coefficient records also exercise
real APIs and exact report equality. Original value/root results retain exact
equality. Additional cases prove constant and imaginary-only polynomials, absent
complex planes, imaginary roots, supported overlap, invalid/nonfinite inputs,
all ten pairwise output intersections and atomic retention. A 50-iteration
nonconvergent example exposes all diagnostics; independently evaluated product
of (x-k), k=1..8, confirms the reported residual within the unchanged 1e-8 check.

Initial Clippy redundant-field/duplicate-branch diagnostics were corrected with
field shorthand and one equivalent pointer-check condition. A newly tightened
case-count assertion initially assumed both fixtures had 32 records; the
unchanged f64 fixture header establishes 33, now checked explicitly. No native
algorithm, fixture or numerical criterion changed. Host tests, warning-denied
all-target Clippy/Rustdoc, formatting, whitespace and generated-header
verification pass. All five required targets pass 66 boundary integration tests
across twelve suites and six focused frame/DSP tests with c-api enabled. Each
log independently contains exact 10,4,4,5,6,3,4,6,6,6,6,6,3,3 passing counts,
zero failed/ignored tests and no error diagnostic. All runners terminate with
exit zero. Actual optimized assert-enabled C callers invoke all 216 exports
on every target; MSVC x86_64, Windows GNU x86_64 and Linux x86_64 also pass Python
ctypes callers. Expanded optimized C source references every declaration.

The ledger retains 334 rows: 112 implemented C source mappings, 220 pending
public/configuration C rows, two nonpublic macro formals and 332 public WASM
rows pending. All six polynomial source functions/aliases now map typed entry
points, and every implemented mapping was checked against the header. Current
family Release, Python32 and static-link callers remain unverified. Remaining
ciglet C interfaces, all liblrhsmm/SHIRO C interfaces and all three WASM
interfaces remain required. This opt-in boundary adds no native numerical
changes and does not replace final combined acceptance. Symbol counts do not
substitute for complete diagnostics, ownership or configuration auditing.

## Standalone prediction recursion and gain

Four f32/f64 entry points expose Levinson recursion and gain with explicit
arithmetic selectors: zero selects Stable, one selects Legacy. Unknown selectors
return three. Recursion requires equal input/output counts; gain requires matching
coefficient/correlation counts. Count errors return two. Native domain and
numerical errors return three, including empty recursion. Successful silence
retains the native leading unit coefficient and zero gain. Scalar gain uses an
output pointer rather than a floating return value across different C ABIs.

Both operations stage owned results before committing. Failures preserve output,
and input/output overlap is supported after input borrowing ends. Caller storage
must remain initialized and stable for reads and exclusively writable for output;
pointer/range checks cannot establish allocation validity or these lifetimes.

Three host boundary tests pass, consuming all 64 original LPC1 records in each
precision. Legacy recursion and gain match the original bits. Stable recursion
uses independent normalized Toeplitz residual bounds of 2e-7 for f32 and 5e-15
for f64. Additional cases cover extreme finite values, subnormal correlations,
compensated gain products, silence, overlap, invalid policies and atomic errors.
Warning-denied all-target Clippy passes. No native numerical kernel, original
fixture or threshold changes. The generated header now declares 220 functions
with ten opaque types. All five required targets pass 69 boundary integration
tests across thirteen suites plus six focused frame/DSP tests, with c-api enabled.
Each log independently verifies exact 10,4,4,5,6,3,4,6,6,6,3,6,6,3,3 passing counts
and no failed/ignored tests or error diagnostics. Windows MSVC x86_64/i686,
Windows GNU x86_64 and Linux GNU x86_64/i686 runners all terminate with exit zero.
Actual optimized assert-enabled C callers invoke all 220 exports on every target;
Python ctypes callers pass on MSVC x86_64, Windows GNU x86_64 and Linux x86_64.
Expanded optimized MSVC C source independently references every declaration.
Warning-denied Rustdoc, formatting, whitespace and generated-header verification
also pass. Current family Release, Python32 and static-link callers remain
unverified; this boundary checkpoint does not replace final combined acceptance.

The ledger retains 334 rows: 115 implemented C source mappings, 217 pending
public/configuration C rows, two nonpublic macro formals and 332 pending public
WASM rows. The cig_levinson/levinson aliases and lpgain now map explicit typed
policy entry points. Every implemented mapping was checked against the header.

This section covers standalone recursion/gain only. Scaled waveform estimation
and complete scaled correlation/gain access are described below. Ordinary sample
and magnitude estimation, reusable spectrum plans, spectra and resonance reports
remain required C interfaces. The corresponding native APIs already exist; no
function is excluded from the accepted port scope. All three WASM interfaces and
the remaining liblrhsmm/SHIRO C interfaces remain required.

## Scaled prediction owners

Eighteen f32/f64 functions expose immutable owned waveform predictions with
individually scaled powers. Construction publishes an owner without requiring
ordinary correlations or scalar gain to be representable. Each precision has
construction, clone, clearing release, length, copied coefficients, copied lag
parts, gain parts, scalar gain and checked ordinary conversion. The header now
declares 238 functions and twelve opaque types. These are additions within the
existing single crate; they do not change the native numerical implementation.

Lag and gain parts use f64 significands and i32 binary exponents, with nonzero
absolute significands in [1,2) and zero represented as (0,0). Ordinary conversion
clones the estimate before consuming that clone. Underflow or overflow therefore
retains the successfully computed original scaled estimate and both output
arrays. A failed scalar-gain call likewise retains its output while gain parts
and coefficients remain available. Cloning follows native allocation behavior;
the existing abort/OOM limitations still apply.

Opaque owners support concurrent immutable reads while release is excluded.
Slots must be initialized and exclusively writable. Successful replacement does
not release an older slot value; callers must retain and release that ownership.
Release clears its slot before dropping the owner and accepts initialized null
slots. Distinct copied output ranges must be pairwise disjoint and independent
of every owner allocation. Failures retain outputs and existing ownership.

Six new host boundary tests pass. Each precision consumes all 64 independent
SWV1 waveform records, comparing all 176 scaled lags with the unchanged 2e-15
bound and coefficients with the unchanged 2e-15/5e-15 bounds. Additional cases
prove maximum/subnormal inputs, checked ordinary-conversion failures with later
successful reads, independent clone lifetime, failed-constructor owner retention,
silence, invalid shapes/pointers, output overlap and a nonrepresentable scalar
gain whose scaled gain remains available. Warning-denied all-target Clippy
passes. Original fixtures, thresholds and native kernels remain unchanged.
The initial Linux x86_64 extreme-value test exposed a test-reconstruction issue:
the runtime computes 2.0.powi(-1074) as zero while split exponent multiplication
produces the correct smallest subnormal, bits one. A separate probe confirms both
paths and identical scaling at exponent 1023. The corrected test splits scaling
into two powers; gain equality, all original fixtures and bounds remain unchanged.

All five required targets pass 75 boundary integration tests across fourteen
suites plus six focused frame/DSP tests, with c-api enabled. Each log independently
verifies exact 10,4,4,5,6,3,4,6,6,6,3,6,6,6,3,3 passing counts, zero failed/ignored
tests and no error diagnostics. Corrected Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686 runners all terminate with exit zero. Actual
optimized assert-enabled C callers invoke all 238 exports on every target; Python
ctypes callers pass on MSVC x86_64, Windows GNU x86_64 and Linux x86_64. Expanded
optimized MSVC C source independently references every declaration. Warning-denied
Rustdoc/Clippy, formatting, whitespace and generated-header verification pass.

The source ledger remains 115 implemented C mappings and 217 pending public C
rows; these scaled owners extend public Rust improvements while original lpc
still needs its ordinary waveform interface. All 332 public WASM rows remain
pending. Current family Release, Python32 and static-link callers remain
unverified. Magnitude estimation/plans, ordinary waveform estimation, spectra and
resonance reports remain required, alongside the other pending ciglet functions,
all liblrhsmm/SHIRO C interfaces and all three WASM interfaces. This checkpoint
does not replace final combined acceptance or reduce the complete public scope.

## Reusable prediction spectrum plans

Twelve f32/f64 functions expose native SpectrumPlan construction, clearing
release, two-value shape, ordinary/scaled magnitude estimation and full-grid
prediction spectra. The generated header now declares 256 functions with fourteen
opaque types. Shape reports transform length and maximum half-spectrum bins.
Native nonempty FFT restrictions apply, including singleton plans; all valid
owners are nonempty. Mutable transform/workspace access is internally serialized
with Mutex. Release excludes all outstanding users, and output storage must be
independent of every owner allocation.

Estimation retains both Nyquist/arithmetic selectors and accepts native bounded
prefix spectra. Legacy discards a Nyquist value only for a full half-spectrum,
so an inactive nonfinite value can be ignored for a four-point plan but rejected
for a longer plan. Ordinary coefficients and correlations commit atomically.
Scaled estimation publishes an independently owned result without requiring
ordinary conversion or scalar gain. Complete owned staging permits input/output
overlap after borrowing ends. Constructor failures retain existing owners;
successful slot replacement leaves older ownership the caller's responsibility.

Spectra return the complete native FFT grid, preserving signed gain and negative
zero. Native exceptional-bin scaled response evaluation handles large/tiny
polynomials without normalizing away small coefficients. A pole on the grid
returns a native error while retaining output. Native errors do not poison the
mutex, and later successful calls can reuse the plan. An observed poisoned mutex
returns status three; metadata remains readable and owners remain releasable.

Five new host integration tests pass. All 48 LPS1 original records per precision
compare correlations, coefficients, gain, isolated spectra and combined spectra
with unchanged f32 bounds 2e-8/3e-8/0/7e-8/8e-8 and f64 bounds
5e-17/2e-15/4e-16/2e-15/3e-15. Extra cases cover repeated estimates, both policies,
full/prefix Nyquist distinctions, maximum/subnormal spectra and scaled estimates,
signed negative zero, singleton plans, failed constructors, pole errors/recovery,
overlap and four concurrent callers sharing scratch. A separate host unit test
induces poison and verifies atomic ordinary/scaled/spectrum failure, readable
shape and clearing release. Warning-denied all-target Clippy passes. No native
numerical kernel, original fixture or threshold changes.

All five required targets pass 86 boundary integration tests across sixteen
suites plus six focused frame/DSP tests. Each log independently verifies exact
10,4,4,5,6,3,4,6,6,6,3,6,6,5,6,6,3,3 passing counts, zero failed/ignored tests and
no error diagnostics. MSVC x86_64/i686, Windows GNU x86_64 and Linux GNU
x86_64/i686 runners terminate with exit zero. Actual optimized assert-enabled C
callers invoke all 256 exports on every target; Python ctypes callers pass on
the three x86_64 environments. Expanded optimized MSVC C source references every
declaration. Warning-denied Rustdoc, formatting, whitespace and generated-header
verification pass. The induced-poison unit proof remains host MSVC x86_64 only.

The ledger has 118 implemented C source mappings, 214 pending public C rows,
two nonpublic macro formals and 332 pending public WASM rows. flpc now includes
both one-shot policies and reusable ordinary/scaled estimation; lpspec maps
complete spectrum evaluation with reusable ownership and shape. Every implemented
mapping was checked against the header. Current family Release, Python32 and
static-link callers remain unverified. Resonance reports and all other remaining
ciglet interfaces, liblrhsmm/SHIRO C interfaces and all three WASM interfaces remain
required. This evidence does not replace final combined acceptance or reduce scope.

## Complete resonance reports

Two f32/f64 functions expose sorted normalized frequencies, complete complex
poles and native root iterations, convergence and maximum residual. Frequency
filtering remains strictly 0.001..0.999 Nyquist. Complex poles use separate real
and imaginary planes; no Rust complex layout crosses the ABI. The header now
declares 258 functions with fourteen opaque types.

All three result planes use a caller capacity; a required usize output reports
the populated prefix. Unused capacity remains unchanged. Constants can have
zero capacity and null planes, reporting count/iterations/residual zero and
convergence one. Insufficient capacity returns two after staging, retaining every
output rather than publishing partial results. Nonconvergence succeeds with the
complete native estimate and diagnostics, even when filtering leaves no poles.

All seven output ranges, including mixed usize/u32/f64 scalars, are validated
pairwise. Scalar alignment is checked with each original type before byte-range
intersection checks. Overlap returns two. Complete owned staging ends all input
borrowing before committing and supports input/output overlap. Pointer checks
cannot establish allocation validity, initialized stable input or exclusive
writable storage; callers retain those obligations.

Six host tests pass. Both precisions consume all eleven LPS1 original resonance
records and compare frequency/pole bits exactly, retaining unused capacity.
Diagnostics agree with the existing native root report. Other cases cover an
analytic imaginary pole, filtered real poles, zero-capacity constants, empty and
invalid coefficients, insufficient capacity, atomic output retention, supported
input overlap, all 21 output-range pairs in each precision and a partial f64/u32
intersection. A 50-iteration unconverged estimate succeeds with complete
diagnostics matching the separately exposed polynomial root API. Sorted retained
poles belong to that root report; unused capacity is preserved. No native kernel,
original fixture or threshold changes. Warning-denied all-target Clippy passes.

All five required targets pass 92 boundary integration tests across seventeen
suites plus six focused frame/DSP tests. Each log independently verifies exact
10,4,4,5,6,3,4,6,6,6,3,6,6,6,5,6,6,3,3 passing counts, zero failed/ignored tests and
no error diagnostics. MSVC x86_64/i686, Windows GNU x86_64 and Linux GNU
x86_64/i686 runners terminate with exit zero. Actual optimized assert-enabled C
callers invoke all 258 exports on every target; Python ctypes callers pass on
the three x86_64 environments. Expanded optimized MSVC C source references every
declaration. Warning-denied Rustdoc/Clippy, formatting, whitespace and
generated-header verification pass.

The ledger has 119 implemented C source mappings, 213 pending public C rows,
two nonpublic macro formals and 332 pending public WASM rows. lpresf now maps
complete frequency/pole estimates and all convergence diagnostics, and every
implemented mapping was checked against the header. Current family Release,
Python32 and static-link callers remain unverified. All remaining ciglet C
functionality, liblrhsmm/SHIRO C interfaces and all three WASM interfaces remain
required. This checkpoint does not replace final combined acceptance or reduce scope.

## One-shot waveform and magnitude estimates

Six f32/f64 entry points expose complete ordinary waveform/magnitude estimates
and owned scaled magnitude estimates. Ordinary outputs include both native
coefficients and autocorrelation, each with exactly order+1 elements. Checked
count overflow, mismatched counts and overlapping output planes return two.
All results are staged together, preserving both arrays on failure and allowing
input/output overlap after input borrowing ends. Arithmetic zero selects Stable,
one selects Legacy; magnitude Nyquist zero selects Preserve, one selects Legacy.
Unknown selectors return three. All selectors reuse the native numerical paths.

One-shot magnitudes retain source transform sizing and the exact native rule for
discarding a Legacy Nyquist bin. Nonfinite values in a discarded bin remain
ignored; Preserve validates them. Scaled magnitude construction publishes the
existing typed scaled owner without attempting gain or ordinary conversion.
Extreme estimates therefore remain available even when ordinary correlations
overflow or underflow. Owner-slot and native allocation contracts remain those
documented for scaled waveform owners. The header now declares 244 functions
with twelve opaque types.

Six host tests pass. Both precisions consume all 64 original LPC1 waveform
records, comparing coefficient/correlation bits exactly under Legacy. All 48
LPS1 magnitude records per precision preserve the existing correlation/coefficient
bounds of 2e-8/3e-8 for f32 and 5e-17/2e-15 for f64. Separate resonance records
are consumed without claiming resonance ABI coverage. Independent cases cover
all arithmetic/Nyquist combinations, isolated Nyquist energy, ignored NaNs,
input/output overlap, unknown selectors, count overflow, overlapping outputs,
empty input, maximum/subnormal scaled magnitudes, conversion failures and owner
retention. Initial test compilation required destructuring scalar references
before conversion; no numerical behavior or acceptance criterion changed.
Warning-denied all-target Clippy passes. All five required targets pass 81
boundary integration tests across fifteen suites plus six focused frame/DSP
tests. Each log independently verifies exact 10,4,4,5,6,3,4,6,6,6,3,6,6,6,6,3,3
passing counts, zero failed/ignored tests and no error diagnostics. MSVC
x86_64/i686, Windows GNU x86_64 and Linux GNU x86_64/i686 runners terminate with
exit zero. Actual optimized assert-enabled C callers invoke all 244 exports on
every target; Python ctypes callers pass on the three x86_64 environments.
Expanded optimized MSVC C source references every declaration. Warning-denied
Rustdoc, formatting, whitespace and generated-header verification pass.

The ledger has 116 implemented C source mappings, 216 pending public C rows,
two nonpublic macro formals and 332 pending public WASM rows. The lpc mapping
includes both ordinary/scaled waveform construction and every scaled-owner
access operation. flpc remains pending because its native mapping also includes
reusable SpectrumPlan::estimate_scaled, which is not yet exposed. Current family
Release, Python32 and static-link callers remain unverified. Reusable spectrum
plans, full prediction spectra and complete resonance reports remain required,
alongside all remaining binding work in the three crates. This boundary evidence
does not replace final combined acceptance or reduce the complete public scope.


## Owned column-major matrices and LU factors

Forty f32/f64 functions expose typed matrix and LU owners. Matrix storage keeps
row + column * rows indexing and the native multiplication accumulation order.
Shape writes exactly three size_t elements: rows, columns, scalar count. Copy,
vector multiplication and solve require exact output counts. Invalid shapes,
counts, indices and dimensional overflow return two; nonfinite arithmetic,
invalid swap partners, singular factors and unknown selectors return three.
Dimensions and scalar counts are limited to INT32_MAX and addressable bytes.

Matrices support construction, zeros, identity, clone, clearing release, shape,
copy, indexed access, matrix/vector products, successive row swaps and source
preliminary pivoting. LU policy zero selects partial pivoting during elimination;
one selects source-compatible unpivoted factorization. Source preliminary pivoting
requires separately applying its swap plan to the right-hand side before using
policy one. Complete packed factors and ordered row swaps can be copied. LU
owners can be cloned, queried, solved and independently released. A clone or
factor owns its data and remains valid after the input owner is released.

Mutating matrix calls require exclusive access. Read-only matrix and factor
calls exclude mutation and release. Outputs and slots are independent of owner
allocations. Constructors publish only on success. Mutations stage a cloned
matrix before commit, retaining both matrix and output on failure. Vector
operations stage complete results before writing, allowing input/output overlap.
Zero-sized buffers may be null; no null slice or null zero-length copy is formed.
Standalone vector swaps preserve NaN payloads and signed-zero bits. Native clone
allocation and abort limitations still apply. Replacing a populated ownership
slot does not release its previous owner; the caller must retain and release it.

The generated header declares 298 functions and eighteen opaque types. Nine
host tests and warning-denied all-target Clippy pass. Four original/corrected
MAT1 corpora cover all matrix products, permutations, preliminary pivots, packed
factors and source solves. Partial-pivot residual bounds remain 2e-6 for f32 and
1e-13 for f64. Independent tests cover rectangular/empty matrices, late zero
pivots with factor reconstruction, singular errors, ownership, atomic mutation
errors and bit-preserving swaps. No native numerical kernel, reference fixture
or tolerance changed. All five targets pass 101 boundary tests plus six focused frame/DSP tests,
with twenty exact suite counts and no failed/ignored tests or error diagnostics.
Actual optimized assert-enabled C callers invoke all 298 functions on every
platform. Python ctypes callers pass on MSVC64, GNU64 and Linux64. GNU's initial
runner omitted the new Rust suite; its corrected complete runner also exits zero.
Expanded optimized MSVC C source independently references all declarations.
Warning-denied Rustdoc initially rejected unquoted swaps[i] text; code formatting
corrects the documentation link without changing executable code. The inventory
now has 134 implemented C mappings, 198 pending public C rows, two nonpublic macro
formals and 332 pending public WASM rows. This checkpoint does not replace final
combined acceptance. Remaining bindings across all three crates stay in scope.


## FIR, convolution and recursive filtering

Twelve f32/f64 functions expose all native filtering.rs operations. Windowed FIR
selects passband zero lowpass, one highpass, two bandpass and three bandstop;
cutoffs use cycles per sample. fir and bandpass_fir retain Nyquist-normalized
cutoffs. fir highpass zero selects false and one true. All seven existing window
codes remain available. FIR counts equal the requested tap length. Fractional
bins, even-length Nyquist replacement, native cosine precision and calculation
order are unchanged. Unknown selectors return three.

Convolution outputs signal_count + impulse_count - 1 elements when both inputs
are nonempty, otherwise zero. Counts and sum overflow are checked before
borrowing. Empty convolution still calls the native input validation, so NaN in
a nonempty operand is rejected even when the other is empty. Native convolution
finite-input and result behavior is retained without adding a new result policy.

filter and forward_backward_filter expose startup zero ZeroState, one
LegacyZeroPrefix. The native convenience filter/forward_backward calls are policy
zero. Both retain the monic denominator convention: denominator element zero is
ignored numerically, without skipping native validation or normalizing other
coefficients. Empty numerator yields zeros. Outputs have exactly signal length.
Two-pass filtering reverses between and after passes without endpoint padding.
All scalar arrays are staged; errors leave output unchanged even when a later
sample overflows. Complete staging permits input/output overlap. Empty null
buffers never form null slices or copies. Input/output safety and allocation
limitations remain those of the shared boundary. Nine arguments on recursive
entry points retain explicit three independent slice lengths and startup rather
than hiding them in unchecked layout assumptions.

Six host tests and warning-denied all-target Clippy pass. Both IIR1 corpora
consume all210 cases, both startup policies and causal/two-pass output with exact
source agreement. Both FRS1 corpora cover all300 FIR and60 convolution records
with unchanged2e-7/2e-15 bounds;105 resampling records are consumed without
claiming resampling ABI coverage. Native-kernel equivalence additionally covers
Nuttall98/MLT sine selectors absent from FRS1. Independent impulse and two-pass
examples, overlap, empty/nonfinite inputs, unknown selectors, invalid tap ranges,
count overflow and later recursive overflow are tested. Initial compilation
required the same RefUnwindSafe bound used by other scalar boundary adapters;
no native kernel, old fixture, dependency or threshold changed. Generated header
has310 functions/eighteen opaque types. Warning-denied Rustdoc passes. All five required targets pass107 boundary tests plus six focused frame/DSP
tests, exact21 summary counts10,4,4,5,6,6,3,4,6,9,6,6,3,6,6,6,5,6,6,3,3 and zero
failed/ignored or compiler-error diagnostics. MSVC64/32, GNU64 and Linux64/32
runners terminate with exit zero. Actual optimized assert-enabled C callers
invoke all310 exports on every target; Python callers pass on all three64bit
environments. Expanded optimized MSVC C source references every declaration.
Formatting, whitespace, generated-header verification and warning-denied Rustdoc
pass. The334-row inventory has142 implemented C mappings,190 pending public C
rows, two nonpublic macro formals and332 pending public WASM rows; every mapped
export exists in the header. Current family Release, Python32 and static-link
callers remain unverified. This family checkpoint does not replace final combined
acceptance. Full remaining binding scope across all three crates is unchanged.


## Resampling and allocation-free output sizing

Eight f32/f64 functions expose all native resampling variants and output length
queries. resample selects IncludeFirst plus Stable. resample_with selects boundary
zero IncludeFirst or one LegacySkipFirst, retaining the Legacy kernel.
resample_with_kernel selects boundary and kernel independently: kernel zero
Stable, one Legacy. Unknown selectors return three. No compatibility policy is
silently substituted. Output counts must exactly match native sizing; mismatch
returns two before sample borrowing or allocation.

The shared internal native output_length helper retains scalar-precision length
conversion/multiplication followed by f64 round. Ratio one returns input count
without scalar conversion and copies original sample bits. Queries allocate no
signal/result and do not validate sample contents. Invalid ratio or native output
range returns three; input counts outside the C legacy range return two. Actual array calls additionally require addressable scalar byte ranges; the query
returns a mathematical native count without allocating or claiming an addressable
array of that count. The native resampling
path now calls this same helper without changing its arithmetic, antialiasing,
16-sample compensation, kernel calculation or accumulation order.

Resampling still validates nonfinite input even when a small ratio rounds to zero
output. Null empty buffers are not borrowed or copied. Complete owned staging
permits input/output overlap and retains all output on failure. Stable near-zero
Lanczos evaluation and original Legacy sine behavior remain independently
available. Ratio-one negative zero and subnormal bits are preserved.

Host eight boundary tests plus five existing native resampling/FIR tests pass;
warning-denied all-target Clippy and Rustdoc pass. Both FRS1 corpora consume all105
resampling records with unchanged2e-7/2e-15 bounds under the original boundary and
kernel. Other FRS1 records are consumed without new FIR/convolution coverage
claims. Separate tests check all policy combinations across fractional rates,
independent analytic near-integer sinc limits, the retained Legacy defect,
float-vs-double length rounding, large ratio-one lengths, bit preservation,
empty/nonfinite inputs, unknown selectors, shape/pointer errors and overlap.
Header318/eighteen opaque types. All five targets pass115 boundary tests plus11 focused native tests with exact
24counts10,4,4,5,6,6,3,4,6,9,6,6,3,6,6,6,5,6,8,6,3,3,3,2, zero failed/ignored
and no error diagnostics. Actual optimized assert-enabled C callers invoke all318
functions on every target; Python passes on MSVC64/GNU64/Linux64. Expanded
optimized MSVC C source independently references every declaration. Header verify,
formatting, whitespace and warning-denied Rustdoc pass.

Complete default-feature Debug native refresh passes on all five targets:
ciglet198, liblrhsmm120, SHIRO58. All15 package logs have exact73/35/36 summary
counts, zero failed/ignored tests and the expected liblrhsmm negative ownership
doctest diagnostics. Ignored tests are included, with Gnuplot/Lua/SPTK enabled.
The tested native resampling source SHA256 is
e99a849d0dd6fbf030e61743b012cedb19d3c48cce6353625fce12417b6c6981.
The334-row inventory now has144 implemented C mappings,188 pending public C,
two nonpublic macro formals and332 pending public WASM rows. Current family
Release/Python32/static callers remain unverified. This checkpoint does not
replace final combined native/bindings acceptance.
No original fixture, tolerance or dependency changed. All remaining public
interfaces and all three WASM bindings remain required.


## Scalar, linear, sinc and missing-value interpolation

Fourteen f32/f64 entry points expose all interpolation.rs public functions.
Irregular linear interpolation shares the source count for coordinates/values,
accepts unordered queries, retains source scalar arithmetic/preceding exact-knot
intervals and clamps endpoints. Uniform linear interpolation retains half-open
bounds and the native count-minus-1.01 clamp, with corrected singleton and empty
query safety. Scalar interpolation exposes the native overflow fallback.

Sinc variants mirror native convenience policies: default IncludeFirst+Stable,
explicit boundary with Legacy kernel, and independent boundary/kernel selection.
Boundary zero includes sample zero, one skips it; kernel zero Stable, one Legacy.
Unknown selectors return three. Missing-value interpolation accepts exact finite
or NaN sentinels, repeats nearest known values at leading/trailing gaps and yields
zero when every value is missing. Known nonfinite values are rejected.

Complete owned staging permits input/output overlap, retains all output on error
and avoids null zero-length slices/copies. Array outputs require exact source or
query counts. Empty queries still validate source values and bounds; empty source
is rejected for linear/sinc but accepted for missing-value interpolation.

Host nine tests, warning-denied all-target Clippy and Rustdoc pass. Both INT1
corpora consume all24 original linear cases perprecision; four EXT1 corpora
consume all28 disclosed-corrected cases perprecision/boundary with exact outputs.
Independent unordered/clamped/singleton/empty/NaN-gap/scalar-extreme/overlap/error
cases pass. Separate binary64 analytic near-integer sinc verification retains
the3e-16 bound and confirms the original Legacy defect plus default Stable policy.
Header332/eighteen opaque types. No native kernel, fixture, dependency or tolerance
changed. All five target runners terminate with exit zero. Exact25 summary counts
10,4,4,5,6,6,3,4,6,9,9,6,6,3,6,6,6,5,6,8,6,3,3,3,2 independently verify124
boundary tests plus11 focused native tests per target, zero failed/ignored and
no error diagnostics. Actual optimized assert-enabled C callers invoke all332
exports on every target; Python passes on MSVC64/GNU64/Linux64. Expanded optimized
MSVC C independently references every declaration. Header verification, formatting
and whitespace checks pass. The334-row source inventory now has152 implemented
C mappings,180 pending public C rows, two nonpublic macro formals and332 pending
public WASM rows; all mapped exports exist in the header. Current family Release,
Python32 and static-link callers remain unverified. This family evidence does not
replace final combined native/bindings acceptance. Linux next-phase artifacts use a
Linux filesystem target directory per official Cargo/Microsoft guidance; source
repositories stay at the required Windows paths. Existing proof/cache retained.
All remaining public interfaces and all three WASM bindings stay in scope.


## Moving averages, RMS and progressive median filtering

Ten f32/f64 entry points expose both native moving average/RMS convenience and
explicit-policy functions plus median filtering. Moving defaults select Stable;
explicit policy zero Stable, one Legacy. Fractional average half-widths and
integer RMS half-widths preserve native endpoint replication, sample-cell layout
and source-compatible cumulative arithmetic. Stable RMS uses scaled squares;
Legacy overflow remains an error. Zero average width copies original sample bits,
zero Stable RMS width gives absolute values. Median windows retain original
growing prefixes and shrinking suffixes, including even-width median averaging
with the native finite-overflow correction.

All array outputs require exact input length. Complete owned staging preserves
output on failure and permits input/output overlap. Null empty buffers are never
borrowed or copied; native width/domain validation still applies. Median requires
positive bounded width even for empty input; nonempty width may not exceed signal
length. Empty signals with positive widths return empty results. Unknown policy
or native arithmetic errors return three; count mismatches return two.

Host eight tests, warning-denied all-target Clippy and Rustdoc pass. Each precision
consumes all60 original SMO1 mean/RMS cases,96 original MED1 cases and330 exact
rational/high-precision SSW1 cases with10,920 Stable outputs. Existing bounds remain
2e-7 f32/4e-15 f64 normal relative error; zero outputs remain exact except the
existing one-minimum-quantum subnormal RMS allowance. Convenience functions and
policy-zero results agree. Independent endpoint/window, maximum finite/Legacy
error, negative-zero identity, in-place, empty/nonfinite/width/count/policy and
atomic-error cases pass. No native kernel, fixture, dependency or threshold
changed. Header342/eighteen opaque types. All five target runners terminate with exit zero. Exact26 summary counts
10,4,4,5,6,6,3,4,6,9,9,6,6,3,6,6,6,5,6,8,8,6,3,3,3,2 independently verify132
boundary tests plus11 focused native tests pertarget, zero failed/ignored and
no error diagnostics. Actual optimized assert-enabled C callers invoke all342
exports on every target; Python passes on MSVC64/GNU64/Linux64. Expanded optimized
MSVC C independently references every declaration. Header verification, formatting
and whitespace checks pass. The334-row inventory now has157 implemented C source
mappings,175 pending public C rows, two nonpublic macro formals and332 pending
public WASM rows; all mapped exports exist in the header. Current family Release,
Python32 and static-link callers remain unverified. This family evidence does not
replace final combined native/bindings acceptance. Remaining
public C and all three WASM interfaces are required.


## Aliased sinc and angular-frequency derivatives

Eight typed f32/f64 exports expose all four native sinc functions. Convenience
functions use Stable arithmetic. Explicit policy zero selects Stable; one selects
Legacy, including the original distinct value/derivative denominator plateaus.
Length remains a signed or fractional scalar, not an integer buffer count.
One staged scalar output preserves caller storage on native errors. Unknown
policies and nonfinite inputs return three; null or misaligned outputs return one.

Host six tests and warning-denied all-target Clippy/Rustdoc pass. Both SNC1 corpora
consume every192 original C records. Float Legacy output bits remain exact;
binary64 normalized error bounds remain2e-16 for values and5e-16 for derivatives,
with exact zero signs in both precisions. Independent integer-length analytic
values/slopes, even-length poles, fractional near-zero slopes, maximum finite
lengths, malformed pointers and atomic errors are checked. The default agrees
with explicit Stable; the source plateaus remain selectable through Legacy.
No native numerical kernel, fixture, tolerance or dependency changed. Header
contains350 status-returning exports and eighteen opaque types. Optimized expanded
MSVC C references every declaration. All five target runners terminate with exit
zero. Exact27 summary counts10,4,4,5,6,6,3,4,6,9,9,6,6,3,6,6,6,5,6,8,6,8,6,3,3,3,2
verify138 boundary plus11 focused native tests pertarget, zero failed/ignored and
no error diagnostics. Actual optimized assert-enabled C callers invoke all350
exports on every target; Python passes on MSVC64/GNU64/Linux64. Header verification,
formatting and whitespace checks pass. The334-row inventory has159 implemented
C source mappings,173 pending public C rows, two nonpublic macro formals and332
pending public WASM rows; all mapped exports exist in the header. Current family
Release, Python32 and static-link callers remain unverified. This family evidence
does not replace final combined native/bindings acceptance. All remaining public
C interfaces and all three WASM bindings remain required.


## Centered sinusoid synthesis

Four f32/f64 entry points expose both native single-tone and ordered multi-tone
synthesis. Phase is specified at sample floor(length /2). Parameters retain signed
frequencies and amplitudes and native scalar recurrence. The disclosed native
amplitude and zero/one-length safety corrections apply. Multi-tone calls accept
three scalar arrays sharing tone_count; zero tones yield silence. Scalar arguments
avoid a cross-language struct layout dependency. Output count must equal length.

All parameters and positive legacy-range sample rates are validated even for
empty output. Inputs are range-checked before borrowing. Complete owned staging
permits parameter/output overlap and preserves output on errors. Null empty
buffers are never borrowed or copied. Pointer errors return one, count/range
errors two and native errors three; panic/allocation limitations remain unchanged.

Host six tests, warning-denied all-target Clippy and Rustdoc pass. Both SYN1 corpora
consume all48 original C cases perprecision with unchanged amplitude-normalized
bounds2e-7 f32 and2e-14 f64. Independent amplitude/centered-phase formulas, ordered
sum equality, empty silence, zero/one lengths, signed parameters, malformed inputs,
rate/count errors, overlap and atomic output checks pass. Header354/eighteen opaque
types; expanded optimized assert-enabled MSVC C references all354 declarations.
No native kernel, source fixture, dependency or tolerance changed. All five target
runners terminate with exit zero. Exact29 summary counts10,4,4,5,6,6,3,4,6,9,9,6,6,
3,6,6,6,5,6,8,6,8,6,6,3,3,3,2,4 verify144 boundary plus15 focused native tests
pertarget, zero failed/ignored and no error diagnostics. Actual optimized
assert-enabled C callers invoke all354 exports on every target; Python passes
on MSVC64/GNU64/Linux64. Header verification, formatting and whitespace checks
pass. The334-row inventory has162 implemented C source mappings,170 pending public
C rows, two nonpublic macro formals and332 pending public WASM rows; all mapped
exports exist in the header. Current family Release, Python32 and static-link
callers remain unverified. This family evidence does not replace final combined
native/bindings acceptance. All remaining public C interfaces and all three WASM
bindings remain required.


## Kalman filtering and backward smoothing

Six typed f32/f64 entry points expose all three native Kalman functions. Filtering
uses either the first measurement or an explicit initial mean. Three input arrays
share the sample count. The mean and variance outputs require the same count;
the likelihood output is always one scalar, including for empty estimates.
Smoothing similarly uses a shared count for means, posterior/process variances
and output. Native source-compatible scalar arithmetic and likelihood rounding
are retained, including the disclosed wide-logarithm domain correction.

Complete owned staging preserves all outputs on failure. Filtering's mean and
variance buffers and likelihood slot must be pairwise disjoint. Inputs may overlap
one another or the outputs because input borrowing ends before the commit. Empty
array pointers may be null; the likelihood slot must remain valid. Array count
mismatches, excessive counts and overlapping outputs return two; invalid pointer
alignment/nullness returns one; native validation/arithmetic errors return three.
The usual allocation/abort and caught-panic limitations apply.

Host eight tests, warning-denied all-target Clippy and Rustdoc pass. Both KLM1
corpora consume all 108 original C cases and 3,672 reference outputs per precision.
Every reference mean, variance, smoothed mean and likelihood matches numerically
exactly. Independent posterior/backward calculations, deterministic uncertainty,
empty estimates, input/output overlap, nonfinite/negative/zero innovation errors,
all three output-pair overlaps, partial overlap and each misaligned output are
checked. Failures after a valid forward/backward step retain every caller output.
No native kernel, fixture, dependency or tolerance changed. Header contains 360
status-returning exports and eighteen opaque types; expanded optimized,
assert-enabled MSVC C references every declaration. All five target runners
terminate with exit zero. Exact 31 summary counts
10,4,4,5,6,6,3,4,6,9,8,9,6,6,3,6,6,6,5,6,8,6,8,6,6,3,3,3,4,2,4 verify
152 boundary plus 19 focused native tests per target, zero failed/ignored and no
error diagnostics. Actual optimized assert-enabled C callers invoke all 360
exports on every target; Python passes on MSVC64/GNU64/Linux64. Header verification,
formatting and whitespace checks pass. The 334-row inventory has 166 implemented
C source mappings, 166 pending public C rows, two nonpublic macro formals and 332
pending public WASM rows; every mapped export exists in the header. Current family
Release, Python32 and static-link callers remain unverified. This family evidence
does not replace final combined native/bindings acceptance. All remaining public
C interfaces and all three WASM bindings remain required.


## Frame correlograms and frequency conversion

Six typed f32/f64 exports expose all three native correlogram functions. Method
codes are zero autocorrelation, one absolute difference, two squared difference
and three YIN. Default compute selects Stable; explicit policy zero Stable or
one Legacy retains the corresponding native arithmetic. Signed sample centers
and positive window lengths share the frame count. Row-major output dimensions
are frame_count by periods. Native strict trailing boundaries and unavailable-lag
values are retained. Frequency conversion accepts a row-major period matrix and
finite queries in caller order, including signed-zero endpoint semantics.

Complete owned staging preserves output on every error and permits input/output
overlap. Compute assembles owned frame descriptors from checked center/length
arrays. Frequency conversion copies the period matrix into native owned storage;
all input borrowing ends before output commit. Counts must match checked dimension
products; mismatches/overflow return two. Nonempty null or misaligned pointers
return one; native domain errors and unknown method/policy codes return three.
Empty buffers are never borrowed or copied. Native allocation/panic limits apply.

Host six tests and warning-denied all-target Clippy/Rustdoc pass. Both CRG1 corpora
consume all 32 original C cases, preserving exact float bits and the unchanged
3e-15 binary64 normalized error bound. Independent ramp/YIN calculations, strict
trailing boundaries, constant/negative correlation, DC-offset stability, maximum
amplitudes, silence, both policies, frequency knots/fractional/endpoints, signed
zero, singleton axes and empty frames are checked. Default equals explicit Stable
for all four methods in both precisions. Input/output overlap, malformed dimensions,
unknown selectors and nonfinite queries are tested. A failure in the second frame
preserves every output; misaligned center, length and result buffers are rejected.
No native kernel, fixture, dependency or tolerance changed. Header has 366 exports
and eighteen opaque types; expanded optimized assert-enabled MSVC C references all
366 declarations. Four prefixed public C method constants also expose the original
codes. They are generated with cbindgen's constants item category; unrelated
Rust-native LEGACY names remain outside this header and their separate C mappings
remain pending. The official [cbindgen export configuration](https://github.com/mozilla/cbindgen/blob/main/docs.md#export)
is used rather than manually editing the header.

All initial five-target runners terminate with exit zero. Exact 33 summary counts
10,4,4,5,6,6,6,3,4,6,9,8,9,6,6,3,6,6,6,5,6,8,6,8,6,6,3,3,3,3,4,2,4 verify
158 boundary plus 22 focused native tests per target, zero failed/ignored and no
error diagnostics. After adding the named method constants, six focused tests
pass again on each target, including assertions against every native Method value.
The final optimized assert-enabled C caller invokes all 366 exports and uses all
four method constants on every target; Python passes again on MSVC64/GNU64/Linux64.
The first constant-specific C compilation failed because the header configuration
excluded constants; enabling that category fixed the generated declarations, and
all final caller runs terminate with exit zero. Header verification, formatting
and whitespace checks pass. The 334-row inventory has 172 implemented C source
mappings, 160 pending public C rows, two nonpublic macro formals and 332 pending
public WASM rows. Every mapped function or constant exists in the header. Current
family Release, Python32 and static-link callers remain unverified. This family
evidence does not replace final combined native/bindings acceptance. All remaining
public C interfaces and all three WASM bindings remain required.


## Synchronous root-finding callbacks

Two f32/f64 exports expose native bracketed scalar root finding with a correctly
typed C callback and opaque context pointer. Calls are synchronous; neither
callback nor context is retained. Null callbacks return one. The context passes
through unchanged and may be null if the callback supports it. Scalar output is
staged and committed only after native success. Invalid bounds or nonfinite
callback values return three; invalid output alignment/nullness returns one.
The callback may mutate its own context but must not mutate output storage.

Nullable callbacks use Option<unsafe extern "C" fn>, following the official
[Rust nullable-pointer rules](https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization)
and [Option representation guarantees](https://doc.rust-lang.org/std/option/index.html#representation).
Callbacks must not unwind across their ordinary C boundary. The surrounding
native catch_unwind cannot protect against a callback panic/foreign exception
crossing that boundary; see the official [FFI unwinding rules](https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding).
Native abort/allocation and callback memory-validity limits remain unchanged.

Host six tests and warning-denied all-target Clippy/Rustdoc pass. Both ROT1 corpora
consume all 25 original C roots with exact numerical values. Independent stateful
context, endpoint roots, nonbracketing midpoint fallback, maximum finite midpoint,
null context, invalid bounds/null callback/misaligned output, first/second and
interior nonfinite callback results and atomic output preservation are checked.
Invalid pointer/bounds calls do not invoke callbacks. Native root arithmetic,
iteration limit, tolerance, fixtures and dependencies are unchanged. Header
contains 368 status-returning exports, eighteen opaque types and four correlogram
method constants. Expanded optimized assert-enabled MSVC C references every export.
Actual C/Python callers exercise linear and nonlinear scalar-return callbacks.
All five target runners terminate with exit zero. Exact 35 summary counts
10,4,4,5,6,6,6,3,4,6,9,8,9,6,6,3,6,6,6,5,6,8,6,6,8,6,6,3,3,3,3,4,2,3,4 verify
164 boundary plus 25 focused native tests per target, zero failed/ignored and no
error diagnostics. Optimized assert-enabled C callers invoke all 368 exports on
every target; Python passes on MSVC64/GNU64/Linux64. Scalar-return callbacks are
therefore exercised on both 32bit native C targets as well. Header verification,
formatting and whitespace checks pass. The 334-row inventory has 174 implemented
C source mappings, 158 pending public C rows, two nonpublic macro formals and 332
pending public WASM rows; every mapped function or constant exists in the header.
Current family Release, Python32 and static-link callers remain unverified. This
family evidence does not replace final combined native/bindings acceptance. All
remaining public C interfaces and all three WASM bindings remain required.

## Quadratic spectral peak interpolation

Two f32/f64 exports expose the native three-bin quadratic interpolation. Calls take
an explicit magnitude count and interior bin, returning fractional FFT-bin position
and capped magnitude through separate scalar outputs. This preserves cig_qifft and
qifft functionality without converting bin position to hertz. Signed magnitudes,
flat/displaced fallback, mixed-precision operations and the center-plus-0.2 cap are
unchanged. Only three participating samples are validated.

Both aligned exclusive output slots must be disjoint. Input range and output
ranges/overlap are checked before computation; an owned native Peak is computed
inside the existing panic boundary before either scalar is written. Every error
preserves both outputs. Input/output overlap is supported after input borrowing
ends. Null input is permitted only for zero count; missing neighbors subsequently
return native error three. Invalid nullness/alignment returns one; range/count or
overlapping outputs return two. Callers guarantee allocation validity, initialized
stable readable samples and exclusive output storage. Native panic/allocation
limits still apply.

Six host tests and warning-denied all-target Clippy/Rustdoc pass. Both original
64-case QIF1 corpora retain exact bits for both outputs. Independent parabola/cap,
flat response, ignored outside NaNs and supported in-place output are checked in
both precisions. Missing neighbors, overflowing arithmetic, participating NaNs,
null/misaligned outputs, aliasing outputs and excessive counts retain both slots.
No native arithmetic, fixture, dependency or existing tolerance changed. Generated
header verification passes with 392 status-returning exports, twenty opaque types
and four correlogram constants. Expanded optimized assert-enabled C main references
every export. All five target runners terminate with exit zero. Exact 39 summary
counts 10,4,4,5,6,6,6,3,4,6,6,9,8,9,6,6,3,6,6,6,5,6,8,6,6,8,6,6,6,3,3,3,3,3,4,2,3,2,4
verify 176 boundary plus 30 focused native tests per target, zero failed/ignored
and no error diagnostics. Optimized assert-enabled C callers invoke all 392
exports on every target; Python passes on MSVC64/GNU64/Linux64. Both source aliases
are mapped and verified against the generated header: the 334-row inventory has
182 implemented C mappings, 150 pending public C rows, two nonpublic macro formals
and 332 pending public WASM rows. Current family Release, Python32 and static-link
callers remain unverified. This evidence does not replace final combined native/
bindings acceptance. All remaining public C interfaces and all three WASM bindings
remain required.


## Owned instantaneous-frequency detectors

Twenty-two f32/f64 exports expose construction from original generated kernels or
four caller-designed kernel planes, independent cloning, release, kernel length
and center frequency queries, center frequency updates, kernel copying and atomic
kernel replacement, and default/explicit-policy estimation. Frequencies remain
sampling-rate ratios; policy zero selects Stable, one selects Legacy. This covers
valid configuration of the original public detector structure without exposing
Rust-owned vector storage. Invalid configurations are rejected before replacement
and leave the previous configuration usable. Native Rust still exposes checked
estimation after kernel editing through kernels_mut.

Each handle owns its detector. Read operations may run concurrently if mutation
and release are excluded; configuration and release require exclusive access.
Clones own independent kernel vectors. Callers retain initialized readable input
buffers and exclusively writable output buffers independent of private owner
storage. Kernel copies require four pairwise disjoint output planes with exactly
kernel_count elements. All lengths, pointer ranges and pairwise overlaps are
validated before copying directly from the retained immutable owner; no temporary
vectors are allocated. Replacement builds a complete validated candidate before
changing the owner. Constructors publish ownership only on success. Release
clears the ownership slot and accepts an already-null initialized slot. All
aliases become invalid on release; constructor output slots must not already own
an unreleased object. Null arrays are accepted only for zero counts.

Scalar estimates are committed only after success, including when the output
overlaps samples. Short samples preserve the native positive-zero result; only
participating samples are checked. Scalar outputs are required even for empty
samples. Status one reports invalid nullness/alignment, two reports invalid ranges,
kernel-copy count or overlapping copy planes, and three reports native domain
errors or unknown policies. Native allocation/panic limits remain applicable.

Host six tests and warning-denied all-target Clippy pass. Both IFD1 corpora consume
all 24 original C cases and four sample datasets per case. Float kernel and Legacy
estimate bits are exact; binary64 keeps the existing 3e-15 kernel and 1e-12 estimate
normalized bounds. Independent configured-kernel checks cover both policies,
default equals Stable, maximum/subnormal amplitudes, ignored outer NaNs, short
samples, in-place scalar output, clone independence, configuration retention and
all six copy-plane alias pairs. No native arithmetic, fixture, dependency or
existing tolerance changed. The generated header has 390 status-returning exports,
twenty opaque types and four correlogram constants. Expanded optimized assert-enabled
C main calls every declared export. All five target runners terminate with exit
zero. Exact 37 summary counts
10,4,4,5,6,6,6,3,4,6,6,9,8,9,6,6,3,6,6,6,5,6,8,6,6,8,6,6,3,3,3,3,3,4,2,3,4
verify 170 boundary plus 28 focused native tests per target, with zero failed or
ignored tests and no error diagnostics. Optimized assert-enabled C callers invoke
all 390 exports on every target; Python passes on MSVC64/GNU64/Linux64. Original C
oracle regeneration checks pass on Windows and Linux: float fixtures are unchanged,
and Linux binary64 maximum kernel/estimate normalized differences are
1.1804333753159537e-16/6.236352585145109e-13, within unchanged 3e-15/1e-12 bounds.
Warning-denied Rustdoc, header verification, formatting and whitespace checks pass.
The 334-row inventory now has 180 implemented C source mappings, 152 pending public
C rows, two nonpublic macro formals and 332 pending public WASM rows. Every mapped
function or constant exists in the header. Current family Release, Python32 and
static-link callers remain unverified. This evidence does not replace final
combined native/bindings acceptance. All remaining public C interfaces and all
three WASM bindings remain required.

## LF model configuration and reusable plans

Twenty f32/f64 exports expose every native public Model field, shape construction,
direct caller configuration, one-shot spectra and sampled periods, and reusable
Prepared owners with independent cloning, model queries, both spectrum methods
and sampled periods. Configuration buffers contain exactly five selected-precision
scalars in this order: period in seconds, closure, peak, return_time as period
ratios, and signed excitation. The complete configuration is copied before model
preparation; private prepared coefficients remain owned by Rust. Model output uses
a stack array and commits all five fields only after native success.

Named precision constants are FAST zero, APPROXIMATE one, EXACT two. The default
spectrum retains Approximate. These select native exponential/trigonometric
precision and preserve all source preparation arithmetic, eight Newton updates,
800 Hz rescaling, stability shortcuts, accurate output atan2, DC positive zero,
sample-index rounding, truncated periods and zero-padded suffixes. Finite source
results are retained; no different preparation optimizer is substituted.

Prepared handles are immutable and support concurrent reads while release is
excluded. Release requires exclusive access, clears its initialized ownership slot
and accepts an already-null slot. Cloning owns a separate native Prepared instance
that remains usable after the source owner is released. Constructors publish the
owner only after success; destination slots must not already own an unreleased
object. Caller output storage must remain independent of private owner memory.

Spectrum calls validate matching frequency/output counts and pairwise disjoint
magnitude/phase planes before computation. Native owned vectors stage both outputs;
both lengths are checked before either plane is committed. Every error preserves
both planes, including a late finite-frequency overflow after an earlier response
was calculated. Input/output overlap is supported after input borrows end. Sampled
periods use owned staging and check requested/output counts before any writes.
Null arrays are allowed only for zero counts; model configuration still requires
five initialized elements. Alignment/null failures return one, count/range/overlap
failures return two, and native domain/unknown-selector failures return three.
Callers retain valid initialized stable readable input allocations and exclusively
writable aligned output ranges. Native panic/allocation limits remain applicable.

Six host tests and warning-denied all-target Clippy pass. Both 53-record LFM1 corpora
compare all 265 model fields exactly and all 8,304 response/wave outputs per precision.
Binary32 outputs retain exact bits; binary64 bounds remain 1e-17 magnitude, 3e-15
phase and 5e-16 waveform normalized error. One-shot and reusable spectra/periods,
default and explicit Approximate, and queried configurations all pass the original
corpora. Independent tests cover all precision selectors, Exact conjugate symmetry,
DC, directly configured signed excitation, zero excitation, clone lifetime,
spectrum input/output overlap, empty calculations, invalid configurations/rates,
null/misaligned pointers, exact/partial plane aliasing, excessive counts, unknown
precision and late overflow with atomic output retention. Native arithmetic,
fixtures, dependencies and existing tolerances are unchanged.

Generated header verification passes with 412 status-returning exports, twenty-two
opaque types and seven named constants. Expanded optimized assert-enabled C main
calls every export. All five target runners terminate with exit zero. Exact 41
summary counts
10,4,4,5,6,6,6,3,4,6,6,6,9,8,9,6,6,3,6,6,6,5,6,8,6,6,8,6,6,6,3,3,3,3,3,3,4,2,3,2,4
verify 182 boundary plus 33 focused native tests per target, zero failed/ignored
and no error diagnostics. Optimized assert-enabled C callers invoke all 412 exports
and use all seven named constants on every target; Python passes on MSVC64/GNU64/
Linux64. Warning-denied Rustdoc, header verification, formatting and whitespace
checks pass. The 334-row inventory has 188 implemented C mappings, 144 pending
public C rows, two nonpublic macro formals and 332 pending public WASM rows; every
mapped function or constant exists in the header. Current family Release, Python32
and static-link callers remain unverified. This evidence does not replace final
combined native/bindings acceptance. All remaining public C interfaces and all
three WASM bindings remain required.

## Spectral-envelope plans and C descriptors

Eighteen f32/f64 exports expose plan construction/release, every public metadata
query (len, bins, is_empty), all three reusable estimation methods and one-shot
estimation. Estimation returns both the full FFT-grid natural-log envelope and
the one-sided cepstrum. Fundamental is a sampling-rate ratio in (0,0.5]. Harmonic
count zero represents None; explicit counts >=2 remain available through the
native bound, and one is rejected. Nyquist and smoothing policies remain independent:
zero is Preserve/Stable and one Legacy. Default = Preserve + Stable.

Named options and output descriptors use unpacked repr(C), passed by value. Options
contain fundamental and harmonics; output describes log_spectrum/log_count and
cepstrum/cepstrum_count. This follows the official [Rust C representation](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)
and [FFI layout/ABI guidance](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc).
Concrete descriptors are generated using cbindgen's documented
[struct export category](https://github.com/mozilla/cbindgen/blob/main/docs.md#export).
Python callers define ordinary ctypes.Structure fields with the
[default native C layout](https://docs.python.org/3/library/ctypes.html#structure-union-layout-alignment-and-byte-order),
without packing, unions or bitfields. Actual foreign callers check member offsets
and pass both descriptors by value; no manually constructed packed byte layout is
used. Rust callers retain the original typed native API.

Opaque plans serialize mutable FFT scratch through a mutex. Metadata queries also
use the lock. Callers retain the live matching-precision owner through every call;
release excludes all outstanding operations, invalidates aliases and clears the
initialized ownership slot. Constructor output slots must not already own an
unreleased object. Poisoned plans return error three. Computation validates output
length n and coefficient count n/2+1 and pairwise disjoint output byte ranges before
locking. Native owned Envelope vectors stage both outputs, with both returned
lengths checked before either plane is committed. All failures retain both outputs.
Input/output overlap is supported after input borrowing ends. Arrays must remain
initialized/readable or exclusively writable, aligned and valid for declared
counts; buffers are independent of private owner memory. Null arrays are accepted
only for zero counts; empty input still fails the native one-sided-bin requirement.
Pointer/alignment errors return one, count/range/overlap errors two and native
calculation/selector/poison errors three. Native panic/allocation limits apply.

Eight host tests and warning-denied all-target Clippy/Rustdoc pass. Both original
144-case ENV1 corpora compare every envelope and coefficient output (18,288 scalars
per precision) with unchanged 3e-7 binary32/5e-16 binary64 normalized bounds, using
source-matching Legacy Nyquist and Legacy smoothing. Independent constant/symmetry,
metadata, implicit/explicit harmonics, corrected zero cutoff, tiny two-point count,
all four policy combinations, repeatable reuse, one-shot/default equivalence and
in-place output are checked. Invalid options/counts/data/policies, null/misaligned
buffers, exact/partial plane overlaps and excessive ranges retain both outputs.
Four scoped workers perform 50 calculations each on the same owner with independent
buffers, verifying repeatable results under scratch serialization. Scopes follow
[std::thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html), joining every
worker before releasing the owner. The first host run reported a filesystem warning
while finalizing an incremental cache; rerunning with incremental compilation
disabled passes cleanly. No native arithmetic, fixture, dependency or tolerance
changed. The existing native acceptance evidence remains applicable.

Generated header verification passes with 430 status-returning exports, twenty-four
opaque types, four concrete descriptors and seven named constants. Expanded
optimized assert-enabled C main references all exports. All initial five-target
runners terminate with exit zero. Exact 43 summary counts
10,4,4,5,6,8,6,6,3,4,6,6,6,9,8,9,6,6,3,6,6,6,5,6,8,6,6,8,6,6,6,3,3,3,3,3,3,4,2,3,3,2,4
verify 190 boundary plus 36 focused native tests per target, zero failed/ignored
and no error diagnostics. After adding independent constant-spectrum and invalid
harmonic-count checks to the foreign callers, final focused caller runners pass
again on all five targets without changing Rust sources or repeating Rust suites.
Optimized assert-enabled C invokes all 430 exports on each target; Python passes
on MSVC64/GNU64/Linux64. The natural C descriptor layout and value passing are
therefore exercised on both 32bit platforms as well. Header verification, formatting
and whitespace checks pass. The 334-row inventory has 190 implemented C mappings,
142 pending public C rows, two nonpublic macro formals and 332 pending public WASM
rows; every mapped function or constant exists in the header. Current family
Release, Python32 and static-link callers remain unverified. This evidence does
not replace final combined native/bindings acceptance. All remaining public C
interfaces and all three WASM bindings remain required.

## STFT analysis and synthesis bindings

Fifteen status-returning exports expose reusable analysis/synthesis owners and
regular-grid helpers in binary32/binary64, plus shared default analysis options.
Six upstream mappings retain arbitrary frame centers/window lengths, independent
magnitude and phase requests, mean subtraction, first-window weight, optional
normalization, signed output offset, endpoint fades and both Nyquist policies.
The signed legacy optimization-level helper preserves every level equivalence
class used by CIG_DEF_STFT; stft/stft_2 select Accurate/Approximate respectively.
Regular analysis uses Blackman and grid.hop normalization. Regular synthesis keeps
zero offset, Accurate polar conversion and the original 32-sample fade.

Fourteen unpacked repr(C) descriptors pass configurations and buffer descriptions
by value with natural platform layout. Six descriptors per precision describe
analysis inputs/outputs, metadata, spectra, synthesis options and sample outputs;
analysis options and frame grids are shared. Selectors use checked integers:
window codes match the existing window API, precision 0 Accurate/1 Approximate,
Nyquist 0 Preserve/1 Legacy and booleans 0/1. A zero normalization_hop denotes
None for explicit analysis. Metadata reports frames, bins and four availability
bits: magnitude 1, phase 2, window weight 4 and normalization 8. Missing scalar
values are zero with their bit cleared. Empty requested spectral arrays remain
present, while an empty frame sequence has neither weight nor normalization.
Disabled spectral output counts are zero; enabled counts equal frames*bins.
Metadata is required even when neither spectral plane is requested.

Reusable opaque owners serialize mutable scratch with a Mutex. Every operation
requires a retained live matching-precision owner; release is excluded during
calls. Release clears the exclusive ownership slot and is idempotent for null
slots. Constructors require a slot without an unreleased owner. Analysis validates
all output lengths/ranges and pairwise byte disjointness, computes owned native
results, checks every returned shape and commits both planes and metadata together.
Synthesis stages an owned sample vector. Errors retain all caller outputs.
Input/output alias is supported because input borrows end before the first write.
Pointer/alignment errors return one, range/count/overlap errors two and native
calculation/selector/poison errors three. Existing panic/allocation limits apply.
Finite extreme inputs can produce nonfinite intermediate/results in native STFT;
the C wrapper preserves that behavior rather than silently changing its arithmetic.
Nonfinite input signal values are rejected by native validation.

Twelve host tests and warning-denied all-target Clippy pass. Both original 480-case
STF1 and 288-case IST1 corpora are read completely per precision. Analysis keeps
normalized magnitude bounds 2e-6 binary32/2e-14 binary64 and the existing
magnitude-weighted phase comparison excluding near-zero bins. Synthesis keeps
2e-7 binary32/2e-15 binary64 normalized bounds. Independent odd-window DC/mean,
all optional planes and metadata states, empty frames, explicit/regular equality,
signed legacy levels, both Nyquist policies, repeated reuse, invalid selectors/
counts/data, null/misaligned/overlapping outputs and retained outputs are checked.
Finite-overflow results match native bit patterns or NaN classifications.
Analysis and synthesis input/output alias are compared with independent buffers.
Four scoped workers run fifty analyses each on one owner with private outputs;
all finish before release. No native arithmetic, fixture, dependency or existing
numerical tolerance changed.

Descriptor layout follows the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)
and [Nomicon](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc).
Python caller declarations use [ctypes Structure layout](https://docs.python.org/3/library/ctypes.html#structure-union-layout-alignment-and-byte-order).
The concurrency check follows [std::thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html).


All five runners terminate with exit zero. Named-suite verification confirms all
46 expected suites and their individual counts: 202 boundary plus 43 focused
native tests per target, with zero failed/ignored tests and no error diagnostics.
Optimized assert-enabled C callers pass on MSVC64/MSVC32/GNU64/Linux64/Linux32;
Python callers pass on MSVC64/GNU64/Linux64. All fifteen STFT functions and fourteen
new descriptors are exercised through the generated header/ctypes declarations,
including independent optional outputs, metadata availability, regular helpers,
legacy precision levels, both reconstruction policies, odd-window DC and atomic
error retention. Header verification proves 445 unique status-returning exports,
26 opaque types, 18 concrete descriptors and eleven constants, with every export
called from expanded optimized C main. Formatting, Rustdoc and whitespace checks
pass. The 334-row inventory now records 196 implemented C mappings, 136 pending
public C rows, two nonpublic formals and 332 pending public WASM rows. Every mapped
function/constant exists in the generated header. Current-family Release,
Python32 and static-link callers remain unverified. Final combined acceptance
and all remaining ciglet, liblrhsmm and SHIRO bindings remain required.

## Complex arithmetic bindings

Sixteen status-returning functions expose binary32/binary64 construction,
conjugation, addition, subtraction, multiplication, division, exponential and
argument. Two concrete repr(C) value descriptors contain real and imaginary
fields. The natural layout has two scalar fields with no packing or private
RustFFT/num-complex representation. Input descriptors pass by value; result
pointers avoid scalar floating return changes on i686. Construction and ordinary
operators preserve native IEEE behavior, including successful nonfinite results
and signed zeros. Division retains native validation and scaled fallback for
intermediate range loss. Exponential/argument select Fast=0, Approximate=1 or
Exact=2 using three generated constants. The original c_exp alias selects
Approximate explicitly. Fast/Approximate retain their binary32 approximation
stages even for binary64 operands; the two host argument levels remain identical.

All output slots must be aligned and exclusively writable for their declared
type. Existing owned-result helpers check pointer/alignment before calculation,
retain outputs on every error and commit a complete descriptor/scalar only on
success. By-value descriptors permit reusing their original storage for results,
including writing an argument into the original real field. Null/misaligned
slots return one; native validation/invalid precision returns three; existing
caught-unwind/allocation limits apply. There are no opaque complex owners.

Six host tests and warning-denied all-target Clippy pass. Both original CPX1
fixtures are consumed completely, comparing all 64 complex cases per precision
with the existing exact bit criteria. Their fifteen scalar approximation tail
records are verified against native APIs using the unchanged double host-exp
one-ULP/f64-epsilon bound; scalar bindings remain tracked independently.
Construction, all operators, all three exponential levels and argument modes are
exercised. Independent arithmetic examples, signed-zero construction/conjugation/
exponential/argument, maximum and minimal-subnormal division, ordinary nonfinite
results, invalid selectors/data/divisors, null/misaligned output retention and
source-storage reuse are checked. No native arithmetic, fixture, dependency or
existing tolerance changed. Descriptor declarations follow the already referenced
Rust repr(C) and ctypes Structure documentation.


All three five-target runners terminate with exit zero. Named-suite verification
confirms all 48 expected suites and their individual counts: 208 boundary plus
46 focused native tests per target, with zero failed/ignored tests and no error
diagnostics. Optimized assert-enabled C callers pass on MSVC64/MSVC32/GNU64/
Linux64/Linux32; Python passes on MSVC64/GNU64/Linux64. Each caller exercises
all sixteen functions, all precision modes, both descriptor layouts, independent
arithmetic, signed zeros, extreme division, ordinary nonfinite results, error
retention and source-storage reuse. Header verification proves 461 unique exports,
26 opaque types, 20 concrete descriptors and fourteen constants. Every export is
referenced by expanded optimized C main. Both new descriptor definitions and
three selectors are checked. Rustdoc, formatting and whitespace checks pass.
The 334-row inventory records 207 implemented C mappings, 125 pending public C
rows, two nonpublic formals and 332 pending public WASM rows; all implemented
function/constant mappings exist in the generated header. Existing native source
and Cargo manifests/lock remain unchanged from the last full native gate.
Current-family Release, Python32 and static-link callers remain unverified.
Final combined acceptance and all remaining bindings remain required.

## Core numerical approximation bindings

Ten status-returning functions expose precision-selected binary32/binary64 sine,
cosine, atan2, natural logarithm and exponential. Three named constants select
Fast=0, Approximate=1 or Exact=2. Sixteen ciglet.h aliases map explicitly to their
required mode. Every calculation delegates to the existing native *_with API,
retaining binary32 approximation stages for binary64 input, source range reduction,
validation, underflow clamp and double host libm before scalar conversion. The
Approximate and Exact atan2 modes preserve the identical source levels two/three.
Core sine/cosine use the source full-range reduction and must not be substituted
for bundled fastsin/fastcos restricted polynomials. All 88 bundled approximation
bindings remain required independently; no bundled mapping is marked by this
checkpoint.

Scalar inputs pass by value and result pointers use existing checked owned-result
staging. Results retain signed zeros. Output slots are aligned and exclusively
writable for one scalar; by-value inputs permit reusing their original storage.
Null/misaligned output returns one; invalid precision/native domain or range errors
return three; existing caught-panic/allocation limits apply. Every failure retains
the output. Exact accepts large finite trigonometric inputs for which native
approximation range checks reject Fast/Approximate. Checked logarithm requires
positive representable approximation input; valid Exact computations retain
native double range. Underflow clamps belong to the approximate modes and do not
replace host exponential behavior.

Eight host tests and warning-denied all-target Clippy pass. Both SCM1 corpora
compare all 64 sine/cosine/atan2 cases per precision with unchanged exact or
one-ULP/f64-epsilon host bounds, including signed zeros. Their comparator tails
are consumed and verified natively. Both CPX1 fifteen-case logarithm/exponential
tails compare through the new C exports, with unchanged exact log/all approximate
criteria and the existing double host-exp one-ULP/f64-epsilon bound. CPX1's complex
section remains independently covered by tests/c_api_complex.rs. Remaining
base-two fields are checked natively and still require bundled C bindings.
Independent Exact identities, signed-zero atan2 mode differences, maximal finite
trigonometric inputs, minimal subnormal logarithm, approximate exponential clamps,
host atan2 equivalence, source-slot reuse, all selectors/domains, nonfinite inputs,
null/misaligned pointers and retained error outputs are checked. No native
arithmetic, fixture, dependency or existing numerical tolerance changed.

Foreign callers check all ten functions and all modes against selected original
SCM1 case 33 and CPX1 scalar cases 9/11 values. Hexadecimal constants retain fixture
bits exactly. Approximate values and every selected log value compare exactly;
host trigonometric/exponential results use the unchanged one-ULP/f64-epsilon
criteria. Early caller sanity checks were strengthened before the five-target
proof; the final callers use source-derived values rather than a broad accuracy
threshold. Independent signed-zero/identity/range/error/clamp/slot-reuse checks
also exercise the actual C/Python calling convention.


All three five-target runners terminate with exit zero. Verification checks all
50 suite names and individual counts: 216 boundary plus 49 focused native tests
per target, with zero failed/ignored tests and no error diagnostics. Cargo stderr
headers and stdout summaries can interleave in merged Windows logs; the verifier
checks both complete ordered inventories before associating them. Optimized
assert-enabled C callers pass on MSVC64/MSVC32/GNU64/Linux64/Linux32, and Python
passes on MSVC64/GNU64/Linux64. All ten functions and three modes are exercised.
Header verification confirms 471 unique status-returning exports, 26 opaque types,
20 concrete descriptors and seventeen constants; every export is referenced by
expanded optimized C main. Three new precision constants are checked. Rustdoc,
formatting and whitespace checks pass. The 334-row inventory records 223
implemented C mappings, 109 pending public C rows, two nonpublic formals and 332
pending public WASM rows. Every implemented mapping exists in the header, and
all 88 bundled fastapprox rows remain pending. Native source and Cargo manifests/
lock are unchanged from the full native gate. Current-family Release, Python32
and static-link callers remain unverified. Final combined acceptance and all
remaining ciglet/liblrhsmm/SHIRO bindings remain required.

## Bundled scalar and four-lane approximation bindings

All 88 bundled fastapprox APIs now have explicit modern C entry points: 44 scalar
and 44 portable four-lane functions. Six additional functions expose every native
lane conversion/bit helper: to_integers, from_integers, to_bits, from_bits, absolute
and from_integer64_lanes. Each bundled function delegates to its audited native
kernel, including independent power exponents, source full/restricted trig
variants, hyperbolic functions, special functions and approved Lambert exponential
corrections. The explicit source-to-Rust mapping distinguishes fasterfc (fast+erfc)
from fastererfc and correctly distinguishes both full-range vector precision
variants. No approximate algorithm is replaced by libm and no native arithmetic,
fixture, dependency or existing tolerance changed.

Four unpacked repr(C) descriptors expose four float, four signed integer, four
unsigned bit or two 64bit signed integer values. Arrays retain contiguous source
index order and their element's natural alignment. There is no __m128/SIMD ABI or
extra alignment requirement. Layout follows the [Rust array rules](https://doc.rust-lang.org/reference/type-layout.html#array-layout)
and [C representation rules](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation).
Python uses [fixed ctypes arrays](https://docs.python.org/3/library/ctypes.html#arrays)
in natural-layout Structures. Context7/Mintlify and these primary references were
checked before introducing descriptors. Cbindgen emits fixed C arrays; actual
foreign callers verify all four sizes, field offsets and natural alignments.

Inputs/descriptors pass by value. Existing checked owned-result staging commits
one scalar or complete lane descriptor only on success; errors retain every lane.
Outputs must be aligned, exclusively writable storage for their declared type.
By-value inputs permit source-storage reuse. Null/misaligned outputs return one;
native domain/range failure returns three; existing caught-unwind/allocation
limits apply. Conversion/bit helpers retain documented IEEE behavior, including
NaN payloads, signed zeros, SSE2 indefinite integer values and nearest-even integer
to-float rounding, rather than imposing approximation domain checks on raw bits.
They do not emulate hardware floating-point exception flags.

Six host tests and warning-denied all-target Clippy pass. Coverage sets prove
that every one of the 44 scalar and 44 vector source functions is invoked by the
original corpora. Scalar trig456/special232/hyperbolic128 records retain original
bit comparisons and approved scalar Lambert-exp correction criteria. Original
basic SSE2 records independently verify corresponding scalar base kernels.
All vector basic80/trig144/special67/hyperbolic48 records retain exact finite bits,
expected nonfinite rejections and independent corrected Lambert roots within one
ULP. Original conversion/absolute records in the basic corpus are also verified
through the six new helpers. Every kernel checks null/misaligned rejection and
retained outputs on invalid scalar input or invalid lanes at each of four positions.
Independent descriptor layout, NaN payload/sign bits, signed zero, lane ordering,
integer conversion, 64bit low/high words, source-storage reuse and separate lane
power exponents are exercised. Approximate power is compared with the audited
scalar operation rather than being incorrectly required to equal mathematical
integer powers.

Optimized assert-enabled C and Python callers contain source-derived input/output
bits for all 88 original functions and exercise all six helpers. Every kernel
checks its original reference result, source-storage reuse, retained output on
invalid input and null rejection. Descriptor tests check size16/offset0 and
platform natural alignment; raw conversion helpers retain NaN payloads and source
lane order. No new result accuracy tolerance is introduced.


All three five-target runners terminate with exit zero. Named-suite verification
checks all 58 names and individual counts: 222 boundary plus 64 focused native
tests per target, zero failed/ignored tests and no error diagnostics. Optimized
assert-enabled C callers pass on MSVC64/MSVC32/GNU64/Linux64/Linux32; Python passes
on MSVC64/GNU64/Linux64. All 565 exports are called from expanded optimized C main.
Header verification proves 565 unique exports, 26 opaque types, 24 concrete
descriptors and seventeen constants, including exact fixed-array field definitions.
Rustdoc, formatting and whitespace checks pass. Native source and Cargo manifests/
lock remain unchanged from the full native gate. The 334-row inventory now records
311 implemented C mappings, 21 pending public C rows, two nonpublic formals and
332 pending public WASM rows. All 88 bundled rows are implemented and every mapped
function/constant exists in the generated header. Six added helper functions are
also verified despite not being separate upstream inventory rows. Current-family
Release, Python32 and static-link callers remain unverified. Remaining ciglet
random/wave/plotting/macros/constants, all liblrhsmm/SHIRO C bindings, all three
WASM bindings and final combined acceptance remain required.

### Source scalar expressions, caller-defined folds and constants

`ciglet_rs_scalar_add_f32/f64` and
`ciglet_rs_scalar_accumulate_square_f32/f64` expose the original `CIG_DEF_ADD`
and `CIG_DEF_ADDSQR` expressions in the selected scalar precision. The square is
multiplied before addition without requesting a fused multiply-add. Ordinary IEEE
results are retained, including NaNs and infinities.
`ciglet_rs_scalar_minimum_f32/f64` and `ciglet_rs_scalar_maximum_f32/f64` delegate
to the native second-operand comparison helpers. Equal or unordered operands
retain the second operand, including its signed zero or NaN payload.

`ciglet_rs_scalar_fold_f32/f64` exposes the customizable `CIG_DEF_N_TO_ONE`
operation as a synchronous C callback receiving accumulator, next input and opaque
context. Input order and caller-provided initial value are preserved. Empty input
returns initial unchanged, permits null input and a null callback, and performs no
callback. Nonempty input requires a callback. Pointer/range errors are rejected
before invoking it; output is committed only after input borrowing ends, allowing
an input element to serve as the eventual result slot. Callbacks must not unwind
or modify input/output storage. They may modify their independent context and are
never retained. Foreign exceptions and panics crossing C callbacks are outside
Rust's `catch_unwind` protection.

`ciglet_rs_math_constants` fills a natural `repr(C)` descriptor containing the
original binary64 `M_EPS`, finite `M_INF`, `M_PI` and `M_PI_2` values. These are
`1e-15`, `1e15`, pi and half-pi. The descriptor uses native Rust constants for pi
and half-pi, avoiding duplicated decimal approximations.

The fold follows the standard library's documented left-associative semantics:
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
The source macro likewise processes every input in increasing index order.
Boundary tests include both unchanged original SCM1 comparator tails, independent
order-sensitive folds with callback counts and in-place output, empty-input signed
zero, invalid pointers/counts/callbacks, IEEE behavior and source constant bits.
Independent callers are `tests/c_api_scalar_smoke.c` and
`tests/c_api_scalar_smoke.py`; the existing aggregate callers cover the previously
published exports separately.
Validation checkpoint: the five executed native targets
(`x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`,
`x86_64-pc-windows-gnu`, `x86_64-unknown-linux-gnu`,
`i686-unknown-linux-gnu`) each passed all 59 selected suites: 227 boundary tests
and 64 focused native tests, with no failures or ignored tests. Both assert-enabled
optimized C callers passed on every target. Both Python callers passed on the three
64bit targets. The header contains 576 functions; the new independent C caller
invokes all 11 added functions and verifies all four descriptor fields. Clippy,
Rust documentation generation, formatting and generated-header verification passed.
Native kernels, Cargo manifest/lock and original fixtures remain unchanged.
This checkpoint covers Debug shared libraries; Release, 32bit Python and static
link callers are not claimed here. Full combined project/binding acceptance remains
required.

### Original random draws and owned seeded generators

The stateless `ciglet_rs_random_uniform_from_draw_f32/f64`,
`ciglet_rs_random_normal_from_draws_f32/f64`,
`ciglet_rs_random_white_from_draw_f32/f64` and
`ciglet_rs_random_white_from_draws_f32/f64` accept the caller's integer draws and
inclusive maximum. The division is evaluated in the selected scalar precision
before the original double-precision stages. Bulk white-noise conversion stages
all output before committing, permits source-storage reuse and validates the
maximum and amplitude even for an empty buffer.

Gaussian mode `CIGLET_RS_RANDOM_STABLE` (zero) uses native stable arithmetic;
`CIGLET_RS_RANDOM_LEGACY` (one) preserves original binary32 approximations and the
finite zero-first-draw artifact. The scale parameter is variance, not standard
deviation. Explicit Stable positive-variance transformation rejects a zero first
draw. Owned Stable sampling uses the native bounded retry policy instead.

`CigletRsRandom` owns a Mutex-protected native Generator. Creation uses exactly
32 caller-provided seed bytes and optionally a configured inclusive maximum in
`1..=i32::MAX`. Clone copies the current continuation into an independent owner;
release clears its pointer slot. Maximum does not consume a draw. Draw, uniform,
normal, normal_with and white_noise expose native sampling in both precisions.
Default normal selects Stable. Zero-variance normal still consumes two grid draws;
zero-amplitude white noise consumes one grid draw per output. Empty white-noise
output consumes none. Invalid parameters and pointers are rejected before drawing.
A later numerical failure may advance state while retaining caller output.
The owner must remain alive throughout all calls and may be shared concurrently;
release requires all calls to have finished. Output storage is independent of the
owner and any concurrently accessed storage.

The seeded portable ChaCha8 stream is distinct from platform-dependent C `rand`.
To retain a platform C stream, provide its original integer draws to the stateless
transformations. No global RNG or OS entropy is introduced. The native sampler
performs range sampling and controls its RNG word consumption.
Primary algorithm/seed/clone documentation:
https://docs.rs/rand_chacha/0.10.0/rand_chacha/struct.ChaCha8Rng.html

Boundary tests compare both unchanged 128-record RND1 corpora with their original
bit criteria, verify Stable/Legacy domains, late-invalid-draw output retention,
input/output alias, cloned continuation, pointer/count errors, invalid-input
nonconsumption and concurrent draws from one owner. Independent C/Python callers
compare selected source fixture bits and compare cloned grid draws with scalar and
buffer sampling from the other owner.
Validation checkpoint: all five native targets passed all 61 selected suites:
234 boundary tests and 69 focused native tests, with no failures or ignored tests.
The three independent optimized assert-enabled C callers passed on each target;
the three Python callers passed on each of the three64bit targets. All 22 random
exports are invoked by the new C caller. The complete header contains 598 functions,
27 opaque types, 25 concrete descriptors and 19 policy/flag constants. Clippy,
Rust documentation, formatting and generated-header verification passed.
Native kernels, Cargo manifest/lock and original fixture files remain unchanged.
This checkpoint validates Debug shared-library boundaries; Release, 32bit Python
and static linkage are not claimed here. Full combined acceptance remains required.

### Owned WAVE decoding and borrowed reader callbacks

The binary32/binary64 `ciglet_rs_wave_decode`, `read_stream` and `read_file`
functions publish immutable owned waveforms only on successful decoding. Memory
decoding accepts an explicit u64 byte position; stream decoding begins at the
caller's current stream position. Each entry point accepts a frame budget checked
by the native decoder before allocating samples. Clone retains all metadata and
samples in an independent owner; release clears its slot. Info exposes source
rate, sample width, channel count, encoding and decoded mono count. Copy requires
the exact sample count and retains caller output on error. Stereo retains the left
channel without downmixing. All native decoding corrections are preserved.

`CigletRsReadStream` contains context and synchronous read/seek function pointers.
Callbacks return zero for success and any nonzero status for I/O error. Read may
return fewer bytes than requested, must initialize the reported bytes, and must
report a count no larger than capacity. Count zero indicates EOF. An oversized
count is rejected before native code can use it. Seek origins are Start=0,
Current=1 and End=2. Start uses the unsigned 64bit absolute argument; Current/End
use the signed 64bit relative argument. The unused argument is zero. A successful
seek must report the new absolute position. This keeps the complete native
`SeekFrom` offset domains. Callbacks/context remain valid for the call, must not
unwind or modify the independent output slot, and are neither retained nor closed.
Stream position may change before a decoding/I/O failure. Output pointer validation
precedes callback invocation.

`CigletRsPath` contains counted bytes and an encoding code. UTF8=0 expects UTF-8;
Native=1 accepts lossless Unix OS bytes or Windows UTF-16 little-endian code units,
including potentially ill-formed native Unicode. Paths omit the terminator and
embedded NUL is rejected. Native Windows byte count must be even. Byte copying
avoids alignment requirements for wide code units. Unsupported platforms reject
Native and can use UTF8. Native file opening and buffered reading are delegated to
`wave::read_file`.

Primary adapter and path contracts:
https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
https://doc.rust-lang.org/std/io/enum.SeekFrom.html
https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStringExt.html
https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStringExt.html

Boundary tests retain both unchanged 36-record WAV1 corpora and their documented
C decoding corrections, independently verify metadata/standard sample values,
clone/release and copy errors, frame budget/truncation, current-position decoding,
short reads/EOF/oversized count/callback errors and invalid-output noninvocation.
Actual files use both path encodings, including a non-UTF8 Unix filename and a
Windows supplementary Unicode filename. New independent C/Python callers exercise
all 14 decoding/ownership exports, real files and synchronous short-read callbacks.
PCM encoding/file/borrowed-writer interfaces remain pending and are required for
complete WAVE I/O binding coverage.
Validation checkpoint: all five native targets passed all 63 selected suites:
240 boundary and 74 focused native tests, with no failures or ignored tests.
Four independent optimized assert-enabled C callers passed per target; four Python
callers passed per64bit target. The header contains 612 functions; all 14 new
reader exports are invoked in the independent C caller. Header inventories are
29 opaque types, 28 concrete descriptors and 26 numeric policy/origin codes.
Clippy, Rust documentation generation, formatting and header verification passed.
The Windows C caller emits the SDK's standard fopen deprecation warning; its
portable fixed-name fixture setup and all assertions execute successfully.
Native kernels, manifests/lock and original fixtures remain unchanged. This
checkpoint covers Debug shared-library boundaries; Release, 32bit Python and
static linkage are not claimed here. WAVE writing remains required, along with
the final combined project/binding acceptance.
