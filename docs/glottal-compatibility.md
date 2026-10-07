# LF glottal model

`glottal::Model::from_shape` covers both Rd-construction aliases. Public fields
map T0/te/tp/ta/Ee to period/closure/peak/return_time/excitation. Period is seconds;
the other timing fields are period ratios. Direct caller configuration is
supported, including finite signed excitation and zero excitation. `prepare`
owns a validated model and reusable derived coefficients;
`spectrum` and `sampled_period` cover both corresponding public alias pairs.

Construction retains the source piecewise thresholds at Rd 0.21 and 2.7,
mixed scalar/double arithmetic and five returned values. Preparation retains
the 800 Hz rescaling threshold, exact host sine/cosine/exp, the source Brent
search through the checked root API and exactly eight Newton updates. Invalid
timings, nonfinite inputs/derived results, zero divisions and allocation failure
are reported as errors. The original unconverged finite eight-update result is
not silently replaced by a different optimizer; broader residual/conditioning
checks remain in the final numerical audit.

Default spectrum and waveform retain binary32 exponential/trigonometric
approximations, selected-scalar complex multiplication/division and source
rounding. Return-phase exponential subtraction is binary32 even for double
models. The stability shortcut for e*(1-e*Ta) < 1 is retained. DC magnitude and
phase are returned directly as positive zero, matching the source's final
NaN-to-zero handling while eliminating the intermediate division by zero.
Other nonfinite responses return errors rather than being silently masked.

`Prepared::spectrum_with` additionally selects Fast/Approximate/Exact complex
exponential/trig. Source approximate trig produces a small positive/negative
frequency asymmetry; compatibility tests retain it, while independent Exact
tests verify conjugate symmetry. This precision option does not change model
preparation, the source stability shortcut or accurate output atan2.

Sampling preserves the rounded closure/period indices, inclusive opening
phase, source sample-time staging and zero-filled suffix. Empty and shortened
outputs are supported. Positive finite sample indices are converted to usize
and clipped to the requested output before slicing, avoiding the source int
conversion/addition range hazards. Nonpositive sampling rates are rejected.

The unchanged C corpus has 53 records (48 generated and five directly configured).
It covers low/high pitch and the exact 800 Hz boundary, zero/signed excitation,
positive/negative frequencies, DC, different sampling rates and
zero/truncated/extended outputs.
It compares 265 model fields and 8,304 magnitude/phase/wave outputs per precision.
Model fields and float response/waveform bits are compared exactly. Initial
Windows MSVC x86_64 double normalized maxima are 2.5905548438963652e-18 for
magnitude, 1.5812666814618965e-15 for phase and 2.0378073940352437e-16 for wave.
Bounds are respectively 1e-17, 3e-15 and 5e-16. Cross-host C regeneration checks
all inputs/model fields/dimensions and float bytes exactly, permits only these
double output bounds, and checks returned zero signs exactly.

Independent tests verify amplitude scaling and sign inversion, period
truncation/padding, plan reuse, source asymmetry and Exact symmetry, caller
configuration, finite large-index clipping and errors. Source approximate sine
at zero can leave a tiny nonzero first waveform sample; that measured source
artifact is retained rather than asserted to be mathematically zero.
GCC float-cast-overflow probes confirm the source sample-index conversion
overflow for a two-second model at INT_MAX sampling rate in both precisions.
Separate float-divide-by-zero probes confirm source DC division in both
precisions. Rust returns the requested finite clipped waveform and direct DC
zero respectively. These four probes stop at the reported source arithmetic
defect without relying on its later undefined/nonfinite output.
The source bounds driver is `tests/reference/glottal_bounds_oracle.c`.
Primary implementation references are the shared complex/root compatibility
documents and [Rust floating-point rounding](https://doc.rust-lang.org/std/primitive.f64.html#method.round),
[numeric cast semantics](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast)
and [fallible Vec reservation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact).

All 138 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686 after the final signed-excitation and
large-index regressions. Formatting, Clippy and warning-as-error Rustdoc pass.
Float fixture regeneration is byte-identical on Windows Zig and Linux GCC;
double regeneration differs only within the stated output bounds.
