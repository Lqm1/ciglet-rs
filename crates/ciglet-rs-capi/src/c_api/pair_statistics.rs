//! Two-input statistics with checked policies and transactional output.
use super::{pair_result, pair_scalar_result, range};

/// Compute dot with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_dot_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::dot,
        )
    }
}

/// Compute covariance with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_covariance_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::covariance,
        )
    }
}

/// Compute correlation with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlation_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::correlation,
        )
    }
}

/// Compute covariance with policy zero for Stable or one for Legacy.
/// Invalid policies return status three. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_covariance_with_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            |left, right| crate::statistics::covariance_with(left, right, policy),
        )
    }
}

/// Compute correlation with policy zero for Stable or one for Legacy.
/// Invalid policies return status three. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlation_with_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            |left, right| crate::statistics::correlation_with(left, right, policy),
        )
    }
}

/// Compute unnormalized nonnegative-lag cross correlation.
/// Input lengths must match; output length selects lag count. Excess lags
/// are zero-filled. Empty inputs or output accept null pointers in those ranges.
/// Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for lags values.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_cross_correlation_f32(
    left: *const f32,
    left_length: usize,
    right: *const f32,
    right_length: usize,
    output: *mut f32,
    lags: usize,
) -> u32 {
    if let Err(status) = range(output.cast_const(), lags) {
        return status;
    }
    // SAFETY: Forward the caller's documented input contracts.
    let result = unsafe {
        pair_result(left, left_length, right, right_length, |left, right| {
            crate::statistics::cross_correlation(left, right, lags)
        })
    };
    match result {
        Ok(values) => {
            if values.len() != lags {
                return 3;
            }
            if lags != 0 {
                // SAFETY: Input borrows ended. The owned result does not overlap
                // checked, caller-provided writable output storage.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, lags) };
            }
            0
        }
        Err(status) => status,
    }
}

/// Compute dot with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_dot_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::dot,
        )
    }
}

/// Compute covariance with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_covariance_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::covariance,
        )
    }
}

/// Compute correlation with the native default arithmetic policy.
/// Lengths must match. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlation_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            crate::statistics::correlation,
        )
    }
}

/// Compute covariance with policy zero for Stable or one for Legacy.
/// Invalid policies return status three. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_covariance_with_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            |left, right| crate::statistics::covariance_with(left, right, policy),
        )
    }
}

/// Compute correlation with policy zero for Stable or one for Legacy.
/// Invalid policies return status three. Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for one value.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_correlation_with_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the caller's documented buffer contracts.
    unsafe {
        pair_scalar_result(
            left,
            left_length,
            right,
            right_length,
            output,
            |left, right| crate::statistics::correlation_with(left, right, policy),
        )
    }
}

/// Compute unnormalized nonnegative-lag cross correlation.
/// Input lengths must match; output length selects lag count. Excess lags
/// are zero-filled. Empty inputs or output accept null pointers in those ranges.
/// Failure leaves output unchanged.
///
/// # Safety
/// Nonempty inputs must be initialized, readable in single allocations and
/// stable during computation. Output must be exclusively writable for lags values.
/// The inputs and output may overlap, without concurrent conflicting accesses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_cross_correlation_f64(
    left: *const f64,
    left_length: usize,
    right: *const f64,
    right_length: usize,
    output: *mut f64,
    lags: usize,
) -> u32 {
    if let Err(status) = range(output.cast_const(), lags) {
        return status;
    }
    // SAFETY: Forward the caller's documented input contracts.
    let result = unsafe {
        pair_result(left, left_length, right, right_length, |left, right| {
            crate::statistics::cross_correlation(left, right, lags)
        })
    };
    match result {
        Ok(values) => {
            if values.len() != lags {
                return 3;
            }
            if lags != 0 {
                // SAFETY: Input borrows ended. The owned result does not overlap
                // checked, caller-provided writable output storage.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, lags) };
            }
            0
        }
        Err(status) => status,
    }
}
