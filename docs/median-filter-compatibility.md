# Median filter compatibility

`median::filter` implements `cig_medfilt` and `medfilt1` for finite f32/f64
samples. Interior windows contain the requested width. The first width/2
outputs use growing prefixes ending at the output index; the last width/2
outputs use shrinking suffixes starting at that index. These original
asymmetric endpoint windows are retained. Even widths are supported, with
the average of the two middle values in each window.

The implementation reuses one scratch vector. It calls standard Rust
`select_nth_unstable_by` once and finds the maximum of the lower partition
for an even-length window. This avoids fully sorting the window and avoids
a per-output allocation. The source's specialized fixed-size median networks
are covered by numeric comparison; Rust uses the standard selection routine
instead of duplicating those networks. Primary Rust documentation:
https://doc.rust-lang.org/std/primitive.slice.html#method.select_nth_unstable_by.

The original even median adds in the selected scalar type, multiplies by the
double literal 0.5, then rounds to that scalar type. Rust retains this path
when the sum is finite. If the sum overflows despite finite inputs, Rust
averages half of each operand in double precision, giving a finite median.
Regressions include repeated f32::MAX and f64::MAX values.

Width zero, widths outside the legacy integer range, widths exceeding a
nonempty signal, and nonfinite samples return errors. Empty input with a
valid positive width returns empty output. Singleton input with width one
returns that sample. The C implementation reads beyond short inputs during
its initial window copy and can overrun output during its endpoint loops.
`tests/reference/median_filter_bounds_oracle.c` reproduces the one-sample,
width-three case with AddressSanitizer. The Rust API rejects it before allocation.

## Reference coverage

`tests/generate_median_reference.py --upstream /path/to/ciglet --check`
rebuilds the unchanged C implementation and its original fast_median helper
with GCC -O2. Each precision has 96 cases and 3,904 output values over lengths
25, 32 and 65, four signal shapes, and widths 1, 2, 3, 4, 5, 7, 9 and 25.
Both generic even-width and specialized odd-width source branches are covered.
Additional Rust tests check exact endpoint examples, invalid widths, NaNs,
empty/singleton inputs and representable extreme medians. All 78 integration
tests pass on Windows MSVC64/32, Windows GNU64 and Linux GNU64/32. Reference
regeneration, formatting and Clippy with warnings denied pass.
