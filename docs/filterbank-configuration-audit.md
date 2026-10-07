# Filterbank configuration compatibility

Source struct fields and current Rust equivalents:

| C field | Rust ownership mapping | Remaining checks |
| --- | --- | --- |
| nchannel | responses/supports lengths and channels() | Zero channels inspect no spectrum values and return no output |
| nf | bins() and per-channel response length | Zero-bin banks support individual spectra and explicit frame counts through apply_frames() |
| fnyq | nyquist_hz(), set_nyquist_hz(), constructor argument | Custom metadata accepts any scalar; changing it leaves application results unchanged |
| fresp | response() and atomic set_channel() | Signed weights and nonfinite values outside the support are retained; active nonfinite weights return errors |
| lower_idx/upper_idx | support() and checked half-open set_channel range | Empty/disjoint supports are retained; application validates only the samples it reads |

These are source-defined configuration capabilities, not missing numeric
function names. The C empty constructor only assigns fnyq; spectrum/spectrogram
application never reads fnyq, nfft or fs. A zero metadata value can therefore
coexist with finite custom weights and a valid finite result. Arbitrary metadata
storage is distinguished from the frequency domain of generated banks. Mel
generation requires positive finite Nyquist and at least one bin. Source PLP
generation also defines finite zero/negative Nyquist values, so these remain
available. PLP banks with no bins or channels do not consume Nyquist, and can
retain nonfinite metadata. Otherwise PLP generation rejects nonfinite Nyquist.

Likewise, nonfinite values outside every active support are not read by the
source application. Configurable half-open supports can intentionally exclude
those values. Zero channels and empty supports are additional cases. Blanket
response/spectrum validation can reject those source-defined uses. Determine
the appropriate validation at the point of consumption; do not weaken checks
for invalid values that an operation actually reads.

The identified gaps above are covered by tests/reference/custom_filterbank_oracle.c,
which calls the unchanged source constructor/application/destructor routines.
Neither nfft nor fs is read by these source application routines. The oracle
passes deliberately unrelated values for these arguments rather than adding
unused parameters to the Rust API.

Windows Zig and Linux GCC regenerate byte-identical C fixtures. All 157
integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64 and Linux
GNU x86_64/i686, including required native Gnuplot rendering. Custom float
outputs have zero measured difference. The maximum double normalized difference
is 1.856135104261755e-16, within the tested 2e-15 allowance for host pow behavior.
Formatting, Clippy and warning-as-error Rustdoc pass. SHIRO's locked Windows
MSVC x86_64 suite also passes all 58 tests, including both real-audio workflows.

The corpus has 33 records and 672 application outputs per precision. It covers
signed weights, unread NaNs/infinities, five metadata values, empty/disjoint
supports, zero channels, zero bins, negative/zero-frequency PLP generation and
three frames under both logarithmic and PLP compression. Rust tests also check
metadata mutation, cloning, response retention, atomic failed updates, active
nonfinite values, shape/size errors and generated-bank domains. No undefined
floating result is accepted as a finite valid application result.

A flattened empty slice cannot distinguish zero-bin frames from zero frames.
apply_frames(slice, frames, compression) preserves the source's explicit nfrm.
apply_spectrogram requires nonzero bins so it can infer the frame count.

Allocation dimensions use checked multiplication before slicing. Output chunk
iteration is entered only for a nonzero channel count, following the standard
library's documented chunk-size precondition. References:
[usize::checked_mul](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_mul)
and [slice::chunks_exact_mut](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact_mut).
Context7 and Mintlify were queried before consulting the primary pages.

Direct source inspection: ciglet.c:1272-1294 and 1338-1364. These checks close
the listed custom-configuration gaps. They do not establish complete generated
Mel-option, numerical-stability, build-configuration or dependency coverage.
