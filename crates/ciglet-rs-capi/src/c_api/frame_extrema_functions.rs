//! Typed frames and global/local extrema entry points.
use super::frame_extrema;

/// Fetch a centered, zero-padded frame of output_length samples.
/// Supports odd/even lengths and out-of-signal centers. Input/output may overlap.
/// Empty buffers accept null. Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fetch_frame_f32(
    input: *const f32,
    input_length: usize,
    center: isize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe { frame_extrema::frame(input, input_length, center, output, output_length) }
}

/// Search the first global extremum with strict tie comparisons.
/// Range start/end is inclusive. Direction zero Maximum, one Minimum.
/// Invalid directions/ranges or nonfinite inspected samples return status three.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Output must be aligned and exclusively writable for one usize. It may overlap
/// input only after input borrowing ends; no conflicting concurrent access is permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_extremum_f32(
    input: *const f32,
    length: usize,
    start: usize,
    end: usize,
    direction: u32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        frame_extrema::search(
            input,
            length,
            start,
            end,
            direction,
            output,
            crate::extrema::extremum,
        )
    }
}

/// Search strict two-sided local peaks, retaining the start-index fallback.
/// Range start/end is inclusive. Direction zero Maximum, one Minimum.
/// Invalid directions/ranges or nonfinite inspected samples return status three.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Output must be aligned and exclusively writable for one usize. It may overlap
/// input only after input borrowing ends; no conflicting concurrent access is permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_peak_f32(
    input: *const f32,
    length: usize,
    start: usize,
    end: usize,
    direction: u32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        frame_extrema::search(
            input,
            length,
            start,
            end,
            direction,
            output,
            crate::extrema::peak,
        )
    }
}

/// Fetch a centered, zero-padded frame of output_length samples.
/// Supports odd/even lengths and out-of-signal centers. Input/output may overlap.
/// Empty buffers accept null. Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Nonempty output must be aligned and exclusively writable for output_length samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fetch_frame_f64(
    input: *const f64,
    input_length: usize,
    center: isize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer contracts.
    unsafe { frame_extrema::frame(input, input_length, center, output, output_length) }
}

/// Search the first global extremum with strict tie comparisons.
/// Range start/end is inclusive. Direction zero Maximum, one Minimum.
/// Invalid directions/ranges or nonfinite inspected samples return status three.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Output must be aligned and exclusively writable for one usize. It may overlap
/// input only after input borrowing ends; no conflicting concurrent access is permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_extremum_f64(
    input: *const f64,
    length: usize,
    start: usize,
    end: usize,
    direction: u32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        frame_extrema::search(
            input,
            length,
            start,
            end,
            direction,
            output,
            crate::extrema::extremum,
        )
    }
}

/// Search strict two-sided local peaks, retaining the start-index fallback.
/// Range start/end is inclusive. Direction zero Maximum, one Minimum.
/// Invalid directions/ranges or nonfinite inspected samples return status three.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be an initialized, stable single-allocation range.
/// Output must be aligned and exclusively writable for one usize. It may overlap
/// input only after input borrowing ends; no conflicting concurrent access is permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_peak_f64(
    input: *const f64,
    length: usize,
    start: usize,
    end: usize,
    direction: u32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        frame_extrema::search(
            input,
            length,
            start,
            end,
            direction,
            output,
            crate::extrema::peak,
        )
    }
}
