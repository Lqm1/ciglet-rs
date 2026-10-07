# Custom window and reduction macros

`Window::generate_custom` covers the callable customization of all six
`CIG_DEF_*` tapered-window macros. The caller supplies an FnMut receiving the
source double angle and returning Result<f32/f64, Error>. Its output precision
is independent of the weight precision. Hann subtracts in the callback result
type before double multiplication. Multi-term windows retain selected weight
coefficients and use binary32 products/accumulation only when both types are
binary32; otherwise C's common double arithmetic is retained. MltSine requests
sine; the other tapered windows request cosine. Negative finite weights remain
supported. Callback errors and nonfinite/unrepresentable results are reported.

Invocation order is sample-major, then increasing harmonic order. This is
explicitly deterministic in Rust; C operand evaluation need not guarantee that
order. Empty, singleton and boxcar generation never invokes the callback.
Singletons retain the existing corrected unity convention, preventing the
source symmetric-window division by zero. Allocation and length checking occur
before callback invocation; errors may retain effects in caller-owned state.

The source Blackman macro ignores its cosfunc argument and directly calls cos_3.
The new custom API honors its argument. Existing generate_with still preserves
both accurate-cosine Blackman aliases, so ordinary source behavior is unchanged.
The C oracle verifies that the original custom Blackman invokes the supplied
callback zero times. For its custom comparison only, two explicitly disclosed
instantiation adapters rebind cos_3 to the requested callback. The header is not
modified; the other five window macros and reduction macro are unmodified.

Each precision fixture contains 60 custom-window records, 528 weights and
callback counts. A nontrigonometric polynomial callback returns either f32 or
f64, exercising all four weight/result precision combinations across six
windows and lengths 0, 2, 3, 8 and 31. Results compare by exact bits, not a
tolerance. The independent callback-order/zero-cosine test confirms custom
Blackman returns its constant term rather than a built-in cosine window.
Domain/state tests cover early allocation validation, singleton/boxcar bypass,
callback failure propagation and invalid/unrepresentable callback results.

`CIG_DEF_N_TO_ONE` maps directly to standard Rust Iterator::fold; no wrapper
is required. Evaluate the chosen initial expression once, then traverse the
original slice from left to right. Keep accumulator/sample operations in the
chosen scalar type. The four named helper operations map to ordinary addition,
selected-scalar multiply-then-add, and scalar::minimum/maximum strict comparisons.

```rust
let samples = [1.0_f32, 2.0, 3.0];
let initial = 0.25_f32;
let result = samples.iter().copied().fold(initial, |accumulator, value| {
    accumulator * 0.5 - value * 0.125
});
assert_eq!(result, -0.5);
```

Twelve C macro records per precision use a noncommutative stateful operation,
lengths 0/1/3/8 and ordinary/negative-zero/maximum seeds. Exact final bits and
invocation counts validate the standard fold mapping, including no calls for
empty input. Do not replace this contract by an unspecified parallel reduction.

`Precision::from_legacy_level` covers arbitrary signed STFT optlv values: below
three selects approximate windows, three and above selects accurate windows.
Direct source inspection confirms optlv is used only by get_window. The regular
STFT macro always chooses Blackman, whose two aliases already use accurate
cosine. FrameGrid and AnalysisOptions retain its centers, dimensions and outputs;
their existing C STFT corpus covers the instantiated level-two/three workflows.
Public Method codes retain CIG_CORR_ACF/AMDF/SQRDIFF/YIN as 0/1/2/3, with checked
conversion rejecting unknown codes. LEGACY_EPSILON and LEGACY_INFINITY_SENTINEL
retain M_EPS=1e-15 and finite M_INF=1e15; IEEE pi constants replace M_PI/M_PI_2.

Primary references:

All final 154 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686, including required Gnuplot rendering. Every
custom-window weight and custom-fold result matches the C fixture bits on all
five targets; Windows Zig and Linux GCC regenerate identical reference bytes.
Accurate/approximate built-in equivalence is checked in both weight precisions,
except the deliberately different custom Blackman approximation. Formatting,
Clippy with warnings denied and warning-as-error Rustdoc pass. Public struct,
complete build/precision and numerical-stability audits remain unfinished.

- https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold
- https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html
