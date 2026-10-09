//! Checked two-output quadratic spectral peak interpolation.
use super::{boundary, disjoint_outputs, range};
use crate::{spectral_peak, statistics::Scalar};
use std::panic::RefUnwindSafe;

unsafe fn interpolate<T: Scalar + RefUnwindSafe>(
    magnitudes: *const T,
    count: usize,
    bin: usize,
    output_bin: *mut T,
    output_magnitude: *mut T,
) -> u32 {
    if let Err(status) = range(magnitudes, count).and_then(|()| {
        disjoint_outputs(output_bin.cast_const(), 1, output_magnitude.cast_const(), 1)
    }) {
        return status;
    }
    let result = boundary(|| {
        let values = if count == 0 {
            &[]
        } else {
            // SAFETY: Caller retains the checked initialized readable allocation.
            unsafe { std::slice::from_raw_parts(magnitudes, count) }
        };
        spectral_peak::interpolate(values, bin)
    });
    match result {
        Ok(peak) => {
            // SAFETY: Both checked exclusive scalar slots are disjoint. Input
            // borrowing ended; committing primitive values cannot fail or allocate.
            unsafe {
                output_bin.write(peak.bin);
                output_magnitude.write(peak.magnitude);
            }
            0
        }
        Err(status) => status,
    }
}

/// Interpolate a three-bin peak, returning fractional bin and capped magnitude.
/// Only participating samples are validated. Both outputs are retained on error.
/// Missing neighbors or native domain errors return three; invalid pointers return
/// one; invalid ranges or overlapping output slots return two.
///
/// # Safety
/// Nonempty magnitudes point to aligned initialized stable readable storage for
/// count elements. Both output slots are aligned, disjoint and exclusively writable
/// for their scalar type. Outputs may overlap input after computation ends. Null
/// input is accepted only for zero count. Allocation validity remains the caller's
/// responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_spectral_peak_interpolate_f32(
    magnitudes: *const f32,
    count: usize,
    bin: usize,
    output_bin: *mut f32,
    output_magnitude: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and exclusivity contracts.
    unsafe { interpolate(magnitudes, count, bin, output_bin, output_magnitude) }
}

/// Interpolate a three-bin peak, returning fractional bin and capped magnitude.
/// Only participating samples are validated. Both outputs are retained on error.
/// Missing neighbors or native domain errors return three; invalid pointers return
/// one; invalid ranges or overlapping output slots return two.
///
/// # Safety
/// Nonempty magnitudes point to aligned initialized stable readable storage for
/// count elements. Both output slots are aligned, disjoint and exclusively writable
/// for their scalar type. Outputs may overlap input after computation ends. Null
/// input is accepted only for zero count. Allocation validity remains the caller's
/// responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_spectral_peak_interpolate_f64(
    magnitudes: *const f64,
    count: usize,
    bin: usize,
    output_bin: *mut f64,
    output_magnitude: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and exclusivity contracts.
    unsafe { interpolate(magnitudes, count, bin, output_bin, output_magnitude) }
}
