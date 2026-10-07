# Upstream bugs to address

These findings refer to ciglet commit 895ba9b1c0eabee83d3544208bbc82420efa3206. Recording a finding does not establish that its Rust replacement is implemented or verified.

## Magnitude prediction sizing and spectrum range loss

flpc with a singleton spectrum derives a zero FFT length, then writes into a
zero allocation. lpspec writes every coefficient into its fixed transform
workspace without checking its length; a sufficiently long polynomial overruns
the allocation. The original-C bounds oracle confirms both heap errors under
AddressSanitizer in float and double. Rust requires at least two magnitude bins,
a bounded transform/order and nonempty coefficients that fit the transform.

lpspec squares FFT components in the selected precision. A maximum constant
polynomial/gain loses the finite unit ratio through an infinite denominator;
a minimum positive constant/gain instead divides by zero after square underflow.
Zero gain at a grid pole produces NaN. The driver confirms each original result.
Rust uses double hypot on square range loss, validates nonzero-gain divisors
and returns signed zeros for zero gain. Independent tests recover finite unit
spectra and zero output and verify plan reuse after failures. Legacy/default
Nyquist behavior remains explicit as in STFT. See prediction-spectrum-compatibility.md.

## Levinson short inputs and silence

cig_levinson unconditionally reads R[1] and writes a[1]. Empty and singleton
arrays therefore access outside their allocation, including the otherwise
valid order-zero lpc workflow. The original-C bounds driver reproduces these
errors under AddressSanitizer for float and double. prediction::levinson
rejects an empty correlation and returns the leading unit coefficient for a
singleton. from_samples supports order zero when a waveform has a sample.

Zero-energy correlation produces 0/0 reflection coefficients and nonfinite
coefficients. The source driver confirms this for both precisions. Rust returns
the zero predictor [1,0,...] for entirely zero correlation, yielding zero gain;
nonzero lags with zero energy are rejected. Negative/nonfinite energy, invalid
shapes and nonpositive recursion residuals return errors. General extreme-scale
stability remains part of the final numerical audit. See prediction-compatibility.md.

## Complex division and approximate argument range loss

c_div squares its divisor and multiplies operands before dividing. Maximum
values divided by themselves yield NaNs through infinity/infinity; smallest
positive values divided by themselves yield NaNs through zero/zero. A finite
quotient can also disappear when numerator products completely underflow.
The original-C bounds driver confirms all three in both precisions.
complex::divide preserves normal source grouping and uses scaled arithmetic
only on intermediate range loss or nonfinite results. Independent Rust tests
recover the unit quotient, maximum finite quotient and the lost tiny quotient.
Zero divisors and unrepresentable results return errors. Partially subnormal
arithmetic remains part of the final numerical-stability audit.

fastatan2 can compute infinity/infinity after the initial y/x ratio overflows,
returning NaN for finite Cartesian components. The same driver confirms this
through c_arg_1. The Rust implementation returns the signed half-pi limit for
an overflowing ratio and uses a reciprocal rational form when only its square
overflows. Ordinary rational arithmetic and its signed-zero conventions are
preserved. See complex-compatibility.md and tests/complex.rs.

## Empty vector helpers and integer progression

diff, cumsum and unwrap access element zero for an empty array. complete_symm
and complete_asymm additionally read element minus one before writing element
zero. The dedicated bounds oracle reproduces the first three with GCC's
undefined-behavior sanitizer and the latter two with AddressSanitizer. The
Rust vector/phase operations return empty output or complete an empty spectrum
without accessing any element.

iota computes `start + i * step` in signed int arithmetic. For the representable
sequence -2000000000, 0, 2000000000, the multiplication at i=2 overflows before
the starting value can be added. The bounds oracle reproduces that signed
overflow. Rust uses i64 intermediates and checks each final i32 value. A final
value outside i32 returns an error. Tests also verify singleton signed zero,
empty/singleton grids, safe interpolation/magnitude range fallbacks and the
documented default/Legacy Nyquist distinction. See vector-phase-compatibility.md.

## Single-sinusoid amplitude and short lengths

The inline gensin function in ciglet.h accepts an amplitude but never uses it. Its initial samples and recurrence produce a unit-amplitude sinusoid regardless of the amplitude argument. It also unconditionally writes indices 0 and 1, so lengths below two access outside the allocated buffer. The Rust synthesis::sinusoid implementation now applies amplitude to its initial recurrence samples. Both synthesis entry points handle zero and one sample lengths safely. tests/synthesis.rs verifies zero, fractional, negative, and greater-than-one amplitudes against an independent centered cosine formula, plus short inputs. Compatibility fixtures use the unmodified single-sinusoid C function only with unit amplitude; multisinusoid C fixtures include nonunit amplitudes. These are intentional corrections, not claims that the original ignored-amplitude behavior is retained.

## WAVE sample decoding and stream ownership

The original 8-bit PCM reader maps byte 128 to 0.5 instead of silence, because it subtracts 0.5 after dividing by 128. Microsoft documents 128 as the unsigned PCM midpoint. wave::read subtracts 128 before scaling.

Extensible PCM is assigned FORMAT_EXT but the sample loop only decodes FORMAT_PCM and FORMAT_IEEE_FLOAT. Its allocated output remains zero. The Rust reader resolves the PCM GUID to integer decoding. The C oracle explicitly records these zero-valued legacy results.

In FP_TYPE=float builds, 32-bit PCM uses float accumulation of an unsigned magnitude followed by subtraction of the sign bias. The encoded integer -1 consequently decodes to zero in the recorded C fixture. Rust sign-extends the integer before normalization; -1 decodes to -1/2^31. Ordinary 8/16/24/32 PCM, extensible PCM, and wider safely representable integer containers use the same standard normalization path.

On header failure wavread_fp closes its caller's FILE pointer, and wavread then closes it again. Rust borrows Read+Seek streams, propagates I/O errors, and does not close borrowed objects. Truncation, invalid metadata, partial frames, duplicate chunks, and missing interior padding return errors. Unknown chunks are skipped by their declared lengths rather than scanning payloads for fmt/data strings. The original writer's unpadded odd final data chunk is still readable.

Source references: https://learn.microsoft.com/en-us/windows/win32/multimedia/devices-and-data-types and https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible. These corrections are intentional behavior differences; unchanged-C comparisons are reported separately.

## WAVE output quantization and padding

The original PCM8 writer maps silence to byte 127 with (sample + 1) * 127, although the unsigned PCM midpoint is 128. The default Rust writer uses the correct midpoint; Pcm8Quantization::Legacy retains original quantization when explicitly requested. The original writer also omits RIFF padding for odd data lengths. Default output adds one zero pad byte and includes it in the RIFF size, while the data chunk size continues to count valid sample bytes only. WriteOptions::legacy preserves original files for compatibility comparisons.

Signed PCM quantization converts to int before clipping. For FP_TYPE=float, the PCM32 maximum 2147483647 rounds to 2147483648 in multiplication, making the subsequent positive-unit float-to-int cast out of range. Larger magnitudes at every depth can likewise leave the conversion domain. Rust clips the product before the cast, rejects nonfinite input before output, and propagates write/flush failures. The C fixtures only exercise defined conversion inputs. Existing destination files are not truncated on validation failure.

Specification references: https://learn.microsoft.com/en-us/windows/win32/multimedia/devices-and-data-types and https://learn.microsoft.com/en-us/windows/win32/xaudio2/resource-interchange-file-format--riff-.


## Resampling first-sample omission

The original cig_rresample interpolation checks ix_base + j > 0, excluding source index zero for every nonidentity ratio. A single nonzero input sample therefore produces only zeros after upsampling. resampling::resample uses corrected first-sample inclusion. resample_with offers BoundaryPolicy::LegacySkipFirst for unchanged-C comparisons and pipelines requiring original boundary values. Ratio one still copies every input sample exactly. The independent impulse regression in tests/filter_resample.rs distinguishes corrected and legacy outputs. See docs/filter-resample-compatibility.md for the original-C corpus and precision measurements.

## Spectral peak neighbor bounds

The original cig_qifft reads both neighbors without checking k or the input length. spectral_peak::interpolate rejects endpoint and missing-neighbor indices before indexing. Flat parabolas take an explicit zero-displacement path rather than computing a zero divisor; their finite outputs retain C behavior. Nonfinite inputs and overflowing polynomial arithmetic return errors. Extreme finite-input rescaling remains under audit. See docs/spectral-peak-compatibility.md.

## Real cepstrum bounds and minimum-phase lifter

The original rceps has no positive-transform-size validation; a zero-length call reaches invalid symmetry writes. irceps copies arbitrary coefficient counts into a fixed transform workspace; a 32-coefficient prefix with FFT length eight writes beyond its allocation. The unmodified C bounds driver reproduces both heap-buffer-overflows for float and double under AddressSanitizer. Rust validates lengths before indexing or writes.

The minphase lifter doubles coefficient nfft/2 and clears negative quefrencies only from nfft/2+2, retaining coefficient nfft/2+1. That retained sample contributes to phase and breaks the intended causal lifter. Lifter::Causal retains DC/Nyquist, doubles positive quefrencies and clears every negative quefrency. Lifter::Legacy preserves both original loops, independently of NyquistPolicy. Analytic tests isolate the retained sample contribution; unchanged-C fixtures compare Legacy output. See docs/real-cepstrum-compatibility.md.

## Aliased sinc pole sign and derivative plateau

The original safe_aliased_sinc returns length whenever the rounded sine denominator is below 1e-4, assigning a positive limit for positive even lengths at 2*pi. The correct limit there is negative length. The original safe_aliased_dsinc returns zero below 1e-5, hiding nonzero small-angle slopes and permitting cancellation elsewhere. The Stable policy evaluates in double, uses small-angle series and retains integer-length pole parity; the Legacy policy preserves original thresholds and scalar rounding. Analytic length-two/three and fractional-slope tests verify the corrections independently of C. See docs/sinc-compatibility.md.

## Itakura-Saito approximate-log artifacts

The source itakura_saito returns an approximate logarithm of the mean divergence. Its binary32 bit-based logs introduce a nonzero identity baseline and produce finite artifacts for zero, overflowing or NaN intermediate ratios/means. The corrected distance policy uses a factored near-identity series, log-domain extreme ratios and scaled compensated accumulation. Identical positive spectra yield zero mean and negative-infinity log mean; a zero against a positive bin yields infinity; a paired zero is an explicit domain error. Legacy preserves original accumulation and bit-approximation outputs, including the tested artifacts. No implicit input floor is introduced. See docs/distance-compatibility.md.

## Instantaneous frequency invalid resolution and undefined response

The C constructor converts unchecked 4/resolution to int and allocates without validating the result. Zero, negative, nonfinite or excessive resolution can yield undefined conversion or zero/invalid lengths. Rust checks the finite positive resolution and nonzero bounded kernel length before allocation. C estimation divides by zero for silence and can overflow for finite extreme amplitudes. Rust reports undefined response explicitly; Stable scales before convolution and quotient. Legacy retains ordinary source arithmetic. See docs/instantaneous-frequency-compatibility.md.
