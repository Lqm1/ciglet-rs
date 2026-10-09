//! Scalar source expressions and synchronous caller-defined left folds.
use super::{scalar_slice_result, scalar_value_result};
use std::ffi::c_void;

/// Original binary64 constants, including a finite infinity sentinel.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsMathConstants {
    /// Original M_EPS, distinct from machine epsilon.
    pub epsilon: f64,
    /// Original M_INF, distinct from IEEE infinity.
    pub infinity_sentinel: f64,
    /// Original M_PI.
    pub pi: f64,
    /// Original M_PI_2.
    pub half_pi: f64,
}

/// Read the four original source constants without scalar precision conversion.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_math_constants(output: *mut CigletRsMathConstants) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe {
        scalar_value_result(output, || {
            Ok(CigletRsMathConstants {
                epsilon: crate::scalar::LEGACY_EPSILON,
                infinity_sentinel: crate::scalar::LEGACY_INFINITY_SENTINEL,
                pi: std::f64::consts::PI,
                half_pi: std::f64::consts::FRAC_PI_2,
            })
        })
    }
}

/// Add two scalars in the selected precision.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_add_f32(
    first: f32,
    second: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(first + second)) }
}

/// Add the square of the second scalar, without fused multiply-add.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_accumulate_square_f32(
    first: f32,
    second: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(first + second * second)) }
}

/// Return the smaller scalar, retaining the second operand on ties or unordered comparisons.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_minimum_f32(
    first: f32,
    second: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(crate::scalar::minimum(first, second))) }
}

/// Return the larger scalar, retaining the second operand on ties or unordered comparisons.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_maximum_f32(
    first: f32,
    second: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(crate::scalar::maximum(first, second))) }
}

/// Apply the callback from left to right, starting with the caller's initial value.
/// The callback receives accumulator, next input and context, and is never retained.
/// Empty input returns initial unchanged, permits null input and a null callback,
/// and performs no callbacks. Nonempty input with null callback returns status one.
/// Invalid ranges/output are rejected before any callback. Ordinary IEEE callback
/// results are preserved. Output is committed after input borrowing ends.
///
/// # Safety
/// Nonempty input must be aligned initialized single-allocation storage for count
/// scalars, stable throughout the call. Output must be aligned writable storage for
/// one scalar and may overlap input only because the write occurs after the fold.
/// Callback must have the declared ABI and not unwind, throw or panic. Context must
/// satisfy callback validity and lifetime requirements. Callback may mutate its own
/// context but must not modify input or output, or create conflicting references.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_fold_f32(
    input: *const f32,
    count: usize,
    initial: f32,
    callback: Option<unsafe extern "C" fn(f32, f32, *mut c_void) -> f32>,
    context: *mut c_void,
    output: *mut f32,
) -> u32 {
    if count != 0 && callback.is_none() {
        return 1;
    }
    // SAFETY: Forward stable input, exclusive eventual output and synchronous
    // non-unwinding callback/context contracts; no pointers are retained.
    unsafe {
        scalar_slice_result(input, count, output, |values| {
            Ok(match callback {
                Some(operator) => values.iter().fold(initial, |accumulator, &value| {
                    operator(accumulator, value, context)
                }),
                None => initial,
            })
        })
    }
}

/// Add two scalars in the selected precision.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_add_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(first + second)) }
}

/// Add the square of the second scalar, without fused multiply-add.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_accumulate_square_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(first + second * second)) }
}

/// Return the smaller scalar, retaining the second operand on ties or unordered comparisons.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_minimum_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(crate::scalar::minimum(first, second))) }
}

/// Return the larger scalar, retaining the second operand on ties or unordered comparisons.
/// Ordinary IEEE values, including NaNs and infinities, are preserved.
///
/// # Safety
/// Output must be aligned, exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_maximum_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's exclusive output contract.
    unsafe { scalar_value_result(output, || Ok(crate::scalar::maximum(first, second))) }
}

/// Apply the callback from left to right, starting with the caller's initial value.
/// The callback receives accumulator, next input and context, and is never retained.
/// Empty input returns initial unchanged, permits null input and a null callback,
/// and performs no callbacks. Nonempty input with null callback returns status one.
/// Invalid ranges/output are rejected before any callback. Ordinary IEEE callback
/// results are preserved. Output is committed after input borrowing ends.
///
/// # Safety
/// Nonempty input must be aligned initialized single-allocation storage for count
/// scalars, stable throughout the call. Output must be aligned writable storage for
/// one scalar and may overlap input only because the write occurs after the fold.
/// Callback must have the declared ABI and not unwind, throw or panic. Context must
/// satisfy callback validity and lifetime requirements. Callback may mutate its own
/// context but must not modify input or output, or create conflicting references.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scalar_fold_f64(
    input: *const f64,
    count: usize,
    initial: f64,
    callback: Option<unsafe extern "C" fn(f64, f64, *mut c_void) -> f64>,
    context: *mut c_void,
    output: *mut f64,
) -> u32 {
    if count != 0 && callback.is_none() {
        return 1;
    }
    // SAFETY: Forward stable input, exclusive eventual output and synchronous
    // non-unwinding callback/context contracts; no pointers are retained.
    unsafe {
        scalar_slice_result(input, count, output, |values| {
            Ok(match callback {
                Some(operator) => values.iter().fold(initial, |accumulator, &value| {
                    operator(accumulator, value, context)
                }),
                None => initial,
            })
        })
    }
}
