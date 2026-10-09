//! Typed Fourier entry points with checked direction codes and owned plans.
use super::{CigletRsFftF32, fourier};

/// One-shot FFT: direction zero is forward; one is normalized inverse.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_f32(
    length: usize,
    real: *const f32,
    imaginary: *const f32,
    direction: u32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe {
        fourier::fft(
            length,
            real,
            imaginary,
            direction,
            output_real,
            output_imaginary,
        )
    }
}

/// Normalized inverse DFT for any positive supported length.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_inverse_dft_f32(
    length: usize,
    real: *const f32,
    imaginary: *const f32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe { fourier::inverse_dft(length, real, imaginary, output_real, output_imaginary) }
}

/// Create a reusable positive power-of-two FFT plan.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_plan_new_f32(
    length: usize,
    output: *mut *mut CigletRsFftF32,
) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe { fourier::new(length, output) }
}

/// Write the immutable plan length.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_plan_length_f32(
    handle: *const CigletRsFftF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe { fourier::plan_length(handle, output) }
}

/// Transform with a reusable plan: zero forward, one normalized inverse.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_plan_transform_f32(
    handle: *const CigletRsFftF32,
    real: *const f32,
    imaginary: *const f32,
    length: usize,
    direction: u32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe {
        fourier::process(
            handle,
            real,
            imaginary,
            length,
            direction,
            output_real,
            output_imaginary,
        )
    }
}

/// Release the plan and clear its initialized slot; a cleared slot succeeds.
/// Null input components mean zeros; null output components are discarded.
/// Present component buffers have length elements. Output planes must not
/// overlap one another; either may overlap input. Calculation failure leaves
/// outputs unchanged. Release always consumes a live plan once ownership transfers.
///
/// # Safety
/// Nonnull aligned handles must be live, correctly typed objects from this
/// library. All caller buffers/slots must be independent of private plan memory.
/// Present inputs must be initialized single-allocation ranges, stable during
/// computation; present outputs must be exclusively writable. Plan transforms
/// serialize scratch access; release excludes all outstanding plan calls/aliases.
/// Pointer-slot constructors transfer ownership; release invalidates every alias.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_plan_release_f32(slot: *mut *mut CigletRsFftF32) -> u32 {
    // SAFETY: Forward the documented buffer and owned-plan contracts.
    unsafe { fourier::release(slot) }
}

/// Rotate frequency bins right by length/2, including odd lengths.
/// Empty buffers accept null; input/output overlap is supported.
///
/// # Safety
/// Nonempty input must be initialized in one stable allocation; nonempty output
/// must be exclusively writable with a matching length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fft_shift_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: Forward the caller's initialized-input/writable-output contract.
    unsafe {
        super::vector_result(input, length, output, output_length, |values| {
            Ok(crate::fourier::fft_shift(values))
        })
    }
}
