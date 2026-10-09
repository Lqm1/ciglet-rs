//! Original scalar-precision arithmetic from caller-provided integer draws.
use super::{generated_result, random_owner::policy, range, scalar_value_result};
use crate::random;
/// Transform one original inclusive C integer draw.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_uniform_from_draw_f32(
    draw: u32,
    maximum: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe { scalar_value_result(output, || random::uniform_from_draw(draw, maximum)) }
}

/// Transform one original white-noise draw, retaining negative or zero amplitude.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_from_draw_f32(
    amplitude: f32,
    draw: u32,
    maximum: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe { scalar_value_result(output, || random::white_from_draw(amplitude, draw, maximum)) }
}

/// Transform two original Gaussian draws; variance is not standard deviation.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_from_draws_f32(
    mean: f32,
    variance: f32,
    first: u32,
    second: u32,
    maximum: u32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe {
        scalar_value_result(output, || {
            random::normal_from_draws(mean, variance, [first, second], maximum, policy(mode)?)
        })
    }
}

/// Transform a complete array of caller-provided inclusive integer draws.
/// Output may overlap input; commits only after the borrowed input ends.
///
/// # Safety
/// Nonempty draws are aligned initialized stable single-allocation storage for
/// count u32 values. Output is independently exclusively writable after computation
/// for count scalars, and may overlap draws. Empty buffers may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_from_draws_f32(
    amplitude: f32,
    draws: *const u32,
    count: usize,
    maximum: u32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(draws, count) {
        return status;
    }
    // SAFETY: Stable checked input is borrowed only inside the computation.
    unsafe {
        generated_result(output, count, || {
            crate::vector::validate(&[amplitude])?;
            // Validate the grid even when the input is empty.
            random::uniform_from_draw::<f32>(0, maximum)?;
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(draws, count)
            };
            let mut result = crate::vector::buffer(count)?;
            for &draw in values {
                result.push(random::white_from_draw(amplitude, draw, maximum)?);
            }
            Ok(result)
        })
    }
}

/// Transform one original inclusive C integer draw.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_uniform_from_draw_f64(
    draw: u32,
    maximum: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe { scalar_value_result(output, || random::uniform_from_draw(draw, maximum)) }
}

/// Transform one original white-noise draw, retaining negative or zero amplitude.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_from_draw_f64(
    amplitude: f64,
    draw: u32,
    maximum: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe { scalar_value_result(output, || random::white_from_draw(amplitude, draw, maximum)) }
}

/// Transform two original Gaussian draws; variance is not standard deviation.
/// Errors retain output. Gaussian mode zero is Stable and one is Legacy.
///
/// # Safety
/// Output is aligned exclusively writable storage for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_from_draws_f64(
    mean: f64,
    variance: f64,
    first: u32,
    second: u32,
    maximum: u32,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward independent exclusive scalar output contract.
    unsafe {
        scalar_value_result(output, || {
            random::normal_from_draws(mean, variance, [first, second], maximum, policy(mode)?)
        })
    }
}

/// Transform a complete array of caller-provided inclusive integer draws.
/// Output may overlap input; commits only after the borrowed input ends.
///
/// # Safety
/// Nonempty draws are aligned initialized stable single-allocation storage for
/// count u32 values. Output is independently exclusively writable after computation
/// for count scalars, and may overlap draws. Empty buffers may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_from_draws_f64(
    amplitude: f64,
    draws: *const u32,
    count: usize,
    maximum: u32,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(draws, count) {
        return status;
    }
    // SAFETY: Stable checked input is borrowed only inside the computation.
    unsafe {
        generated_result(output, count, || {
            crate::vector::validate(&[amplitude])?;
            // Validate the grid even when the input is empty.
            random::uniform_from_draw::<f64>(0, maximum)?;
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(draws, count)
            };
            let mut result = crate::vector::buffer(count)?;
            for &draw in values {
                result.push(random::white_from_draw(amplitude, draw, maximum)?);
            }
            Ok(result)
        })
    }
}
