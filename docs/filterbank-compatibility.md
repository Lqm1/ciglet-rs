# Filterbank compatibility findings

Filterbank construction and application are still pending. This records source findings for the next implementation step, not completed API coverage.

The pinned source is ciglet commit 895ba9b1c0eabee83d3544208bbc82420efa3206. `cig_create_melfreq_filterbank` uses `melspace`, scalar frequency arithmetic, floor/ceil/round bin boundaries, and triangular weights. The frequency coordinate uses `nf / fnyq`, rather than `(nf - 1) / fnyq`; retain this convention for compatibility. `melspace` returns `nchannel + 1` points, so the final `freqs[j + 1]` access is valid.

## Small-grid buffer overflow

The following original-C call produces a heap buffer overflow with AddressSanitizer:

```c
filterbank *bank = cig_create_melfreq_filterbank(
    65, 8000, 20, 50, 8000, 1, 400);
cig_delete_filterbank(bank);
```

Reproduce with `gcc -O1 -g -fsanitize=address -DFP_TYPE=float -ffunction-sections -fdata-sections -I<upstream> probe.c <upstream>/ciglet.c -Wl,--gc-sections -lm`. No FFT source is needed after unused functions are discarded. The observed failure is a four-byte write immediately before a 260-byte response allocation at ciglet.c:1320. The original first-channel lower index starts at zero, but rounding the center to zero followed by `lower_idx = min(lower_idx, centr_idx - 1)` changes it to minus one.

Rust must support the corresponding small grid without negative indexing. Preserve original responses for valid in-range C boundaries, constrain corrected support to the available bins, and test the low-center case independently. Undefined C writes cannot serve as normative response fixtures. Empty/collapsed supports and out-of-range warped centers also require explicit validation or a documented safe correction.

## PLP and application arithmetic

PLP channel centers use integer Bark coordinates. Equal-loudness weights use source scalar squaring, and the Bark mask uses single-precision `fastpow`, including in the double build. The first channel has zero equal-loudness weight. Retain these details rather than adopting a different auditory model.

Application sums weighted spectral values in FP_TYPE over each stored half-open support. Its two compression modes are double `pow(sum, 0.33)` or single-precision `fastlog(sum + 1e-15)`. The FFT length and sample-rate parameters are unused in the original application functions; the Rust interface should use the bank's validated shape instead. Spectrum and spectrogram operations must share the same scalar accumulation order. Owned response storage and validated support ranges replace manual destruction and unvalidated public pointers.
