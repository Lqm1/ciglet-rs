//! Statistics adapters retain native scalar policies and special-value semantics.
use super::scalar_slice_result;

/// Compute sum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sum_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            Ok(crate::statistics::sum(values))
        })
    }
}

/// Compute sum squares using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sum_squares_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            Ok(crate::statistics::sum_squares(values))
        })
    }
}

/// Compute mean using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mean_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::mean) }
}

/// Compute variance using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_variance_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::variance) }
}

/// Compute median using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_median_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::median) }
}

/// Compute minimum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_minimum_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::minimum(values)
                .ok_or(crate::Error("extremum requires at least one value"))
        })
    }
}

/// Compute maximum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_maximum_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::maximum(values)
                .ok_or(crate::Error("extremum requires at least one value"))
        })
    }
}

/// Compute mean with policy zero for Stable or one for Legacy.
/// Other policy integers return status three without touching output.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mean_with_f32(
    input: *const f32,
    length: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::mean_with(values, policy)
        })
    }
}

/// Compute variance with policy zero for Stable or one for Legacy.
/// Other policy integers return status three without touching output.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_variance_with_f32(
    input: *const f32,
    length: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::variance_with(values, policy)
        })
    }
}

/// Compute sum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sum_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            Ok(crate::statistics::sum(values))
        })
    }
}

/// Compute sum squares using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sum_squares_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            Ok(crate::statistics::sum_squares(values))
        })
    }
}

/// Compute mean using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mean_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::mean) }
}

/// Compute variance using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_variance_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::variance) }
}

/// Compute median using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_median_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe { scalar_slice_result(input, length, output, crate::statistics::median) }
}

/// Compute minimum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_minimum_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::minimum(values)
                .ok_or(crate::Error("extremum requires at least one value"))
        })
    }
}

/// Compute maximum using the native default policy.
/// Empty extrema, mean, variance and median return status three.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_maximum_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::maximum(values)
                .ok_or(crate::Error("extremum requires at least one value"))
        })
    }
}

/// Compute mean with policy zero for Stable or one for Legacy.
/// Other policy integers return status three without touching output.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mean_with_f64(
    input: *const f64,
    length: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::mean_with(values, policy)
        })
    }
}

/// Compute variance with policy zero for Stable or one for Legacy.
/// Other policy integers return status three without touching output.
///
/// # Safety
/// Nonempty input must be initialized, readable in one allocation and stable
/// during computation. Output must be exclusively writable for one value.
/// Input and output may overlap. Failure leaves output unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_variance_with_f64(
    input: *const f64,
    length: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    let policy = match policy {
        0 => crate::statistics::Policy::Stable,
        1 => crate::statistics::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: The documented caller buffer contract is forwarded unchanged.
    unsafe {
        scalar_slice_result(input, length, output, |values| {
            crate::statistics::variance_with(values, policy)
        })
    }
}
