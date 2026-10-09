# WebAssembly interface

The accepted native and binding gates are verified within the agreed execution
scope. The final requirement decision, exact evidence and authorized deferred
targets are recorded in the SHIRO package's `docs/requirements-verification.md`.
Historical checkpoints below retain their original scope.


Current binding inventories cover every portable public computation. OS
process operations retain their native implementation under ADR 0007. The
following sections include historical checkpoints; their older pending
statements are not the current implementation status. Final latest-source
platform and composed acceptance remains in progress.


The optional `wasm` feature exposes bindings from the existing library crate.
It uses `wasm-bindgen` 0.2.129 and `js-sys` 0.3.106 on
`wasm32-unknown-unknown`. The CLI must match the `wasm-bindgen` crate version.
The declared Rust requirements are 1.81 for the libraries and 1.86 for the CLI;
the current pinned Rust 1.94 builds both successfully.

Current platform inventory contains 329 implemented portable public mappings,
three native-only OS process lifecycle mappings and two nonpublic macro formals.
The native-only entries are plotopen, plotopenv and plotclose; their complete
Figure APIs remain implemented in Rust and C ABI. Browser/Node WASM uses complete
portable plotting command streams instead of spawning OS child processes.
There are no pending ciglet binding inventory rows. This inventory result does
not prove final three-crate acceptance or replace public API/ownership audits.

```sh
cargo build --locked --lib --features wasm --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/ciglet_rs.wasm --target web --out-dir pkg-web
wasm-bindgen target/wasm32-unknown-unknown/debug/ciglet_rs.wasm --target nodejs --out-dir pkg-node
node tests/wasm_window_node.cjs ./pkg-node
node tests/wasm_all_node.cjs ./pkg-node
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
twenty-eight; basic lane kernels add ten and sinc adds two. Later sections record
the remaining completed families. Both remaining crates still require complete
WASM interfaces and final combined acceptance.

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

## Aliased sinc and its derivative

Eight functions expose the entire native sinc module in both scalar precisions.
`aliased_sinc_f32/f64` and `aliased_sinc_derivative_f32/f64` use the native
Stable default. Corresponding `aliased_sinc_with_f32/f64` and
`aliased_sinc_derivative_with_f32/f64` accept `SincPolicy.Stable` or
`SincPolicy.Legacy`. Length is a scalar value, supporting signed and fractional
lengths; angle is angular frequency. No native operation or policy is omitted.

Legacy retains the original different denominator plateaus, 1e-4 for values
and 1e-5 for derivatives, plus scalar intermediate rounding. Stable retains
double intermediates, small-angle evaluation and removable integer-length
poles, including even-length sign changes at odd turns. Fractional poles are
not clamped. Original finite-input/output validation remains in effect.
Positive-length Stable derivatives at zero can retain negative zero where
Legacy returns positive zero. Binary32 Stable intermediates can remain finite
for maximum length where Legacy scalar multiplication overflows.

Actual Node.js and browser checks pass all 384 original C cases, with 916
comparisons. Legacy binary32 values compare exact bits; binary64 normalized
bounds remain 2e-16 for values and 5e-16 for derivatives, with zero bits exact.
Independent integer-length identities and small-angle analytic checks retain
existing native bounds. Additional checks cover full/default Stable agreement,
signed/fractional/zero/unit lengths, integer and fractional poles, Legacy
plateaus, extreme intermediates, signed zeros, nonfinite/domain errors, invalid
policies and reuse after errors. No numerical algorithm or tolerance is changed.

Runtime entry points are `node tests/wasm_sinc_node.cjs ./pkg-node` and
`tests/wasm_sinc_browser.html`, sharing `wasm_sinc_checks.mjs`. Related native
and C-interface tests, WASM Clippy and Rustdoc with warnings denied pass.
All eight functions are absent without the feature. Two upstream mappings
bring coverage to 223 implemented mappings; 109 public mappings remain pending.
Native source, C interfaces, dependencies, fixtures and previous bindings remain
unchanged. Remaining DSP families and combined three-crate acceptance are required.

## Complete complex values

`ComplexF32` and `ComplexF64` expose owned native complex values. Constructors
take real and imaginary components. Both fields have getters and setters.
`from_components`, `set_components` and `components` use exactly two typed-array
values, preserving NaN payloads, signed zeros and subnormals without Number
conversion. Replacement errors leave the original value intact. `cloned` and
returned component arrays are independent and remain valid after source release.

`conjugate`, `add`, `subtract` and `multiply` retain ordinary native IEEE
arithmetic. `divide` retains finite validation and scaled overflow/underflow
recovery. `exponential(mode)` and `argument(mode)` expose Fast=0,
Approximate=1 and Exact=2. Mode arguments use the unsigned 32-bit ABI and are
validated inside Rust Result handling, so invalid codes do not poison an owner
borrow. Owners remain reusable after errors and expose generated `free()`.

Actual Node.js and browser checks consume both original CPX1 complex corpora,
128 records, and perform 2,468 numerical comparisons plus storage, payload,
lifetime, same-owner operands, atomic errors and scaled-division checks.
Conjugation, arithmetic and all complex exponential modes match C exactly.
Binary32 arguments also match exactly. Four binary64 arguments differ by one
ULP in both Approximate and Exact modes, eight comparisons in total, with
maximum normalized error 9.503968395502985e-17. These use the existing WASM
atan2 bound of one ULP and f64 epsilon; native/C bounds are unchanged.
The separately verified CPX1 scalar tail is structurally consumed in full.

Run `node tests/wasm_complex_node.cjs ./pkg-node` or serve
`tests/wasm_complex_browser.html`. Native complex and C-interface tests pass,
as do WASM Clippy and Rustdoc with warnings denied. Both owner classes are
absent without the feature. Eleven original complex mappings bring coverage
to 234 implemented mappings and 98 pending public mappings. Remaining ciglet
features, liblrhsmm and SHIRO WASM, and combined acceptance remain required.

## Complete Kalman filtering and smoothing

`kalman_filter_f32/f64` initializes the mean from the first measurement.
`kalman_filter_with_initial_f32/f64` accepts an explicit initial mean.
`kalman_smooth_f32/f64` performs the native backward smoothing operation.
All functions retain native length, finite-input, nonnegative-variance and
overflow validation. Empty inputs produce empty arrays and zero likelihood.
Zero posterior/process uncertainty retains the corrected zero backward gain.
The native wide-logarithm fallback outside the binary32 approximation domain
is also retained.

`KalmanEstimateF32/F64` exposes every public native estimate field: independently
mutable `means`, `variances` and `log_likelihood`, plus `cloned()` and generated
`free()`. Its constructor accepts arbitrary field contents and independently
sized arrays, as does the native public struct. Getters return independent
typed arrays. `likelihood_value()` and `set_likelihood_value(array)` preserve
all scalar bits through a one-element typed array, including signaling NaN
payloads. Invalid replacement lengths leave the likelihood unchanged.
Computation functions validate their inputs separately from arbitrary storage.

Actual Node.js and browser checks consume both complete KLM1 C corpora,
216 cases and 21,452 comparisons, with exact equality of filtered means,
variances, likelihoods and smoothed values. Independent posterior identities,
deterministic limits, both initialization paths, empty arrays, shape/domain
and overflow errors, corrected wide likelihoods, full-bit field storage,
clone/setter independence and snapshots surviving owner release also pass.
Run `node tests/wasm_kalman_node.cjs ./pkg-node` or serve
`tests/wasm_kalman_browser.html`. All twelve related native/C tests pass,
as do WASM Clippy and Rustdoc with warnings denied. All six functions and
both owners are absent without the feature. Four original mappings bring
coverage to 238 implemented and 94 pending public mappings. Native/C source,
fixtures, dependencies and prior bindings remain unchanged. Full remaining
DSP coverage and combined three-crate acceptance are still required.

## Complete spectral peak interpolation

`spectral_peak_interpolate_f32/f64(magnitudes, bin)` returns an owned
`SpectralPeakF32/F64`. Both native fields, `bin` and `magnitude`, have getters
and setters. Constructors accept arbitrary field values. `from_values`,
`set_values` and `values` use exactly two typed values and preserve every
native bit, including signaling NaNs and signed zeros. Invalid replacement
shapes leave the original peak unchanged. `cloned()` and array snapshots are
independent; snapshots remain valid after generated `free()` releases owners.

The bin argument uses the unsigned 32-bit WebAssembly ABI. Interpolation
retains native interior-bin validation, mixed-precision polynomial arithmetic,
the strict one-bin displacement condition and the center-plus-0.2 magnitude
cap. Signed log magnitudes are accepted. Only the participating three bins
are validated, so nonfinite values elsewhere remain ignored.

Actual Node.js and browser checks consume both complete QIF1 C corpora,
128 cases and 1,176 exact comparisons, plus independent parabola/cap/flat/
displaced checks, signed magnitudes, all neighbor error positions, boundaries,
overflow, input immutability and full native field storage/lifetimes. Run
`node tests/wasm_spectral_peak_node.cjs ./pkg-node` or serve
`tests/wasm_spectral_peak_browser.html`. Eight related native/C tests pass,
as do WASM Clippy and Rustdoc with warnings denied. Both functions and owners
are absent without the feature. Two original mappings bring coverage to
240 implemented mappings and 92 pending public mappings. Native/C source,
fixtures, dependencies and prior bindings remain unchanged. Full remaining
DSP coverage and combined three-crate acceptance are still required.

## Complete bracketed root search

`root_find_f32/f64(callback, start, end)` exposes the complete native search.
The callback receives a Number rounded to the selected scalar precision and
must return a Number, likewise converted to that precision. Captured state
replaces the original C context pointer. Exceptions preserve the original
JavaScript value, including thrown null and undefined. Non-number, nonfinite
or binary32-overflow callback results produce errors. Rust Result handling
retains recovery after callbacks fail; independent nested searches are valid.

Native ordered-bound validation occurs before callbacks. Both endpoints are
evaluated in order even when the first is a root. Endpoint roots, signed zero,
the nonbracketing midpoint fallback, overflow-safe midpoint evaluation,
absolute residual tolerance and native iteration limit remain unchanged.

Actual Node.js and browser checks consume both complete ROT1 corpora,
50 original C cases and 967 comparisons, with exact root equality and
selected-precision callback arguments. Polynomial, cosine, exponential and
linear callbacks retain their native scalar expression order. Captured state,
endpoint call order, fallback, invalid bounds/results, late failures, exception
identity, recovery and nested searches also pass. Run
`node tests/wasm_root_node.cjs ./pkg-node` or serve
`tests/wasm_root_browser.html`. Nine related native/C tests pass, as do WASM
Clippy and Rustdoc with warnings denied. Both functions are absent without
the feature. Two original mappings bring coverage to 242 implemented and
90 pending public mappings. Native/C source, fixtures, dependencies and prior
bindings remain unchanged. Full remaining DSP coverage and combined
three-crate acceptance are still required.

## Complete centered sinusoid synthesis

`sinusoid_f32/f64(tone, sample_rate, length)` takes an owned
`SinusoidF32/F64` configuration. All three native fields, `frequency_hz`,
`amplitude` and `phase_radians`, have getters and setters. Constructors,
`from_values`, `set_values`, `values`, `cloned()` and generated `free()` cover
complete native storage. Three-element typed arrays preserve arbitrary field
bits and snapshots survive owner release. Invalid replacement shapes are
atomic. Arbitrary configuration storage is separate from synthesis validation.

`sinusoids_f32/f64(frequencies, amplitudes, phases, sample_rate, length)` accepts
three equal-length arrays and sums tones in input order. Both functions retain
centered phase at floor(length/2), selected-scalar recurrence and the native
single-tone amplitude correction. Negative frequencies and amplitudes are
valid. Empty tone lists produce silence; zero and one sample lengths are safe.
Lengths/rates use the unsigned 32-bit ABI and native legacy limits. Nonfinite
parameters, invalid dimensions/rates and overflowing step/phase evaluations
return errors. Native IEEE overflow during output summation remains retained.

Actual Node.js and browser checks consume both complete SYN1 C corpora,
96 cases and 21,492 comparisons. Maximum amplitude-normalized C error is zero
for binary32 and 3.1086244689504383e-15 for binary64, within unchanged native
bounds 2e-7 and 2e-14. Independent binary64 centered cosine checks retain
1e-12; binary32 initial-sample checks use selected-scalar phase arithmetic.
Additional checks cover negative parameters, input-order sums, source
immutability, short/empty outputs, invalid shapes/domains, owner recovery,
all mutable fields and arbitrary full-bit storage/clone/snapshot lifetimes.
Run `node tests/wasm_synthesis_node.cjs ./pkg-node` or serve
`tests/wasm_synthesis_browser.html`. Ten related native/C tests pass, as do
WASM Clippy and Rustdoc with warnings denied. Four functions and two owners
are absent without the feature. Three original mappings bring coverage to
245 implemented and 87 pending public mappings. Native/C source, fixtures,
dependencies and previous bindings remain unchanged. Remaining DSP features
and combined three-crate acceptance are still required.

## Complete band-feature cepstra

`BandCepstralOptions(order, include_dc)` exposes both independently mutable
native fields, `cloned()`, `coefficient_count(bands)` and generated `free()`.
Order and band counts use the unsigned 32-bit WebAssembly ABI. The native
order clamp remains applied; zero bands and bands outside the legacy signed
range are errors. Failed computations release the options borrow for reuse.

`band_to_cepstrum_f32/f64(bands, options)` returns the requested prefix.
`band_to_cepstrum_legacy_f32/f64` returns the complete original band-count
buffer, including untouched DCT suffix values. Both retain upstream scaling
and optional DC placement after the non-DC coefficients. Signed log-band
features are accepted without applying an additional logarithm.
`band_spectrogram_to_cepstra_f32/f64(features, bands, options)` processes
flattened frame-major features. Empty frame sets and zero coefficient counts
retain native behavior; invalid shapes and nonfinite later frames are errors.
All returned arrays own their storage independently of inputs and options.

Actual Node.js and browser checks consume both complete CEP1 C corpora,
96 cases and 15,592 comparisons, with zero measured C difference. Unchanged
native normalized bounds remain 1e-7 for binary32 and 1e-15 for binary64.
Checks include all original order/DC variants, full suffix and prefix,
frame layout, mutable/maximal/zero order, singleton and empty results,
shape/domain/late errors, input immutability and options reuse. A complete
existing Mel filterbank to floored log-band to cepstrum pipeline also passes.
Run `node tests/wasm_cepstrum_node.cjs ./pkg-node` or serve
`tests/wasm_cepstrum_browser.html`. Seven related native/C tests pass, as do
WASM Clippy and Rustdoc with warnings denied. All six functions and the options
owner are absent without the feature. Two original mappings bring coverage
to 247 implemented and 85 pending public mappings. Native/C source, fixtures,
dependencies and prior bindings remain unchanged. Full remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete owned two-dimensional arrays

`Array2U8/U16/U32/U64/F32/F64` expose the complete native row-major operations
for the four original byte widths and both floating precisions. Constructors
copy `(rows, columns, typed_values)`. `zeros`, `filled` and
`from_rows(packed_values, row_lengths)` cover initialization and row copying;
row lengths distinguish empty rows from an empty row collection. Ragged,
truncated and trailing packed storage are rejected. All dimension and index
arguments use the unsigned 32-bit WebAssembly ABI. U64 scalar values use
BigInt and arrays use BigUint64Array, preserving all 64 bits. Integer scalar
conversion retains the documented wasm-bindgen wrapping behavior.

Readonly metadata is `rows`, `columns`, `length` and `is_empty()`. `values`,
`flatten`, `row` and `get` expose storage; missing rows or cells return
undefined. `set_values`, `set_row` and `set` cover all native mutable access.
Missing cell updates return false. Shape/row replacement errors are atomic.
`reshape`, `transpose`, `transposed` and `cloned` retain native algorithms,
including zero-by-large empty shapes and allocation-before-mutation behavior.
Typed storage preserves arbitrary bits, including signaling NaNs, signed
zeros, infinities and subnormals.

`into_row_major()` consumes the owner and moves its native buffer into the
returned typed array. The generated JavaScript owner is then invalid for
further calls. This avoids an extra native clone; JavaScript output storage
is independent. Other getters and copies also own their arrays, and snapshots
remain valid after generated `free()` releases their original owners.

Actual Node.js and browser checks consume all 240 original ARR1 C records,
testing six element types in 360 cases and 330,720 exact byte/metadata
comparisons. Coverage includes initialization, all original copy/flatten/
transpose results, row construction, consumed owners, arbitrary floating
payloads, mutable storage, shape changes, atomic errors, source immutability
and snapshot lifetimes. Run `node tests/wasm_array_node.cjs ./pkg-node` or
serve `tests/wasm_array_browser.html`. Seven related native/C tests pass,
as do WASM Clippy and Rustdoc with warnings denied. All six owners are absent
without the feature. Six original mappings bring coverage to 253 implemented
and 79 pending public mappings. Native/C source, fixtures, dependencies and
prior bindings remain unchanged. These array owners will carry correlogram
and other DSP results; those bindings and combined acceptance remain required.

## Complete correlogram and frequency mapping

`CorrelogramMethod` exposes Autocorrelation=0, AbsoluteDifference=1,
SquaredDifference=2 and Yin=3. `CorrelogramPolicy` exposes Stable=0 and
Legacy=1. `correlogram_compute_f32/f64(samples, centers, lengths, periods,
method)` uses Stable by default; `correlogram_compute_with_f32/f64` accepts
an additional policy. Centers use Int32Array and lengths use Uint32Array.
Results are complete independent `Array2F32/F64` owners. Native method-specific
arithmetic, strict trailing boundary, negative-center clamp and unavailable
lag values remain unchanged. Stable ACF scaling/DC conditioning and Legacy
prefix-sum arithmetic are both retained.

`CorrelogramFrame(center, length)` exposes both mutable native fields,
`cloned()`, generated `free()` and single-frame `compute_f32/f64` methods.
All method/policy arguments use numeric codes validated inside Rust Result
handling; invalid codes do not poison frame borrows. Dimensions and lengths
use the unsigned 32-bit ABI; frame centers use signed 32-bit values. Storage
accepts the complete field range, with native validation during computation.

`correlogram_to_frequency_f32/f64(periods, sampling_rate, frequencies)` returns
an independent array while retaining frequency order. The signed sample rate
must be positive. Positive zero frequency selects the last lag; negative
frequency and negative zero select lag zero. Singleton lags, empty frequency
sets, native validation and source-array reuse after errors are preserved.

Actual Node.js and browser checks consume both complete CRG1 C corpora,
64 cases and 21,971 comparisons, with zero measured C difference. Unchanged
criteria are binary32 exact equality and binary64 normalized error at most
3e-15. Independent ramp/ACF/difference/YIN identities, both policies/defaults,
DC conditioning/extreme scaling, strict boundaries, silence, signed centers,
maximal frame storage, errors/owner recovery, signed-zero mapping and returned
owner independence also pass. Run
`node tests/wasm_correlogram_node.cjs ./pkg-node` or serve
`tests/wasm_correlogram_browser.html`. Nine related native/C tests pass, as do
WASM Clippy and Rustdoc with warnings denied. Six functions and the frame
owner are absent without the feature. Six original mappings bring coverage
to 259 implemented and 73 pending public mappings. Native/C source, fixtures,
dependencies and prior bindings remain unchanged. Full remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete Chirp-Z transforms

`CztPlanF32/F64(length, angular_step, direction)` owns a complete reusable
native plan and cached work storage. Directions use the existing
`FftDirection.Forward=0` and `Inverse=1` constants and fallible numeric-code
validation. Plans expose `length`, `is_empty()`, `transform(real, imaginary)`,
`transform_into(real, imaginary, output)` and generated `free()`. Missing
planes use undefined/null and represent zero; present planes match plan
length. Both transforms retain native forward phase and normalized inverse
conventions. Inverse direction negates the step and divides by length.

`transform` returns existing complete `ComplexValuesF32/F64` owners without
an extra native plane copy. `transform_into` writes alternating real and
imaginary values to the caller's typed array after successful computation.
Invalid component/output shapes preserve the output. Input/output arrays
may overlap because input borrowing finishes before commit. Native IEEE
propagation of nonfinite samples is retained. Length, finite step, phase
range and direction validation remain native; errors leave plans reusable.
`czt_f32/f64` supplies a one-shot equivalent with the same full arguments.

Actual Node.js and browser checks consume both complete CZT1 C corpora,
2,400 cases and 526,128 comparisons. Maximum normalized C error is
1.4301623138882108e-7 for binary32 and 3.43811001174242e-16 for binary64,
within unchanged native 5e-7 and 2e-15 bounds. All scenarios, component masks,
directions and original grids are covered. Checks include plan reuse,
one-shot agreement, complete output-buffer commit/aliasing, input immutability,
owner lifetimes, absent planes, ordinary IEEE behavior and recovery after
shape errors. Independent arbitrary-grid direct sums cover both precisions.
Binary64 FFT-grid and inverse-roundtrip checks retain the existing native
1e-11/1e-12 criteria. Binary32 chirp phases retain source scalar rounding;
an ideal FFT-grid equality claim is not made for that precision.

Run `node tests/wasm_czt_node.cjs ./pkg-node` or serve
`tests/wasm_czt_browser.html`. Nine related native/C tests pass, as do WASM
Clippy and Rustdoc with warnings denied. Existing actual Node FFT checks also
pass all 720 records and 559,776 comparisons after sharing internal direction
validation and result storage. Both new functions/plans are absent without
the feature. Three original mappings bring coverage to 262 implemented and
70 pending public mappings. Native/C source, fixtures and dependencies remain
unchanged. Existing FFT changes are internal visibility only. Full remaining
DSP coverage and combined three-crate acceptance are still required.

## Complete real cepstrum and minimum phase

`RealCepstrumPlanF32/F64(length)` exposes the complete reusable native plan:
`length`, `bins`, `is_empty()`, `analyze(logs, policy)`,
`reconstruct(coefficients, policy)`,
`minimum_phase_with(logs, policy, lifter_code)`,
`spectrogram_to_cepstra(magnitudes, count, policy)`,
`cepstra_to_spectrogram(coefficients, frames, count, policy)` and generated
`free()`. Outputs are independent typed arrays. Input arrays remain unchanged.
Lengths and counts use the WASM32 unsigned integer range.

Nyquist policies use existing `SpectrumNyquist.Preserve=0` and `Legacy=1`.
The new `CepstralLifter.Causal=0` and `Legacy=1` constants select the lifter
independently. Numeric codes are validated inside Rust Result, so invalid
codes leave the owner reusable. Preserve includes the supplied Nyquist bin;
Legacy mirrors the preceding bin and ignores the unused Nyquist input.
The causal lifter retains DC/Nyquist and clears all negative quefrencies.
The legacy lifter preserves the source's doubled Nyquist coefficient and
first negative-quefrency coefficient.

`real_cepstrum_from_log_spectrum_f32/f64(logs)`,
`real_cepstrum_to_log_spectrum_f32/f64(coefficients, length)` and
`real_cepstrum_minimum_phase_f32/f64(logs)` supply all native one-shot entry
points. They use Preserve, with Causal for minimum phase. Analysis accepts
already logarithmic spectra. Reconstruction accepts a coefficient prefix
up to transform length and returns the full native log-spectrum buffer.
Matrix conversion preserves frame-major storage, approximate log with the
native 1e-15 floor, approximate exp, explicit frame count and empty coefficient
frames. Transform length must be a positive power of two; one-shot analysis
and minimum phase infer length from at least two spectrum bins.

Actual Node.js and browser checks pass all 144 original RCP1 cases and 50,860
comparisons. The seven comparisons cover legacy analysis, reconstruction
from C and computed coefficients, legacy minimum phase, matrix analysis,
and matrix reconstruction from C and computed coefficients. Unchanged native
normalized bounds are `[3e-8, 1.2e-7, 2e-7, 1.6e-7, 7e-9, 0, 2.5e-6]` for
binary32 and `[5e-17, 3e-16, 3e-16, 3e-16, 3e-17, 0, 0]` for binary64.
Observed maxima are `[1.4858453763329853e-8, 6.939960699696553e-8,
1.0104661339460338e-7, 1.411484763920191e-7, 4.170882985242948e-9, 0, 0]`
and `[4.364540199625453e-17, 2.1510171894736168e-16,
2.1510171894736168e-16, 2.121702267680549e-16,
2.4212402950875138e-17, 0, 0]`, respectively.

Checks also cover all Nyquist/lifter combinations, one-shot agreement,
independent binary64 cosine reconstruction and causal phase, the legacy
lifter difference, ignored input regions, zero coefficient frames,
singletons, empty matrices, checked shape/size failures, invalid numeric
codes, recovery after late frame errors, input immutability, returned-array
lifetimes and reusable plans. No native algorithm or numerical bound changed.

Run `node tests/wasm_real_cepstrum_node.cjs ./pkg-node` or serve
`tests/wasm_real_cepstrum_browser.html`. Nine related native/C tests pass,
along with WASM Clippy and Rustdoc with warnings denied. The six functions
and two plans are absent without the feature. Five original mappings bring
coverage to 267 implemented and 65 pending public mappings. Native/C source,
fixtures, dependencies and previous bindings remain unchanged. Remaining
DSP coverage and combined three-crate acceptance are still required.

## Complete polynomial evaluation and roots

`polynomial_evaluate_f32/f64(coefficients, real, imaginary)` accepts existing
`ComplexValuesF32/F64` coefficient owners. The corresponding
`polynomial_evaluate_real_f32/f64` accepts a real typed array. Both return
an independently owned complete complex value through `ComplexValues`, with
length one. Coefficients retain descending order and the native source power
accumulation. The implementation does not change rounding through Horner
evaluation. Arguments and coefficients must be finite; empty coefficients
and overflowing evaluation return ordinary errors.

`polynomial_roots_f32/f64(coefficients)` and
`polynomial_roots_real_f32/f64(coefficients)` expose both native root entry
points and return `PolynomialRootsF32/F64` owners. Root order, complex
normalization, source initial values, level-one binary32 trigonometry,
radius clamp and the 50-step limit remain native. Nonzero constants return
no roots; zero leading coefficients and singular/overflowing computations
are rejected. Unconverged finite estimates remain available.

Each report exposes all four mutable native fields: `values`, `iterations`,
`converged`, and binary64 `maximum_residual`, plus `cloned()` and generated
`free()`. Values use independent `ComplexValues` owners on both read and
replacement. The constructor accepts all four fields and preserves arbitrary
metadata and complex payloads, matching the native public struct. Iterations
use the WASM32 unsigned integer range. `residual_storage()` and
`set_residual_storage(values)` retain arbitrary binary64 payload bits through
one-element typed storage. Invalid storage lengths leave the report reusable.
Clones and snapshots outlive their source and retain independent storage.

Actual Node.js and browser checks pass all 65 original POL1 polynomial cases
and 2,460 evaluation/root/ownership comparisons with exact C results, using
the unchanged native equality criterion. Both complete 61-case source
trigonometry tails also match. Tests cover real/complex entry-point and full
report agreement, source ordering, input immutability, constants, independent
linear/imaginary roots, invalid and overflowing inputs, post-error owner
recovery, complete field mutation, arbitrary NaN/signed-zero/subnormal bits,
cloning and lifetimes. The original eight-root nonconvergence case retains
50 iterations and its full report; independent product-of-linear-factors
residual checking uses the unchanged native relative 1e-8 criterion.

Run `node tests/wasm_polynomial_node.cjs ./pkg-node` or serve
`tests/wasm_polynomial_browser.html`. Ten related native/C tests pass, as do
WASM Clippy and Rustdoc with warnings denied. All eight functions and two
reports are absent without the feature. Six original mappings bring coverage
to 273 implemented and 59 pending public mappings. Native/C source, fixtures,
dependencies and previous bindings remain unchanged. Remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete instantaneous frequency detector

`InstantaneousFrequencyDetectorF32/F64(center_frequency, resolution)` owns
the complete native detector. Frequency arguments and results are ratios of
the sampling rate. Kernel length retains source truncation of `4/resolution`;
generated filters retain the four-term cosine window and its modulated
derivative. Resolution must produce a positive bounded length. The owner
exposes `center_frequency`, fallible `set_center_frequency(value)`, `kernels`,
`estimate(samples)`, `estimate_with(samples, policy_code)`, `cloned()` and
generated `free()`.

`InstantaneousFrequencyPolicy.Stable=0` and `Legacy=1` select both native
modes. The default is Stable, which scales samples and filters and computes
the quotient in double precision. Legacy retains selected-scalar accumulation
and quotient. Only the centered participating sample region is inspected.
Short buffers return positive zero after kernel validation. Zero response,
nonfinite participating samples and invalid numeric policy codes return
ordinary errors and leave the detector reusable.

`InstantaneousFrequencyKernelsF32/F64(real, imaginary, derivative_real,
derivative_imaginary)` exposes all four mutable native arrays and `cloned()`.
Getters return independent typed arrays. Constructors and setters retain
arbitrary lengths and payloads, reflecting unrestricted native public fields.
`Detector.from_kernels(center_frequency, kernels)` validates and copies all
four arrays. Reading `detector.kernels` creates an independent owner; assigning
`detector.kernels = kernels` replaces all fields, including temporarily invalid
edits, matching native `kernels_mut()`. Estimation validates edits again.
Returned snapshots and cloned detectors outlive their sources.

Actual Node.js and browser checks pass all 48 IFD1 C cases, 192 estimates and
17,504 comparisons. Generated binary32 kernels and legacy estimates match
exactly, retaining the native exact checks. Binary64 maximum normalized kernel
error is 7.050229737040669e-17 within the unchanged 3e-15 bound; maximum
normalized legacy estimate error is 1.0216802890740152e-13 within the unchanged
1e-12 bound. Supplied exact C kernels also retain reference estimate criteria.
Both complete fixture files are consumed.

Checks cover full kernel/center configuration, independent clones, immutable
input arrays, default/explicit Stable agreement, ignored outer samples, short
buffers, zero responses, invalid parameters and policy codes, errors after
editing every kernel, repair and reuse, all four arbitrary typed payloads,
signed-zero center metadata and owner lifetimes. Independent custom-filter
estimates cover ordinary, maximum finite and smallest subnormal sample scales
with unchanged native analytic criteria. A binary64 sinusoid retains the
native absolute 1e-12 criterion. No algorithm or numerical bound changed.

Run `node tests/wasm_instantaneous_frequency_node.cjs ./pkg-node` or serve
`tests/wasm_instantaneous_frequency_browser.html`. Nine related native/C tests
pass, as do WASM Clippy and Rustdoc with warnings denied. All four owners are
absent without the feature. Six original mappings bring coverage to 279
implemented and 53 pending public mappings. Native/C source, fixtures,
dependencies and previous bindings remain unchanged. Remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete STFT analysis and synthesis

`StftPlanF32/F64(length, window_code, precision_code)` exposes native
`analyze(signal, centers, window_lengths, options)` and
`synthesize(magnitude, phase, options)`. Window codes use all seven existing
`WindowKind` values, precision uses `WindowPrecision.Accurate=0` and
`Approximate=1`, and synthesis Nyquist uses `SpectrumNyquist.Preserve=0`
and `Legacy=1`. Codes are validated inside Rust Result. Plans retain cached
transform/window storage and remain reusable after ordinary errors.
Centers use Int32Array; lengths use Uint32Array. Arrays remain frame-major.

`StftAnalysisOptions()` exposes all mutable fields and native defaults:
`subtract_mean=false`, `magnitude=true`, `phase=true`,
`normalization_hop=undefined`. Explicit positive spacing supports a single
frame. `StftFrameGrid(hop, frames, window_factor, zero_padding_factor)` exposes
all four mutable fields. Both owners support `cloned()` and generated `free()`.
All lengths/counts use the WASM32 unsigned range; offsets/centers use signed
32-bit integers. Native checked shape/allocation and legacy-range validation
remain in effect.

`StftSynthesisOptionsF32/F64(hop, offset, fade_samples, normalization, nyquist)`
exposes all five mutable fields and `cloned()`. Typed one-element
`normalization_storage()` and `set_normalization_storage(values)` preserve
arbitrary scalar payloads; invalid lengths leave the owner unchanged.
Output length retains `hop * frames + offset`, clipping, normalized inverse
FFT, source level-two polar conversion and Hann endpoint fades. Preserve
retains real DC/Nyquist; Legacy reproduces preceding-bin substitution.

`StftAnalysisF32/F64` exposes all six mutable native result fields:
`frames`, `bins`, optional `magnitude`, optional `phase`, optional
`window_weight`, optional `normalization`. Its constructor accepts all fields,
including arbitrary public metadata/storage. Getters return independent typed
arrays. Optional values use undefined/null for absence; present empty arrays
remain distinct. Typed `window_weight_storage()`/`normalization_storage()` and
their setters preserve scalar payloads and optional presence. Every result
supports `cloned()` and generated `free()`; snapshots outlive owners.

`stft_analyze_regular_f32/f64(signal, grid, precision_code, options)` and
`stft_synthesize_regular_f32/f64(magnitude, phase, grid, normalization,
nyquist_code)` expose both convenience operations. Regular analysis uses
Blackman and explicit grid hop. Regular synthesis retains zero offset and
32-sample fade. `stft_analyze_regular_legacy_f32/f64(signal, grid, level,
options)` also exposes the signed source macro level: levels at least three
use Accurate; all lower signed levels use Approximate. The named stft/stft_2
entry points correspond to explicit Accurate/Approximate codes, respectively.

Actual Node.js and browser checks consume all four original fixture files:
960 STF1 analysis cases, 576 IST1 synthesis cases and 65,920 comparisons.
Magnitude normalized errors retain native bounds 2e-6/2e-14; maxima are
5.960464477539062e-7/1.7763568394002505e-15. Phase comparisons retain native
frame-amplitude weighting and exclude 7,808 phase values whose magnitudes are
below 1e-6, where phase is undefined. Weighted phase maxima are
2.3841857910156197e-7/4.440892098500626e-16, within the same bounds.
Inverse normalized errors retain 2e-7/2e-15 bounds; maxima are
1.1920928955078125e-7/3.3306690738754696e-16. No numerical criterion changed.

Checks cover all output masks, window/precision selections, reused plans,
normalization metadata, variable/odd windows, negative-center padding,
both Nyquist policies, offset/fade, empty/singleton cases, signed macro levels,
regular/explicit agreement, all mutable fields and arbitrary typed payloads,
source immutability, cloning, snapshots and recovery after invalid options
and shapes. Binary64 regular audio reconstruction retains the native 1e-4
interior absolute bound. Existing odd-window and variable-window corrections
remain in the native implementation.

Run `node tests/wasm_stft_node.cjs ./pkg-node` or serve
`tests/wasm_stft_browser.html`. Nineteen related native/C tests pass, along
with WASM Clippy and Rustdoc with warnings denied. All six functions and eight
owners are absent without the feature. Six original mappings bring coverage
to 285 implemented and 47 pending public mappings. Native/C source, fixtures,
dependencies and prior bindings remain unchanged. Remaining DSP coverage and
combined three-crate acceptance are still required.

## Complete column-major matrices and LU

`MatrixF32/F64(rows, columns, values)` accepts finite column-major storage:
element `(row, column)` is at `row + column * rows`. Static `zeros(rows,
columns)` and `identity(size)` retain native empty-shape and bounded-dimension
behavior. Owners expose `rows`, `columns`, `as_slice()`, optional `get(row,
column)`, `apply_row_swaps(partners)`, `pivot_rows()`, `multiply(right)`,
`multiply_vector(values)`, `cloned()`, consuming `into_column_major()` and
generated `free()`. Copied arrays and cloned owners have independent storage.
Out-of-bounds cells return undefined. Consumed owners cannot be reused.

Row partners use Uint32Array and describe successive swaps `(i, partners[i])`,
in listed order. Complete validation precedes mutation. Preliminary pivoting
retains the source square-matrix column scan, first-largest tie handling and
ordered row exchanges. Rectangular row exchange retains the corrected native
column-major stride. Multiplication retains source column/inner/row arithmetic
order and rejects nonfinite products.

`LuF32/F64.factor(matrix)` consumes a matrix and performs native partial
pivoting during elimination. `factor_unpivoted(matrix)` consumes already
pivoted input with the source exact accumulation order. Consumption also
applies to failing factorization, matching native ownership. Clone the matrix
first when it must be retained. Factors expose `packed()` as an independent
complete Matrix owner, `row_swaps()` as copied Uint32Array, `solve(right)`,
`cloned()` and generated `free()`. Packed L has an implicit unit diagonal;
U includes the diagonal. Solving applies ordered row swaps and native forward
and backward substitution. Singular/overflowing arithmetic and wrong shapes
return ordinary errors; existing immutable source owners remain reusable.

`matrix_apply_swaps_f32/f64(values, partners)` mutates the caller's typed
array exactly as the native generic vector helper does, including arbitrary
IEEE payloads. The same helper is available for u8/u16/u32/u64/i8/i16/i32/i64
typed arrays; 64-bit integers use BigUint64Array/BigInt64Array. Invalid plans
leave every byte unchanged. Matrix dimensions and indices use the WASM32
unsigned range, with native i32 dimension/product limits retained.

Actual Node.js and browser checks fully consume all four original/corrected
MAT1 corpora: 216 matrix products, 34 permutations, 40 solves and 16,298
comparisons. Source products, swaps, preliminary pivots, unpivoted packed LU
and substitution match exactly with the unchanged native equality criterion.
Robust solve residual maxima are 1.5354756289308175e-7 for binary32 and
2.656234526205982e-16 for binary64, within unchanged native normalized bounds
2e-6/1e-13. Independent late-pivot solution and `P*A=L*U` checks retain native
analytic bounds. No algorithm or numerical criterion changed.

Checks also cover rectangular identity/vector products, all lookups, consuming
storage, clone/snapshot lifetimes, complete packed factors and swaps, source
immutability, empty products/factors, singular inputs, invalid dimensions and
plans, overflow, post-error recovery, signed zeros and arbitrary permutation
payloads. All eight integer permutation widths retain complete stored bits.

Run `node tests/wasm_matrix_node.cjs ./pkg-node` or serve
`tests/wasm_matrix_browser.html`. Fifteen related native/C tests pass, as do
WASM Clippy and Rustdoc with warnings denied. All ten vector functions and four
matrix/LU owners are absent without the feature. Fifteen original mappings
bring coverage to 300 implemented and 32 pending public mappings. Native/C
source, fixtures, dependencies and previous bindings remain unchanged.
Remaining DSP coverage and combined three-crate acceptance are still required.

## Complete LF glottal model

`GlottalModelF32/F64(period, closure, peak, return_time, excitation)` exposes
all five mutable native fields. Period is in seconds; closure, peak and
return_time are period ratios. Signed finite excitation may invert the
waveform. Constructors and setters retain intermediate invalid configurations,
matching the native public struct; calculation validates physical timing.
`from_shape(shape, period, excitation)` retains the source piecewise Rd mapping.
`from_components(values)`, `components()` and atomic `set_components(values)`
use five-element typed storage to preserve arbitrary scalar payloads.
Every model supports `cloned()` and generated `free()`.

Models expose `prepare()`, `spectrum(frequencies)` and
`sampled_period(sampling_rate, count)`. The native model is Copy, so these
methods retain the caller's owner and prepare an independent value.
`GlottalPreparedF32/F64` exposes complete native public operations:
`model()` returns an independent original configuration, `spectrum(frequencies)`
uses Approximate, `spectrum_with(frequencies, code)` selects Fast=0,
Approximate=1 or Exact=2, `sampled_period(sampling_rate, count)` reuses derived
parameters, and `cloned()` copies the prepared model. Numeric precision is
validated inside Rust Result, leaving owners reusable after invalid codes.

`GlottalSpectrumF32/F64(magnitude, phase)` exposes both mutable native arrays,
including arbitrary mismatched storage and IEEE payloads. Getters return
independent typed arrays; owners support `cloned()` and generated `free()`.
Queries are finite hertz values. DC explicitly returns positive zero in both
arrays. Phase uses native accurate atan2. Samples start at time zero and
entries after the period remain positive zero. Rates use signed 32-bit values;
counts use WASM32 unsigned values. Native index checks, truncation, high-pitch
scaling and level-two waveform approximation remain unchanged.

Actual Node.js and browser checks consume both complete LFM1 original C
corpora: 106 cases and 16,608 comparisons. Model fields and all binary32
outputs retain exact source bits. Binary64 magnitude and waveform comparisons
also have zero observed error; normalized phase error reaches
1.2983319909810025e-16. Unchanged native binary64 bounds remain
`[1e-17, 3e-15, 5e-16]` for magnitude, phase and waveform. No bound changed.

Both complete 208-case LFP1 corrected endpoint corpora additionally exercise
WASM preparation, retained model fields and empty sampled periods, 416 cases
total. Private derived parameters are checked by the existing native unit
tests, which pass along with the original-versus-corrected endpoint regression.
The browser checks do not claim direct access to private prepared fields.

Checks cover all public fields, reusable/one-shot/default/explicit precision
agreement, independent cloning and owner/snapshot lifetimes, immutable query
arrays, all three precision modes, DC, approximate/exact conjugate symmetry,
amplitude doubling, negative excitation, truncated/empty periods, large sample
indices, invalid physical timings, raw model/spectrum payload storage, atomic
shape errors, late frequency overflow and post-error recovery. Existing native
analytic criteria are retained; binary32 amplitude doubling is exact.

Run `node tests/wasm_glottal_node.cjs ./pkg-node` or serve
`tests/wasm_glottal_browser.html`. Twelve related native/unit/C tests pass,
along with WASM Clippy and Rustdoc with warnings denied. All six owners are
absent without the feature. Six original mappings bring coverage to 306
implemented and 26 pending public mappings. Native/C source, fixtures,
dependencies and previous bindings remain unchanged. Remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete spectral envelope estimation

`SpectralEnvelopePlanF32/F64(length)` owns the complete native reusable
cepstral workspace. Length must be a power of two of at least two. Owners
expose `length`, `bins`, `is_empty()`, `estimate(spectrum, fundamental,
harmonics)`, `estimate_with(spectrum, fundamental, harmonics, nyquist_code)`,
`estimate_with_smoothing(spectrum, fundamental, harmonics, nyquist_code,
smoothing_code)` and generated `free()`. `spectral_envelope_estimate_f32/f64`
accepts spectrum, FFT length, fundamental and harmonics for the native
one-shot equivalent. Input arrays remain unchanged.

Fundamental is a sampling-rate ratio in `(0, 0.5]`. Harmonics use optional
WASM32 unsigned integers; undefined/null selects the source convenience
wrapper's implicit count, while explicit counts must be 2 through i32::MAX.
`SpectrumNyquist.Preserve=0` and `Legacy=1` select both native Nyquist modes.
Smoothing independently accepts Stable=0 and Legacy=1. Defaults are Preserve
and Stable. Numeric codes are validated inside Rust Result, so invalid modes
leave plans reusable. Native validation inspects every nonnegative spectrum
bin, including Nyquist even in Legacy mode.

`SpectralEnvelopeF32/F64(log_spectrum, cepstrum)` retains both complete mutable
native output fields. The log spectrum is the full symmetric FFT grid in
natural-log units, without an exponential conversion. Cepstrum retains DC
and Nyquist. Constructors and setters accept arbitrary public storage and
payloads, including mismatched lengths. Getters return independent typed
arrays; `cloned()` and generated `free()` preserve independent ownership.
Snapshots outlive result and plan owners.

Actual Node.js and browser checks consume both complete ENV1 C corpora,
288 cases and 36,576 comparisons. Unchanged normalized bounds are 3e-7 for
binary32 and 5e-16 for binary64. Observed maximum errors for log spectrum and
cepstrum are `[8.094401442487092e-8, 1.3996163455650517e-8]` and
`[2.8748543874196476e-16, 2.720128377107085e-17]`, respectively.
No numerical criterion or native algorithm changed.

Checks include all independent Nyquist/smoothing combinations, explicit and
implicit harmonics, default/one-shot/full-option agreement, full output shapes,
metadata, reuse, immutable inputs, complete field mutation, raw IEEE payloads,
clones, snapshots, invalid codes and parameters and recovery after workspace
errors. Independent binary64 checks retain native constant-spectrum, full
symmetry and zero-cutoff criteria. Both precisions cover the two-point and
large implicit harmonic-count boundaries. Existing safe cutoff and harmonic
conversion corrections remain in the native implementation.

Run `node tests/wasm_envelope_node.cjs ./pkg-node` or serve
`tests/wasm_envelope_browser.html`. Eleven related native/C tests pass, along
with WASM Clippy and Rustdoc with warnings denied. Both functions and all four
owners are absent without the feature. Two original mappings bring coverage
to 308 implemented and 24 pending public mappings. Native/C source, fixtures,
dependencies and previous bindings remain unchanged. Remaining DSP coverage
and combined three-crate acceptance are still required.

## Complete linear prediction

Twenty-four `prediction_*_f32/f64` functions expose every native one-shot
operation: Levinson recursion, waveform estimation, magnitude estimation,
scaled estimation, gain and resonances. Explicit arithmetic accepts
`PredictionArithmetic.Stable=0` and `Legacy=1`; Nyquist independently accepts
Preserve=0 and Legacy=1. Defaults retain native Stable/Preserve behavior.
Codes are fallibly validated inside Rust Result.

`PredictionSpectrumPlanF32/F64(length)` exposes length, `is_empty()`,
`estimate`, `estimate_with_arithmetic`, `estimate_scaled` and `spectrum`.
All power-of-two lengths permitted by the native implementation are retained,
including a singleton forward spectrum. Inputs remain unchanged and errors
release workspace borrows so plans can be reused.

`PredictionF32/F64(coefficients, autocorrelation)` retains both full mutable
native fields, including arbitrary mismatched lengths and IEEE payloads.
`ScaledPredictionF32/F64` exposes copied coefficients, complete
`autocorrelation_parts()`, `gain_parts()`, `gain()`, `cloned()` and consuming
`into_prediction()`. Conversion consumes the owner on success or error.
Correlation parts contain copied Float64Array fractions and Int32Array
exponents plus length. Gain parts expose mutable binary64 fraction and i32
exponent, with typed `fraction_storage()` for payload-preserving copies.
Nonrepresentable ordinary correlations or gains fail without discarding the
available scaled representation in an independent owner.

`PredictionResonanceF32/F64(frequency_nyquist, real, imaginary)` exposes the
full native frequency and complex pole, `from_components`, `components`,
`set_components`, `pole`, `set_pole` and `cloned()`. Typed replacement validates
shape atomically. `PredictionResonanceEstimateF32/F64(resonances, iterations,
converged, maximum_residual)` retains all four mutable native fields.
Resonances are frequency/real/imaginary triples; `resonance(index)` returns an
independent owner or undefined. Binary64 `residual_storage()` and
`set_residual_storage()` preserve residual payloads. Arbitrary public result
storage is retained; no invented convergence or physical validation is added.
All twelve owners provide generated `free()`. Returned arrays outlive owners.

Actual Node.js and browser verification consumes all eight original and
high-precision corpora: 128 waveform, 96 spectrum, 22 resonance, 200 stable
recursion and 128 scaled waveform cases, with 10,664 numeric comparisons.
Waveform Legacy coefficients, correlations and gains, and resonance
frequencies/poles match their C references exactly. Spectrum comparisons
retain the five native normalized bounds: binary32
`[2e-8, 3e-8, 0, 7e-8, 8e-8]`, binary64
`[5e-17, 2e-15, 4e-16, 2e-15, 3e-15]`.
Stable coefficient maxima are 9.436895704622536e-16 and
1.926515763819101e-11, within unchanged 2e-15/6e-11 bounds. Scaled correlation
and coefficient comparisons retain native criteria; no bound was widened.

Additional checks cover independent regularized Toeplitz equations and
analytic spectra/poles, all policy combinations and methods, full public
field mutation, raw IEEE payloads, cloning, copied storage lifetime, consuming
success/error, invalid input/mode recovery, silence, ignored Nyquist,
maximum/subnormal magnitudes and waveforms, FFT overflow, cancellation,
normalized fractions/exponents and subnormal FFT coefficient estimation.
Run `node tests/wasm_prediction_node.cjs ./pkg-node` or serve
`tests/wasm_prediction_browser.html`. Forty-one related native/C tests pass,
with WASM Clippy/Rustdoc warnings denied and actual feature-disabled absence
of all twenty-four functions and twelve owners. Seven original mappings bring
coverage to 315 implemented and 17 pending. Native/C source, dependencies,
fixtures and previous bindings remain unchanged. Remaining families and
combined three-crate acceptance are still required.

## Complete WAVE byte processing

`wave_read_f32/f64(bytes, max_frames)` and
`wave_read_at_f32/f64(bytes, position, max_frames)` expose the native decoder.
Positions are unsigned 64-bit JavaScript BigInts, while the frame budget is
WASM32 usize. `WaveF32/F64` exposes every native mutable public field:
sample_rate, bits_per_sample, channels, encoding and samples. Encoding accepts
`WaveEncoding.Pcm=0` and `Float=1`; invalid codes fail atomically. Constructors
retain arbitrary public metadata and typed sample payloads. `position` reports
the native decoder's final seek position. `cloned()` and generated `free()`
provide independent ownership; sample snapshots outlive owners.

`WaveWriteOptions()` retains native defaults (16 bits, centered PCM8,
padding enabled). `WaveWriteOptions.legacy(bits)` selects source quantization
and disables padding. All three fields remain independently mutable.
`WavePcm8Quantization.Centered=0` and `Legacy=1` select valid modes; invalid
codes fail inside Result and leave options usable. Native validation of
sample widths, sample rate and finite output samples is unchanged.

`wave_write_pcm_f32/f64(samples, rate, options)` returns complete WAVE bytes.
`wave_write_pcm_at_f32/f64(bytes, position, samples, rate, options)` writes to
an independent byte stream, retaining existing prefix and suffix, filling a
seek gap with zero bytes and reporting the final position. `WaveBytes` exposes
copied bytes, position, `cloned()`, consuming `into_bytes()` and `free()`.
Input arrays and owners remain unchanged. Offset/size/allocation errors are
returned. Browser/Node bindings use byte streams; OS filename operations and
external stream callbacks remain native, as required by the binding design.
File and stream source mappings share the same complete byte codec.

Actual Node.js and browser tests consume both complete WAV1 corpora, 72
decode cases with 504 samples. The 308 unchanged samples match C exactly;
196 samples affected by the already disclosed native unsigned-PCM,
extensible-format and binary32 32-bit PCM fixes match independent standard
decode values. Both complete WVW1 corpora cover 24 encode cases and 1,396
exact C output bytes. No decoder or quantization algorithm changed.

Additional checks cover every byte-multiple PCM width through 128 bits,
IEEE float formats, stereo left-channel behavior, all public fields and raw
payloads, both quantizers with independent padding, safe clipping, metadata,
stream offsets/overwrite/seek gaps, clone and snapshot lifetimes, consuming
byte ownership, frame budgets, every truncated prefix, unknown padded chunks,
empty/odd final data, partial frames, duplicate/missing chunks, extension sizes,
extensible GUID/valid-bit validation, invalid modes and error recovery.
Run `node tests/wasm_wave_node.cjs ./pkg-node` or serve
`tests/wasm_wave_browser.html`. Twenty-one native/C tests pass. WASM
Clippy/Rustdoc deny warnings; feature-disabled checks confirm absence of all
eight functions and four owners. Four mappings bring coverage to 319
implemented and 13 pending. Native/C code, dependencies, fixtures and previous
bindings remain unchanged. Other families and combined acceptance remain.

## Complete random sampling

Six `random_*_f32/f64` functions expose `uniform_from_draw`,
`white_from_draw` and `normal_from_draws`. Inclusive draws and maximum are
u32, restricted to the native grid maximum 1 through i32::MAX. Gaussian
variance is variance, not standard deviation. `RandomPolicy.Stable=0` and
`Legacy=1` retain both native arithmetic policies; numeric codes fail inside
Result. Uniform/white arithmetic and Legacy Gaussian transforms retain their
selected-scalar division and source approximation behavior.

`RandomWordSource.from_seed(seed)` owns the same explicitly seeded ChaCha8
algorithm as the native Generator. Seeds must contain 32 bytes. Sources expose
`next_u32()`, BigInt `next_u64()`, `fill_bytes(count)`, copied `seed()`, BigInt
`stream()`, `set_stream(stream)`, `word_position()` and
`set_word_position(bytes)`. Word positions use complete 16-byte little-endian
u128 storage; the generator itself defines its 68-bit period wrap.
`cloned()` preserves independent ChaCha state.

`RandomWordSource(callback)` accepts a caller-owned JavaScript function
returning an underlying u32 word, not an already sampled grid draw.
`callback()` returns that function unchanged (or undefined for ChaCha).
Custom sources use the native rand inclusive range sampler. Their u64 words
place the first u32 in the low half; byte fills use little-endian words.
Cloning an external source shares the caller's function state, as a cloned
function handle does. ChaCha-specific state methods fail for external sources.
Thrown callback values retain identity; non-integer, non-finite and nonnumeric
returns are rejected. After the first callback failure no further callback
is invoked for that operation. Placeholder words only unwind the infallible
native RNG interface, and its result is discarded before returning the error.

`RandomSampler(source, maximum)` consumes the source and preserves all
native Sampler operations: maximum, draw, uniform_f32/f64, normal_f32/f64,
normal_with_f32/f64 and white_noise_f32/f64. `from_seed(seed)` selects the
native default grid. `cloned()` preserves stream state and `into_rng()`
consumes the sampler to return the complete underlying word source, retaining
external callback identity. Both owners provide generated `free()`.
Internal fallible borrowing makes reentrant sampling a normal Result error;
after a caught callback/reentrant error the owner is reusable and can be freed.
Invalid Gaussian/noise parameters and allocations consume no draws.
Zero variance consumes two grid draws; zero-amplitude noise still consumes
one per sample. Stable positive-variance sampling retains the 128-attempt
zero-first retry limit, while Legacy consumes exactly two draws.

Actual Node.js and browser tests consume both complete RND1 corpora: 256
cases and 1,024 exact C transform comparisons. Independent ChaCha8 tests
compare 140 words across zero/nonzero keys, streams, block boundaries and
68-bit wrap. Tests also cover source bytes/u64 order, state restoration,
consuming ownership, clone/snapshot lifetime, external source identity,
sampling consumption, zero-source retries, callback exception identity,
invalid words, range rejection recovery and reentrant error recovery/free.
Independent analytic/domain tests retain native bounds. The 200,000-sample
uniform/Gaussian moment tests use the unchanged native acceptance criteria.

Run `node tests/wasm_random_node.cjs ./pkg-node` or serve
`tests/wasm_random_browser.html`. Twelve native/C tests pass, along with
WASM Clippy/Rustdoc warnings denied and actual feature-disabled absence of
six functions/two owners. Three mappings bring coverage to 322 implemented
and 10 pending. Native/C algorithms, dependencies, fixtures and previous
bindings remain unchanged. Remaining families and combined acceptance remain.

## Complete plotting command streams

`PlotCommands(bytes)` owns the complete native `Commands<Vec<u8>>` stream.
`line_f32/f64(x, y, color_code)` accepts optional x coordinates; undefined or
null selects selected-scalar indices. Extra x values are ignored and a short
x buffer fails before any output. Color codes are `PlotColor.Blue=0`, Red=1,
Green=2, Black=3, Cyan=4 and Yellow=5. Invalid codes fail inside Rust Result.
`plot_color_from_character(string)` exposes native `Color::from(char)`:
lowercase r/g/k/c/y select their colors and every other Unicode scalar selects
blue. Empty or multiple-character input is rejected.

`image_f32/f64(rows, columns, values)` accepts the full native row-major
matrix, including zero dimensions. Shape, arithmetic overflow and native
dimension limits are checked before writing. All numeric and nonfinite
formatting remains in the native command implementation, retaining signed
zero, signed NaN, infinities, six-decimal coordinates and C-style scientific
image exponents. Input arrays remain unchanged.

Mutable `bytes` getter/setter provides complete arbitrary writer storage;
`append(bytes)` accepts raw configuration or other Gnuplot commands, including
non-UTF8 bytes. `cloned()` creates an independent stream, consuming
`into_inner()` returns its bytes, and generated `free()` releases the owner.
Snapshots outlive owners. These byte commands can be passed to a host renderer.
Native Figure process lifecycle (open/open_with/spawn, child ID, close/status,
captured output and Drop) remains available through native Rust and C ABI;
browser/Node WASM does not create OS child processes. Lifecycle mappings are
classified as native-only in the binding inventory; they are not claimed as
implemented WASM process APIs.

Actual Node.js and browser tests consume both complete original POSIX C
transcripts, 16 line and six image commands, with all 6,980 bytes exact.
Checks cover every color, optional/explicit/extra x, empty lines and matrices,
nonfinite/signed-zero/extreme values, raw stream replacement/append, invalid
shape/dimension/color atomicity, post-error reuse, consuming ownership, cloning
and snapshot lifetime. Twenty-one related native/C tests pass with real
Gnuplot required, including actual SVG generation, broken pipes, exit status,
process cleanup and concurrent output draining. WASM Clippy/Rustdoc deny
warnings; feature-disabled checks confirm absence of the color function and
command owner. Run `node tests/wasm_plotting_node.cjs ./pkg-node` or serve
`tests/wasm_plotting_browser.html`. Two command mappings bring coverage to
324 implemented and eight pending. Native/C code, dependencies, fixtures and
previous bindings remain unchanged. Combined acceptance remains required.

## Source scalar expressions, arbitrary folds and constants

`scalar_add_f32/f64(first, second)` and
`scalar_accumulate_square_f32/f64(accumulator, value)` expose the source
ADD and ADDSQR expressions in the selected scalar precision. Squaring precedes
addition without requesting fused multiply-add. Ordinary IEEE results, including
NaN and infinity, are retained.

`scalar_fold_f32/f64(values, initial, callback, context)` exposes the complete
caller-defined N_TO_ONE operation. It processes inputs in increasing order;
the callback receives accumulator, next value and the unchanged context object.
Its numeric result is converted to the selected scalar after each invocation.
Empty input returns the initial value unchanged and permits a missing callback.
Nonempty input requires a callback. JavaScript exceptions retain identity and
stop immediately; nonnumeric returns fail, while numeric NaN/infinity remain
valid IEEE values. Callbacks are synchronous and are not retained. Nested folds
are permitted, and input arrays are copied by the numeric-slice binding.

`scalar_math_constants()` returns an independent Float64Array containing
M_EPS, finite M_INF, pi and half-pi. Original bits are
`3cd203af9ee75616`, `430c6bf526340000`, `400921fb54442d18` and
`3ff921fb54442d18`. Native Rust pi constants avoid duplicate approximations.

Actual Node.js and browser verification consumes the complete RED1 tails from
both CUS1 corpora: 24 stateful custom folds and 72 callback invocations, with
exact C results. The preceding window records are verified separately by the
complete window binding suite, not counted as scalar comparisons. Tests include
42 selected-scalar expression checks, four exact constant comparisons, an
explicit non-fused square regression, order-sensitive context mutation,
signed-zero empty seeds, IEEE exceptional values, invalid/missing callbacks,
exception identity/early stopping, input immutability and nested folds.
No numerical criterion changed.

Run `node tests/wasm_scalar_node.cjs ./pkg-node` or serve
`tests/wasm_scalar_browser.html`. Eight related native/C tests pass, together
with WASM Clippy/Rustdoc warnings denied and feature-disabled absence of all
seven functions. Five mappings bring coverage to 329 implemented and three
pending public mappings at that checkpoint. Those three native process lifecycle
entries are now classified as native-only. Native/C code, dependencies,
fixtures and previous bindings remain unchanged; combined acceptance remains.

## Combined ciglet runtime verification

`tests/wasm_all_checks.mjs` explicitly runs all 40 existing family verifiers
sequentially against the same final module instance. Every family retains its
complete source corpus, numerical criteria, public configuration and ownership
checks. `wasm_all_node.cjs` independently compares its declared family list with
all individual Node suite files before running. Optional third argument saves
the full per-family result JSON. Serve `wasm_all_browser.html` to run the same
40 families in the browser. The page reports progress and marks success only
after every verifier returns successfully.

Actual Node.js 24.14.1 and the in-app browser both pass all 40 families against
the final complete WASM binary. `wasm_all_off.cjs` runs every individual
feature-disabled export check against the actual module without the feature;
all 40 confirm absence. These combined checks supplement the family evidence,
and do not claim Release, other WASM engines or final three-crate acceptance.

The combined Windows x86_64 MSVC native run uses
`cargo test --locked --all-features --tests` with actual Gnuplot required.
All 92 current integration suite files are present in the execution log;
93 summaries including the library suite report 463 passing tests, zero failed
and zero ignored. This is a refreshed host run, not a refreshed five-platform
or final three-crate gate. Native algorithms and individual verifiers were
unchanged during this combined verification checkpoint.

## Primary references

- [Supported numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [Mutable numeric slices](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html)
- [Optional numeric values](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/numbers.html)
- [Integer BigInt arguments](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/numbers.html#bigint-to-u64-i64-u128-and-i128)
- [JavaScript callbacks](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/reference/receiving-js-closures-in-rust.md)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [BigUint64Array and native u64 storage](https://wasm-bindgen.github.io/wasm-bindgen/api/js_sys/struct.BigUint64Array.html)
- [Returned Result errors](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [Exception destructor behavior](https://docs.rs/wasm-bindgen/0.2.129/wasm_bindgen/fn.throw_str.html)
- [Rust 1.94 atan2 precision](https://doc.rust-lang.org/1.94.0/std/primitive.f64.html#method.atan2)
- [Rust logarithm precision](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
- [Version-matched CLI installation](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/wasm-bindgen-test/usage.md)
- [Release library manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/Cargo.toml)
- [Release CLI manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/crates/cli/Cargo.toml)
- [rand 0.10 RNG trait migration](https://rust-random.github.io/book/update-0.10.html)
- [rand_chacha 0.10 ChaCha8 state](https://docs.rs/rand_chacha/0.10.0/rand_chacha/struct.ChaCha8Rng.html)
