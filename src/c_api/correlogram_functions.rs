//! Row-major frame correlograms and period-to-frequency conversion.
use super::{generated_result, range};
use crate::{
    Error,
    array::Array2,
    correlogram::{self, Frame, Method, Policy},
    statistics::Scalar,
};
use std::panic::RefUnwindSafe;
/// Autocorrelation correlogram method code.
pub const CIGLET_RS_CORRELOGRAM_AUTOCORRELATION: i32 = 0;
/// Absolute-difference correlogram method code.
pub const CIGLET_RS_CORRELOGRAM_ABSOLUTE_DIFFERENCE: i32 = 1;
/// Squared-difference correlogram method code.
pub const CIGLET_RS_CORRELOGRAM_SQUARED_DIFFERENCE: i32 = 2;
/// YIN correlogram method code.
pub const CIGLET_RS_CORRELOGRAM_YIN: i32 = 3;

unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains initialized stable checked readable allocation.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
#[allow(clippy::too_many_arguments)]
unsafe fn compute<T: Scalar + RefUnwindSafe>(
    samples: *const T,
    sample_count: usize,
    centers: *const i32,
    lengths: *const usize,
    frame_count: usize,
    periods: usize,
    method: i32,
    policy: Option<u32>,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if frame_count.checked_mul(periods) != Some(output_count) {
        return 2;
    }
    if let Err(status) = range(samples, sample_count)
        .and_then(|()| range(centers, frame_count))
        .and_then(|()| range(lengths, frame_count))
    {
        return status;
    }
    // SAFETY: Checked readable inputs remain stable; all input borrowing ends
    // before complete row-major result commit, permitting caller buffer overlap.
    unsafe {
        generated_result(output, output_count, || {
            let method = Method::try_from(method)?;
            let policy = match policy {
                None => None,
                Some(0) => Some(Policy::Stable),
                Some(1) => Some(Policy::Legacy),
                _ => return Err(Error("unknown correlogram policy")),
            };
            let mut frames = Vec::new();
            frames
                .try_reserve_exact(frame_count)
                .map_err(|_| Error("correlogram frame allocation failed"))?;
            frames.extend(
                input(centers, frame_count)
                    .iter()
                    .zip(input(lengths, frame_count))
                    .map(|(&center, &length)| Frame { center, length }),
            );
            let values = match policy {
                None => {
                    correlogram::compute(input(samples, sample_count), &frames, periods, method)
                }
                Some(policy) => correlogram::compute_with(
                    input(samples, sample_count),
                    &frames,
                    periods,
                    method,
                    policy,
                ),
            }?;
            Ok(values.into_row_major())
        })
    }
}
#[allow(clippy::too_many_arguments)]
unsafe fn frequency<T: Scalar + RefUnwindSafe>(
    values: *const T,
    frames: usize,
    periods: usize,
    input_count: usize,
    sample_rate: i32,
    frequencies: *const T,
    frequency_count: usize,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if frames.checked_mul(periods) != Some(input_count)
        || frames.checked_mul(frequency_count) != Some(output_count)
    {
        return 2;
    }
    if let Err(status) =
        range(values, input_count).and_then(|()| range(frequencies, frequency_count))
    {
        return status;
    }
    // SAFETY: Initialized checked readable inputs remain stable; owned source copy
    // and complete native result staging end all borrows before exclusive commit.
    unsafe {
        generated_result(output, output_count, || {
            let values = Array2::from_slice(frames, periods, input(values, input_count))?;
            Ok(correlogram::to_frequency(
                &values,
                sample_rate,
                input(frequencies, frequency_count),
            )?
            .into_row_major())
        })
    }
}

/// Frame correlogram with native Stable policy.
/// Methods: zero autocorrelation, one absolute difference, two squared difference,
/// three YIN. Centers are signed sample indices; lengths are positive window sizes.
/// Output is frame_count by periods in row-major order. Native strict trailing
/// boundaries and unavailable-lag values are retained. Complete staging preserves
/// output on error and permits input/output overlap. Unknown selectors return three.
/// Native validation, allocation and panic limitations apply.
///
/// # Safety
/// Samples are aligned initialized readable storage for sample_count scalars;
/// centers/lengths are aligned initialized readable arrays for frame_count entries.
/// Output is aligned exclusive writable single-allocation storage for output_count
/// scalars. All inputs remain stable during calculation. Null buffers are accepted
/// only for zero associated counts. Input buffers may overlap one another.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_compute_f32(
    samples: *const f32,
    sample_count: usize,
    centers: *const i32,
    lengths: *const usize,
    frame_count: usize,
    periods: usize,
    method: i32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and complete staged exclusive output.
    unsafe {
        compute(
            samples,
            sample_count,
            centers,
            lengths,
            frame_count,
            periods,
            method,
            None,
            output,
            output_count,
        )
    }
}

/// Frame correlogram with explicit policy zero Stable, one Legacy.
/// Methods: zero autocorrelation, one absolute difference, two squared difference,
/// three YIN. Centers are signed sample indices; lengths are positive window sizes.
/// Output is frame_count by periods in row-major order. Native strict trailing
/// boundaries and unavailable-lag values are retained. Complete staging preserves
/// output on error and permits input/output overlap. Unknown selectors return three.
/// Native validation, allocation and panic limitations apply.
///
/// # Safety
/// Samples are aligned initialized readable storage for sample_count scalars;
/// centers/lengths are aligned initialized readable arrays for frame_count entries.
/// Output is aligned exclusive writable single-allocation storage for output_count
/// scalars. All inputs remain stable during calculation. Null buffers are accepted
/// only for zero associated counts. Input buffers may overlap one another.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_compute_with_f32(
    samples: *const f32,
    sample_count: usize,
    centers: *const i32,
    lengths: *const usize,
    frame_count: usize,
    periods: usize,
    method: i32,
    policy: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and complete staged exclusive output.
    unsafe {
        compute(
            samples,
            sample_count,
            centers,
            lengths,
            frame_count,
            periods,
            method,
            Some(policy),
            output,
            output_count,
        )
    }
}

/// Map row-major period values to finite query frequencies, retaining query order.
/// Input is frames by periods; output is frames by frequency_count. Positive zero
/// queries map to the last lag; negative values/zero clamp to lag zero. Native
/// sample-rate validation and singleton/empty-axis handling are retained. Complete
/// source copying and staging preserve output on errors and permit input overlap.
/// Allocation and panic limitations remain unchanged.
///
/// # Safety
/// Values/frequencies are aligned initialized stable readable single-allocation
/// storage for input_count/frequency_count scalars. Output is aligned exclusively
/// writable storage for output_count scalars. Null buffers are accepted only for
/// zero associated counts. All declared dimensions must match their counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_to_frequency_f32(
    values: *const f32,
    frames: usize,
    periods: usize,
    input_count: usize,
    sample_rate: i32,
    frequencies: *const f32,
    frequency_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized checked readable inputs and staged output contract.
    unsafe {
        frequency(
            values,
            frames,
            periods,
            input_count,
            sample_rate,
            frequencies,
            frequency_count,
            output,
            output_count,
        )
    }
}

/// Frame correlogram with native Stable policy.
/// Methods: zero autocorrelation, one absolute difference, two squared difference,
/// three YIN. Centers are signed sample indices; lengths are positive window sizes.
/// Output is frame_count by periods in row-major order. Native strict trailing
/// boundaries and unavailable-lag values are retained. Complete staging preserves
/// output on error and permits input/output overlap. Unknown selectors return three.
/// Native validation, allocation and panic limitations apply.
///
/// # Safety
/// Samples are aligned initialized readable storage for sample_count scalars;
/// centers/lengths are aligned initialized readable arrays for frame_count entries.
/// Output is aligned exclusive writable single-allocation storage for output_count
/// scalars. All inputs remain stable during calculation. Null buffers are accepted
/// only for zero associated counts. Input buffers may overlap one another.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_compute_f64(
    samples: *const f64,
    sample_count: usize,
    centers: *const i32,
    lengths: *const usize,
    frame_count: usize,
    periods: usize,
    method: i32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and complete staged exclusive output.
    unsafe {
        compute(
            samples,
            sample_count,
            centers,
            lengths,
            frame_count,
            periods,
            method,
            None,
            output,
            output_count,
        )
    }
}

/// Frame correlogram with explicit policy zero Stable, one Legacy.
/// Methods: zero autocorrelation, one absolute difference, two squared difference,
/// three YIN. Centers are signed sample indices; lengths are positive window sizes.
/// Output is frame_count by periods in row-major order. Native strict trailing
/// boundaries and unavailable-lag values are retained. Complete staging preserves
/// output on error and permits input/output overlap. Unknown selectors return three.
/// Native validation, allocation and panic limitations apply.
///
/// # Safety
/// Samples are aligned initialized readable storage for sample_count scalars;
/// centers/lengths are aligned initialized readable arrays for frame_count entries.
/// Output is aligned exclusive writable single-allocation storage for output_count
/// scalars. All inputs remain stable during calculation. Null buffers are accepted
/// only for zero associated counts. Input buffers may overlap one another.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_compute_with_f64(
    samples: *const f64,
    sample_count: usize,
    centers: *const i32,
    lengths: *const usize,
    frame_count: usize,
    periods: usize,
    method: i32,
    policy: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked readable inputs and complete staged exclusive output.
    unsafe {
        compute(
            samples,
            sample_count,
            centers,
            lengths,
            frame_count,
            periods,
            method,
            Some(policy),
            output,
            output_count,
        )
    }
}

/// Map row-major period values to finite query frequencies, retaining query order.
/// Input is frames by periods; output is frames by frequency_count. Positive zero
/// queries map to the last lag; negative values/zero clamp to lag zero. Native
/// sample-rate validation and singleton/empty-axis handling are retained. Complete
/// source copying and staging preserve output on errors and permit input overlap.
/// Allocation and panic limitations remain unchanged.
///
/// # Safety
/// Values/frequencies are aligned initialized stable readable single-allocation
/// storage for input_count/frequency_count scalars. Output is aligned exclusively
/// writable storage for output_count scalars. Null buffers are accepted only for
/// zero associated counts. All declared dimensions must match their counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlogram_to_frequency_f64(
    values: *const f64,
    frames: usize,
    periods: usize,
    input_count: usize,
    sample_rate: i32,
    frequencies: *const f64,
    frequency_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward initialized checked readable inputs and staged output contract.
    unsafe {
        frequency(
            values,
            frames,
            periods,
            input_count,
            sample_rate,
            frequencies,
            frequency_count,
            output,
            output_count,
        )
    }
}
