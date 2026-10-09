//! Typed LF model and reusable plan entry points.
use super::{
    fourier,
    glottal::{self, CigletRsGlottalF32, CigletRsGlottalF64},
};

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_from_shape_f32(
    shape: f32,
    period: f32,
    excitation: f32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::from_shape(shape, period, excitation, output, count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_spectrum_f32(
    fields: *const f32,
    field_count: usize,
    frequencies: *const f32,
    count: usize,
    magnitude: *mut f32,
    phase: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::model_spectrum(
            fields,
            field_count,
            frequencies,
            count,
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_sampled_period_f32(
    fields: *const f32,
    field_count: usize,
    rate: i32,
    count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::model_period(fields, field_count, rate, count, output, output_count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_create_f32(
    fields: *const f32,
    count: usize,
    output: *mut *mut CigletRsGlottalF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::create(fields, count, output) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_clone_f32(
    handle: *const CigletRsGlottalF32,
    output: *mut *mut CigletRsGlottalF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::clone_owner(handle, output) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_release_f32(
    handle: *mut *mut CigletRsGlottalF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_model_f32(
    handle: *const CigletRsGlottalF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::get_model(handle, output, count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_spectrum_f32(
    handle: *const CigletRsGlottalF32,
    frequencies: *const f32,
    count: usize,
    magnitude: *mut f32,
    phase: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::prepared_spectrum(
            handle,
            frequencies,
            count,
            None,
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_spectrum_with_f32(
    handle: *const CigletRsGlottalF32,
    frequencies: *const f32,
    count: usize,
    precision: u32,
    magnitude: *mut f32,
    phase: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::prepared_spectrum(
            handle,
            frequencies,
            count,
            Some(precision),
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_sampled_period_f32(
    handle: *const CigletRsGlottalF32,
    rate: i32,
    count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::prepared_period(handle, rate, count, output, output_count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_from_shape_f64(
    shape: f64,
    period: f64,
    excitation: f64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::from_shape(shape, period, excitation, output, count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_spectrum_f64(
    fields: *const f64,
    field_count: usize,
    frequencies: *const f64,
    count: usize,
    magnitude: *mut f64,
    phase: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::model_spectrum(
            fields,
            field_count,
            frequencies,
            count,
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_model_sampled_period_f64(
    fields: *const f64,
    field_count: usize,
    rate: i32,
    count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::model_period(fields, field_count, rate, count, output, output_count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_create_f64(
    fields: *const f64,
    count: usize,
    output: *mut *mut CigletRsGlottalF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::create(fields, count, output) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_clone_f64(
    handle: *const CigletRsGlottalF64,
    output: *mut *mut CigletRsGlottalF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::clone_owner(handle, output) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_release_f64(
    handle: *mut *mut CigletRsGlottalF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_model_f64(
    handle: *const CigletRsGlottalF64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::get_model(handle, output, count) }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_spectrum_f64(
    handle: *const CigletRsGlottalF64,
    frequencies: *const f64,
    count: usize,
    magnitude: *mut f64,
    phase: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::prepared_spectrum(
            handle,
            frequencies,
            count,
            None,
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_spectrum_with_f64(
    handle: *const CigletRsGlottalF64,
    frequencies: *const f64,
    count: usize,
    precision: u32,
    magnitude: *mut f64,
    phase: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        glottal::prepared_spectrum(
            handle,
            frequencies,
            count,
            Some(precision),
            magnitude,
            phase,
            output_count,
        )
    }
}

/// Checked LF model configuration and calculation operation.
/// Model fields are period (seconds), closure, peak, return_time (period ratios),
/// excitation; field count must be five. Spectrum outputs match frequency count
/// and are disjoint. Native arithmetic is retained. Default spectrum precision is
/// Approximate; explicit codes use CIGLET_RS_GLOTTAL_FAST/APPROXIMATE/EXACT.
/// All outputs are retained on error. Ownership is published only on success.
///
/// # Safety
/// Nonempty input arrays are aligned initialized stable readable storage for their
/// declared counts. Output arrays are aligned exclusive writable storage for their
/// counts; magnitude and phase are disjoint. Input/output overlap is permitted once
/// computation ends. Null arrays are accepted only for zero counts. Handles are
/// live matching-precision owners; reads exclude release. Release requires exclusive
/// owner access, invalidates aliases and clears the initialized ownership slot.
/// Output ownership slots are independent and must not already own an unreleased
/// object. Owner/output storage is independent of private owner memory. Allocation
/// validity remains the caller's responsibility; native panic/allocation limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_glottal_prepared_sampled_period_f64(
    handle: *const CigletRsGlottalF64,
    rate: i32,
    count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { glottal::prepared_period(handle, rate, count, output, output_count) }
}
