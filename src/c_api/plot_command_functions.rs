//! Checked native plotting formats in both scalar precisions.
use super::{
    arrays::{CigletRsArrayF32, CigletRsArrayF64},
    byte_payload::CigletRsBytes,
    plot_commands,
    wave_writer_stream::CigletRsWriteStream,
};

/// Format native line commands into owned bytes. Null x with zero x_count selects indices; extra x is ignored.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Y is stable initialized aligned storage for count scalars. Present x supplies at least count readable aligned scalars and x_count >= count. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_line_bytes_f32(
    x: *const f32,
    x_count: usize,
    y: *const f32,
    count: usize,
    color: u8,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::line_memory(x, x_count, y, count, color, output) }
}

/// Write native line commands to a borrowed writer, then flush. Pointer/shape errors cause no callbacks.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Y and present x are stable initialized aligned allocations for the used count; x_count >= count. Null x requires zero x_count. Both callbacks/context remain valid, must not unwind or modify/conflict with borrowed samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_line_stream_f32(
    stream: CigletRsWriteStream,
    x: *const f32,
    x_count: usize,
    y: *const f32,
    count: usize,
    color: u8,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::line_stream(stream, x, x_count, y, count, color) }
}

/// Format complete native matrix image commands from a typed owned array.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Array is a live immutable owner of the declared precision; output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_image_bytes_f32(
    array: *const CigletRsArrayF32,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::image_memory::<CigletRsArrayF32>(array, output) }
}

/// Write and flush native matrix image commands through a borrowed writer.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Array remains a live immutable owner of the declared precision. Both callbacks/context remain valid, must not unwind or mutate/conflict with the borrowed array.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_image_stream_f32(
    stream: CigletRsWriteStream,
    array: *const CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::image_stream::<CigletRsArrayF32>(stream, array) }
}

/// Format native line commands into owned bytes. Null x with zero x_count selects indices; extra x is ignored.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Y is stable initialized aligned storage for count scalars. Present x supplies at least count readable aligned scalars and x_count >= count. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_line_bytes_f64(
    x: *const f64,
    x_count: usize,
    y: *const f64,
    count: usize,
    color: u8,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::line_memory(x, x_count, y, count, color, output) }
}

/// Write native line commands to a borrowed writer, then flush. Pointer/shape errors cause no callbacks.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Y and present x are stable initialized aligned allocations for the used count; x_count >= count. Null x requires zero x_count. Both callbacks/context remain valid, must not unwind or modify/conflict with borrowed samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_line_stream_f64(
    stream: CigletRsWriteStream,
    x: *const f64,
    x_count: usize,
    y: *const f64,
    count: usize,
    color: u8,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::line_stream(stream, x, x_count, y, count, color) }
}

/// Format complete native matrix image commands from a typed owned array.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Array is a live immutable owner of the declared precision; output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_image_bytes_f64(
    array: *const CigletRsArrayF64,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::image_memory::<CigletRsArrayF64>(array, output) }
}

/// Write and flush native matrix image commands through a borrowed writer.
/// Native color chars b/r/g/k/c/y and blue fallback are retained. NaN/infinity samples are preserved.
/// Process/Figure lifecycle bindings are separate and remain required.
///
/// # Safety
/// Array remains a live immutable owner of the declared precision. Both callbacks/context remain valid, must not unwind or mutate/conflict with the borrowed array.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_plot_image_stream_f64(
    stream: CigletRsWriteStream,
    array: *const CigletRsArrayF64,
) -> u32 {
    // SAFETY: Forward the documented stable input, callback and independent output contracts.
    unsafe { plot_commands::image_stream::<CigletRsArrayF64>(stream, array) }
}
