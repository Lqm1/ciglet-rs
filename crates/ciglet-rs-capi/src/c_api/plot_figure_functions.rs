//! Typed native Figure drawing operations.
use super::{
    arrays::{CigletRsArrayF32, CigletRsArrayF64},
    plot_figure::CigletRsFigure,
    plot_figure_commands,
};

/// Draw a native line and flush. Optional/extra x and color fallback match Commands::line.
/// # Safety
/// Figure is retained with no concurrent close/release. Y and present x are stable
/// initialized aligned independent allocations for count scalars. Present x requires
/// x_count >= count; null x requires zero x_count. No caller pointers are retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_line_f32(
    figure: *const CigletRsFigure,
    x: *const f32,
    x_count: usize,
    y: *const f32,
    count: usize,
    color: u8,
) -> u32 {
    // SAFETY: Forward the live figure and stable independent sample contracts.
    unsafe { plot_figure_commands::line(figure, x, x_count, y, count, color) }
}
/// Draw a native matrix image and flush, preserving nonfinite samples and empty shapes.
/// # Safety
/// Figure and typed immutable array are live independent owners retained throughout;
/// no conflicting mutations, close or release are permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_image_f32(
    figure: *const CigletRsFigure,
    array: *const CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward retained independent figure/array ownership contracts.
    unsafe { plot_figure_commands::image::<CigletRsArrayF32>(figure, array) }
}

/// Draw a native line and flush. Optional/extra x and color fallback match Commands::line.
/// # Safety
/// Figure is retained with no concurrent close/release. Y and present x are stable
/// initialized aligned independent allocations for count scalars. Present x requires
/// x_count >= count; null x requires zero x_count. No caller pointers are retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_line_f64(
    figure: *const CigletRsFigure,
    x: *const f64,
    x_count: usize,
    y: *const f64,
    count: usize,
    color: u8,
) -> u32 {
    // SAFETY: Forward the live figure and stable independent sample contracts.
    unsafe { plot_figure_commands::line(figure, x, x_count, y, count, color) }
}
/// Draw a native matrix image and flush, preserving nonfinite samples and empty shapes.
/// # Safety
/// Figure and typed immutable array are live independent owners retained throughout;
/// no conflicting mutations, close or release are permitted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_figure_image_f64(
    figure: *const CigletRsFigure,
    array: *const CigletRsArrayF64,
) -> u32 {
    // SAFETY: Forward retained independent figure/array ownership contracts.
    unsafe { plot_figure_commands::image::<CigletRsArrayF64>(figure, array) }
}
