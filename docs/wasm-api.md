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
added. The inventory records nineteen complete window mappings. The Fourier
coverage below adds six more. Other DSP families and both remaining crates still
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

## Primary references

- [Supported numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [JavaScript callbacks](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/reference/receiving-js-closures-in-rust.md)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Version-matched CLI installation](https://github.com/wasm-bindgen/wasm-bindgen/blob/main/guide/src/wasm-bindgen-test/usage.md)
- [Release library manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/Cargo.toml)
- [Release CLI manifest](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.129/crates/cli/Cargo.toml)
