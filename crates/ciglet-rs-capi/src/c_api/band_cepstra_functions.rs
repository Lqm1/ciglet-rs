//! Typed band-cepstrum C entry points.
use super::band_cepstra::{self, Output};

/// Return the selected non-DC coefficients, optionally followed by DC.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_to_cepstrum_f32(
    input: *const f32,
    input_length: usize,
    order: usize,
    include_dc: u32,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Selected,
        )
    }
}

/// Preserve the complete legacy band-sized buffer, including its DCT suffix.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_to_cepstrum_legacy_f32(
    input: *const f32,
    input_length: usize,
    order: usize,
    include_dc: u32,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Legacy,
        )
    }
}

/// Transform a flattened frame-major matrix into selected cepstra.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_spectrogram_to_cepstra_f32(
    input: *const f32,
    input_length: usize,
    bands: usize,
    order: usize,
    include_dc: u32,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Matrix { bands },
        )
    }
}

/// Return the selected non-DC coefficients, optionally followed by DC.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_to_cepstrum_f64(
    input: *const f64,
    input_length: usize,
    order: usize,
    include_dc: u32,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Selected,
        )
    }
}

/// Preserve the complete legacy band-sized buffer, including its DCT suffix.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_to_cepstrum_legacy_f64(
    input: *const f64,
    input_length: usize,
    order: usize,
    include_dc: u32,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Legacy,
        )
    }
}

/// Transform a flattened frame-major matrix into selected cepstra.
/// Order clamps to band count minus one; include_dc accepts only zero or one.
/// Explicit output length must match the native shape. Errors retain output.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length
/// scalars. Input/output may overlap; input borrowing ends before output commit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_band_spectrogram_to_cepstra_f64(
    input: *const f64,
    input_length: usize,
    bands: usize,
    order: usize,
    include_dc: u32,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe {
        band_cepstra::transform(
            input,
            input_length,
            order,
            include_dc,
            output,
            output_length,
            Output::Matrix { bands },
        )
    }
}
