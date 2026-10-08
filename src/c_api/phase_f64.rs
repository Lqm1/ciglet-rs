//! Typed DCT and phase entry points.
use super::{phase, vector_result};

/// Compute the native orthonormal DCT-II with its legacy cosine approximation.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_dct_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe { vector_result(input, length, output, output_length, crate::cosine::dct) }
}

/// Unwrap phase using the native single-turn correction.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_phase_unwrap_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe { vector_result(input, length, output, output_length, crate::phase::unwrap) }
}

/// Wrap one finite angle using native turn rounding.
/// Errors leave output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_phase_wrap_f64(angle: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe { phase::scalar(output, || crate::phase::wrap(angle)) }
}

/// Compute the native directed circular phase difference.
/// Errors leave output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_phase_difference_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe { phase::scalar(output, || crate::phase::difference(first, second)) }
}

/// Compute complex magnitudes from equally sized real/imaginary planes.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_magnitude_f64(
    first: *const f64,
    first_length: usize,
    second: *const f64,
    second_length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe {
        phase::paired(
            first,
            first_length,
            second,
            second_length,
            output,
            output_length,
            phase::magnitude,
        )
    }
}

/// Compute complex arguments from equally sized real/imaginary planes.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_argument_f64(
    first: *const f64,
    first_length: usize,
    second: *const f64,
    second_length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe {
        phase::paired(
            first,
            first_length,
            second,
            second_length,
            output,
            output_length,
            phase::argument,
        )
    }
}

/// Compute polar real components with the native cosine approximation.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polar_real_f64(
    first: *const f64,
    first_length: usize,
    second: *const f64,
    second_length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe {
        phase::paired(
            first,
            first_length,
            second,
            second_length,
            output,
            output_length,
            crate::phase::polar_real,
        )
    }
}

/// Compute polar imaginary components with the native sine approximation.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polar_imaginary_f64(
    first: *const f64,
    first_length: usize,
    second: *const f64,
    second_length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe {
        phase::paired(
            first,
            first_length,
            second,
            second_length,
            output,
            output_length,
            crate::phase::polar_imaginary,
        )
    }
}

/// Complete a spectrum. Symmetry: zero symmetric, one antisymmetric.
/// Nyquist: zero preserves the supplied bin, one retains legacy substitution.
/// Other option codes return status three.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, stable single-allocation ranges.
/// Nonempty output must be aligned and exclusively writable for output_length
/// values. Empty buffers accept null; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complete_spectrum_f64(
    input: *const f64,
    length: usize,
    symmetry: u32,
    nyquist: u32,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage contracts.
    unsafe { phase::complete(input, length, symmetry, nyquist, output, output_length) }
}
