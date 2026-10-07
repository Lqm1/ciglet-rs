# Installed bundled approximation interfaces

The pinned ciglet Makefile installs `external/fastapprox-all.h` alongside
`ciglet.h`, which includes it by default. Its public approximation functions
are therefore tracked separately from the original own-header inventory.
They remain part of the complete accepted functionality scope.

The installed header defines 44 scalar functions and 44 SSE2 four-lane
functions. `bundled-approximation-inventory.csv` records each one. Thirteen
scalar functions already map to existing approximation APIs. Eight further
scalar functions now map to `approximation::hyperbolic`, leaving 23 scalar
and all 44 four-lane functions pending. Existing mappings do not establish
complete bundled-specific verification by themselves.

## Hyperbolic and sigmoid functions

| Source | Rust call |
| --- | --- |
| fastsinh / fastersinh | hyperbolic::sine with Approximate / Fast |
| fastcosh / fastercosh | hyperbolic::cosine with Approximate / Fast |
| fasttanh / fastertanh | hyperbolic::tangent with Approximate / Fast |
| fastsigmoid / fastersigmoid | hyperbolic::sigmoid with Approximate / Fast |

The APIs share the existing selected exponential kernels rather than adding
eight duplicated entry points. Formula stages remain binary32, including
subtraction/addition, multiplication by one half and the tangent/logistic
quotients. `Precision::Exact` additionally selects the host exponential before
binary32 conversion; it does not replace the surrounding formulas with a
different hyperbolic solver or a numerically saturated rewrite.

Nonfinite inputs and unrepresentable exponential/results return errors.
Domain limits follow the selected exponential inputs: for example tangent
evaluates at `-2 * value`. An unrepresentable exponential is rejected even if
a differently arranged mathematical formula could yield a finite answer.
Ordinary valid source values preserve operation order and rounding.

The independent oracle includes the installed original header unchanged.
Sixteen values per function cover -40 through 40, small magnitudes, both
signs and both zero signs. All 128 results match Rust binary32 bits. Windows
Zig and Linux GCC regenerate byte-identical fixtures with `-O2` and contraction
disabled. Additional tests verify nonfinite/range errors and zero values in
Exact mode. No Cargo dependency or original source modification is introduced.

## Remaining interfaces

Error/complementary-error and inverse-error functions, log-gamma/digamma,
Lambert W variants, the faster power and restricted-range trigonometric
functions still require implementation and independent C verification.
The four-lane functions require their own source arithmetic audit; mapping
each to a scalar call must not be presumed bit-equivalent. Portable owned
four-element arrays can replace platform vector types without requiring C/SSE
layout in the Rust API. Conversion/index/splat/absolute-value helper macros
and union representations also require explicit safe ownership/bit-operation
mappings. They are not removed from the audit because they are macros/types.

The header's include guards, compiler conditional selection and the single-file
packaging mode have no independent runtime algorithm. The single Cargo library
replaces that packaging configuration. Fastapprox's BSD attribution is retained
in `LICENSES/fastapprox-BSD.txt` for existing and new adapted formulas.
