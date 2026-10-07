# Public configuration coverage audit

Function coverage alone is not the complete port acceptance criterion. A direct
scan of all header `#define` names found the following items absent as distinct
rows in api-inventory.csv. Several are already implemented and documented under
their generated functions or ownership equivalents. Absence from that CSV does
not imply omission or establish completion. This audit remains open until each
semantic mapping and precision/build configuration has concrete evidence.

| Header item | Current mapping or required review |
| --- | --- |
| CIG_CORR_ACF, AMDF, SQRDIFF, YIN | correlogram::Method has all four methods; verify numeric code mapping for later bindings |
| malloc2d, copy2d, free2d, flatten, reshape, transpose | Array2 ownership/shape operations; existing array compatibility and C corpus cover both macro and implementation names |
| CIG_DEF_N_TO_ONE | Source left-to-right fold with caller operation and initial value; document standard Rust iterator fold as the extensibility equivalent and check selected-scalar accumulation |
| CIG_DEF_ADD, ADDSQR, MAX, MIN | Arithmetic closures/statistics and scalar strict comparisons; retain original operation grouping and tie behavior |
| CIG_DEF_HANNING, HAMMING, MLTSINE | Generated window functions exist; review arbitrary caller-selected trig customization, not only instantiated names |
| CIG_DEF_BLACKMAN_HARRIS, NUTTALL98, BLACKMAN | Same custom-trig review, including coefficient precision |
| CIG_DEF_STFT | FrameGrid instantiations exist; audit arbitrary optlv mapping against the full source forward-STFT options |
| M_EPS, M_INF | Internal uses retain source literals in multiple numerical families; audit whether callers need named public equivalents |
| M_PI, M_PI_2 | Standard Rust scalar constants replace these; audit every selected-scalar/double promotion site |
| CIGLET_H | Include guard has no runtime API; Rust module/package ownership replaces repeated header inclusion |

Public header structs also require field/configuration review: cplx,
ifdetector, filterbank, lfmodel and the POSIX-only figure. Complex scalars reuse
num-complex; detector kernels and LF fields are public. Ownership invariants
may replace raw field writes with checked constructors/mutators, but every
defined configuration capability must remain available. Figure remains part of
the owned plotting implementation, with mutable Rust writer access. Verify filterbank custom support/response
configuration independently of construction fixtures.

This initial audit covers ciglet.h only. A complete audit must also examine
implementation export locations, build-selected FP_TYPE behavior and bundled
public interfaces, plus the independent SHIRO/liblrhsmm inventories. Numeric
stability fixes and native workflow acceptance remain separate requirements.
