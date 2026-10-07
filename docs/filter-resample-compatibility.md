# FIR, convolution and resampling compatibility

`filtering::windowed_fir` implements cig_winfir with typed lowpass, highpass,
bandpass and bandstop responses. Cutoffs use cycles per sample in 0..=0.5.
The `fir` and `bandpass_fir` convenience functions implement fir1 and fir1bp
with cutoffs normalized to Nyquist. Length means tap count, including for odd
and non-power-of-two lengths. The implementation retains source scalar cutoff
multiplication, fractional edge-bin assignment, the source even-length Nyquist
replacement, direct normalized inverse DFT, frequency shift and precise window
weights. The original window dispatcher supports boxcar, Hann, Hamming,
Blackman and Blackman-Harris; the typed Rust window API also supports its
existing Nuttall and MLT sine definitions.

`filtering::convolve` produces the complete nx+nh-1 linear convolution with
the original coefficient accumulation order. It indexes the borrowed input
directly and eliminates the original padded input allocation. Empty operands
produce empty output. Nonfinite operands, dimension overflow and allocation
failure return errors.

`resampling::resample` implements the original rounded output length and
eight-position Lanczos interpolation with the source level-two sine kernel.
Downsampling first uses the original 32-tap Hamming lowpass response and
16-sample delay compensation. Ratio one copies the original samples and bits
without scalar length rounding. Other ratios use source-precision multiplication
and division, with the round operation evaluated in double precision as in C.
Positive finite ratios and finite inputs are required. Output lengths and
interpolation indices are checked before integer conversion.

The C source excludes sample zero with `ix_base + j > 0`. Default Rust
`BoundaryPolicy::IncludeFirst` corrects this to include index zero. Explicit
`LegacySkipFirst` reproduces the original behavior for comparisons or existing
pipelines that require its exact boundary convention. With a single 0.75 sample
and ratio two, original C produces two zeros; corrected Rust retains 0.75 at
the first output sample. This correction does not change a file format.

## Original-C reference

Pinned ciglet commit: 895ba9b1c0eabee83d3544208bbc82420efa3206.
The driver is `tests/reference/filter_resample_oracle.c` and links unmodified
ciglet.c and external/fftsg_h.c. Build independently for FP_TYPE=float and
FP_TYPE=double:

```text
gcc -O2 -DFP_TYPE=float -ffunction-sections -fdata-sections -I<upstream> driver.c <upstream>/ciglet.c <upstream>/external/fftsg_h.c -Wl,--gc-sections -lm -o oracle
oracle fixture.bin
```

FRS1 starts with uint32 scalar width and counts 300/60/105 for FIR,
convolution and resampling. FIR records contain uint32 tap count/window kind/
passband kind, scalar lower/upper cutoffs, then tap-count scalar coefficients.
Convolution records contain uint32 signal/kernel lengths, input signal and
kernel scalars, then full output scalars. Resampling records contain uint32
input length, scalar ratio, input scalars, uint32 output length, then output
scalars. All fields are little-endian.

FIR cases cover lengths 2/3/8/31/32, five original window types, four passbands,
and three cutoff pairs including coincident zero, integer-bin, fractional-bin
and Nyquist edges. Convolution covers lengths 1/2/5/32/65 and kernels 1/2/7/32.
Resampling covers those signal lengths and ratios 0.25/0.5/0.9/1/1.1/1.5/2.
Signal patterns are silence, a first-sample impulse and a signed repeating ramp.
There are 4,560 FIR coefficients, 1,830 convolution samples and 2,298 resampled
samples per precision. Legacy resampling comparisons use LegacySkipFirst.

Initial Windows MSVC x86_64 comparisons match all f32 outputs exactly.
Maximum normalized f64 differences are 4.163336342344337e-17 for FIR,
zero for convolution and 8.326672684688674e-17 for resampling. Scoped bounds
are 2e-7 for f32 and 2e-15 for f64, relative to max(1,abs(expected)).
Independent checks cover convolution against a hand-computed polynomial,
empty inputs, identity resampling bit preservation, corrected first-sample
behavior, output rounding, invalid cutoffs/ratios and nonfinite rejection.
All 56 current integration tests pass on Windows MSVC x86_64/i686, Windows GNU x86_64 and Linux GNU x86_64/i686. All f32 reference outputs match exactly on every target. Formatting and Clippy with warnings denied pass. Original -Ofast comparisons remain pending.

Rust primary references for borrowed slices, allocation and checked dimensions:
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact,
https://doc.rust-lang.org/std/primitive.slice.html and
https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add.
