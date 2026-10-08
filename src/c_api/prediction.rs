//! Standalone prediction recursion/gain with explicit native arithmetic policy.
use super::{generated_result, pair_scalar_result, range};
use crate::{prediction, statistics::Scalar};
use std::panic::RefUnwindSafe;
fn arithmetic(code: u32) -> Result<prediction::ArithmeticPolicy, u32> {
    match code {
        0 => Ok(prediction::ArithmeticPolicy::Stable),
        1 => Ok(prediction::ArithmeticPolicy::Legacy),
        _ => Err(3),
    }
}
pub(super) unsafe fn levinson<T: Scalar + RefUnwindSafe>(
    input: *const T,
    count: usize,
    mode: u32,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(input, count) {
        return status;
    }
    let policy = match arithmetic(mode) {
        Ok(policy) => policy,
        Err(status) => return status,
    };
    // SAFETY: Caller input is initialized and stable; output is writable. Staged
    // owned coefficients end input borrowing before output commit, permitting overlap.
    unsafe {
        generated_result(output, output_count, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            prediction::levinson_with(input, policy)
        })
    }
}
pub(super) unsafe fn gain<T: Scalar + RefUnwindSafe>(
    coefficients: *const T,
    coefficient_count: usize,
    correlation: *const T,
    correlation_count: usize,
    mode: u32,
    output: *mut T,
) -> u32 {
    let policy = match arithmetic(mode) {
        Ok(policy) => policy,
        Err(status) => return status,
    };
    // SAFETY: Forward stable initialized input and writable scalar output contracts.
    unsafe {
        pair_scalar_result(
            coefficients,
            coefficient_count,
            correlation,
            correlation_count,
            output,
            |coefficients, correlation| prediction::gain_with(coefficients, correlation, policy),
        )
    }
}
