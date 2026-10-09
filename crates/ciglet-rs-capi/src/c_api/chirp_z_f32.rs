//! Typed chirp-Z entry points with caller-owned planar outputs.
use super::{CigletRsCztF32, chirp_z, fourier};

/// Transform on the grid k * angular_step, for any supported positive length.
/// Direction zero is forward; one negates the step and divides by length.
/// Inverse inverts forward only on orthogonal DFT grids; other grids retain
/// the native normalized inverse-direction transform.
/// Null input components mean zeros; null output components are discarded.
/// Present components contain length elements; outputs must be disjoint.
/// Either output may overlap input. Errors leave output unchanged.
///
/// # Safety
/// Present inputs must be initialized, stable single-allocation ranges. Present
/// outputs must be aligned and exclusively writable. Buffers and pointer slots
/// must be independent of private plan memory. Handles must be live, correctly
/// typed plans from this library. Concurrent transforms may share a plan but
/// require stable inputs and independent writable outputs. Release excludes
/// all outstanding calls, consumes ownership and invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_czt_f32(
    length: usize,
    angular_step: f32,
    direction: u32,
    real: *const f32,
    imaginary: *const f32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented caller buffer and owned-plan contracts.
    unsafe {
        chirp_z::one_shot(
            length,
            angular_step,
            direction,
            real,
            imaginary,
            output_real,
            output_imaginary,
        )
    }
}

/// Create a plan with fixed angular step and direction: zero forward, one inverse.
/// Output must be an independent initialized pointer slot; successful construction
/// transfers ownership and must not overwrite an unreleased handle. Failure retains
/// its previous value. Nonfinite step and unknown direction return status three.
/// # Safety
/// Present inputs must be initialized, stable single-allocation ranges. Present
/// outputs must be aligned and exclusively writable. Buffers and pointer slots
/// must be independent of private plan memory. Handles must be live, correctly
/// typed plans from this library. Concurrent transforms may share a plan but
/// require stable inputs and independent writable outputs. Release excludes
/// all outstanding calls, consumes ownership and invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_czt_plan_new_f32(
    length: usize,
    angular_step: f32,
    direction: u32,
    output: *mut *mut CigletRsCztF32,
) -> u32 {
    // SAFETY: Forward the documented caller buffer and owned-plan contracts.
    unsafe { chirp_z::new(length, angular_step, direction, output) }
}

/// Write the immutable plan length to one independent writable usize.
/// # Safety
/// Present inputs must be initialized, stable single-allocation ranges. Present
/// outputs must be aligned and exclusively writable. Buffers and pointer slots
/// must be independent of private plan memory. Handles must be live, correctly
/// typed plans from this library. Concurrent transforms may share a plan but
/// require stable inputs and independent writable outputs. Release excludes
/// all outstanding calls, consumes ownership and invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_czt_plan_length_f32(
    handle: *const CigletRsCztF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller buffer and owned-plan contracts.
    unsafe { chirp_z::length(handle, output) }
}

/// Transform with the plan's fixed grid/direction; scratch access is serialized.
/// Observed mutex poison returns status three, leaving output unchanged.
/// Null input components mean zeros; null output components are discarded.
/// Present components contain length elements; outputs must be disjoint.
/// Either output may overlap input. Errors leave output unchanged.
///
/// # Safety
/// Present inputs must be initialized, stable single-allocation ranges. Present
/// outputs must be aligned and exclusively writable. Buffers and pointer slots
/// must be independent of private plan memory. Handles must be live, correctly
/// typed plans from this library. Concurrent transforms may share a plan but
/// require stable inputs and independent writable outputs. Release excludes
/// all outstanding calls, consumes ownership and invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_czt_plan_transform_f32(
    handle: *const CigletRsCztF32,
    real: *const f32,
    imaginary: *const f32,
    length: usize,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented caller buffer and owned-plan contracts.
    unsafe {
        chirp_z::process(
            handle,
            real,
            imaginary,
            length,
            output_real,
            output_imaginary,
        )
    }
}

/// Release a live plan and clear its initialized pointer slot.
/// Releasing an already cleared slot succeeds. Ownership is consumed even if
/// destruction produces a caught unwind; no outstanding plan call is permitted.
/// # Safety
/// Present inputs must be initialized, stable single-allocation ranges. Present
/// outputs must be aligned and exclusively writable. Buffers and pointer slots
/// must be independent of private plan memory. Handles must be live, correctly
/// typed plans from this library. Concurrent transforms may share a plan but
/// require stable inputs and independent writable outputs. Release excludes
/// all outstanding calls, consumes ownership and invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_czt_plan_release_f32(slot: *mut *mut CigletRsCztF32) -> u32 {
    // SAFETY: Forward the documented caller buffer and owned-plan contracts.
    unsafe { fourier::release(slot) }
}
