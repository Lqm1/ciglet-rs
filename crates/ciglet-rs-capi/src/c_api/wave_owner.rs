//! Owned decoded waveforms and checked publication/copy operations.
use super::{
    generated_result, range, scalar_value_result,
    wave_path::{self, CigletRsPath},
    wave_stream::{CigletRsReadStream, Reader},
};
use crate::{
    Error,
    statistics::Scalar,
    wave::{self, Encoding, Wave},
};
use std::{
    io::Cursor,
    panic::{RefUnwindSafe, UnwindSafe},
};

/// Decoded PCM encoding; source sample width is reported separately.
pub const CIGLET_RS_WAVE_PCM: u32 = 0;
/// Decoded IEEE float encoding; source sample width is reported separately.
pub const CIGLET_RS_WAVE_FLOAT: u32 = 1;
/// Complete source metadata; decoded output is mono, retaining the left channel.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CigletRsWaveInfo {
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub channels: u16,
    pub encoding: u32,
    pub sample_count: usize,
}
/// Opaque immutable decoded binary32 waveform.
pub struct CigletRsWaveF32(Wave<f32>);
/// Opaque immutable decoded binary64 waveform.
pub struct CigletRsWaveF64(Wave<f64>);

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(wave: Wave<Self::Value>) -> Self;
    fn wave(&self) -> &Wave<Self::Value>;
}
impl Handle for CigletRsWaveF32 {
    type Value = f32;
    fn new(wave: Wave<f32>) -> Self {
        Self(wave)
    }
    fn wave(&self) -> &Wave<f32> {
        &self.0
    }
}
impl Handle for CigletRsWaveF64 {
    type Value = f64;
    fn new(wave: Wave<f64>) -> Self {
        Self(wave)
    }
    fn wave(&self) -> &Wave<f64> {
        &self.0
    }
}
unsafe fn publish<H: Handle>(
    output: *mut *mut H,
    compute: impl FnOnce() -> Result<Wave<H::Value>, Error> + UnwindSafe,
) -> u32 {
    // SAFETY: Caller supplies an independent exclusively writable ownership slot.
    unsafe {
        scalar_value_result(output, || {
            compute().map(|w| Box::into_raw(Box::new(H::new(w))))
        })
    }
}
pub(super) unsafe fn memory<H: Handle>(
    bytes: *const u8,
    count: usize,
    start: u64,
    budget: usize,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(bytes, count) {
        return status;
    }
    // SAFETY: Stable checked bytes are borrowed only until decoding ends; output is independent.
    unsafe {
        publish(output, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(bytes, count)
            };
            let mut reader = Cursor::new(input);
            reader.set_position(start);
            wave::read(&mut reader, budget).map_err(|_| Error("WAVE decode failed"))
        })
    }
}
pub(super) unsafe fn stream<H: Handle>(
    stream: CigletRsReadStream,
    budget: usize,
    output: *mut *mut H,
) -> u32 {
    if stream.read.is_none() || stream.seek.is_none() {
        return 1;
    }
    // SAFETY: Caller retains synchronous non-unwinding callbacks/context and independent output.
    unsafe {
        publish(output, || {
            wave::read(&mut Reader(stream), budget).map_err(|_| Error("WAVE stream decode failed"))
        })
    }
}
pub(super) unsafe fn file<H: Handle>(
    path: CigletRsPath,
    budget: usize,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(path.bytes, path.byte_count) {
        return status;
    }
    // SAFETY: Stable checked path bytes are borrowed before decoding/publication.
    unsafe {
        publish(output, || {
            let bytes = if path.byte_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(path.bytes, path.byte_count)
            };
            wave::read_file(wave_path::path(bytes, path.encoding)?, budget)
                .map_err(|_| Error("WAVE file decode failed"))
        })
    }
}
pub(super) unsafe fn clone_owner<H: Handle>(handle: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner remains retained until independent publication.
    unsafe { publish(output, || Ok((&*handle).wave().clone())) }
}
pub(super) unsafe fn release<H: Handle>(slot: *mut *mut H) -> u32 {
    if let Err(status) = range(slot.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized ownership slot transfers its live Box allocation, without outstanding users.
    unsafe {
        let handle = slot.read();
        if handle.is_null() {
            return 0;
        }
        if let Err(status) = range(handle.cast_const(), 1) {
            return status;
        }
        slot.write(std::ptr::null_mut());
        drop(Box::from_raw(handle));
    }
    0
}
pub(super) unsafe fn info<H: Handle>(handle: *const H, output: *mut CigletRsWaveInfo) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner and independent exclusively writable metadata.
    unsafe {
        scalar_value_result(output, || {
            let w = (&*handle).wave();
            Ok(CigletRsWaveInfo {
                sample_rate: w.sample_rate,
                bits_per_sample: w.bits_per_sample,
                channels: w.channels,
                encoding: match w.encoding {
                    Encoding::Pcm => CIGLET_RS_WAVE_PCM,
                    Encoding::Float => CIGLET_RS_WAVE_FLOAT,
                },
                sample_count: w.samples.len(),
            })
        })
    }
}
pub(super) unsafe fn samples<H: Handle>(
    handle: *const H,
    output: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller retains immutable owner and supplies independent exclusive output.
    if unsafe { (&*handle).wave().samples.len() } != count {
        return 2;
    }
    // SAFETY: Owned temporary sample copy is staged before writing the checked output.
    unsafe { generated_result(output, count, || Ok((&*handle).wave().samples.clone())) }
}
