//! Checked moving average/RMS and progressive median filtering.
use super::{generated_result, range};
use crate::{
    Error, median,
    smoothing::{self, Policy},
    statistics::Scalar,
};
use std::panic::RefUnwindSafe;
fn policy(code: u32) -> Result<Policy, Error> {
    match code {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(Error("unknown smoothing policy")),
    }
}
unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains initialized stable range-checked readable storage.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
pub(super) unsafe fn average<T: Scalar + RefUnwindSafe>(
    signal: *const T,
    count: usize,
    width: T,
    mode: u32,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(signal, count) {
        return status;
    }
    // SAFETY: Native owned staging retains outputs on error and ends all input
    // borrows before writable output commit, permitting input/output overlap.
    unsafe {
        generated_result(output, output_count, || {
            smoothing::moving_average_with(input(signal, count), width, policy(mode)?)
        })
    }
}
#[derive(Clone, Copy)]
pub(super) enum Integer {
    Rms(u32),
    Median,
}
pub(super) unsafe fn integer<T: Scalar + RefUnwindSafe>(
    signal: *const T,
    count: usize,
    width: usize,
    mode: Integer,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if output_count != count {
        return 2;
    }
    if let Err(status) = range(signal, count) {
        return status;
    }
    // SAFETY: Initialized stable input fully staged through native processing;
    // exclusive output writes occur only after input borrows end.
    unsafe {
        generated_result(output, output_count, || {
            let signal = input(signal, count);
            match mode {
                Integer::Rms(code) => smoothing::moving_rms_with(signal, width, policy(code)?),
                Integer::Median => median::filter(signal, width),
            }
        })
    }
}
