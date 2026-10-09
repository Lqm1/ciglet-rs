//! Native PCM encoding adapters; no quantization algorithms are duplicated here.
use super::{
    boundary,
    byte_payload::CigletRsBytes,
    range, scalar_value_result,
    wave_path::{self, CigletRsPath},
    wave_write_options::{self, CigletRsWaveWriteOptions},
    wave_writer_stream::{CigletRsWriteStream, Writer},
};
use crate::{Error, statistics::Scalar, wave_write};
use std::panic::{RefUnwindSafe, UnwindSafe};
unsafe fn input<'a, T>(samples: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller keeps a checked aligned initialized allocation stable for this borrow.
        unsafe { std::slice::from_raw_parts(samples, count) }
    }
}
pub(super) unsafe fn memory<T: Scalar + UnwindSafe + RefUnwindSafe>(
    samples: *const T,
    count: usize,
    rate: u32,
    options: CigletRsWaveWriteOptions,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    if let Err(status) = range(samples, count) {
        return status;
    }
    // SAFETY: Borrowed samples end before output owner publication, and remain stable while encoding.
    unsafe {
        scalar_value_result(output, || {
            let mut bytes = Vec::new();
            wave_write::write_pcm(
                &mut bytes,
                input(samples, count),
                rate,
                wave_write_options::options(options)?,
            )
            .map_err(|_| Error("WAVE encoding failed"))?;
            Ok(Box::into_raw(Box::new(CigletRsBytes(bytes))))
        })
    }
}
pub(super) unsafe fn stream<T: Scalar + UnwindSafe + RefUnwindSafe>(
    stream: CigletRsWriteStream,
    samples: *const T,
    count: usize,
    rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    if let Err(status) = range(samples, count) {
        return status;
    }
    if stream.write.is_none() {
        return 1;
    }
    // SAFETY: Synchronous callbacks/context and stable sample allocation satisfy caller contract.
    match boundary(|| unsafe {
        wave_write::write_pcm(
            &mut Writer(stream),
            input(samples, count),
            rate,
            wave_write_options::options(options)?,
        )
        .map_err(|_| Error("WAVE stream encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
pub(super) unsafe fn file<T: Scalar + UnwindSafe + RefUnwindSafe>(
    path: CigletRsPath,
    samples: *const T,
    count: usize,
    rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    if let Err(status) = range(samples, count).and_then(|()| range(path.bytes, path.byte_count)) {
        return status;
    }
    // SAFETY: Stable sample and path byte allocations remain readable throughout file encoding.
    match boundary(|| unsafe {
        let bytes = input(path.bytes, path.byte_count);
        wave_write::write_pcm_file(
            wave_path::path(bytes, path.encoding)?,
            input(samples, count),
            rate,
            wave_write_options::options(options)?,
        )
        .map_err(|_| Error("WAVE file encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
