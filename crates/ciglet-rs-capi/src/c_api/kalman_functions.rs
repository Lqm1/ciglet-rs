//! Staged scalar Kalman filtering and backward smoothing.
use super::{boundary, disjoint_outputs, generated_result, range};
use crate::{kalman, statistics::Scalar};
use std::panic::RefUnwindSafe;

unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains stable initialized checked readable allocation.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
#[allow(clippy::too_many_arguments)]
unsafe fn filter<T: Scalar + RefUnwindSafe>(
    measurements: *const T,
    process: *const T,
    noise: *const T,
    count: usize,
    initial: Option<T>,
    means: *mut T,
    variances: *mut T,
    output_count: usize,
    likelihood: *mut T,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(measurements, count)
        .and_then(|()| range(process, count))
        .and_then(|()| range(noise, count))
        .and_then(|()| disjoint_outputs(means.cast_const(), count, variances.cast_const(), count))
        .and_then(|()| disjoint_outputs(means.cast_const(), count, likelihood.cast_const(), 1))
        .and_then(|()| disjoint_outputs(variances.cast_const(), count, likelihood.cast_const(), 1))
    {
        return status;
    }
    let result = boundary(|| {
        // SAFETY: Checked caller inputs remain readable and stable during calculation.
        let (measurements, process, noise) = unsafe {
            (
                input(measurements, count),
                input(process, count),
                input(noise, count),
            )
        };
        match initial {
            Some(value) => kalman::filter_with_initial(measurements, process, noise, value),
            None => kalman::filter(measurements, process, noise),
        }
    });
    match result {
        Err(status) => status,
        Ok(value) => {
            if value.means.len() != count || value.variances.len() != count {
                return 3;
            }
            // SAFETY: All inputs borrows ended. Distinct exclusively writable output
            // ranges cannot overlap the separately allocated staged native estimate.
            unsafe {
                if count != 0 {
                    std::ptr::copy_nonoverlapping(value.means.as_ptr(), means, count);
                    std::ptr::copy_nonoverlapping(value.variances.as_ptr(), variances, count);
                }
                likelihood.write(value.log_likelihood);
            }
            0
        }
    }
}
unsafe fn smooth<T: Scalar + RefUnwindSafe>(
    means: *const T,
    posterior: *const T,
    process: *const T,
    count: usize,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(means, count)
        .and_then(|()| range(posterior, count))
        .and_then(|()| range(process, count))
    {
        return status;
    }
    // SAFETY: Inputs are checked readable storage. Full native output staging
    // ends all borrows before committing to the exclusive output, permitting overlap.
    unsafe {
        generated_result(output, count, || {
            kalman::smooth(
                input(means, count),
                input(posterior, count),
                input(process, count),
            )
        })
    }
}

/// Kalman filtering with the first measurement as initial mean.
/// Inputs share count; output_count must equal count. Native scalar likelihood
/// arithmetic is retained. Empty estimates produce zero likelihood. Complete
/// staging preserves all three outputs on failure and allows input/output overlap.
/// Output buffers and the likelihood slot must be pairwise disjoint.
/// Native validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// means/variances are exclusive writable allocations for output_count scalars.
/// Likelihood is an aligned exclusive writable scalar, including for empty input.
/// Null arrays are accepted only for zero counts; input arrays may overlap.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_filter_f32(
    measurements: *const f32,
    process: *const f32,
    noise: *const f32,
    count: usize,
    means: *mut f32,
    variances: *mut f32,
    output_count: usize,
    likelihood: *mut f32,
) -> u32 {
    // SAFETY: Forward initialized stable inputs and distinct exclusive outputs.
    unsafe {
        filter(
            measurements,
            process,
            noise,
            count,
            None,
            means,
            variances,
            output_count,
            likelihood,
        )
    }
}

/// Kalman filtering with an explicit initial mean.
/// Inputs share count; output_count must equal count. Native scalar likelihood
/// arithmetic is retained. Empty estimates produce zero likelihood. Complete
/// staging preserves all three outputs on failure and allows input/output overlap.
/// Output buffers and the likelihood slot must be pairwise disjoint.
/// Native validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// means/variances are exclusive writable allocations for output_count scalars.
/// Likelihood is an aligned exclusive writable scalar, including for empty input.
/// Null arrays are accepted only for zero counts; input arrays may overlap.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_filter_with_initial_f32(
    measurements: *const f32,
    process: *const f32,
    noise: *const f32,
    count: usize,
    initial: f32,
    means: *mut f32,
    variances: *mut f32,
    output_count: usize,
    likelihood: *mut f32,
) -> u32 {
    // SAFETY: Forward initialized stable inputs and distinct exclusive outputs.
    unsafe {
        filter(
            measurements,
            process,
            noise,
            count,
            Some(initial),
            means,
            variances,
            output_count,
            likelihood,
        )
    }
}

/// Smooth filtered means backward with native posterior and process variances.
/// Inputs share count; output_count equals count. Full staging preserves output
/// on failure and permits input/output overlap. Empty inputs return an empty result.
/// Native variance validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// output is exclusive writable single-allocation storage for output_count scalars.
/// Null buffers are accepted only for zero associated counts. Inputs may overlap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_smooth_f32(
    means: *const f32,
    posterior: *const f32,
    process: *const f32,
    count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and staged exclusive output contract.
    unsafe { smooth(means, posterior, process, count, output, output_count) }
}

/// Kalman filtering with the first measurement as initial mean.
/// Inputs share count; output_count must equal count. Native scalar likelihood
/// arithmetic is retained. Empty estimates produce zero likelihood. Complete
/// staging preserves all three outputs on failure and allows input/output overlap.
/// Output buffers and the likelihood slot must be pairwise disjoint.
/// Native validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// means/variances are exclusive writable allocations for output_count scalars.
/// Likelihood is an aligned exclusive writable scalar, including for empty input.
/// Null arrays are accepted only for zero counts; input arrays may overlap.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_filter_f64(
    measurements: *const f64,
    process: *const f64,
    noise: *const f64,
    count: usize,
    means: *mut f64,
    variances: *mut f64,
    output_count: usize,
    likelihood: *mut f64,
) -> u32 {
    // SAFETY: Forward initialized stable inputs and distinct exclusive outputs.
    unsafe {
        filter(
            measurements,
            process,
            noise,
            count,
            None,
            means,
            variances,
            output_count,
            likelihood,
        )
    }
}

/// Kalman filtering with an explicit initial mean.
/// Inputs share count; output_count must equal count. Native scalar likelihood
/// arithmetic is retained. Empty estimates produce zero likelihood. Complete
/// staging preserves all three outputs on failure and allows input/output overlap.
/// Output buffers and the likelihood slot must be pairwise disjoint.
/// Native validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// means/variances are exclusive writable allocations for output_count scalars.
/// Likelihood is an aligned exclusive writable scalar, including for empty input.
/// Null arrays are accepted only for zero counts; input arrays may overlap.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_filter_with_initial_f64(
    measurements: *const f64,
    process: *const f64,
    noise: *const f64,
    count: usize,
    initial: f64,
    means: *mut f64,
    variances: *mut f64,
    output_count: usize,
    likelihood: *mut f64,
) -> u32 {
    // SAFETY: Forward initialized stable inputs and distinct exclusive outputs.
    unsafe {
        filter(
            measurements,
            process,
            noise,
            count,
            Some(initial),
            means,
            variances,
            output_count,
            likelihood,
        )
    }
}

/// Smooth filtered means backward with native posterior and process variances.
/// Inputs share count; output_count equals count. Full staging preserves output
/// on failure and permits input/output overlap. Empty inputs return an empty result.
/// Native variance validation, allocation and panic limitations remain unchanged.
///
/// # Safety
/// Inputs are aligned initialized stable readable allocations for count scalars;
/// output is exclusive writable single-allocation storage for output_count scalars.
/// Null buffers are accepted only for zero associated counts. Inputs may overlap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_kalman_smooth_f64(
    means: *const f64,
    posterior: *const f64,
    process: *const f64,
    count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and staged exclusive output contract.
    unsafe { smooth(means, posterior, process, count, output, output_count) }
}
