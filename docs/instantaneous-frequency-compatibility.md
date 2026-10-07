# Instantaneous frequency detector

`instantaneous_frequency::Detector<f32/f64>` covers both creation aliases,
both deletion aliases through ownership/Drop, and both estimate aliases.
`Kernels` exposes all four original impulse responses; mutable access supports
custom filters. Length is derived from the arrays and checked before estimation.
The center-frequency metadata may be changed without regenerating filters,
as with the original structure. Frequencies and resolution are sampling-rate ratios.

Generation retains the selected-scalar division `4 / resolution`, truncation,
integer center for odd lengths, four cosine coefficients, exact host trig and
mixed-precision accumulation. Invalid resolution, empty/mismatched/nonfinite
filters, overflow and allocation failure return errors. Short input returns zero;
only the participating centered samples are validated.

Legacy estimation retains selected-scalar convolution, products and quotient,
followed by double division by two and pi. Undefined zero-energy or overflowing
results return errors instead of source NaN/infinity. Stable estimation scales
input and kernels independently, convolves in double precision and normalizes
the complex response before the quotient. It supports the tested maximum and
minimum nonzero amplitudes without introducing an energy floor. Cancellation
and response conditioning remain relevant; arbitrary custom filters are not
guaranteed to have a defined instantaneous frequency.

The unchanged C corpus uses 24 combinations of four centers and six resolutions,
including odd/even lengths and negative/zero centers. Four buffers per setting
exercise short inputs and both centering parities. Float kernels and Legacy
estimates must match bits exactly. Double kernels use normalized tolerance 3e-15;
double Legacy estimates use 1e-12. The initial Windows comparison measured a
maximum estimate error of 7.732053879099037e-13. Tiny host trig differences can
be amplified by weak responses. Regeneration checks all inputs and dimensions
exactly and permits only these measured double output bounds.

Independent tests verify a sinusoid at 0.125, analytic caller-designed filters,
extreme amplitudes, silent input, discarded nonfinite samples, mutable-filter
validation, cloning and invalid parameters. Broader off-center/chirped-signal
accuracy and conditioning remain part of the final numerical audit.

All 129 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686. Formatting, Clippy and warning-as-error
Rustdoc pass. Cross-host C regeneration measured maximum normalized double
kernel error 1.1804333753159537e-16 and estimate error 6.236352585145109e-13;
float fixtures are byte-identical. The original corpus function bodies are
extracted without modification, with no C linked into the Rust library.
