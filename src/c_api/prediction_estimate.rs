//! Complete paired prediction outputs and magnitude-domain scaled owners.
use super::{boundary, disjoint_outputs, prediction::arithmetic, prediction_scaled, range};
use crate::{prediction, statistics::Scalar};
use std::panic::RefUnwindSafe;

#[derive(Clone, Copy)]
pub(super) struct Output<T> {
    pub coefficients: *mut T,
    pub correlations: *mut T,
    pub count: usize,
}
#[derive(Clone, Copy)]
pub(super) enum Domain {
    Samples,
    Magnitude { nyquist: u32 },
}
pub(super) fn nyquist(code: u32) -> Result<prediction::NyquistPolicy, u32> {
    match code {
        0 => Ok(prediction::NyquistPolicy::Preserve),
        1 => Ok(prediction::NyquistPolicy::Legacy),
        _ => Err(3),
    }
}
pub(super) fn outputs<T>(output: Output<T>, order: usize) -> Result<(), u32> {
    if order.checked_add(1) != Some(output.count) {
        return Err(2);
    }
    disjoint_outputs(
        output.coefficients.cast_const(),
        output.count,
        output.correlations.cast_const(),
        output.count,
    )
}
pub(super) unsafe fn commit<T: Copy>(
    output: Output<T>,
    result: Result<prediction::Prediction<T>, u32>,
) -> u32 {
    match result {
        Ok(value) => {
            if value.coefficients.len() != output.count
                || value.autocorrelation.len() != output.count
            {
                return 3;
            }
            // SAFETY: Caller ranges were checked, pairwise disjoint, exclusively
            // writable and independent of owner allocations. All input borrowing
            // ended before both complete staged arrays are committed.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    value.coefficients.as_ptr(),
                    output.coefficients,
                    output.count,
                );
                std::ptr::copy_nonoverlapping(
                    value.autocorrelation.as_ptr(),
                    output.correlations,
                    output.count,
                );
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn ordinary<T: Scalar + RefUnwindSafe>(
    input: *const T,
    count: usize,
    order: usize,
    domain: Domain,
    policy: u32,
    output: Output<T>,
) -> u32 {
    if let Err(status) = range(input, count).and_then(|()| outputs(output, order)) {
        return status;
    }
    let policy = match arithmetic(policy) {
        Ok(value) => value,
        Err(status) => return status,
    };
    let nyquist = match domain {
        Domain::Samples => prediction::NyquistPolicy::Preserve,
        Domain::Magnitude { nyquist: code } => match nyquist(code) {
            Ok(value) => value,
            Err(status) => return status,
        },
    };
    let result = boundary(|| {
        let values = if count == 0 {
            &[]
        } else {
            // SAFETY: Checked initialized stable readable single-allocation input.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        match domain {
            Domain::Samples => prediction::from_samples_with(values, order, policy),
            Domain::Magnitude { .. } => {
                prediction::from_magnitude_with_arithmetic(values, order, nyquist, policy)
            }
        }
    });
    // SAFETY: Checked distinct writable outputs; every input borrow ended above.
    unsafe { commit(output, result) }
}
pub(super) unsafe fn scaled_magnitude<H: prediction_scaled::Handle>(
    input: *const H::Value,
    count: usize,
    order: usize,
    policy: u32,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(input, count) {
        return status;
    }
    let policy = match nyquist(policy) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Caller supplies initialized stable input and independent aligned
    // ownership slot. Publishing does not require ordinary conversion or gain.
    unsafe {
        prediction_scaled::publish(output, || {
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            prediction::from_magnitude_scaled(values, order, policy)
        })
    }
}
