# Moving average and RMS compatibility

`smoothing::moving_average_with(..., Policy::Legacy)` implements `cig_moving_avg` and `moving_avg` for
f32/f64 samples and a fractional half-width. It retains source cumulative sums,
repeated endpoint padding, uniform-grid interpolation and operation order.
An integer half-width h averages the 2h samples from i-h+1 through i+h, with
endpoint replication. For h=1, output is the average of the current and next
sample, including a repeated final sample at the end. This right bias is retained.

Fractional widths retain the source's unusual cumulative-sum interpolation grid.
The integer padding is the truncated width, while interpolation starts at the
negative fractional width and ends at length plus integer padding. This is not
replaced with a conventional centered fractional window. Accumulation and division
remain in the selected scalar type; the final multiplication by 0.5 follows C's
double literal before rounding to that type.

`moving_rms_with(..., Policy::Legacy)` implements the original helper, whose integer `order` argument is
actually passed as a half-width. It squares in the selected scalar type, uses
moving_average, and takes a double-precision square root before rounding to the
scalar type. Nonpositive mean-square values return zero, matching the source.
Input slices are borrowed, and returned vectors are owned.

Empty input returns empty output. Width zero returns the original values for
moving_average and absolute values for moving_rms, correcting C division by zero.
Singleton input is safe because uniform interpolation checks its endpoint instead
of reading past the accumulator. Negative or nonfinite widths, nonfinite samples,
overflowing squares/sums/results and excessive padded lengths return errors.

## C references

`tests/generate_smoothing_reference.py --upstream /path/to/ciglet --check`
rebuilds the unchanged-C references using the pinned upstream and GCC -O2.
Each precision contains sixty cases and 2,280 output values across lengths 9,
16 and 32, constant/periodic/impulse/quadratic signals, and half-widths 0.75,
1, 1.5, 3 and 7.25. The RMS reference uses integer width truncation plus one
so all reference calls avoid the original width-zero bug. Every average and
RMS output is checked independently against C. Rust regressions verify width
zero, empty and singleton input, the retained right bias and invalid values.
All 75 integration tests pass on Windows MSVC64/32, Windows GNU64 and
Linux GNU64/32, including these three smoothing tests. Fixture regeneration,
formatting and Clippy with warnings denied pass.

## Stable default arithmetic

The default operations now select `Policy::Stable`. Integer window locations
and endpoint replication remain unchanged. Fractional widths integrate the same
unit-spaced sample cells rather than stretching the source cumulative grid.
This corrects the source's loss of DC: nine unit samples at half-width 1.5
previously produced values below one. Positive widths below one average the
current and next sample; zero width explicitly remains an identity operation.
This discontinuity follows the retained right-biased window definition.

Compensated exponent buckets store rolling sums, signed endpoint contributions
and scaled squared samples. Each input enters and leaves the rolling window
once. Large widths do not allocate repeated endpoint padding. RMS avoids raw
square overflow and underflow. The private shared accumulator uses about 68 KiB
of fixed stack storage; the returned vector remains heap allocated.

`tests/generate_stable_smoothing_reference.py --check` independently integrates
sample-cell overlaps using exact rational arithmetic. RMS roots use 220 decimal
digits, and both outputs are directly rounded to the selected IEEE format.
Each precision covers 330 cases and 10,920 outputs, including maximum values,
cancellation, subnormals, large offsets, impulses, singleton arrays, zero and
tiny positive widths, fractional widths and widths above a billion samples.
On Windows MSVC x86_64 every float output agrees. Double normal results use a
4e-15 relative bound; subnormal RMS permits one minimum quantum because rounding
the binary64 mean-square can place its root on a subnormal rounding boundary.
Zero and subnormal averages retain exact comparison. This is not a universal
correct-rounding claim. The unchanged sixty-case C corpus selects Legacy and
continues to match exactly. All 165 integration tests pass on Windows MSVC
x86_64/i686, Windows GNU x86_64 and Linux GNU x86_64/i686, including required
Gnuplot rendering. Original-C and mathematical reference regeneration,
formatting, Clippy and warning-as-error Rustdoc pass.
