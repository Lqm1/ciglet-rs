# Open filterbank configuration findings

Source struct fields and current Rust equivalents:

| C field | Rust ownership mapping | Remaining checks |
| --- | --- | --- |
| nchannel | responses/supports lengths and channels() | Zero-channel source application inspects no samples; current Rust still validates every spectrum value |
| nf | bins() and per-channel response length | Empty-bin source banks need a defined Rust representation/application policy |
| fnyq | nyquist_hz(), empty/mel/plp constructor argument | empty rejects zero/negative metadata even though source empty construction/application does not calculate with it |
| fresp | response() and atomic set_channel() | set_channel currently rejects all nonfinite values, including outside inactive support |
| lower_idx/upper_idx | support() and checked half-open set_channel range | Source application reads only active supports; Rust apply/apply_spectrogram currently reject nonfinite samples in unused bins |

These are source-defined configuration capabilities, not missing numeric
function names. The C empty constructor only assigns fnyq; spectrum/spectrogram
application never reads fnyq, nfft or fs. A zero metadata value can therefore
coexist with finite custom weights and a valid finite result. Arbitrary metadata
storage must be distinguished from the positive-frequency domain of generated
mel/PLP banks. Test field getters/copies/mutation alongside actual results.

Likewise, nonfinite values outside every active support are not read by the
source application. Configurable half-open supports can intentionally exclude
those values. Zero channels and empty supports are additional cases. Blanket
response/spectrum validation can reject those source-defined uses. Determine
the appropriate validation at the point of consumption; do not weaken checks
for invalid values that an operation actually reads.

All findings above were established by direct source inspection of
ciglet.c:1272-1285 and 1338-1364 and current filterbank.rs. They are not yet closed
by corrected Rust code or complete custom-C reference coverage. Existing mel/PLP
construction fixtures do not prove full public-field configuration compatibility.
The next audit should compare manually configured C structs, including signed
weights, unused NaNs/infinities, zero/negative metadata, empty/disjoint supports,
zero channels and spectrum/spectrogram equivalence, before changing acceptance.
