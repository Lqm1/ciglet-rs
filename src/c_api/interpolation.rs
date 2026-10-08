//! Complete staged scalar, irregular, uniform and missing-value interpolation.
use super::{generated_result, range, scalar_value_result};
use crate::{
    interpolation::{self, BoundaryPolicy, KernelPolicy},
    statistics::Scalar,
};
use std::panic::RefUnwindSafe;
unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains initialized stable checked readable storage.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
pub(super) unsafe fn between<T: Scalar + RefUnwindSafe>(
    start: T,
    end: T,
    ratio: T,
    output: *mut T,
) -> u32 {
    // SAFETY: Exclusively writable scalar, staged native computation.
    unsafe { scalar_value_result(output, || interpolation::between(start, end, ratio)) }
}
pub(super) unsafe fn linear<T: Scalar + RefUnwindSafe>(
    coordinates: *const T,
    values: *const T,
    count: usize,
    positions: *const T,
    query_count: usize,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != query_count {
        return 2;
    }
    if let Err(status) = range(coordinates, count)
        .and_then(|()| range(values, count))
        .and_then(|()| range(positions, query_count))
    {
        return status;
    }
    // SAFETY: Readable input slices remain stable until complete owned staging;
    // output overlaps only after all input borrows have ended.
    unsafe {
        generated_result(output, output_count, || {
            interpolation::linear(
                input(coordinates, count),
                input(values, count),
                input(positions, query_count),
            )
        })
    }
}
#[derive(Clone, Copy)]
pub(super) enum Uniform {
    Linear,
    Sinc { boundary: u32, kernel: u32 },
}
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn uniform<T: Scalar + RefUnwindSafe>(
    start: T,
    end: T,
    values: *const T,
    count: usize,
    positions: *const T,
    query_count: usize,
    output: *mut T,
    output_count: usize,
    mode: Uniform,
) -> u32 {
    if output_count != query_count {
        return 2;
    }
    if let Err(status) = range(values, count).and_then(|()| range(positions, query_count)) {
        return status;
    }
    let sinc = match mode {
        Uniform::Linear => None,
        Uniform::Sinc { boundary, kernel } => {
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
            Some((boundary, kernel))
        }
    };
    // SAFETY: Complete native staging ends stable input borrows before output commit.
    unsafe {
        generated_result(output, output_count, || {
            let values = input(values, count);
            let positions = input(positions, query_count);
            match sinc {
                None => interpolation::linear_uniform(start, end, values, positions),
                Some((boundary, kernel)) => interpolation::sinc_uniform_with_kernel(
                    start, end, values, positions, boundary, kernel,
                ),
            }
        })
    }
}
pub(super) unsafe fn fill<T: Scalar + RefUnwindSafe>(
    values: *const T,
    count: usize,
    blank: T,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(values, count) {
        return status;
    }
    // SAFETY: Native accepts missing NaNs according to sentinel; readable initialized
    // scalar storage fully staged before exclusive output writes.
    unsafe {
        generated_result(output, output_count, || {
            interpolation::fill_missing(input(values, count), blank)
        })
    }
}
