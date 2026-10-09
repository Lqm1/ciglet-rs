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
six mappings, DCT/phase adds fifteen and vectors add six. Other DSP families and both remaining crates still
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
Inverse.

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
Eight native and three C-interface Fourier regression tests pass. WASM Clippy
and Rustdoc with warnings denied pass; an actual feature-disabled module
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

## Primary references

- [Supported numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [JavaScript callbacks](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/reference/receiving-js-closures-in-rust.md)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Rust 1.94 atan2 precision](https://doc.rust-lang.org/1.94.0/std/primitive.f64.html#method.atan2)
- [Version-matched CLI installation](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/wasm-bindgen-test/usage.md)
- [Release library manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/Cargo.toml)
- [Release CLI manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/crates/cli/Cargo.toml)
