# Native named-field value audit

The public native modules in crates/ciglet-rs/src/lib.rs contain twenty named-field value,
configuration and result types with sixty-eight public fields. The source-derived
set is recorded in native-value-field-bindings.csv. This complements the pinned
upstream five-structure/twenty-field ledger; the two counts describe different
sets and must not be added together as distinct original capabilities.

Run `python tests/audit_native_value_fields.py` to compare every public module's
named-field declarations with the ledger. The checker rejects duplicate types,
missing or extra fields, reordered field snapshots, unsupported declarations
and missing source/evidence paths. It proves inventory integrity only. Referenced
tests and direct source inspection establish the representation contracts.

## Representation contracts

The C interface does not need to allocate an opaque owner for every result.
Kalman estimates, roots, resonances, ordinary prediction, glottal spectra,
envelopes and interpolated peaks expose complete caller-owned planes and scalar
outputs. Complex values use real/imaginary planes; root and resonance diagnostics
include iterations, convergence and maximum residual. These remain editable
caller storage after the call. Configuration arguments or natural C descriptors
carry all native option fields, with checked integer enum/boolean codes.

STFT results have explicit availability bits, dimensions, optional scalar
metadata and complete requested magnitude/phase planes. Zero normalization hop
in the C request means absence; a native/WASM explicitly present zero is invalid
on calculation. This is a validation representation difference rather than an
omitted valid analysis configuration.

C waveform owners are immutable decoded results. Metadata and sample-copy calls
expose every field, and caller-owned copies are editable. They do not supply an
arbitrary waveform-owner constructor or in-place owner setters. Native/WASM Wave
values do. Native encoding writes mono sample slices with sample-rate and full
WriteOptions rather than consuming source Wave metadata; this remains available
through the C write interfaces. The ledger discloses this distinction instead of
claiming interchangeable mutable-owner APIs.

WASM value owners have complete arbitrary constructors and field replacement.
Typed storage helpers preserve scalar IEEE payloads where JavaScript Number
conversion cannot retain every bit. Detector validation/temporary-edit boundaries
remain described in public-field-binding-audit.md. Private plan allocations and
tuple Error storage are outside this named-field set; their functional API and
error translation remain in the existing inventories. Enum variants and bundled
approximation/build mappings retain their separate configuration audits.

## Verification scope

The current Windows x86_64 run passes seventy-four tests across twelve selected
native/C ABI suites: band cepstra, correlogram, Kalman, polynomial, prediction
estimation/resonances, peaks, STFT, synthesis, waveform reading/writing and native
envelopes. Exported C functions in those suites are called from Rust; this is not
a new external C/Python execution or a five-platform refresh. The separate
c_api_envelope suite exercises C output descriptors and transactions.

Actual Node runs pass the six existing Kalman, polynomial, prediction, STFT,
wave and envelope verifiers using the retained scalar-checkpoint modules. Native
and WASM implementation sources have not changed since those modules were
generated. These runs include complete result/option storage, independent clones,
failure behavior and original C corpora. The previous forty-family actual browser
run remains separate evidence; no new browser run is claimed here.

This records all current native named-field representations. It does not promote
function inventories or establish complete end-to-end acceptance. Current-source
native execution on the required Windows/Linux platforms, external callers,
combined three-package workflows and remaining host-tool gates are still required.
