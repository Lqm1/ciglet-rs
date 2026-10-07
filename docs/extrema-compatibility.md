# Extrema and strict local peaks

`extrema::extremum` implements `cig_find_extrema`, `find_maxima` and
`find_minima` through `Direction::Maximum` and `Direction::Minimum`.
`extrema::peak` implements `cig_find_peak`, `find_peak` and `find_valley`.
Both accept an inclusive index range and return an exact usize index.

Global extrema use strict comparisons, so the first equal extreme remains
selected. Local peak search requires strict comparison with both neighbors.
It retains the original starting-index fallback when no peak beats that
starting sample. A start sample higher than all actual local peaks remains
the returned index, even when it is not itself a local peak. Plateaus do not
count as strict local peaks. Neighbor comparisons may inspect values just
outside the requested range, as in C.

Whole-array endpoints have no two-sided local peak and are skipped safely.
The starting-index fallback still applies. Empty, reversed or out-of-bounds
ranges return errors. Only inspected values must be finite: global search
validates its range, local search validates its actual candidate neighborhood
and fallback value. Unused nonfinite values outside that neighborhood do not
prevent the operation. Bounds use Rust slices and windows(3), following the
standard slice documentation:
https://doc.rust-lang.org/std/primitive.slice.html#method.windows.

C evaluates x[i-1] at index zero and x[i+1] at the last index. AddressSanitizer
reproduces the first case as a stack-buffer-underflow at ciglet.c:371 using
`tests/reference/extrema_bounds_oracle.c`. Rust avoids these invalid reads.
The C return index is first stored in FP_TYPE, losing
integer precision for sufficiently large f32 arrays; Rust keeps integer
indices throughout. The typed direction covers the two signs documented
by the source, without multiplying each inspected value by an orientation.

## Reference coverage

`tests/generate_extrema_reference.py --upstream /path/to/ciglet --check`
rebuilds the unchanged source using GCC -O2. Each precision has 180 cases,
comparing 360 exact integer results across lengths 9, 32 and 65, five signal
shapes, six inclusive ranges, both directions, and both search operations.
Original C references use interior ranges with valid neighbor storage.
Independent Rust tests verify endpoint fallback, empty/reversed/out-of-bounds
ranges, first-equal tie selection, one-element input, and nonfinite values
inside and outside the actual inspected neighborhood. All 81 integration tests
pass on Windows MSVC64/32, Windows GNU64 and Linux GNU64/32. Fixture regeneration,
formatting and Clippy with warnings denied pass on the primary target.
