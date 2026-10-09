//! Checked precision-selected core numerical operations.
use super::scalar_value_result;
use crate::{
    Error,
    approximation::{self, Precision},
};
/// Fast approximation mode.
pub const CIGLET_RS_APPROXIMATION_FAST: u32 = 0;
/// Source-compatible approximation mode.
pub const CIGLET_RS_APPROXIMATION_APPROXIMATE: u32 = 1;
/// Host libm mode, retaining native conversion and validation.
pub const CIGLET_RS_APPROXIMATION_EXACT: u32 = 2;
fn precision(code: u32) -> Result<Precision, Error> {
    match code {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(Error("unknown approximation precision selector")),
    }
}

/// Precision-selected sine: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64. Source full-range reduction is retained.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f32.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_sine_f32(
    value: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::sine_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected cosine: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64. Source full-range reduction is retained.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f32.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_cosine_f32(
    value: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::cosine_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected atan2: Fast=0, Approximate=1, Exact=2.
/// Both host atan2 modes preserve the identical source levels two/three.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f32.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_atan2_f32(
    y: f32,
    x: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::atan2_with(y, x, precision(mode)?)
        })
    }
}

/// Precision-selected logarithm: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f32.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_logarithm_f32(
    value: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::logarithm_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected exponential: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f32.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_exponential_f32(
    value: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::exponential_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected sine: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64. Source full-range reduction is retained.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f64.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_sine_f64(
    value: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::sine_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected cosine: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64. Source full-range reduction is retained.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f64.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_cosine_f64(
    value: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::cosine_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected atan2: Fast=0, Approximate=1, Exact=2.
/// Both host atan2 modes preserve the identical source levels two/three.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f64.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_atan2_f64(
    y: f64,
    x: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::atan2_with(y, x, precision(mode)?)
        })
    }
}

/// Precision-selected logarithm: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f64.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_logarithm_f64(
    value: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::logarithm_with(value, precision(mode)?)
        })
    }
}

/// Precision-selected exponential: Fast=0, Approximate=1, Exact=2.
/// Approximate modes retain source binary32 stages, including for binary64.
/// Native validation/domain/clamping applies; errors retain the result slot.
///
/// # Safety
/// Output is aligned, exclusively writable storage for one f64.
/// Scalar inputs are copied by value, permitting reuse of their original storage.
/// Null/misaligned outputs and unknown selectors are checked before writing.
/// Existing caught-panic and allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_approximation_exponential_f64(
    value: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Shared scalar staging checks the slot and writes only on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::exponential_with(value, precision(mode)?)
        })
    }
}
