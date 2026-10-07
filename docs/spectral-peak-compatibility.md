# Quadratic spectral peak interpolation

`spectral_peak::interpolate` implements `cig_qifft` and its `qifft` alias from
ciglet commit `895ba9b1c0eabee83d3544208bbc82420efa3206`. It returns an owned
`Peak` containing the fractional FFT-bin position and interpolated magnitude.
Convert bin position to frequency using the caller's sample rate and FFT size.
Signed input values are supported, including log magnitudes. The function does
not search for a peak or require the center sample to be a local maximum.

The three samples define the same quadratic polynomial as C. The neighbor sum
and subsequent polynomial operations use the selected scalar precision; C's
double literals promote the curvature calculation, half-offset calculation
and amplitude cap at the same points. Displacements strictly smaller than one
bin are retained. Flat polynomials and larger displacements return the original
bin. The interpolated magnitude is capped at the center magnitude plus 0.2.

The C implementation reads `magn[k-1]` and `magn[k+1]` without bounds checks.
Rust rejects missing neighbors, including empty input and excessively large
indices. Only the three participating samples are validated. Nonfinite samples
and overflowing polynomial arithmetic return errors. An infinite displacement
ratio is rejected as a displacement and falls back to zero, matching C's
existing one-bin rule. General finite-input rescaling for extreme polynomial
arithmetic remains a numerical stability audit item.

`tests/generate_spectral_peak_reference.py` extracts the unchanged function
body from the pinned source and compiles it with the unchanged header at `-O2`,
without `-Ofast`. The binary fixtures contain 64 cases per scalar precision,
including flat spectra, symmetric peaks, linear triples, asymmetric peaks,
signed samples, out-of-range displacements and amplitude caps. Each case stores
three input samples and the two C outputs, totaling 128 compared outputs per
precision. The tests require exact output bits. Independent tests check an
analytic parabola, the amplitude cap and input errors.

Regenerate with `python tests/generate_spectral_peak_reference.py --upstream
PATH`; use `--check` to verify existing fixtures. Windows generation supports
`--cc zig --zig`; Linux uses GCC by default. Windows Zig and Linux GCC must
produce identical fixture bytes for both precisions.
