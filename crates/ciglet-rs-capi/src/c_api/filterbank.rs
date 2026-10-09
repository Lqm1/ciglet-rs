//! Owned filterbanks with checked caller buffers and exclusive mutations.
use super::{boundary, generated_result, range};
use crate::{
    Error,
    filterbank::{Compression, FilterBank},
    statistics::Scalar,
};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque binary32 filterbank. Release through its matching typed function.
pub struct CigletRsFilterBankF32(FilterBank<f32>);
/// Opaque binary64 filterbank. Release through its matching typed function.
pub struct CigletRsFilterBankF64(FilterBank<f64>);

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(bank: FilterBank<Self::Value>) -> Self;
    fn bank(&self) -> &FilterBank<Self::Value>;
    fn bank_mut(&mut self) -> &mut FilterBank<Self::Value>;
}
impl Handle for CigletRsFilterBankF32 {
    type Value = f32;
    fn new(bank: FilterBank<f32>) -> Self {
        Self(bank)
    }
    fn bank(&self) -> &FilterBank<f32> {
        &self.0
    }
    fn bank_mut(&mut self) -> &mut FilterBank<f32> {
        &mut self.0
    }
}
impl Handle for CigletRsFilterBankF64 {
    type Value = f64;
    fn new(bank: FilterBank<f64>) -> Self {
        Self(bank)
    }
    fn bank(&self) -> &FilterBank<f64> {
        &self.0
    }
    fn bank_mut(&mut self) -> &mut FilterBank<f64> {
        &mut self.0
    }
}
pub(super) unsafe fn create<H: Handle>(
    output: *mut *mut H,
    compute: impl FnOnce() -> Result<FilterBank<H::Value>, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| compute().map(H::new).map(Box::new)) {
        Ok(bank) => {
            // SAFETY: Caller owns an independent, aligned writable pointer slot.
            unsafe { output.write(Box::into_raw(bank)) };
            0
        }
        Err(status) => status,
    }
}
unsafe fn read<H: Handle, R>(
    handle: *const H,
    compute: impl FnOnce(&FilterBank<H::Value>) -> Result<R, Error> + UnwindSafe,
) -> Result<R, u32> {
    range(handle, 1)?;
    boundary(|| {
        // SAFETY: Caller retains a live correctly typed handle and excludes mutation.
        compute(unsafe { &*handle }.bank())
    })
}
pub(super) unsafe fn cloned<H: Handle>(handle: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller provides a live immutable handle and independent owner slot.
    unsafe { create(output, || Ok((&*handle).bank().clone())) }
}
pub(super) unsafe fn shape<H: Handle>(handle: *const H, output: *mut usize, count: usize) -> u32 {
    if count != 2 {
        return 2;
    }
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    // SAFETY: Caller retains an immutable live handle.
    match unsafe { read(handle, |bank| Ok([bank.bins(), bank.channels()])) } {
        Ok(shape) => {
            // SAFETY: The handle borrow ended; output is independent and writable.
            unsafe { std::ptr::copy_nonoverlapping(shape.as_ptr(), output, 2) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn nyquist<H: Handle>(handle: *const H, output: *mut H::Value) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller retains an immutable live handle.
    match unsafe { read(handle, |bank| Ok(bank.nyquist_hz())) } {
        Ok(value) => {
            // SAFETY: Caller output is independent and exclusively writable.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn set_nyquist<H: Handle>(handle: *mut H, value: H::Value) -> u32 {
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Caller grants exclusive access to a live correctly typed handle.
        unsafe { &mut *handle }.bank_mut().set_nyquist_hz(value);
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
pub(super) unsafe fn support<H: Handle>(
    handle: *const H,
    channel: usize,
    output: *mut usize,
    count: usize,
) -> u32 {
    if count != 2 {
        return 2;
    }
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    // SAFETY: Caller retains an immutable live handle.
    match unsafe {
        read(handle, |bank| {
            bank.support(channel)
                .map(|r| [r.start, r.end])
                .ok_or(Error("invalid filterbank channel"))
        })
    } {
        Ok(support) => {
            // SAFETY: Caller output is independent and writable for two indices.
            unsafe { std::ptr::copy_nonoverlapping(support.as_ptr(), output, 2) };
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn response<H: Handle>(
    handle: *const H,
    channel: usize,
    output: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller holds a live immutable handle; output is independent.
    let bank = unsafe { &*handle }.bank();
    if count != bank.bins() {
        return 2;
    }
    // SAFETY: Caller output contract is forwarded; responses remain immutable.
    unsafe {
        generated_result(output, count, || {
            let values = bank
                .response(channel)
                .ok_or(Error("invalid filterbank channel"))?;
            let mut result = crate::vector::buffer(count)?;
            result.extend_from_slice(values);
            Ok(result)
        })
    }
}
pub(super) unsafe fn set_channel<H: Handle>(
    handle: *mut H,
    channel: usize,
    input: *const H::Value,
    count: usize,
    start: usize,
    end: usize,
) -> u32 {
    if let Err(status) = range(handle.cast_const(), 1).and_then(|()| range(input, count)) {
        return status;
    }
    match boundary(|| {
        let input = if count == 0 {
            &[]
        } else {
            // SAFETY: Caller input is initialized and stable, independent of the handle.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        // SAFETY: Caller grants exclusive access, excluding all other handle uses.
        unsafe { &mut *handle }
            .bank_mut()
            .set_channel(channel, input, start..end)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
#[derive(Clone, Copy)]
pub(super) enum Frames {
    Single,
    Inferred,
    Explicit(usize),
}
pub(super) unsafe fn apply<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    count: usize,
    mode: u32,
    output: *mut H::Value,
    output_count: usize,
    frames: Frames,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, count)) {
        return status;
    }
    let compression = match mode {
        0 => Compression::Logarithmic,
        1 => Compression::Plp,
        _ => return 3,
    };
    // SAFETY: Caller retains a correctly typed immutable handle through computation.
    let bank = unsafe { &*handle }.bank();
    let frame_count = match frames {
        Frames::Single => {
            if count != bank.bins() {
                return 3;
            }
            1
        }
        Frames::Inferred => {
            if bank.bins() == 0 || !count.is_multiple_of(bank.bins()) {
                return 3;
            }
            count / bank.bins()
        }
        Frames::Explicit(n) => {
            if n.checked_mul(bank.bins()) != Some(count) {
                return 3;
            }
            n
        }
    };
    if frame_count.checked_mul(bank.channels()) != Some(output_count) {
        return 2;
    }
    // SAFETY: Caller initialized input and independent handle remain stable. Owned
    // staged results end input borrowing before writing, permitting input/output overlap.
    unsafe {
        generated_result(output, output_count, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            match frames {
                Frames::Single => bank.apply(input, compression),
                Frames::Inferred => bank.apply_spectrogram(input, compression),
                Frames::Explicit(n) => bank.apply_frames(input, n, compression),
            }
        })
    }
}
