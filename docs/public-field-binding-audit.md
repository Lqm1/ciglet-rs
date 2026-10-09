# Own-header field binding audit

The pinned upstream ciglet.h at commit
895ba9b1c0eabee83d3544208bbc82420efa3206 declares five public structures and
twenty fields. Every field has an explicit Native/C/WASM mapping in
public-field-bindings.csv. This complements public-configuration-audit.md and
the function inventories. The ledger is not a replacement for runtime evidence.

Run `python tests/audit_public_fields.py /path/to/pinned/ciglet.h` to compare the
actual typedef declarations with the ledger and verify every source/evidence
path. It rejects missing/duplicate/extra fields. It does not establish behavior
from method names or passing file-existence checks.

| Type | Fields | Native/C/WASM representation |
| --- | --- | --- |
| cplx | real, imag | Native Complex re/im, natural C scalar descriptors and independent WASM component owners; typed component arrays preserve arbitrary payloads |
| ifdetector | fc, nh, hr, hi, hdr, hdi | Editable center frequency and four impulse-response planes; length follows owned vectors and checked construction/replacement |
| filterbank | nchannel, nf, fnyq, fresp, lower_idx, upper_idx | Owned dimensions, arbitrary Nyquist metadata, full channel responses and half-open supports with checked replacement |
| lfmodel | T0, te, tp, ta, Ee | Five mutable native/configuration values; C arrays retain every field in source order and WASM exposes both properties and typed component storage |
| figure | handle | Native Figure lifetime or caller-owned Write transport; C synchronous context/write/flush callbacks; WASM arbitrary byte writer storage under ADR 0007 |

## Ownership and validation

Counts and allocation pointers cannot be changed independently inside an owned
detector or filterbank. A caller reconstructs a differently sized object from
complete storage, as already documented in the native configuration audit.
Valid computational configurations and custom response data remain accessible.

Native kernels_mut and the WASM kernel setter can retain temporarily invalid
edits, which estimation checks before use. C detector replacement validates the
complete four equal-length, nonempty vectors before changing the owner. C callers
retain and edit their own temporary planes until a complete replacement is valid.
The ledger discloses this boundary; it does not claim identical invalid-storage
states across all interfaces. Field access is not reduced to generated filters.

Filterbank metadata changes do not regenerate responses. Half-open support and
full response storage remain separately observable, with active scalar validation
on channel replacement. Inactive arbitrary IEEE storage survives independent
snapshots and clones. LF raw configuration remains mutable, with physical
validation on preparation/calculation. Arbitrary plot bytes are retained without
being restricted to the provided line/image helpers. Process execution remains
native; browser plotting produces bytes for a host renderer.

## Current evidence and limits

Current Windows x86_64 Native/C ABI tests pass 44 tests across complex,
instantaneous_frequency, filterbank, glottal and c_api_plot_commands suites.
These include both precisions, original C corpora, complete custom configuration,
payload storage, cloning, overlaps and failure transactions. The C ABI cases
here call exported C functions from Rust; they are not new external C/Python
process executions. External callers and actual Gnuplot execution retain their
separate recorded evidence and final refresh requirements.

Actual Node and browser runs pass all forty existing WASM family verifiers once
in each runtime, including full configuration/payload/ownership checks for these
five structures. The generated scalar checkpoint modules contain the current
implementation; the subsequent all-family commit changed tests only. The first
Node attempt selected an older plotting module and failed on the later scalar
export. That failed artifact is excluded from the accepted results.

This closes the explicit own-header structure mapping. It does not alone prove
all Rust-added value/result/options types, installed bundled fastapprox storage
helpers, every macro/build configuration or final native platform execution.
Their existing mappings still require the remaining field/source acceptance
review and the final five-target and combined three-package gates. Full original
functionality remains the objective; inventory counts are not completion claims.

The complementary native named-field set is now recorded in
native-value-field-bindings.csv and native-value-field-audit.md. Its checker
compares all public native module sources with twenty types and sixty-eight
fields, including caller-owned C result representations and immutable Wave
owner boundaries. Final execution gates remain separate.
