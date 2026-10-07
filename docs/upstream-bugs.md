# Upstream bugs to address

These findings refer to ciglet commit 895ba9b1c0eabee83d3544208bbc82420efa3206. Recording a finding does not establish that its Rust replacement is implemented or verified.

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
