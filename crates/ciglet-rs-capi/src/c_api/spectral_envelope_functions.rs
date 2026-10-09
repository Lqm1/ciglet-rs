//! Typed spectral-envelope C entry points.
use super::{
    fourier,
    spectral_envelope::{
        self, CigletRsEnvelopeF32, CigletRsEnvelopeF64, CigletRsEnvelopeOptionsF32,
        CigletRsEnvelopeOptionsF64, CigletRsEnvelopeOutputF32, CigletRsEnvelopeOutputF64,
    },
};
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_create_f32(
    length: usize,
    output: *mut *mut CigletRsEnvelopeF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::create(length, output) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_release_f32(
    handle: *mut *mut CigletRsEnvelopeF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { fourier::release(handle) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_len_f32(
    handle: *const CigletRsEnvelopeF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Length) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_bins_f32(
    handle: *const CigletRsEnvelopeF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Bins) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_is_empty_f32(
    handle: *const CigletRsEnvelopeF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Empty) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_f32(
    handle: *const CigletRsEnvelopeF32,
    spectrum: *const f32,
    count: usize,
    options: CigletRsEnvelopeOptionsF32,
    output: CigletRsEnvelopeOutputF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Default,
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_with_f32(
    handle: *const CigletRsEnvelopeF32,
    spectrum: *const f32,
    count: usize,
    options: CigletRsEnvelopeOptionsF32,
    nyquist: u32,
    output: CigletRsEnvelopeOutputF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Nyquist(nyquist),
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_with_smoothing_f32(
    handle: *const CigletRsEnvelopeF32,
    spectrum: *const f32,
    count: usize,
    options: CigletRsEnvelopeOptionsF32,
    nyquist: u32,
    smoothing: u32,
    output: CigletRsEnvelopeOutputF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Both(nyquist, smoothing),
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_one_shot_f32(
    length: usize,
    spectrum: *const f32,
    count: usize,
    options: CigletRsEnvelopeOptionsF32,
    output: CigletRsEnvelopeOutputF32,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::one_shot(length, spectrum, count, options.into(), output.into()) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_create_f64(
    length: usize,
    output: *mut *mut CigletRsEnvelopeF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::create(length, output) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_release_f64(
    handle: *mut *mut CigletRsEnvelopeF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { fourier::release(handle) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_len_f64(
    handle: *const CigletRsEnvelopeF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Length) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_bins_f64(
    handle: *const CigletRsEnvelopeF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Bins) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_is_empty_f64(
    handle: *const CigletRsEnvelopeF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::query(handle, output, spectral_envelope::Query::Empty) }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_f64(
    handle: *const CigletRsEnvelopeF64,
    spectrum: *const f64,
    count: usize,
    options: CigletRsEnvelopeOptionsF64,
    output: CigletRsEnvelopeOutputF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Default,
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_with_f64(
    handle: *const CigletRsEnvelopeF64,
    spectrum: *const f64,
    count: usize,
    options: CigletRsEnvelopeOptionsF64,
    nyquist: u32,
    output: CigletRsEnvelopeOutputF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Nyquist(nyquist),
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_estimate_with_smoothing_f64(
    handle: *const CigletRsEnvelopeF64,
    spectrum: *const f64,
    count: usize,
    options: CigletRsEnvelopeOptionsF64,
    nyquist: u32,
    smoothing: u32,
    output: CigletRsEnvelopeOutputF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe {
        spectral_envelope::process(
            handle,
            spectrum,
            count,
            options.into(),
            spectral_envelope::Operation::Both(nyquist, smoothing),
            output.into(),
        )
    }
}
/// Checked spectral-envelope operation with original native arithmetic.
/// Harmonic count zero selects None; one is invalid. Default preserves Nyquist
/// and uses Stable smoothing. Explicit policy zero is Preserve/Stable, one Legacy.
/// Metadata returns length, bins or is_empty as usize zero/one. Outputs are the
/// complete natural-log FFT grid and one-sided cepstrum; failures retain both.
///
/// # Safety
/// Handle is a live matching-precision constructor owner, with release excluded
/// during all calls. Reusable scratch is serialized. Release requires exclusive
/// owner access, invalidates aliases and clears its initialized slot. Constructors
/// publish ownership only on success; output slot must not own an unreleased object.
/// Nonempty arrays are aligned initialized stable readable inputs or exclusively
/// writable outputs for their declared counts. Output planes are disjoint and
/// independent of private owner memory. Input/output overlap is permitted after
/// calculation ends. Descriptors use platform C layout, passed by value. Null arrays
/// are accepted only for zero counts. Allocation validity is the caller's duty;
/// native panic/allocation and poisoned-plan limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_envelope_one_shot_f64(
    length: usize,
    spectrum: *const f64,
    count: usize,
    options: CigletRsEnvelopeOptionsF64,
    output: CigletRsEnvelopeOutputF64,
) -> u32 {
    // SAFETY: Forward the documented caller storage and owner contracts.
    unsafe { spectral_envelope::one_shot(length, spectrum, count, options.into(), output.into()) }
}
