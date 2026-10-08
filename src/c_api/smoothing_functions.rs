//! Binary32/binary64 moving statistics and median filtering.
use super::smoothing::{self, Integer};

/// Stable moving average; fractional half-widths and endpoint replication retained.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_average_f32(
    signal: *const f32,
    count: usize,
    half_width: f32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::average(signal, count, half_width, 0, output, output_count) }
}

/// Moving average policy zero Stable, one Legacy cumulative interpolation grid.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_average_with_f32(
    signal: *const f32,
    count: usize,
    half_width: f32,
    policy: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::average(signal, count, half_width, policy, output, output_count) }
}

/// Stable moving RMS with scaled squares and endpoint replication.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_rms_f32(
    signal: *const f32,
    count: usize,
    half_width: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe {
        smoothing::integer(
            signal,
            count,
            half_width,
            Integer::Rms(0),
            output,
            output_count,
        )
    }
}

/// Moving RMS policy zero Stable, one Legacy source arithmetic.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_rms_with_f32(
    signal: *const f32,
    count: usize,
    half_width: usize,
    policy: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe {
        smoothing::integer(
            signal,
            count,
            half_width,
            Integer::Rms(policy),
            output,
            output_count,
        )
    }
}

/// Median filter with original growing-prefix and shrinking-suffix endpoints.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_median_filter_f32(
    signal: *const f32,
    count: usize,
    width: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::integer(signal, count, width, Integer::Median, output, output_count) }
}

/// Stable moving average; fractional half-widths and endpoint replication retained.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_average_f64(
    signal: *const f64,
    count: usize,
    half_width: f64,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::average(signal, count, half_width, 0, output, output_count) }
}

/// Moving average policy zero Stable, one Legacy cumulative interpolation grid.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_average_with_f64(
    signal: *const f64,
    count: usize,
    half_width: f64,
    policy: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::average(signal, count, half_width, policy, output, output_count) }
}

/// Stable moving RMS with scaled squares and endpoint replication.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_rms_f64(
    signal: *const f64,
    count: usize,
    half_width: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe {
        smoothing::integer(
            signal,
            count,
            half_width,
            Integer::Rms(0),
            output,
            output_count,
        )
    }
}

/// Moving RMS policy zero Stable, one Legacy source arithmetic.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_moving_rms_with_f64(
    signal: *const f64,
    count: usize,
    half_width: usize,
    policy: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe {
        smoothing::integer(
            signal,
            count,
            half_width,
            Integer::Rms(policy),
            output,
            output_count,
        )
    }
}

/// Median filter with original growing-prefix and shrinking-suffix endpoints.
/// Complete staged output has exactly input length. Errors preserve all output;
/// input/output overlap is supported after input borrowing ends. Null empty buffers
/// are not borrowed or copied. Native finite-input, width and allocation validation
/// still applies, including to zero-sized calls where relevant.
///
/// # Safety
/// Nonempty input is aligned initialized stable readable single-allocation storage.
/// Output is aligned exclusively writable single-allocation storage for its count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_median_filter_f64(
    signal: *const f64,
    count: usize,
    width: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller buffer contracts and complete native staging.
    unsafe { smoothing::integer(signal, count, width, Integer::Median, output, output_count) }
}
