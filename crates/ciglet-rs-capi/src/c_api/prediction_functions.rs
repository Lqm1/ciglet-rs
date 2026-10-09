//! Typed standalone Levinson recursion and prediction gain.
use super::prediction;

/// Estimate coefficients from nonnegative-lag autocorrelation.
/// Arithmetic zero Stable, one Legacy. Output count equals input count.
/// Failures retain output; zero-energy handling remains native.
///
/// # Safety
/// Nonempty input must be initialized and stable in one allocation. Output
/// must be aligned and exclusively writable. Input/output may overlap after
/// input borrowing ends. Empty buffers accept null, but empty recursion fails.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_levinson_f32(
    input: *const f32,
    input_count: usize,
    arithmetic: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the caller's initialized/writable buffer contracts.
    unsafe { prediction::levinson(input, input_count, arithmetic, output, output_count) }
}

/// Compute prediction gain from matching nonempty coefficient/correlation arrays.
/// Arithmetic zero Stable, one Legacy. Scalar overflow/invalid energy remain native errors.
///
/// # Safety
/// Inputs must be initialized readable single-allocation ranges, stable during
/// computation. Output must be aligned and exclusively writable for one value.
/// Input/output may overlap after input borrowing ends. Failures retain output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_gain_f32(
    coefficients: *const f32,
    coefficient_count: usize,
    correlation: *const f32,
    correlation_count: usize,
    arithmetic: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        prediction::gain(
            coefficients,
            coefficient_count,
            correlation,
            correlation_count,
            arithmetic,
            output,
        )
    }
}

/// Estimate coefficients from nonnegative-lag autocorrelation.
/// Arithmetic zero Stable, one Legacy. Output count equals input count.
/// Failures retain output; zero-energy handling remains native.
///
/// # Safety
/// Nonempty input must be initialized and stable in one allocation. Output
/// must be aligned and exclusively writable. Input/output may overlap after
/// input borrowing ends. Empty buffers accept null, but empty recursion fails.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_levinson_f64(
    input: *const f64,
    input_count: usize,
    arithmetic: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the caller's initialized/writable buffer contracts.
    unsafe { prediction::levinson(input, input_count, arithmetic, output, output_count) }
}

/// Compute prediction gain from matching nonempty coefficient/correlation arrays.
/// Arithmetic zero Stable, one Legacy. Scalar overflow/invalid energy remain native errors.
///
/// # Safety
/// Inputs must be initialized readable single-allocation ranges, stable during
/// computation. Output must be aligned and exclusively writable for one value.
/// Input/output may overlap after input borrowing ends. Failures retain output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_gain_f64(
    coefficients: *const f64,
    coefficient_count: usize,
    correlation: *const f64,
    correlation_count: usize,
    arithmetic: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's input and scalar output contracts.
    unsafe {
        prediction::gain(
            coefficients,
            coefficient_count,
            correlation,
            correlation_count,
            arithmetic,
            output,
        )
    }
}
