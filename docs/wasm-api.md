# WebAssembly interface

The optional `wasm` feature exposes bindings from the existing library crate.
It uses `wasm-bindgen` 0.2.129 and `js-sys` 0.3.106 on
`wasm32-unknown-unknown`. The CLI must match the `wasm-bindgen` crate version.
The declared Rust requirements are 1.81 for the libraries and 1.86 for the CLI;
the current pinned Rust 1.94 builds both successfully.

```sh
cargo build --locked --lib --features wasm --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/ciglet_rs.wasm --target web --out-dir pkg-web
wasm-bindgen target/wasm32-unknown-unknown/debug/ciglet_rs.wasm --target nodejs --out-dir pkg-node
node tests/wasm_window_node.cjs ./pkg-node
```

The web output is an ES module with explicit asynchronous initialization. Node
output uses CommonJS. Neither requires a bundler. Serve the browser test over
HTTP with `/bindings/` pointing to the web output and `/tests/` to the existing
test directory; open `tests/wasm_window_browser.html`. The shared checks run in
both actual runtimes. Generated packages are build artifacts, not extra Rust
crates or repositories.

## Window operations

`WindowKind` covers Boxcar, Hann, Hamming, Blackman, BlackmanHarris, Nuttall98
and MltSine. `WindowPrecision` covers Accurate and Approximate. `window_f32`
and `window_f64` return independent `Float32Array` and `Float64Array` results.
Window lengths use the WebAssembly unsigned 32-bit integer ABI; callers must
pass integers in that range. Native legacy length validation remains enforced.

Four custom functions expose every callback/output precision combination:
`window_custom_f32_f32`, `window_custom_f32_f64`, `window_custom_f64_f32` and
`window_custom_f64_f64`. The callback receives radians and must return a finite
number representable at its selected precision. Empty, singleton and Boxcar
windows bypass the callback. Thrown JavaScript values retain their identity,
including `undefined` and `null`. Errors stop callback consumption at the failing
call; previous external callback effects are not rolled back. Blackman custom
callbacks retain the documented native correction.

Actual Node.js 24.14.1 and the Chromium-based in-app browser pass all six
functions against the unchanged C fixtures: 224 built-in cases, 120 custom
cases and 35,132 scalar comparisons. Built-in tolerances remain 1e-7 for f32
and 1e-15 for f64; custom results compare exact bits. Additional checks cover
degenerate windows, original thrown-value identity, invalid callback values,
draw counts, range errors, continued instance use and independent typed arrays.
WASM Clippy and Rustdoc with warnings denied pass. The six existing native
window regression tests and host all-target/all-feature Clippy also pass.
An actual feature-disabled WASM module exposes none of the six functions.
Every existing locked package is retained; only optional WASM packages were
added. The inventory records nineteen complete window mappings. Fourier adds
six mappings, DCT/phase adds fifteen, vectors add six and interpolation adds
seven; resampling adds two, filtering adds eight and statistical arithmetic adds
nine; ordering adds eight, median/smoothing adds five and frequency/distance adds
seven; frame/extrema adds seven, filterbanks add twelve and scalar approximation
adds thirty-six; hyperbolic scalar and lane kernels add sixteen and trigonometric
scalar and lane kernels add twenty; special scalar and lane kernels add
twenty-eight; basic lane kernels add ten. Other DSP families and both remaining crates still
require complete WASM interfaces and final combined acceptance.

Ordinary validation errors become JavaScript exceptions. A WebAssembly panic
abort is not recovered by these interfaces. No native filesystem, process or
plotting operation is implicitly emulated in the browser.

## Fourier operations

`FftPlanF32` and `FftPlanF64` retain both native plans and reusable scratch
storage. Constructors require a positive power-of-two length within the native
legacy range. `length` and `is_empty()` expose the complete native plan queries.
`transform(real, imaginary, direction)` accepts optional typed-array planes.
`null` or `undefined` means a zero plane; a present empty array is checked as
a present mismatched input. `process(interleaved, direction)` accepts alternating
real/imaginary input. Boundary copies keep every input array unchanged; the
native calculation reuses the plan and scratch. Inverse FFTs divide by length.

`fft_f32` and `fft_f64` provide equivalent one-shot transforms.
`inverse_dft_f32` and `inverse_dft_f64` retain the native quadratic normalized
inverse DFT for arbitrary positive legacy-range lengths. `fft_shift_f32` and
`fft_shift_f64` retain odd/even/empty ordering. `FftDirection` is Forward or
Inverse. The direction parameter uses these numeric constants through the
unsigned 32-bit ABI and is validated within the Rust `Result` path. Invalid
direction codes throw an ordinary exception without retaining the plan's
borrow, so queries, later transforms and `free()` remain usable. Direct enum
ABI conversion was observed to retain the plan borrow on an invalid code;
this validation also covers both one-shot transforms.

`ComplexValuesF32` and `ComplexValuesF64` own complete complex results. Their
constructors accept equal-length real and imaginary planes, including empty
arrays and arbitrary nonfinite values. `length`, `real`, `imaginary` and
`interleaved` expose all fields. Getters return independent typed arrays. Call
`free()` on plans and result owners when finished; copied arrays remain valid.

Actual Node.js and browser verification compares all 720 original C FFT/IDFT
cases, including all missing-plane combinations and normalization modes.
Each FFT case checks one-shot, reusable-plane and reusable-interleaved paths,
for 559,776 scalar comparisons. Existing normalized bounds remain 2e-7 for f32
and 1e-15 for f64. Tests also check repeated reuse, invalid shapes, immutable
inputs, result/plan lifetimes, original frequency ordering, independent getters,
and exact arbitrary scalar bits including quiet/signaling NaN payloads,
infinities, signed zero and subnormals. Window checks pass on the new artifact.
Eight native and three C-interface Fourier regression tests pass. The full
original Node/browser corpus also passes after testing invalid
direction codes 2, 9 and 0xffffffff on both reusable methods, checking queries
and successful transforms on the same plan after each error. Both one-shot
functions reject the same invalid codes. Filterbank Node checks pass on this
artifact. This correction changes no native algorithms or comparison bounds.
WASM Clippy and Rustdoc with warnings denied pass; an actual feature-disabled module
contains none of the six functions or four owner classes.

## DCT and phase operations

Eighteen functions expose both precisions of the complete native DCT and phase
operations: `dct`, `phase_wrap`, `phase_unwrap`, `phase_difference`, `magnitude`,
`argument`, `polar_real`, `polar_imaginary` and `complete_spectrum`, with `_f32`
or `_f64` suffixes. DCT is the original directly computed orthonormal DCT-II
with approximate binary32 cosine in both precisions. The upstream public API
has no inverse DCT or reusable DCT plan.

Magnitude/argument accept complete real and imaginary typed-array planes;
one-element planes implement the scalar complex aliases. Polar conversion
retains approximate trigonometric stages. Spectrum completion accepts
`SpectrumSymmetry` and optional `SpectrumNyquist`; omitted or null policy
preserves supplied DC/Nyquist values. Legacy policy retains the original
preceding-bin substitution, including positive antisymmetric Nyquist. Inputs
remain unchanged and outputs own independent storage. Native validation,
single-turn unwrap, directed differences and scaled magnitude fallbacks remain.

Actual browser and Node checks compare all 64 DCT and 48 phase C cases, plus
boundary cases, for 6,830 scalar comparisons. Original DCT normalized bounds
remain 2e-7 for f32 and 1e-15 for f64. Phase comparisons remain bit-exact except
binary64 argument: on the pinned WASM backend, 22 original `atan2` results
differ by one ULP, at most 4.440892098500626e-16 absolute error. The full-corpus
measurement established a one-ULP bound only for that operation and precision;
signs and zeros remain exact. Native Rust calculations and C fixtures were not
changed. Rust 1.94 documents `atan2` precision as platform-dependent, so this
bound is evidence for the tested backend/corpus rather than a universal math
guarantee. Tests include both Nyquist policies, ignored lower bins, empty and
singleton cases, errors, immutable inputs, independent outputs, magnitude range
fallbacks and continued instance use.

Window/Fourier Node checks pass on the new artifact. Thirteen native/C-interface
regression tests pass. WASM Clippy and Rustdoc with warnings denied pass, and an
actual feature-disabled module exposes none of the eighteen functions. Fifteen
upstream DCT/phase mappings are recorded, bringing WASM coverage to forty
implemented mappings; the remaining 292 public mappings stay pending.

## Vector operations

Eleven functions expose the complete native vector module and scalar
interpolation: `difference`, `cumulative_sum`, `reverse`, `linear_grid` and
`interpolate_between` with `_f32`/`_f64` suffixes, plus `integer_sequence`.
Floating arrays return independent `Float32Array`/`Float64Array` storage;
integer progressions return `Int32Array`. Integer arguments use the signed
32-bit ABI and lengths use the unsigned 32-bit ABI. Callers must supply integers
in those ranges; native allocation limits and checked arithmetic remain.

Differences retain the first sample, cumulative sums retain source operation
order and the first zero sign, and reversal preserves arbitrary scalar bits.
Inclusive grids retain multiply-before-divide rounding, with native finite
fallbacks for overflowing endpoint differences. Scalar interpolation supports
extrapolation. Integer sequences use wide intermediate arithmetic and reject
results outside the signed 32-bit range.

Actual Node.js and browser checks pass 48 original C vector cases, 36 grid and
scalar-interpolation cases, and 18 integer-sequence cases. The 2,938 scalar
comparisons are exact, including signed zeros. Additional checks cover NaN
payloads and infinities in reversal, immutable inputs, independent outputs,
empty/singleton inputs, range fallbacks, overflow errors and continued use.
The shared tests are `wasm_vector_checks.mjs`, with
`node tests/wasm_vector_node.cjs ./pkg-node` and
`tests/wasm_vector_browser.html` as runtime entry points.

Window, Fourier and DCT/phase Node checks also pass on this artifact. Thirteen
native/C-interface regression tests pass. WASM Clippy and Rustdoc with warnings
denied pass; all eleven functions are absent from an actual feature-disabled
module. Six upstream mappings bring coverage to 46 implemented mappings;
286 public mappings remain pending. Native implementations, C interfaces,
dependencies and C fixtures remain unchanged.

## Interpolation operations

Twelve additional functions complete the native interpolation API alongside the
previous scalar `interpolate_between` bindings. Both precisions expose
`interpolate_linear`, `interpolate_uniform`, `interpolate_sinc`,
`interpolate_sinc_with`, `interpolate_sinc_with_kernel` and
`interpolate_fill_missing`, each with `_f32`/`_f64` suffixes. Results own
independent numeric typed arrays and inputs remain unchanged.

Linear interpolation accepts strictly increasing coordinates, unordered queries
and clamped endpoints. Uniform coordinates occupy `[start, end)` and retain the
original near-final-sample clamp. Four-lobe Lanczos interpolation exposes all
native boundary/kernel choices through `BoundaryPolicy` and `KernelPolicy`.
The default includes sample zero and uses the corrected near-zero kernel;
`interpolate_sinc_with` selects the boundary with the original kernel;
`interpolate_sinc_with_kernel` selects both independently. These are the native
correction and compatibility policies, not new WASM behavior.

Missing-value interpolation supports exact sentinels, NaN sentinels, repeated
nearest known values outside the known range and the all-missing zero fallback.
Ordinary native validation and arithmetic failures become JavaScript exceptions.
Arrays remain independent across calls, including after validation errors.

Actual Node.js and browser checks pass all 48 original C linear cases and 112
disclosed-corrected C extended cases from both precision/boundary corpora.
The 9,282 scalar comparisons are exact. Default and explicit stable Lanczos
calls also reproduce the included-boundary corpus. Additional checks cover an
independent analytic near-zero kernel limit, all four policy combinations,
NaN/infinity sentinels, signed zeros, unordered and empty queries, singleton
sources, invalid coordinates/policies, overflow, input immutability, independent
outputs and reuse after errors. Runtime entry points are
`node tests/wasm_interpolation_node.cjs ./pkg-node` and
`tests/wasm_interpolation_browser.html`, sharing `wasm_interpolation_checks.mjs`.

All previously implemented WASM families pass their Node checks on this artifact.
Seventeen native/C-interface interpolation regression tests pass, as do WASM
Clippy and Rustdoc with warnings denied. All twelve functions are absent from an
actual feature-disabled module. Seven upstream mappings bring WASM coverage to
53 implemented mappings; 279 public mappings remain pending. Native source,
C interfaces, dependencies and original comparison fixtures remain unchanged.

## Resampling operations

Eight functions expose both precisions of `resample`, `resample_with`,
`resample_with_kernel` and `resample_output_length`, with `_f32`/`_f64` suffixes.
They use the existing `BoundaryPolicy`/`KernelPolicy` enums. The default includes
sample zero and uses the corrected kernel; the boundary-only function retains
original kernel arithmetic, and the full function selects both independently.
The ratio is output rate divided by input rate. Downsampling retains the native
32-tap Hamming FIR and 16-sample delay compensation. Results own independent
typed arrays; inputs remain unchanged.

Output sizing uses native source-precision multiplication followed by positive
half-away rounding, with checked legacy bounds. Ratio one retains exact input
sample bits and count, including large counts that cannot be represented exactly
as binary32. The sizing functions allocate no sample arrays. Counts must be
unsigned 32-bit integers; native validation further restricts supported lengths.

Actual Node.js and browser checks pass all 210 original C resampling records,
including downsampling and upsampling, for 17,098 scalar comparisons across
corpus and boundary checks. Original normalized C bounds remain 2e-7 for f32
and 2e-15 for f64; convenience functions and input preservation compare exact
values. Tests cover all four policies, fractional rates, independent near-integer
impulse limits, retained Legacy behavior, sample-zero inclusion/omission, signed
zero identity, length rounding, empty outputs, invalid ratios/enums/nonfinite
samples, size overflow, independent outputs and reuse after errors. Entry points
are `node tests/wasm_resampling_node.cjs ./pkg-node` and
`tests/wasm_resampling_browser.html`, sharing `wasm_resampling_checks.mjs`.
The FIR/convolution portions of the shared corpus are parsed here but are not
claimed as exposed WASM filtering functions.

All previous WASM families pass Node checks on this artifact. Thirteen related
native/C-interface tests pass. WASM Clippy and Rustdoc with warnings denied pass;
all eight functions are absent from an actual feature-disabled module. Two
upstream mappings bring coverage to 55 implemented mappings; 277 public mappings
remain pending. Native source, C interfaces, dependencies and original fixtures
remain unchanged.

## Filtering operations

Sixteen functions complete the native filtering API in both precisions:
`windowed_fir`, `fir`, `bandpass_fir`, `convolve`, `filter`, `filter_with`,
`forward_backward_filter` and `forward_backward_filter_with`, with `_f32`/`_f64`
suffixes. `FilterPassband` selects Lowpass, Highpass, Bandpass or Bandstop and
the existing `WindowKind` supports all seven windows. `FilterStartup` selects
ZeroState or LegacyZeroPrefix. All computations delegate to the native API.

Windowed FIR cutoffs use cycles per sample; the simpler FIR functions normalize
cutoffs to Nyquist. Lowpass/highpass use only the lower cutoff. Fractional-bin
weights, even-length Nyquist replacement and accurate window multiplication
remain unchanged. Convolution returns the full linear output in original
coefficient accumulation order, or an empty array for empty operands.

Recursive filters retain the original monic denominator convention: denominator
element zero is ignored, without coefficient normalization. Default startup is
the native corrected zero state; LegacyZeroPrefix preserves the skipped leading
samples in equal-length coefficient fast paths. Forward/backward filtering
retains the simple two-pass workflow without endpoint padding. Every output owns
independent typed-array storage; inputs remain unchanged.

Actual Node.js and browser checks pass 600 original C FIR cases, 120 convolution
cases and 420 original/disclosed-corrected C recursive-filter cases. The 52,526
scalar comparisons cover both startups, causal/two-pass calls and their defaults.
Recursive results and convenience wrappers compare exact values; original FIR
and convolution normalized bounds remain 2e-7 for f32 and 2e-15 for f64.
Additional checks cover every window/passband combination, independent impulse
recurrence and convolution, monic behavior, short/empty inputs, ignored cutoff,
shape/range/enum/nonfinite errors, recursive overflow, unchanged inputs,
independent outputs and reuse after errors. Entry points are
`node tests/wasm_filtering_node.cjs ./pkg-node` and
`tests/wasm_filtering_browser.html`, sharing `wasm_filtering_checks.mjs`.

All earlier WASM families pass their Node checks on this artifact. Thirteen
native/C-interface regression tests pass. WASM Clippy and Rustdoc with warnings
denied pass, and all sixteen functions are absent from an actual feature-disabled
module. Eight upstream mappings bring coverage to 63 implemented mappings;
269 public mappings remain pending. Native implementations, C interfaces,
dependencies and comparison fixtures remain unchanged.

## Statistical arithmetic

Twenty-four functions expose both precisions of `sum`, `sum_squares`, `mean`,
`mean_with`, `variance`, `variance_with`, `dot`, `covariance`, `covariance_with`,
`correlation`, `correlation_with` and `cross_correlation`, with `_f32`/`_f64`
suffixes. `StatisticsPolicy` selects the complete Stable/Legacy policies.
Defaults use the native compensated, centered and scaled binary64 moments;
explicit Legacy retains original selected-precision reductions and cancellation.
Population variance and Pearson correlation retain native range/constant-input
errors. Stable correlation preserves scaled information even if a standalone
variance would overflow or round to zero.

Sum, sum of squares, dot and cross correlation retain IEEE nonfinite behavior
and positive-zero reduction identities. Cross correlation is unnormalized,
uses nonnegative lags and zero-fills excess lags. Lag counts must be unsigned
32-bit integers within native validation bounds. Returned arrays own independent
storage and every input remains unchanged.

Actual Node.js and browser checks pass 64 original C statistic cases and 96
independent rational-moment cases, for 4,255 comparisons. Legacy finite results
compare exact values and nonfinite outputs retain the source classifications.
Stable reference bounds remain 2e-7 for f32 and 4e-15 for f64; zero results are
exact. Default and explicit Stable calls agree exactly. Additional checks cover
extreme constant moments, scaled correlation, original overflow, positive zero,
empty/mismatched vectors, constant correlation errors, nonfinite inputs,
invalid policies/lags, independent results and use after errors. Entry points
are `node tests/wasm_statistics_node.cjs ./pkg-node` and
`tests/wasm_statistics_browser.html`, sharing `wasm_statistics_checks.mjs`.

All previous WASM families pass their Node checks on this artifact. Seventeen
native/C-interface regression tests pass. WASM Clippy and Rustdoc with warnings
denied pass; all twenty-four functions are absent from an actual feature-disabled
module. Nine upstream mappings bring coverage to 72 implemented mappings;
260 public mappings remained pending at that checkpoint. The ordering interface
below completes minimum, maximum, median, selection and sorting. Native source,
C interfaces, dependencies and comparison
fixtures remain unchanged.

## Ordering and selection

Ten functions complete the native statistics API: `minimum`, `maximum`,
`median`, `select_nth` and `sort` with `_f32`/`_f64` suffixes. Empty minimum
and maximum return `undefined`; a present NaN remains a numeric NaN. Comparison
folds retain native second-operand ties and NaN behavior. Median uses IEEE total
ordering and native finite overflow-safe averaging.

Selection accepts a mutable typed array and updates it in place through the
documented numeric-slice copyback interface. It returns the selected scalar and
retains the complete native partition in the original array. Invalid indices
fail before mutation. Callers must pass an unsigned 32-bit index. Sorting leaves
inputs unchanged and returns `SortedValuesF32` or `SortedValuesF64`, with full
`values`, `indices` and value `length` getters. Original indices use Uint32Array,
matching WASM's 32-bit usize. Getters copy independent arrays that outlive
`free()`. Constructors retain both complete fields, including arbitrary scalar
bits and indices, without normalizing their contents or lengths.

Actual Node.js and browser tests pass all 64 original/disclosed-sort-corrected
C cases and 848 corpus selections, with 43,514 exact comparisons. Every
selection checks copyback, partition invariants and the complete value multiset.
Additional checks cover signed zero, quiet/signaling and signed NaN payloads,
infinities, subnormals, stable tie indices, optional absence versus present NaN,
finite median overflow, unchanged shared inputs, atomic invalid selection,
independent getters, complete constructors and arrays retained after owner
release. Scalar NaN results have JavaScript Number semantics; exact payloads are
verified through the returned or updated typed-array storage. Runtime entry
points are `node tests/wasm_ordering_node.cjs ./pkg-node` and
`tests/wasm_ordering_browser.html`, sharing `wasm_ordering_checks.mjs`.

Statistical arithmetic checks pass on the new artifact. Seventeen native/C
regression tests, WASM Clippy and Rustdoc with warnings denied pass. All ten
functions and both classes are absent from an actual feature-disabled module.
Eight mappings bring coverage to 80 implemented mappings; 252 public mappings
remain pending. Native source, C interfaces, dependencies and fixtures remain
unchanged.

## Median filtering and smoothing

Ten functions expose `median_filter`, `moving_average`, `moving_average_with`,
`moving_rms` and `moving_rms_with` in both precisions. `SmoothingPolicy` selects
Stable or Legacy for average/RMS computations. Every call delegates to the
unchanged native implementation and returns independently owned typed arrays.

Median filtering accepts finite samples and retains the original growing-prefix
and shrinking-suffix endpoint windows. Positive integer widths are bounded;
nonempty signals require width no greater than signal length. Empty signals
return empty after validating the width. Finite even-window medians retain the
overflow-safe averaging correction.

Stable averages use compensated sample-cell windows with fractional half-widths;
Stable RMS uses scaled squares and integer half-widths. Endpoints are replicated.
Zero average width returns an unchanged copy, and zero RMS width returns
absolute values. Positive tiny average widths retain the native adjacent-cell
limit. Legacy retains original cumulative-grid or square-average-root arithmetic,
including its validated overflow errors. Integer widths must be unsigned 32-bit
values within the native legacy bounds.

Actual Node.js and browser tests pass 192 original C median cases, 120 original
C Legacy smoothing cases and 660 independent high-precision integral-moment
cases. The 77,116 comparisons include complete source immutability and exact
default/explicit Stable agreement. Original median/Legacy comparisons remain
exact. Stable relative bounds remain 2e-7 for f32 and 4e-15 for f64, with only
the existing one-quantum allowance for subnormal RMS reference values. Additional
checks cover endpoint rules, fractional DC preservation, tiny widths, signed
zero, empty signals, finite extremes, invalid/nonfinite widths or inputs,
Legacy overflow, invalid policies, independent outputs and reuse after errors.
Entry points are `node tests/wasm_smoothing_node.cjs ./pkg-node` and
`tests/wasm_smoothing_browser.html`, sharing `wasm_smoothing_checks.mjs`.

Seventeen related native/C-interface tests, WASM Clippy and Rustdoc with warnings
denied pass. All ten functions are absent from an actual feature-disabled module.
Five upstream mappings bring coverage to 85 implemented mappings; 247 public
mappings remain pending. Native/C implementations, dependencies, existing WASM
families and comparison fixtures remain unchanged.

## Frequency conversions and spectral distance

Eighteen functions complete both native modules in both precisions. Frequency
operations are `frequency_to_mel`, `mel_to_frequency`, `frequency_to_bark`,
`bark_to_frequency`, `equal_loudness` and `mel_space`, with `_f32`/`_f64` suffixes.
Mel conversions retain original approximate binary32 logarithm/exponential
stages, even with binary64 inputs. Bark conversions use native hyperbolic
functions; equal loudness retains selected-precision powers. Mel grids return
intervals plus one independently owned typed-array points, preserving native
scalar interpolation. Intervals must be unsigned 32-bit integers within native
positive legacy bounds.

Distance operations are `itakura_saito`, `log_itakura_saito` and
`log_itakura_saito_with`, with both precision suffixes. `DistancePolicy` selects
Stable or Legacy. Stable scaled log accumulation retains finite extreme-ratio
log distances, adjacent-value divergence and native zero/infinity semantics.
Identical positive spectra have zero mean divergence and negative-infinite log
distance; a single zero against a positive bin gives infinity, while paired
zeros are undefined. Legacy retains original approximate-log and selected-scalar
accumulation. Nonempty equally sized nonnegative finite arrays remain required.

Actual Node.js and browser tests pass 24 original C frequency cases, ten Mel
grids and 106 Legacy distance cases, with 2,438 comparisons including source
preservation. Frequency normalized bounds remain 1e-7 for f32 and 1e-15 for f64;
Mel grids and Legacy distances compare exact values. Additional independent
analytic tests cover divergence/log divergence and scale invariance, stable
default agreement, identical/zero spectra, adjacent values, extreme ratios,
signed Bark round trips, invalid domains/shapes/policies/counts, unchanged
spectra, independent grids and reuse after errors. The separate approximation
fields in the frequency corpus are parsed without claiming their WASM coverage.
Entry points are `node tests/wasm_frequency_distance_node.cjs ./pkg-node` and
`tests/wasm_frequency_distance_browser.html`, sharing
`wasm_frequency_distance_checks.mjs`.

Thirteen related native/C-interface tests, WASM Clippy and Rustdoc with warnings
denied pass; all eighteen functions are absent from an actual feature-disabled
module. Seven mappings bring coverage to 92 implemented mappings; 240 public
mappings remain pending. Native source, C interfaces, dependencies, existing
WASM families and comparison fixtures remain unchanged.

## Frames and extrema

Six functions complete both native modules in both precisions: `fetch_frame`,
`extremum` and `peak`, with `_f32`/`_f64` suffixes. `ExtremumDirection` selects
Maximum or Minimum. Frames use the native centered start convention and positive
zero padding outside the input; copied samples retain arbitrary scalar bits.
Center is a signed 32-bit integer in WASM, while length and inclusive range
endpoints are unsigned 32-bit integers. Native allocation and checked-index
limits remain enforced. Returned frames own independent typed-array storage.

Global extremum search retains the first winning index and first-index ties.
Peak search retains strict two-sided neighbor comparisons, endpoint skipping
and the original starting-index fallback. Only inspected samples require finite
values, including neighbors inspected by peak search. Inputs remain unchanged.

Actual Node.js and browser checks pass all 50 original C frame cases and 360
original C search cases, with 13,672 exact comparisons. Additional checks cover
empty/odd/even/distant frames, complete quiet/signaling NaN payloads, infinities,
signed zero and subnormal frame storage, positive-zero padding, signed center
overflow, invalid lengths/ranges/enums, tie handling, singleton/fallback behavior,
nonfinite samples outside the inspected region, immutable sources, independent
frames and reuse after errors. Runtime entry points are
`node tests/wasm_frame_extrema_node.cjs ./pkg-node` and
`tests/wasm_frame_extrema_browser.html`, sharing `wasm_frame_extrema_checks.mjs`.

Ten related native/C-interface tests, WASM Clippy and Rustdoc with warnings
denied pass; all six functions are absent from an actual feature-disabled module.
Seven upstream mappings bring coverage to 99 implemented mappings; 233 public
mappings remain pending. Native source, C interfaces, dependencies, existing
WASM families and comparison fixtures remain unchanged.

## Filterbanks

`FilterBankF32` and `FilterBankF64` expose every native bank operation. Their
constructors create custom zero-filled banks; `mel`, `mel_default` and `plp`
construct generated banks. `MelOptionsF32` and `MelOptionsF64` expose all four
mutable native fields, `minimum_hz`, `maximum_hz`, `scale` and
`minimum_width_hz`. Mel construction copies these fields; the configuration
owner can be reused, changed or freed independently of the resulting bank.

The bank exposes `bins`, `channels`, `nyquist_hz`, `set_nyquist_hz`, `response`,
`support`, `set_channel`, `cloned`, `apply`, `apply_spectrogram` and
`apply_frames`, plus generated `free()`. Responses copy complete scalar storage
into independent typed arrays; supports return independent `Uint32Array`
start/end pairs. Missing channels return `undefined`. Mutation retains native
atomic validation, signed weights and nonfinite inactive values. Metadata is
inert and may be nonfinite. Clones retain independent responses and supports.

Compression accepts the numeric values `FilterBankCompression.Logarithmic`
and `FilterBankCompression.Plp`. Values are checked within the Rust method and
invalid values return ordinary errors, keeping the owner usable. Using direct
enum ABI conversion was observed to bypass borrowed-owner cleanup after an
invalid value on the pinned backend. The numeric ABI uses unsigned 32-bit
integers; pass integer compression codes and dimension/support/frame counts
within that range. Native shape, support and overflow checks remain enforced.

All three application methods retain frame-major output and native arithmetic.
Explicit frame counts support zero-bin banks; inferred counts reject zero bins.
Zero-channel banks return empty outputs, and inactive input bins are uninspected.
Returned arrays and getter snapshots outlive both original and cloned owners.

Actual Node.js and browser checks pass all 144 generated and 66 custom original
C cases with 2,044,726 comparisons. Generated responses retain normalized bounds
2e-7 for f32 and 2e-14 for f64, and outputs retain 2e-6 and 2e-13. Custom responses
compare exact values; custom outputs retain 2e-7 and 2e-15. Additional checks
cover all mutable options, default/full Mel agreement, signed weights, empty
dimensions/supports, arbitrary inactive quiet/signaling NaN payloads, infinities,
signed zero and subnormal bits, independent clones/getters/results, immutable
sources, atomic errors, frame overflow and reuse after invalid compression.
Entry points are `node tests/wasm_filterbank_node.cjs ./pkg-node` and
`tests/wasm_filterbank_browser.html`, sharing `wasm_filterbank_checks.mjs`.

Twelve related native/C-interface tests, WASM Clippy and Rustdoc with warnings
denied pass. All four owner classes are absent from an actual feature-disabled
module. Twelve upstream mappings bring coverage to 111 implemented mappings;
221 public mappings remain pending. Native source, C interfaces, dependencies,
existing WASM families and comparison fixtures remain unchanged.

## Scalar approximation and comparisons

Thirty-two functions cover every public root `approximation` operation and
both scalar comparison operations and constants. `ApproximationPrecision`
selects Fast, Approximate or Exact. `approximation_sine_f32/f64`,
`approximation_cosine_f32/f64`, `approximation_atan2_f32/f64`,
`approximation_logarithm_f32/f64` and `approximation_exponential_f32/f64`
expose every native precision mode. Both `approximation_atan2_fast_f32/f64`
convenience functions are included. Exact evaluates native double math before
conversion; the approximation stages retain native binary32 rounding, even
when returned through a binary64 interface.

The binary32 convenience functions use the `approximation_` prefix followed
by the native names: `sine`, `sine_fast`, `cosine`, `cosine_fast`, `logarithm2`,
`logarithm2_fast`, `logarithm`, `logarithm_fast`, `exponential2`,
`exponential2_fast`, `exponential`, `exponential_fast`, `power` and
`power_fast`. All checks and full-range reductions delegate native kernels.
Original underflow clamps, signed-zero branch behavior and range errors remain.

`scalar_minimum_f32/f64` and `scalar_maximum_f32/f64` retain the second operand
on ties and unordered comparisons. `legacy_epsilon()` returns 1e-15 and
`legacy_infinity_sentinel()` returns finite 1e15. Scalar NaN classification is
preserved; payload transport through JavaScript Number is not promised.

Actual Node.js and browser verification passes 128 original trig/atan2 cases,
30 original log/exp cases, 48 frequency-tail approximation cases and 456
bundled scalar cases, with 2,516 comparisons. All fourteen bundled root
kernels have independent original C coverage, including scalar results from
the original SSE2 corpus. These checks do not declare the lane bindings done.
Approximation modes, scalar comparisons and frequency-tail outputs compare
exact values. Existing host math checks retain their one-ULP and normalized
machine-epsilon bounds. Binary64 Exact logarithm has one measured one-ULP
difference on the pinned WASM backend, so its WASM comparator alone uses the
same bounded host criterion. The native exact-log criterion is unchanged.
Atan2 contributes six one-ULP differences under its existing host criterion.
Rust's official `f64::ln` documentation describes platform-dependent precision;
these measured results do not establish an unlimited tolerance for other math.

Additional checks cover precision constants, convenience agreement, default
kernel composition, full-range Exact versus legacy range rejection, positive
log domain, nonfinite errors, original underflow clamps, signed zeros, NaN
comparisons and reuse after errors. Runtime entry points are
`node tests/wasm_approximation_node.cjs ./pkg-node` and
`tests/wasm_approximation_browser.html`, sharing `wasm_approximation_checks.mjs`.
Related native/C-interface checks and WASM Clippy/Rustdoc with warnings denied
pass; all thirty-two functions are absent without the feature. Thirty-six
upstream mappings bring coverage to 147 implemented mappings; 185 public
mappings remain pending at this checkpoint. The following hyperbolic section
completes that submodule, and the later trigonometric section completes its
scalar and lane submodules. The special section completes that submodule;
the basic lane section completes the remaining lane kernels and conversions.
Native source, C interfaces, dependencies, fixtures and prior bindings are
unchanged.

## Hyperbolic scalar and lane formulas

Eight functions complete the native hyperbolic submodules: `hyperbolic_sine`,
`hyperbolic_cosine`, `hyperbolic_tangent` and `hyperbolic_sigmoid`, and the
corresponding names with `_lanes` appended. All use the existing complete
`ApproximationPrecision` selection. These native APIs use binary32 for scalar
inputs and every lane; there is no additional native binary64 variant to omit.

Lane functions accept exactly four values and return independent
`Float32Array` storage in native index order. A wrong shape or any invalid lane
rejects the whole call, and input arrays remain unchanged. Scalar and lane
calculations delegate unchanged native kernels. Fast and Approximate retain
original exponential formulas; Exact uses double exponential followed by
binary32 conversion and source formula stages. It does not substitute a
different host sinh/cosh/tanh implementation. Source underflow clamps and
intermediate overflow rejection remain in effect.

Actual Node.js and browser checks pass all 128 original scalar C cases and
48 original four-lane SSE2 cases, with 644 exact comparisons. All eight source
variants for each interface are covered. Additional tests independently
reconstruct Exact binary32 stages, compare scalar and lane outputs in all three
modes, reject nonfinite and overflow values at every lane position, verify
unchanged input bits after late errors, reject incomplete and oversized arrays
and invalid modes, and verify independent results and reuse after errors.
Runtime entry points are `node tests/wasm_hyperbolic_node.cjs ./pkg-node` and
`tests/wasm_hyperbolic_browser.html`, sharing `wasm_hyperbolic_checks.mjs`.

Ten related native/C-interface tests, WASM Clippy and Rustdoc with warnings
denied pass. All eight functions are absent from an actual feature-disabled
module. The preceding scalar approximation checks also pass on this artifact.
Sixteen upstream mappings bring coverage to 163 implemented mappings;
169 public mappings remain pending. Native source, C interfaces, dependencies,
fixtures and previously implemented bindings remain unchanged.

## Trigonometric scalar and lane formulas

Ten functions expose every native trigonometric submodule operation in all
three `ApproximationPrecision` modes. `trigonometric_sine`,
`trigonometric_cosine`, `trigonometric_tangent` and
`trigonometric_tangent_full` cover the scalar APIs. Corresponding `_lanes`
functions cover those four operations plus `trigonometric_sine_full_lanes`
and `trigonometric_cosine_full_lanes`. Native scalar full sine and cosine
remain available through the previously exposed root approximation functions.
All these native submodule inputs and outputs are binary32.

Lane calls require exactly four angles and return independent `Float32Array`
storage. Any invalid lane rejects the whole call without modifying the input.
The restricted polynomial kernels retain evaluation outside their calibration
interval and perform no periodic reduction. Full-range formulas retain native
reduction direction and checked signed-turn conversion. Exact selects native
double sin/cos/tan before conversion, retaining signed-zero results. Original
SSE2-specific polynomial expression order and cosine's equality branch remain
distinct from scalar formulas; vector cosine at binary32 half-pi returns zero
in Approximate mode while the scalar strict-greater branch differs.

Actual Node.js and browser checks pass all 256 original scalar C cases and
144 original SSE2 four-lane cases, with 826 exact comparisons. All eight scalar
source variants and twelve lane variants are covered, including native rejection
of original nonfinite outputs. Additional tests check all three precision modes,
independent Exact formulas, vector equality versus scalar branching, finite
polynomial results outside calibration versus full-range reduction, signed
zeros, invalid modes and shapes, nonfinite values in every lane position,
turn overflow, immutable source bits, independent outputs and reuse after errors.
No comparison bound or native numerical implementation is changed.

Runtime entry points are `node tests/wasm_trigonometric_node.cjs ./pkg-node`
and `tests/wasm_trigonometric_browser.html`, sharing
`wasm_trigonometric_checks.mjs`. Ten related native/C-interface tests, WASM
Clippy and Rustdoc with warnings denied pass. All ten functions are absent
without the feature. Hyperbolic Node checks also pass on this artifact. Twenty
upstream mappings bring coverage to 183 implemented mappings; 149 public
mappings remain pending. Native source, C interfaces, dependencies, comparison
fixtures and previously implemented bindings remain unchanged.

## Special scalar and lane kernels

Twenty-eight functions expose every public operation of the native scalar and
lane special-function modules. The `special_` prefix names the scalar functions
`complementary_error`, `complementary_error_fast`, `error`, `error_fast`,
`inverse_error`, `inverse_error_fast`, `log_gamma`, `log_gamma_fast`, `digamma`,
`digamma_fast`, `lambert_w`, `lambert_w_fast`, `lambert_w_exp` and
`lambert_w_exp_fast`. Appending `_lanes` selects each corresponding four-lane
function. These native APIs use binary32; approximation variants are separate
functions rather than precision modes.

Lane inputs must have exactly four values. Results own independent
`Float32Array` storage; native domain checks apply to every lane, and any error
rejects the entire call without changing the input. Source-defined negative
intervals for log-gamma and digamma remain available; they are not replaced by
a blanket positive-input requirement. Inverse error, gamma poles and principal
Lambert W retain native validation. No mathematical domain is silently saturated.

Lambert W of exp(value) preserves ordinary positive finite source outputs and
retains the existing native correction only when the original refinement is
nonpositive or nonfinite. Extreme positive values avoid constructing their
exponential; underflowed results round to positive zero. This binding adds no
new numerical correction and changes no previous criterion.

Actual Node.js and browser checks pass 232 original scalar C cases and 67
original SSE2 lane cases, with 613 exact comparisons and 19 corrected-reference
checks. All fourteen kernels in both interfaces are covered. Original valid
outputs compare exact values. Invalid scalar Lambert source outputs retain the
existing positive-result requirement; invalid vector source outputs compare
the independent corrected-root fixture within its existing one-ULP bound, with
underflow zero exact. Additional checks cover every lane's nonfinite/domain
errors, invalid shapes, unchanged source bits after late failure, scalar/lane
agreement, allowed negative intervals, extreme corrected inputs, independent
arrays and reuse after errors.

Runtime entry points are `node tests/wasm_special_node.cjs ./pkg-node` and
`tests/wasm_special_browser.html`, sharing `wasm_special_checks.mjs`. Ten related
native/C-interface tests, WASM Clippy and Rustdoc with warnings denied pass.
All twenty-eight functions are absent without the feature. Trigonometric Node
checks also pass on this artifact. Twenty-eight upstream mappings bring
coverage to 211 implemented mappings; 121 public mappings remain pending.
Native source, C interfaces, dependencies, reference fixtures and existing
bindings remain unchanged.

## Basic lane operations and conversions

Sixteen functions complete the native root lane API. The ten approximation
functions use the `lanes_` prefix with native names `exponential2`,
`exponential2_fast`, `exponential`, `exponential_fast`, `logarithm2`,
`logarithm2_fast`, `logarithm`, `logarithm_fast`, `power` and `power_fast`.
Every numeric buffer contains exactly four lanes. Power requires four bases
and four independently selected exponents. A wrong shape or invalid lane
rejects the entire call, preserving all input buffers. Results own independent
`Float32Array` storage; all native kernels and bounds remain unchanged.

`lanes_to_integers` returns `Int32Array` using the original truncation and
SSE2 indefinite i32::MIN for nonfinite or out-of-range inputs.
`lanes_from_integers` rounds `Int32Array` values to binary32 with nearest-even
conversion. `lanes_to_bits` and `lanes_from_bits` exchange `Float32Array`
and `Uint32Array` without numeric conversion, retaining full NaN payloads and
signed zeros. `lanes_absolute` clears only the sign bit. All five buffer
conversion operations enforce exactly four lanes and copy independent storage.

`lanes_from_integer64(first, second)` accepts two signed 64-bit JavaScript
BigInts and returns `Int32Array` in first-low, first-high, second-low,
second-high word order. BigInt avoids Number's integer precision limit. Use
values in the i64 range; the documented wasm-bindgen ABI wraps values outside
that range. Floats and Integers lane aliases are represented by complete
typed-array storage with every index accessible; no hidden SIMD alignment is
required. Floating-point exception flags remain outside the native portable API.

Actual Node.js and browser checks pass all 80 original SSE2 cases, with 1,532
exact comparisons. All ten original kernels and three source conversion
operations are covered. Additional checks verify both independently selected
power inputs, scalar/kernel agreement, every invalid lane position and late
error immutability, wrong shapes, independent output storage and reuse after
errors. Full quiet/signaling NaN payload round trips, sign-only absolute,
conversion indefinite values, truncation, nearest-even integer boundaries and
signed BigInt min/max and low/high word order pass independently.

Runtime entry points are `node tests/wasm_lanes_node.cjs ./pkg-node` and
`tests/wasm_lanes_browser.html`, sharing `wasm_lanes_checks.mjs`. Nine related
native/C-interface tests, WASM Clippy and Rustdoc with warnings denied pass.
All sixteen functions are absent without the feature. Special-function Node
checks also pass on this artifact. Ten upstream mappings bring coverage to
221 implemented mappings; 111 public mappings remain pending. All native
approximation submodules are now exposed; other DSP families and final combined
acceptance remain required. Native source, C interfaces, dependencies, fixtures
and previously implemented bindings remain unchanged.

## Primary references

- [Supported numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [Mutable numeric slices](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html)
- [Optional numeric values](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/numbers.html)
- [Integer BigInt arguments](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/numbers.html#bigint-to-u64-i64-u128-and-i128)
- [JavaScript callbacks](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/reference/receiving-js-closures-in-rust.md)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Returned Result errors](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [Exception destructor behavior](https://docs.rs/wasm-bindgen/0.2.129/wasm_bindgen/fn.throw_str.html)
- [Rust 1.94 atan2 precision](https://doc.rust-lang.org/1.94.0/std/primitive.f64.html#method.atan2)
- [Rust logarithm precision](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
- [Version-matched CLI installation](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/wasm-bindgen-test/usage.md)
- [Release library manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/Cargo.toml)
- [Release CLI manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/crates/cli/Cargo.toml)
