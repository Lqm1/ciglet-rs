# LF preparation conditioning audit

The preparation corpus contains 208 configurations per precision. Eighty use
the Rd mapping, including both sides of 0.21 and 2.7, shapes from 0.01 to 100
and periods from 10 microseconds to 100 milliseconds. Another 128 combine
four closure ratios, peak ratios approaching half or all of the closure,
return ratios approaching zero or the available closing interval, and two
periods on opposite sides of the 800 Hz rescaling boundary.

`tests/reference/glottal_preparation_oracle.c` includes the pinned original
`ciglet.c` translation unit so its private preparation, function and derivative
can be inspected directly. Library expressions are not copied into the oracle.
Each record contains the five model fields, twelve prepared fields, two function
residuals and the Newton derivative. The generator compiles at `-O2` with
contraction disabled and runs original and endpoint-corrected variants.

## Endpoint correction

The temporary corrected source adds only `fa == 0` and `fb == 0` returns before
the original root fallback. The upstream checkout is never changed. This
matches the previously documented Rust endpoint correction; see
[root compatibility](root-compatibility.md).

Eight float cases change prepared values: 80, 88, 96, 104, 145, 153, 161 and
169. In case 80 the original root result is 500,000,000 with residual 0.5.
The corrected endpoint is 1,000,000,000 with residual zero. Private Rust
preparation matches the corrected C fields exactly for all 208 float cases.
On primary Windows, the maximum normalized double field difference is
1.1489779368115758e-15, below the corpus-specific 2e-14 bound. These comparisons
check ten stored prepared fields per case, 2,080 comparisons per precision.
The peak and integral are C metadata that Rust need not store separately.

Windows Zig/Clang generates the fixtures and Linux GCC regenerates both
variants. Model and preparation fields are exact across hosts in float mode.
Diagnostic residuals and derivatives may differ after cancellation of double
host exponential terms even when all float preparation fields are identical.
Their cross-host difference is scaled by one plus the sum of the absolute
terms in the corresponding equation, rather than by the near-zero residual.
The measured maximum float cross-host scaled difference is
1.0561295183219422e-16; double is 1.1489779368115758e-15.

## Finite values do not establish convergence

All 208 records have finite stored parameters, residuals and derivatives in
both variants and precisions. They do not all solve the defining equations
accurately. Endpoint handling does not correct the following separate limits.

Case 176 has closure 0.95, peak approximately 0.47500095 and return ratio
approximately 5e-8. Its very short return interval interacts with the original
upper bracket `2 / (Ta + 1e-9)`. The root routine takes its nonbracketing
midpoint fallback; the decay residual is approximately 0.94117647. This is a
finite fallback value, not evidence of an accurate decay root.

Case 108 has closure 0.2, peak approximately 0.1999998 and return ratio 0.4.
After exactly eight source Newton updates, the growth residual is approximately
-2016.82 in float and -2017.96 in double. Scaling by one plus the absolute
equation terms gives residuals approximately 0.07428259 and 0.07432174.
The growth reaches millions and the stored initial excitation rounds to zero.
This corpus therefore does not establish universally usable waveforms or
spectra for every ordered timing configuration.

The compatibility implementation retains the source bracket and eight-update
algorithm. This audit changes no solver, numeric tolerance, output policy or
public API. Existing invalid/nonfinite output checks remain separate from
these finite preparation comparisons. Further numerical work must distinguish
source compatibility, genuine convergence and any explicit alternative policy.
The final native acceptance gate and broader ciglet conditioning audit remain
open.
