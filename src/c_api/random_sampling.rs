//! Scalar and buffer sampling from synchronized caller-owned generators.
use super::{
    generated_result,
    random_owner::{CigletRsRandom, policy, sample},
    range,
};
use crate::Error;
/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_uniform_f32(
    handle: *const CigletRsRandom,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe { sample(handle, output, |state| state.uniform()) }
}

/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_f32(
    handle: *const CigletRsRandom,
    mean: f32,
    variance: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe { sample(handle, output, |state| state.normal(mean, variance)) }
}

/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_with_f32(
    handle: *const CigletRsRandom,
    mean: f32,
    variance: f32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe {
        sample(handle, output, |state| {
            state.normal_with(mean, variance, policy(mode)?)
        })
    }
}

/// Sample a complete white-noise buffer, committing output only on success.
/// Zero count permits null output and consumes no draws. Invalid pointer/count or
/// amplitude consumes none; a later numerical error may advance generator state.
///
/// # Safety
/// Handle is live and retained. Nonempty output is aligned exclusively writable
/// single-allocation storage for count scalars, independent of the owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_noise_f32(
    handle: *const CigletRsRandom,
    amplitude: f32,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner is locked during native sampling; output is staged.
    unsafe {
        generated_result(output, count, || {
            (&*handle)
                .generator()
                .lock()
                .map_err(|_| Error("random state mutex is poisoned"))?
                .white_noise(amplitude, count)
        })
    }
}

/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_uniform_f64(
    handle: *const CigletRsRandom,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe { sample(handle, output, |state| state.uniform()) }
}

/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_f64(
    handle: *const CigletRsRandom,
    mean: f64,
    variance: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe { sample(handle, output, |state| state.normal(mean, variance)) }
}

/// Sample from the owned generator using native validation and draw consumption.
/// Normal uses Stable arithmetic; normal_with selects zero Stable or one Legacy.
/// Invalid pointers/parameters consume no draws; numerical failures may advance state.
///
/// # Safety
/// Handle is live and retained; output is aligned independently exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_normal_with_f64(
    handle: *const CigletRsRandom,
    mean: f64,
    variance: f64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward live synchronized owner and exclusive output contracts.
    unsafe {
        sample(handle, output, |state| {
            state.normal_with(mean, variance, policy(mode)?)
        })
    }
}

/// Sample a complete white-noise buffer, committing output only on success.
/// Zero count permits null output and consumes no draws. Invalid pointer/count or
/// amplitude consumes none; a later numerical error may advance generator state.
///
/// # Safety
/// Handle is live and retained. Nonempty output is aligned exclusively writable
/// single-allocation storage for count scalars, independent of the owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_random_white_noise_f64(
    handle: *const CigletRsRandom,
    amplitude: f64,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner is locked during native sampling; output is staged.
    unsafe {
        generated_result(output, count, || {
            (&*handle)
                .generator()
                .lock()
                .map_err(|_| Error("random state mutex is poisoned"))?
                .white_noise(amplitude, count)
        })
    }
}
