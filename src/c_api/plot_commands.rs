//! Native plot command formatting, staged payloads and synchronous external writers.
use super::{
    arrays::Handle,
    boundary,
    byte_payload::CigletRsBytes,
    range, scalar_value_result,
    wave_writer_stream::{CigletRsWriteStream, Writer},
};
use crate::{
    Error,
    plotting::{Color, Commands},
    statistics::Scalar,
};
use std::{
    io::Write,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub(super) fn writer_valid(stream: CigletRsWriteStream) -> Result<(), u32> {
    if stream.write.is_none() || stream.flush.is_none() {
        Err(1)
    } else {
        Ok(())
    }
}
fn line_valid<T>(x: *const T, x_count: usize, y: *const T, count: usize) -> Result<(), u32> {
    range(y, count)?;
    if x.is_null() {
        if x_count != 0 {
            return Err(2);
        }
    } else {
        if x_count < count {
            return Err(2);
        }
        // Extra x coordinates are ignored; only the prefix used by y is borrowed.
        range(x, count)?;
    }
    Ok(())
}
unsafe fn line<T: Scalar, W: Write>(
    writer: W,
    x: *const T,
    y: *const T,
    count: usize,
    color: u8,
) -> Result<(), Error> {
    // SAFETY: Caller retains checked stable input prefixes throughout synchronous formatting.
    let (x, y) = unsafe {
        let xs = if x.is_null() {
            None
        } else {
            Some(if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(x, count)
            })
        };
        let ys = if count == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(y, count)
        };
        (xs, ys)
    };
    Commands::new(writer)
        .line(x, y, Color::from(char::from(color)))
        .map_err(|_| Error("plot line output failed"))
}
pub(super) unsafe fn line_memory<T: Scalar + RefUnwindSafe + UnwindSafe>(
    x: *const T,
    x_count: usize,
    y: *const T,
    count: usize,
    color: u8,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    if let Err(status) = line_valid(x, x_count, y, count) {
        return status;
    }
    // SAFETY: Stable inputs and independent exclusively writable owner output.
    unsafe {
        scalar_value_result(output, || {
            let mut bytes = Vec::new();
            line(&mut bytes, x, y, count, color)?;
            Ok(Box::into_raw(Box::new(CigletRsBytes(bytes))))
        })
    }
}
pub(super) unsafe fn line_stream<T: Scalar + RefUnwindSafe + UnwindSafe>(
    stream: CigletRsWriteStream,
    x: *const T,
    x_count: usize,
    y: *const T,
    count: usize,
    color: u8,
) -> u32 {
    if let Err(status) = line_valid(x, x_count, y, count).and_then(|()| writer_valid(stream)) {
        return status;
    }
    // SAFETY: Synchronous non-unwinding callbacks and stable input prefixes are retained.
    match boundary(|| unsafe { line(Writer(stream), x, y, count, color) }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
pub(super) unsafe fn image_memory<H: Handle>(
    array: *const H,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    if let Err(status) = range(array, 1) {
        return status;
    }
    // SAFETY: Live immutable array owner and independent exclusively writable output slot.
    unsafe {
        scalar_value_result(output, || {
            let mut commands = Commands::new(Vec::new());
            commands
                .image((&*array).array())
                .map_err(|_| Error("plot image output failed"))?;
            Ok(Box::into_raw(Box::new(CigletRsBytes(
                commands.into_inner(),
            ))))
        })
    }
}
pub(super) unsafe fn image_stream<H: Handle>(stream: CigletRsWriteStream, array: *const H) -> u32 {
    if let Err(status) = range(array, 1).and_then(|()| writer_valid(stream)) {
        return status;
    }
    // SAFETY: Immutable array owner remains stable while valid callbacks format/flush.
    match boundary(|| unsafe {
        Commands::new(Writer(stream))
            .image((&*array).array())
            .map_err(|_| Error("plot image output failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
/// Retain arbitrary counted Gnuplot configuration bytes in an immutable payload.
/// Empty input may be null. Errors retain caller output.
///
/// # Safety
/// Nonempty bytes are stable initialized single-allocation storage for count bytes.
/// Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_configuration_bytes(
    bytes: *const u8,
    count: usize,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    if let Err(status) = range(bytes, count) {
        return status;
    }
    // SAFETY: Borrow ends before publication to the independent ownership slot.
    unsafe {
        scalar_value_result(output, || {
            let mut commands = Commands::new(Vec::new());
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(bytes, count)
            };
            commands
                .writer_mut()
                .write_all(input)
                .map_err(|_| Error("plot configuration output failed"))?;
            Ok(Box::into_raw(Box::new(CigletRsBytes(
                commands.into_inner(),
            ))))
        })
    }
}
/// Write and flush arbitrary configuration through synchronous caller-owned callbacks.
/// Empty input permits null bytes and still flushes. Partial I/O failure is reported.
///
/// # Safety
/// Bytes are stable initialized single-allocation input for count bytes. Both
/// callbacks/context are valid throughout, must not unwind or modify borrowed input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_configuration_stream(
    stream: CigletRsWriteStream,
    bytes: *const u8,
    count: usize,
) -> u32 {
    if let Err(status) = range(bytes, count).and_then(|()| writer_valid(stream)) {
        return status;
    }
    // SAFETY: Stable input and synchronous non-unwinding callback/context contracts.
    match boundary(|| unsafe {
        let mut commands = Commands::new(Writer(stream));
        let input = if count == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(bytes, count)
        };
        commands
            .writer_mut()
            .write_all(input)
            .and_then(|()| commands.writer_mut().flush())
            .map_err(|_| Error("plot configuration output failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
