//! Explicit binary32/binary64 filtering entry points.
use super::filtering::{self, Design};

/// Design a frequency-sampling FIR. Passband 0 low, 1 high, 2 band, 3 stop; cutoffs in cycles/sample.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_windowed_fir_f32(
    length: usize,
    passband: u32,
    lower: f32,
    upper: f32,
    window: u32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            lower,
            upper,
            window,
            output,
            count,
            Design::Windowed(passband),
        )
    }
}

/// Design FIR with Nyquist-normalized cutoff. Highpass zero false, one true.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fir_f32(
    length: usize,
    cutoff: f32,
    highpass: u32,
    window: u32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            cutoff,
            0.0f32,
            window,
            output,
            count,
            Design::Fir(highpass),
        )
    }
}

/// Design bandpass FIR with Nyquist-normalized cutoffs.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bandpass_fir_f32(
    length: usize,
    lower: f32,
    upper: f32,
    window: u32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            lower,
            upper,
            window,
            output,
            count,
            Design::Bandpass,
        )
    }
}

/// Full convolution in source accumulation order; either empty input produces empty output.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_convolve_f32(
    signal: *const f32,
    signal_count: usize,
    impulse: *const f32,
    impulse_count: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe { filtering::convolve(signal, signal_count, impulse, impulse_count, output, count) }
}

/// Filter with startup zero corrected zero state, one source legacy zero prefix. Denominator element zero is not normalized.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filter_f32(
    signal: *const f32,
    signal_count: usize,
    numerator: *const f32,
    numerator_count: usize,
    denominator: *const f32,
    denominator_count: usize,
    startup: u32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::recursive(
            signal,
            signal_count,
            numerator,
            numerator_count,
            denominator,
            denominator_count,
            startup,
            output,
            count,
            false,
        )
    }
}

/// Filter with startup zero corrected zero state, one source legacy zero prefix. Denominator element zero is not normalized.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_forward_backward_filter_f32(
    signal: *const f32,
    signal_count: usize,
    numerator: *const f32,
    numerator_count: usize,
    denominator: *const f32,
    denominator_count: usize,
    startup: u32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::recursive(
            signal,
            signal_count,
            numerator,
            numerator_count,
            denominator,
            denominator_count,
            startup,
            output,
            count,
            true,
        )
    }
}

/// Design a frequency-sampling FIR. Passband 0 low, 1 high, 2 band, 3 stop; cutoffs in cycles/sample.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_windowed_fir_f64(
    length: usize,
    passband: u32,
    lower: f64,
    upper: f64,
    window: u32,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            lower,
            upper,
            window,
            output,
            count,
            Design::Windowed(passband),
        )
    }
}

/// Design FIR with Nyquist-normalized cutoff. Highpass zero false, one true.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fir_f64(
    length: usize,
    cutoff: f64,
    highpass: u32,
    window: u32,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            cutoff,
            0.0f64,
            window,
            output,
            count,
            Design::Fir(highpass),
        )
    }
}

/// Design bandpass FIR with Nyquist-normalized cutoffs.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bandpass_fir_f64(
    length: usize,
    lower: f64,
    upper: f64,
    window: u32,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::design(
            length,
            lower,
            upper,
            window,
            output,
            count,
            Design::Bandpass,
        )
    }
}

/// Full convolution in source accumulation order; either empty input produces empty output.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_convolve_f64(
    signal: *const f64,
    signal_count: usize,
    impulse: *const f64,
    impulse_count: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe { filtering::convolve(signal, signal_count, impulse, impulse_count, output, count) }
}

/// Filter with startup zero corrected zero state, one source legacy zero prefix. Denominator element zero is not normalized.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filter_f64(
    signal: *const f64,
    signal_count: usize,
    numerator: *const f64,
    numerator_count: usize,
    denominator: *const f64,
    denominator_count: usize,
    startup: u32,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::recursive(
            signal,
            signal_count,
            numerator,
            numerator_count,
            denominator,
            denominator_count,
            startup,
            output,
            count,
            false,
        )
    }
}

/// Filter with startup zero corrected zero state, one source legacy zero prefix. Denominator element zero is not normalized.
/// Results are fully staged; errors retain output. Counts must match the complete
/// native output shape. Unknown selectors return three. Empty buffers permit null.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging permits input/output overlap after all input borrowing ends.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_forward_backward_filter_f64(
    signal: *const f64,
    signal_count: usize,
    numerator: *const f64,
    numerator_count: usize,
    denominator: *const f64,
    denominator_count: usize,
    startup: u32,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward checked native input/output contracts and full staging.
    unsafe {
        filtering::recursive(
            signal,
            signal_count,
            numerator,
            numerator_count,
            denominator,
            denominator_count,
            startup,
            output,
            count,
            true,
        )
    }
}
