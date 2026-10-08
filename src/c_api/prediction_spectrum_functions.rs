//! Typed reusable prediction spectrum plans and complete outputs.
use super::{
    fourier,
    prediction_estimate::Output,
    prediction_scaled::{CigletRsScaledPredictionF32, CigletRsScaledPredictionF64},
    prediction_spectrum::{
        self, CigletRsPredictionSpectrumF32, CigletRsPredictionSpectrumF64, Estimate,
    },
};

/// Create a reusable native prediction FFT plan; failures retain output ownership.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_new_f32(
    length: usize,
    output: *mut *mut CigletRsPredictionSpectrumF32,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { prediction_spectrum::new(length, output) }
}

/// Release a plan and clear its slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized, aligned and exclusively writable. Release exactly once
/// a typed owner from this API, excluding every outstanding call and alias.
/// A released owner must never be reused.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_release_f32(
    slot: *mut *mut CigletRsPredictionSpectrumF32,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy transform length and maximum half-spectrum bin count into two usize values.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_shape_f32(
    handle: *const CigletRsPredictionSpectrumF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { prediction_spectrum::shape(handle, output, count) }
}

/// Return coefficients and correlations; Nyquist zero Preserve/one Legacy, arithmetic zero Stable/one Legacy.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit C policies and paired output planes.
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_estimate_f32(
    handle: *const CigletRsPredictionSpectrumF32,
    input: *const f32,
    input_count: usize,
    order: usize,
    nyquist: u32,
    arithmetic: u32,
    coefficients: *mut f32,
    correlations: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::estimate(
            handle,
            input,
            input_count,
            Estimate {
                order,
                nyquist,
                arithmetic,
            },
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Publish a scaled estimate without requiring ordinary conversion; Nyquist zero Preserve/one Legacy.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_estimate_scaled_f32(
    handle: *const CigletRsPredictionSpectrumF32,
    input: *const f32,
    input_count: usize,
    order: usize,
    nyquist: u32,
    output: *mut *mut CigletRsScaledPredictionF32,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::estimate_scaled(handle, input, input_count, order, nyquist, output)
    }
}

/// Evaluate the full native FFT-grid magnitude response, retaining signed gain.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_spectrum_f32(
    handle: *const CigletRsPredictionSpectrumF32,
    coefficients: *const f32,
    coefficient_count: usize,
    gain: f32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::spectrum(
            handle,
            coefficients,
            coefficient_count,
            gain,
            output,
            output_count,
        )
    }
}

/// Create a reusable native prediction FFT plan; failures retain output ownership.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_new_f64(
    length: usize,
    output: *mut *mut CigletRsPredictionSpectrumF64,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { prediction_spectrum::new(length, output) }
}

/// Release a plan and clear its slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized, aligned and exclusively writable. Release exactly once
/// a typed owner from this API, excluding every outstanding call and alias.
/// A released owner must never be reused.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_release_f64(
    slot: *mut *mut CigletRsPredictionSpectrumF64,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy transform length and maximum half-spectrum bin count into two usize values.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_shape_f64(
    handle: *const CigletRsPredictionSpectrumF64,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe { prediction_spectrum::shape(handle, output, count) }
}

/// Return coefficients and correlations; Nyquist zero Preserve/one Legacy, arithmetic zero Stable/one Legacy.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit C policies and paired output planes.
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_estimate_f64(
    handle: *const CigletRsPredictionSpectrumF64,
    input: *const f64,
    input_count: usize,
    order: usize,
    nyquist: u32,
    arithmetic: u32,
    coefficients: *mut f64,
    correlations: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::estimate(
            handle,
            input,
            input_count,
            Estimate {
                order,
                nyquist,
                arithmetic,
            },
            Output {
                coefficients,
                correlations,
                count: output_count,
            },
        )
    }
}

/// Publish a scaled estimate without requiring ordinary conversion; Nyquist zero Preserve/one Legacy.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_estimate_scaled_f64(
    handle: *const CigletRsPredictionSpectrumF64,
    input: *const f64,
    input_count: usize,
    order: usize,
    nyquist: u32,
    output: *mut *mut CigletRsScaledPredictionF64,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::estimate_scaled(handle, input, input_count, order, nyquist, output)
    }
}

/// Evaluate the full native FFT-grid magnitude response, retaining signed gain.
///
/// # Safety
/// Retain typed live owners throughout the call, excluding release. Nonempty
/// inputs are initialized, stable, readable single-allocation storage. Outputs
/// are aligned, exclusively writable, pairwise disjoint and independent of
/// owner allocations. Complete staged results permit input/output overlap.
/// Keep and release older ownership before successful slot replacement; no
/// implicit release occurs. Failures preserve all outputs. Mutable scratch
/// is internally serialized, permitting concurrent calls excluding release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_prediction_spectrum_spectrum_f64(
    handle: *const CigletRsPredictionSpectrumF64,
    coefficients: *const f64,
    coefficient_count: usize,
    gain: f64,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained typed owner and validated caller buffer contracts.
    unsafe {
        prediction_spectrum::spectrum(
            handle,
            coefficients,
            coefficient_count,
            gain,
            output,
            output_count,
        )
    }
}
