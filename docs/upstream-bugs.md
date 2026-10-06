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
