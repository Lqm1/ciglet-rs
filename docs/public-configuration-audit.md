# Public configuration coverage audit

Function coverage alone is not the complete port acceptance criterion. A direct
scan of all header `#define` names found the following items absent as distinct
rows in api-inventory.csv. Several are already implemented and documented under
their generated functions or ownership equivalents. Absence from that CSV does
not imply omission or establish completion. Each semantic mapping and precision
configuration needs concrete evidence. The completed mappings are now recorded
below; final native execution is documented
in SHIRO's native acceptance audit.

| Header item | Current mapping or required review |
| --- | --- |
| CIG_CORR_ACF, AMDF, SQRDIFF, YIN | Explicit Method i32 codes 0/1/2/3 and checked conversion; tests/public_configuration.rs |
| malloc2d, copy2d, free2d, flatten, reshape, transpose | Array2 ownership/shape operations; existing array compatibility and C corpus cover both macro and implementation names |
| CIG_DEF_N_TO_ONE | Standard slice Iterator::fold; noncommutative/stateful operation, initial seed, counts and bits checked against twelve C records per precision |
| CIG_DEF_ADD, ADDSQR, MAX, MIN | Scalar closures/statistics and strict scalar comparisons; existing statistics/scalar C fixtures retain grouping/ties |
| CIG_DEF_HANNING, HAMMING, MLTSINE | Window::generate_custom supports mutable/fallible caller trig and independent callback/weight precision; sixty C window records per precision cover all six macros |
| CIG_DEF_BLACKMAN_HARRIS, NUTTALL98, BLACKMAN | Same custom API with source coefficient staging; disclosed Blackman callback fix compared to explicit C instantiation adapters |
| CIG_DEF_STFT | FrameGrid/analyze_regular plus Precision::from_legacy_level; source get_window has only below-three/at-least-three classes, and regular Blackman aliases share accurate cosine |
| M_EPS, M_INF | Public scalar::LEGACY_EPSILON and finite LEGACY_INFINITY_SENTINEL, exactly 1e-15/1e15 |
| M_PI, M_PI_2 | Standard Rust constants replace both names; mixed-precision use sites remain part of the broader numeric audit |
| CIGLET_H | Include guard has no runtime API; Rust module/package ownership replaces repeated header inclusion |

## Own-header structure fields

| C fields | Rust configuration and evidence |
| --- | --- |
| cplx.real, imag | complex::Complex.re, im; construction/arithmetic/bit comparisons in tests/complex.rs |
| ifdetector.fc | Detector::center_frequency, set_center_frequency and from_kernels; tests/instantaneous_frequency.rs |
| ifdetector.nh, hr, hi, hdr, hdi | Kernels::real, imaginary, derivative_real, derivative_imaginary; length follows the vectors; from_kernels and kernels_mut preserve caller-designed filters and revalidate them before use |
| filterbank.nchannel, nf | FilterBank::channels and bins; checked owned construction ties dimensions to response storage |
| filterbank.fnyq | nyquist_hz and set_nyquist_hz; arbitrary metadata remains supported without changing custom responses |
| filterbank.fresp, lower_idx, upper_idx | response, support and set_channel expose channel response and half-open support; 33 unchanged-C records per precision cover signed/custom responses and empty-frame behavior |
| lfmodel.T0, te, tp, ta, Ee | Model::period, closure, peak, return_time, excitation; all five remain public; tests/glottal.rs and the 208-case preparation corpus |
| figure.handle | Commands<W> accepts a caller-owned Write implementation; Figure owns native process/stdin lifetime and exposes mutable command writer access; tests/plotting.rs exercises both command bytes and actual Gnuplot |

Independent dimension or owning-pointer mutation is replaced by checked owned
construction. Callers can reconstruct resized detectors/banks rather than leave
counts inconsistent with allocations. Mutable detector kernels are checked on
use; filterbank channel changes validate shape/support before committing.
These mappings describe safe Rust functionality, not original C layout/ABI.
See instantaneous-frequency-compatibility.md, filterbank-configuration-audit.md,
complex-compatibility.md, glottal-compatibility.md and plotting-audit.md for
source arithmetic, corrections and reference reproduction.

The installed fastapprox header has a separate 88-function inventory with all
function mappings implemented. Its conversion, index, splat, bit and absolute
value helpers are mapped in bundled-approximation-compatibility.md. C/SSE
storage unions become typed arrays and checked conversions, with source-specific
sign/order behavior retained or disclosed as a correction.

The original Makefile defaults to FP_TYPE=float and supports double builds.
Rust scalar-storage APIs use selected f32/f64 types within one package rather
than rebuilding an incompatible global typedef. Scalar/bundled fastapprox
kernels intentionally retain their original binary32 storage; generic outer
arithmetic preserves the documented source conversion stages. Both precisions
are exercised by the algorithm-specific C corpora and independent tests.
Public exports are the modules in src/lib.rs and corresponding symbols in
api-inventory.csv, including ownership equivalents. This maps packaging and
precision selection without adding a second package or hiding standalone APIs.

The original Makefile installs only ciglet.h and external/fastapprox-all.h.
Its linked external FFT, median and WAV implementation objects are represented
by the corresponding Rust algorithms and waveform APIs. Single-file generation,
archive naming, PIC and include guards are C build/packaging mechanisms.
OpenMP is an implementation execution choice rather than a separate public
algorithm; Rust checked owned outputs replace shared C buffers.
Native target execution and numerical corrections remain separate from these
public-definition mappings.

Custom window/reduction evidence and the intentionally corrected Blackman
argument behavior are detailed in custom-window-compatibility.md. This closes
those extensibility mappings. The own-header structures, installed bundled
helpers, package exports and selected precision now have explicit mappings;
final acceptance still uses the execution and numerical evidence as well.
