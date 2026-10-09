//! Typed scaled-prediction ownership and complete correlation/gain access.
use super::{
    fourier,
    prediction_scaled::{self, CigletRsScaledPredictionF32, CigletRsScaledPredictionF64},
};

/// Compute an owned scaled prediction without converting correlations or gain.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_from_samples_f32(
    input: *const f32,
    input_count: usize,
    order: usize,
    output: *mut *mut CigletRsScaledPredictionF32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::samples(input, input_count, order, output) }
}

/// Clone a scaled estimate into an independent owner; failure retains the output slot.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_clone_f32(
    input: *const CigletRsScaledPredictionF32,
    output: *mut *mut CigletRsScaledPredictionF32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::clone_handle(input, output) }
}

/// Release an owner and clear its slot; an initialized null slot succeeds.
///
/// # Safety
/// Slot must be aligned, initialized and exclusively writable. Its nonnull owner must
/// originate from this typed API exactly once. Exclude all outstanding calls, aliases
/// and concurrent release. A released owner must never be reused.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_release_f32(
    slot: *mut *mut CigletRsScaledPredictionF32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Read the coefficient and correlation-lag count.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_length_f32(
    input: *const CigletRsScaledPredictionF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::length(input, output) }
}

/// Copy all coefficients into a caller array of exactly the reported length.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_coefficients_f32(
    input: *const CigletRsScaledPredictionF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::coefficients(input, output, count) }
}

/// Copy all lag significands and exponents; nonzero values are fraction * 2^exponent.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_correlation_parts_f32(
    input: *const CigletRsScaledPredictionF32,
    fractions: *mut f64,
    exponents: *mut i32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::correlation_parts(input, fractions, exponents, count) }
}

/// Return gain significand and exponent without imposing the selected scalar range.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_gain_parts_f32(
    input: *const CigletRsScaledPredictionF32,
    fraction: *mut f64,
    exponent: *mut i32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::gain_parts(input, fraction, exponent) }
}

/// Return scalar gain, retaining output when nonzero gain cannot be represented.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_gain_f32(
    input: *const CigletRsScaledPredictionF32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::gain(input, output) }
}

/// Copy ordinary coefficients and correlations only when all nonzero lags survive conversion.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_ordinary_f32(
    input: *const CigletRsScaledPredictionF32,
    coefficients: *mut f32,
    correlations: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::ordinary(input, coefficients, correlations, count) }
}

/// Compute an owned scaled prediction without converting correlations or gain.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_from_samples_f64(
    input: *const f64,
    input_count: usize,
    order: usize,
    output: *mut *mut CigletRsScaledPredictionF64,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::samples(input, input_count, order, output) }
}

/// Clone a scaled estimate into an independent owner; failure retains the output slot.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_clone_f64(
    input: *const CigletRsScaledPredictionF64,
    output: *mut *mut CigletRsScaledPredictionF64,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::clone_handle(input, output) }
}

/// Release an owner and clear its slot; an initialized null slot succeeds.
///
/// # Safety
/// Slot must be aligned, initialized and exclusively writable. Its nonnull owner must
/// originate from this typed API exactly once. Exclude all outstanding calls, aliases
/// and concurrent release. A released owner must never be reused.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_release_f64(
    slot: *mut *mut CigletRsScaledPredictionF64,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Read the coefficient and correlation-lag count.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_length_f64(
    input: *const CigletRsScaledPredictionF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::length(input, output) }
}

/// Copy all coefficients into a caller array of exactly the reported length.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_coefficients_f64(
    input: *const CigletRsScaledPredictionF64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::coefficients(input, output, count) }
}

/// Copy all lag significands and exponents; nonzero values are fraction * 2^exponent.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_correlation_parts_f64(
    input: *const CigletRsScaledPredictionF64,
    fractions: *mut f64,
    exponents: *mut i32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::correlation_parts(input, fractions, exponents, count) }
}

/// Return gain significand and exponent without imposing the selected scalar range.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_gain_parts_f64(
    input: *const CigletRsScaledPredictionF64,
    fraction: *mut f64,
    exponent: *mut i32,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::gain_parts(input, fraction, exponent) }
}

/// Return scalar gain, retaining output when nonzero gain cannot be represented.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_gain_f64(
    input: *const CigletRsScaledPredictionF64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::gain(input, output) }
}

/// Copy ordinary coefficients and correlations only when all nonzero lags survive conversion.
///
/// # Safety
/// Retain typed live immutable owners through every call, excluding concurrent release.
/// Nonempty inputs must be initialized, stable, readable single-allocation ranges.
/// Outputs must be aligned, exclusively writable, pairwise disjoint and independent
/// of every owner allocation. Keep ownership of any previous slot value on success;
/// replacement does not release that value. All outputs are retained on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_scaled_prediction_ordinary_f64(
    input: *const CigletRsScaledPredictionF64,
    coefficients: *mut f64,
    correlations: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the caller's typed ownership and independent buffer contracts.
    unsafe { prediction_scaled::ordinary(input, coefficients, correlations, count) }
}
