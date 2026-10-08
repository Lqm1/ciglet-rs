//! Native resampling output sizing and independent compatibility policies.
use super::{generated_result, range, scalar_value_result};
use crate::{
    resampling::{self, BoundaryPolicy, KernelPolicy},
    statistics::Scalar,
};
use std::panic::RefUnwindSafe;
pub(super) unsafe fn length<T: Scalar + RefUnwindSafe>(
    count: usize,
    ratio: T,
    output: *mut usize,
) -> u32 {
    if count > i32::MAX as usize {
        return 2;
    }
    // SAFETY: Aligned exclusively writable scalar result, committed on success.
    unsafe { scalar_value_result(output, || resampling::output_length(count, ratio)) }
}
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn resample<T: Scalar + RefUnwindSafe>(
    input: *const T,
    count: usize,
    ratio: T,
    boundary: u32,
    kernel: u32,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if let Err(status) = range(input, count) {
        return status;
    }
    let boundary = match boundary {
        0 => BoundaryPolicy::IncludeFirst,
        1 => BoundaryPolicy::LegacySkipFirst,
        _ => return 3,
    };
    let kernel = match kernel {
        0 => KernelPolicy::Stable,
        1 => KernelPolicy::Legacy,
        _ => return 3,
    };
    let expected = match resampling::output_length(count, ratio) {
        Ok(n) => n,
        Err(_) => return 3,
    };
    if expected != output_count {
        return 2;
    }
    // SAFETY: Checked initialized stable readable input, exclusively writable
    // full output; owned staging ends input borrowing before overlapped commit.
    unsafe {
        generated_result(output, output_count, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            resampling::resample_with_kernel(input, ratio, boundary, kernel)
        })
    }
}
