//! Native process stream operations use the same checked command kernels.
use super::{
    arrays::Handle,
    plot_commands,
    plot_figure::{CigletRsFigure, with},
    range,
};
use crate::{Error, statistics::Scalar};
use std::{
    io::Write,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub(super) unsafe fn line<T: Scalar + RefUnwindSafe + UnwindSafe>(
    figure: *const CigletRsFigure,
    x: *const T,
    x_count: usize,
    y: *const T,
    count: usize,
    color: u8,
) -> u32 {
    if let Err(e) = plot_commands::line_valid(x, x_count, y, count) {
        return e;
    }
    // SAFETY: Figure owner is retained and sample prefixes are stable independent inputs.
    match unsafe {
        with(figure, |figure| {
            plot_commands::line(figure.commands_mut().writer_mut(), x, y, count, color)
        })
    } {
        Ok(()) => 0,
        Err(e) => e,
    }
}
pub(super) unsafe fn image<H: Handle>(figure: *const CigletRsFigure, array: *const H) -> u32 {
    if let Err(e) = range(array, 1) {
        return e;
    }
    // SAFETY: Figure and immutable array are live independent owners retained throughout the call.
    match unsafe {
        with(figure, |figure| {
            figure
                .commands_mut()
                .image((&*array).array())
                .map_err(|_| Error("plot image output failed"))
        })
    } {
        Ok(()) => 0,
        Err(e) => e,
    }
}
/// Write arbitrary counted native configuration and flush the figure pipe.
/// Empty input permits null bytes and still flushes. Errors may leave a written prefix.
///
/// # Safety
/// Figure is a live owner with no concurrent close/release. Input is stable initialized
/// counted storage and independent of the figure; no caller pointers are retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_configuration(
    figure: *const CigletRsFigure,
    bytes: *const u8,
    count: usize,
) -> u32 {
    if let Err(e) = range(bytes, count) {
        return e;
    }
    // SAFETY: Native operation is serialized; bytes remain stable until writing and flushing end.
    match unsafe {
        with(figure, |figure| {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(bytes, count)
            };
            let writer = figure.commands_mut().writer_mut();
            writer
                .write_all(input)
                .and_then(|()| writer.flush())
                .map_err(|_| Error("plot configuration output failed"))
        })
    } {
        Ok(()) => 0,
        Err(e) => e,
    }
}
