# Moving average and RMS compatibility

`smoothing::moving_average` implements `cig_moving_avg` and `moving_avg` for
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

`moving_rms` implements the original helper, whose integer `order` argument is
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
