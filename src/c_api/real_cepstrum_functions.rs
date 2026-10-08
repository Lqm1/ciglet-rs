//! Typed real-cepstrum plans and modern default convenience functions.
use super::real_cepstrum::{
    self, CigletRsRealCepstrumF32, CigletRsRealCepstrumF64, OneShot, Operation,
};

/// Create a positive power-of-two plan within the native legacy bound.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_new_f32(
    length: usize,
    output: *mut *mut CigletRsRealCepstrumF32,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::new(length, output) }
}

/// Release sole ownership and clear the slot, excluding outstanding calls. Null stored handles succeed.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_release_f32(
    slot: *mut *mut CigletRsRealCepstrumF32,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { super::fourier::release(slot) }
}

/// Copy length and bins to exactly two indices. Length is positive, so the plan is never empty.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_shape_f32(
    handle: *const CigletRsRealCepstrumF32,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::shape(handle, output, output_count) }
}

/// Analyze exactly bins already-logarithmic values into length coefficients. No log is applied.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_analyze_f32(
    handle: *const CigletRsRealCepstrumF32,
    input: *const f32,
    input_count: usize,
    nyquist: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::Analyze,
        )
    }
}

/// Reconstruct a coefficient prefix, including empty prefixes, into length full-grid log values.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_reconstruct_f32(
    handle: *const CigletRsRealCepstrumF32,
    input: *const f32,
    input_count: usize,
    nyquist: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::Reconstruct,
        )
    }
}

/// Return length minimum-phase angles from bins log values. Lifter zero Causal, one Legacy.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_minimum_phase_f32(
    handle: *const CigletRsRealCepstrumF32,
    input: *const f32,
    input_count: usize,
    nyquist: u32,
    lifter: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::MinimumPhase(lifter),
        )
    }
}

/// Convert flattened one-sided magnitudes using native approximate log(magnitude+1e-15).
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f32(
    handle: *const CigletRsRealCepstrumF32,
    input: *const f32,
    input_count: usize,
    coefficients: usize,
    nyquist: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::ToCepstra(coefficients),
        )
    }
}

/// Convert explicit coefficient frames using native approximate exp, including zero-coefficient rows.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f32(
    handle: *const CigletRsRealCepstrumF32,
    input: *const f32,
    input_count: usize,
    frames: usize,
    coefficients: usize,
    nyquist: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::ToSpectrogram {
                frames,
                coefficients,
            },
        )
    }
}

/// Analyze log values with Preserve Nyquist, inferring length=2*(bins-1). Requires at least two bins.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_from_log_spectrum_f32(
    input: *const f32,
    input_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::one_shot(input, input_count, output, output_count, OneShot::Analyze) }
}

/// Reconstruct a coefficient prefix with Preserve Nyquist and output_count transform length.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_to_log_spectrum_f32(
    input: *const f32,
    input_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::one_shot(
            input,
            input_count,
            output,
            output_count,
            OneShot::Reconstruct,
        )
    }
}

/// Return minimum-phase angles with Preserve Nyquist and Causal lifter, inferring length=2*(bins-1).
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_minimum_phase_default_f32(
    input: *const f32,
    input_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::one_shot(
            input,
            input_count,
            output,
            output_count,
            OneShot::MinimumPhase,
        )
    }
}

/// Create a positive power-of-two plan within the native legacy bound.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_new_f64(
    length: usize,
    output: *mut *mut CigletRsRealCepstrumF64,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::new(length, output) }
}

/// Release sole ownership and clear the slot, excluding outstanding calls. Null stored handles succeed.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_release_f64(
    slot: *mut *mut CigletRsRealCepstrumF64,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { super::fourier::release(slot) }
}

/// Copy length and bins to exactly two indices. Length is positive, so the plan is never empty.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_shape_f64(
    handle: *const CigletRsRealCepstrumF64,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::shape(handle, output, output_count) }
}

/// Analyze exactly bins already-logarithmic values into length coefficients. No log is applied.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_analyze_f64(
    handle: *const CigletRsRealCepstrumF64,
    input: *const f64,
    input_count: usize,
    nyquist: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::Analyze,
        )
    }
}

/// Reconstruct a coefficient prefix, including empty prefixes, into length full-grid log values.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_reconstruct_f64(
    handle: *const CigletRsRealCepstrumF64,
    input: *const f64,
    input_count: usize,
    nyquist: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::Reconstruct,
        )
    }
}

/// Return length minimum-phase angles from bins log values. Lifter zero Causal, one Legacy.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_minimum_phase_f64(
    handle: *const CigletRsRealCepstrumF64,
    input: *const f64,
    input_count: usize,
    nyquist: u32,
    lifter: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::MinimumPhase(lifter),
        )
    }
}

/// Convert flattened one-sided magnitudes using native approximate log(magnitude+1e-15).
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f64(
    handle: *const CigletRsRealCepstrumF64,
    input: *const f64,
    input_count: usize,
    coefficients: usize,
    nyquist: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::ToCepstra(coefficients),
        )
    }
}

/// Convert explicit coefficient frames using native approximate exp, including zero-coefficient rows.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f64(
    handle: *const CigletRsRealCepstrumF64,
    input: *const f64,
    input_count: usize,
    frames: usize,
    coefficients: usize,
    nyquist: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::process(
            handle,
            input,
            input_count,
            nyquist,
            output,
            output_count,
            Operation::ToSpectrogram {
                frames,
                coefficients,
            },
        )
    }
}

/// Analyze log values with Preserve Nyquist, inferring length=2*(bins-1). Requires at least two bins.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_from_log_spectrum_f64(
    input: *const f64,
    input_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe { real_cepstrum::one_shot(input, input_count, output, output_count, OneShot::Analyze) }
}

/// Reconstruct a coefficient prefix with Preserve Nyquist and output_count transform length.
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_to_log_spectrum_f64(
    input: *const f64,
    input_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::one_shot(
            input,
            input_count,
            output,
            output_count,
            OneShot::Reconstruct,
        )
    }
}

/// Return minimum-phase angles with Preserve Nyquist and Causal lifter, inferring length=2*(bins-1).
/// Nyquist selectors are zero Preserve, one Legacy; unknown selectors fail.
/// Explicit output counts must match the native shape; failures retain outputs.
///
/// # Safety
/// Handles must be live, correctly typed library allocations retained throughout
/// calls. Scratch is serialized; release excludes outstanding calls. Independent
/// buffers and owner slots must not reference handles or their owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Empty buffers accept null. Transform input/output may overlap after borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_real_cepstrum_minimum_phase_default_f64(
    input: *const f64,
    input_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented caller ownership and buffer contracts.
    unsafe {
        real_cepstrum::one_shot(
            input,
            input_count,
            output,
            output_count,
            OneShot::MinimumPhase,
        )
    }
}
