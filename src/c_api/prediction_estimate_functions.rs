//! Typed ordinary and scaled one-shot prediction estimates.
use super::{
    prediction_estimate::{self, Domain, Output},
    prediction_scaled::{CigletRsScaledPredictionF32, CigletRsScaledPredictionF64},
};

/// Estimate from samples, returning both coefficients and autocorrelation.
/// Arithmetic zero Stable, one Legacy. Sample order must be smaller than input count.
/// Each output plane must contain exactly order+1 elements. All outputs are
/// retained on failure, including unrepresentable ordinary correlations.
///
/// # Safety
/// Nonempty input must be initialized, stable, readable storage in one allocation.
/// Outputs must be aligned, exclusively writable and pairwise disjoint. Input/output
/// overlap is supported after input borrowing ends and all results are staged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_from_samples_f32(
    input: *const f32,
    input_count: usize,
    order: usize,
    arithmetic: u32,
    coefficients: *mut f32,
    correlations: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized input and distinct writable plane contracts.
    unsafe {
        prediction_estimate::ordinary(
            input,
            input_count,
            order,
            Domain::Samples,
            arithmetic,
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Estimate from magnitude, returning both coefficients and autocorrelation.
/// Arithmetic zero Stable, one Legacy. Nyquist zero Preserve, one Legacy.
/// Each output plane must contain exactly order+1 elements. All outputs are
/// retained on failure, including unrepresentable ordinary correlations.
///
/// # Safety
/// Nonempty input must be initialized, stable, readable storage in one allocation.
/// Outputs must be aligned, exclusively writable and pairwise disjoint. Input/output
/// overlap is supported after input borrowing ends and all results are staged.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit independent C policy selectors and output planes.
pub unsafe extern "C" fn ciglet_rs_prediction_from_magnitude_f32(
    input: *const f32,
    input_count: usize,
    order: usize,
    nyquist: u32,
    arithmetic: u32,
    coefficients: *mut f32,
    correlations: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized input and distinct writable plane contracts.
    unsafe {
        prediction_estimate::ordinary(
            input,
            input_count,
            order,
            Domain::Magnitude { nyquist },
            arithmetic,
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Estimate scaled correlations from magnitudes without requiring ordinary conversion.
/// Nyquist zero Preserve, one Legacy. Native source transform sizing is retained.
///
/// # Safety
/// Nonempty input must be initialized stable readable single-allocation storage.
/// Output is an independent initialized aligned exclusively writable owner slot.
/// Retain and release ownership of any older slot value before successful replacement;
/// replacement does not release that value. Failures preserve the output slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_from_magnitude_f32(
    input: *const f32,
    input_count: usize,
    order: usize,
    nyquist: u32,
    output: *mut *mut CigletRsScaledPredictionF32,
) -> u32 {
    // SAFETY: Forward initialized input and independent ownership slot contracts.
    unsafe { prediction_estimate::scaled_magnitude(input, input_count, order, nyquist, output) }
}

/// Estimate from samples, returning both coefficients and autocorrelation.
/// Arithmetic zero Stable, one Legacy. Sample order must be smaller than input count.
/// Each output plane must contain exactly order+1 elements. All outputs are
/// retained on failure, including unrepresentable ordinary correlations.
///
/// # Safety
/// Nonempty input must be initialized, stable, readable storage in one allocation.
/// Outputs must be aligned, exclusively writable and pairwise disjoint. Input/output
/// overlap is supported after input borrowing ends and all results are staged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_from_samples_f64(
    input: *const f64,
    input_count: usize,
    order: usize,
    arithmetic: u32,
    coefficients: *mut f64,
    correlations: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized input and distinct writable plane contracts.
    unsafe {
        prediction_estimate::ordinary(
            input,
            input_count,
            order,
            Domain::Samples,
            arithmetic,
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Estimate from magnitude, returning both coefficients and autocorrelation.
/// Arithmetic zero Stable, one Legacy. Nyquist zero Preserve, one Legacy.
/// Each output plane must contain exactly order+1 elements. All outputs are
/// retained on failure, including unrepresentable ordinary correlations.
///
/// # Safety
/// Nonempty input must be initialized, stable, readable storage in one allocation.
/// Outputs must be aligned, exclusively writable and pairwise disjoint. Input/output
/// overlap is supported after input borrowing ends and all results are staged.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit independent C policy selectors and output planes.
pub unsafe extern "C" fn ciglet_rs_prediction_from_magnitude_f64(
    input: *const f64,
    input_count: usize,
    order: usize,
    nyquist: u32,
    arithmetic: u32,
    coefficients: *mut f64,
    correlations: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized input and distinct writable plane contracts.
    unsafe {
        prediction_estimate::ordinary(
            input,
            input_count,
            order,
            Domain::Magnitude { nyquist },
            arithmetic,
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Estimate scaled correlations from magnitudes without requiring ordinary conversion.
/// Nyquist zero Preserve, one Legacy. Native source transform sizing is retained.
///
/// # Safety
/// Nonempty input must be initialized stable readable single-allocation storage.
/// Output is an independent initialized aligned exclusively writable owner slot.
/// Retain and release ownership of any older slot value before successful replacement;
/// replacement does not release that value. Failures preserve the output slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_from_magnitude_f64(
    input: *const f64,
    input_count: usize,
    order: usize,
    nyquist: u32,
    output: *mut *mut CigletRsScaledPredictionF64,
) -> u32 {
    // SAFETY: Forward initialized input and independent ownership slot contracts.
    unsafe { prediction_estimate::scaled_magnitude(input, input_count, order, nyquist, output) }
}
