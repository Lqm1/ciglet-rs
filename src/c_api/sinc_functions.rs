//! Typed aliased-sinc values and angular-frequency derivatives.
use super::scalar_value_result;
use crate::{
    Error,
    sinc::{self, Policy},
};
fn policy(code: u32) -> Result<Policy, Error> {
    match code {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(Error("unknown aliased-sinc policy")),
    }
}

/// Stable aliased sinc or its angular-frequency derivative.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_aliased_f32(
    length: f32,
    angle: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::aliased(length, angle)) }
}

/// Explicit policy zero Stable, one Legacy source denominator plateau.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_aliased_with_f32(
    length: f32,
    angle: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::aliased_with(length, angle, policy(mode)?)) }
}

/// Stable aliased sinc or its angular-frequency derivative.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_derivative_f32(
    length: f32,
    angle: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::derivative(length, angle)) }
}

/// Explicit policy zero Stable, one Legacy source denominator plateau.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_derivative_with_f32(
    length: f32,
    angle: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe {
        scalar_value_result(output, || {
            sinc::derivative_with(length, angle, policy(mode)?)
        })
    }
}

/// Stable aliased sinc or its angular-frequency derivative.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_aliased_f64(
    length: f64,
    angle: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::aliased(length, angle)) }
}

/// Explicit policy zero Stable, one Legacy source denominator plateau.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_aliased_with_f64(
    length: f64,
    angle: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::aliased_with(length, angle, policy(mode)?)) }
}

/// Stable aliased sinc or its angular-frequency derivative.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_derivative_f64(
    length: f64,
    angle: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe { scalar_value_result(output, || sinc::derivative(length, angle)) }
}

/// Explicit policy zero Stable, one Legacy source denominator plateau.
/// Selected scalar precision and native arithmetic are retained. Complete scalar
/// staging preserves output on failure. Nonfinite inputs and unknown policy codes
/// return three. Native allocation/abort and panic limitations still apply.
///
/// # Safety
/// Output is aligned exclusively writable single-allocation storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sinc_derivative_with_f64(
    length: f64,
    angle: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the exclusive scalar output contract and staged native result.
    unsafe {
        scalar_value_result(output, || {
            sinc::derivative_with(length, angle, policy(mode)?)
        })
    }
}
