//! Checked caller-owned array generation without transferring allocator ownership.
use super::generated_result;

/// Generate a checked i32 arithmetic progression.
/// Zero length accepts a null output. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be exclusively writable for length i32 values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_integer_sequence(
    start: i32,
    step: i32,
    output: *mut i32,
    length: usize,
) -> u32 {
    // SAFETY: The caller supplies the documented output storage.
    unsafe {
        generated_result(output, length, || {
            crate::vector::integer_sequence(start, step, length)
        })
    }
}

/// Generate an inclusive linear grid, retaining the native rounding policy.
/// Zero length accepts a null output; one point equals start.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be exclusively writable for length values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_linear_grid_f32(
    start: f32,
    end: f32,
    output: *mut f32,
    length: usize,
) -> u32 {
    // SAFETY: The caller supplies the documented output storage.
    unsafe {
        generated_result(output, length, || {
            crate::vector::linear_grid(start, end, length)
        })
    }
}

/// Generate an inclusive linear grid, retaining the native rounding policy.
/// Zero length accepts a null output; one point equals start.
/// Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be exclusively writable for length values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_linear_grid_f64(
    start: f64,
    end: f64,
    output: *mut f64,
    length: usize,
) -> u32 {
    // SAFETY: The caller supplies the documented output storage.
    unsafe {
        generated_result(output, length, || {
            crate::vector::linear_grid(start, end, length)
        })
    }
}
