# Public configuration coverage audit

Function coverage alone is not the complete port acceptance criterion. A direct
scan of all header `#define` names found the following items absent as distinct
rows in api-inventory.csv. Several are already implemented and documented under
their generated functions or ownership equivalents. Absence from that CSV does
not imply omission or establish completion. This audit remains open until each
semantic mapping and precision/build configuration has concrete evidence.

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

Public header structs also require field/configuration review: cplx,
ifdetector, filterbank, lfmodel and the POSIX-only figure. Complex scalars reuse
num-complex; detector kernels and LF fields are public. Ownership invariants
may replace raw field writes with checked constructors/mutators, but every
defined configuration capability must remain available. Figure remains part of
the owned plotting implementation, with mutable Rust writer access. Verify
filterbank custom support/response, metadata and explicit empty-frame
configurations are now checked independently of construction fixtures through
33 unchanged-C records per precision. See filterbank-configuration-audit.md.

This initial audit covers ciglet.h only. A complete audit must also examine
implementation export locations, build-selected FP_TYPE behavior and bundled
public interfaces, plus the independent SHIRO/liblrhsmm inventories. Numeric
stability fixes and native workflow acceptance remain separate requirements.

Custom window/reduction evidence and the intentionally corrected Blackman
argument behavior are detailed in custom-window-compatibility.md. This closes
those extensibility mappings, not the complete structure/build/precision audit.
