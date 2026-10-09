//! Frequency coordinates and spectral distances using native numerical policies.
use super::{generated_result, pair_scalar_result, scalar_value_result};

/// Compute native frequency to mel; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_frequency_to_mel_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::frequency_to_mel(value)) }
}

/// Compute native mel to frequency; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mel_to_frequency_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::mel_to_frequency(value)) }
}

/// Compute native frequency to bark; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_frequency_to_bark_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::frequency_to_bark(value)) }
}

/// Compute native bark to frequency; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bark_to_frequency_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::bark_to_frequency(value)) }
}

/// Compute native equal loudness; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_equal_loudness_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::equal_loudness(value)) }
}

/// Generate intervals+1 native mel-spaced points, retaining approximate conversions.
/// Output length must equal intervals+1. Invalid counts/ranges leave output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for output_length scalars.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mel_space_f32(
    minimum: f32,
    maximum: f32,
    intervals: usize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    if intervals.checked_add(1) != Some(output_length) {
        return 2;
    }
    // SAFETY: Forward the caller's output buffer contract.
    unsafe {
        generated_result(output, output_length, || {
            crate::frequency::mel_space(minimum, maximum, intervals)
        })
    }
}

/// Compute stable mean divergence for matching nonempty spectra.
/// Uses the native Stable policy.
/// Stable identical positive spectra give zero; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_itakura_saito_f32(
    source: *const f32,
    source_length: usize,
    reference: *const f32,
    reference_length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            crate::distance::itakura_saito,
        )
    }
}

/// Compute natural log of mean divergence for matching nonempty spectra.
/// Uses the native Stable policy.
/// Stable identical positive spectra give negative infinity; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_log_itakura_saito_f32(
    source: *const f32,
    source_length: usize,
    reference: *const f32,
    reference_length: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            crate::distance::log_itakura_saito,
        )
    }
}

/// Compute natural log of mean divergence for matching nonempty spectra.
/// Policy zero is Stable, one Legacy; other codes return status three.
/// Stable identical positive spectra give negative infinity; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_log_itakura_saito_with_f32(
    source: *const f32,
    source_length: usize,
    reference: *const f32,
    reference_length: usize,
    policy: u32,
    output: *mut f32,
) -> u32 {
    let policy = match policy {
        0 => crate::distance::Policy::Stable,
        1 => crate::distance::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            |source, reference| crate::distance::log_itakura_saito_with(source, reference, policy),
        )
    }
}

/// Compute native frequency to mel; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_frequency_to_mel_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::frequency_to_mel(value)) }
}

/// Compute native mel to frequency; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mel_to_frequency_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::mel_to_frequency(value)) }
}

/// Compute native frequency to bark; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_frequency_to_bark_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::frequency_to_bark(value)) }
}

/// Compute native bark to frequency; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bark_to_frequency_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::bark_to_frequency(value)) }
}

/// Compute native equal loudness; finite/domain checks remain native.
/// Failure leaves output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for one scalar.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_equal_loudness_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the caller's scalar output contract.
    unsafe { scalar_value_result(output, || crate::frequency::equal_loudness(value)) }
}

/// Generate intervals+1 native mel-spaced points, retaining approximate conversions.
/// Output length must equal intervals+1. Invalid counts/ranges leave output unchanged.
///
/// # Safety
/// Output must be aligned and exclusively writable for output_length scalars.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_mel_space_f64(
    minimum: f64,
    maximum: f64,
    intervals: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    if intervals.checked_add(1) != Some(output_length) {
        return 2;
    }
    // SAFETY: Forward the caller's output buffer contract.
    unsafe {
        generated_result(output, output_length, || {
            crate::frequency::mel_space(minimum, maximum, intervals)
        })
    }
}

/// Compute stable mean divergence for matching nonempty spectra.
/// Uses the native Stable policy.
/// Stable identical positive spectra give zero; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_itakura_saito_f64(
    source: *const f64,
    source_length: usize,
    reference: *const f64,
    reference_length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            crate::distance::itakura_saito,
        )
    }
}

/// Compute natural log of mean divergence for matching nonempty spectra.
/// Uses the native Stable policy.
/// Stable identical positive spectra give negative infinity; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_log_itakura_saito_f64(
    source: *const f64,
    source_length: usize,
    reference: *const f64,
    reference_length: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            crate::distance::log_itakura_saito,
        )
    }
}

/// Compute natural log of mean divergence for matching nonempty spectra.
/// Policy zero is Stable, one Legacy; other codes return status three.
/// Stable identical positive spectra give negative infinity; a single zero against a
/// positive bin gives infinity. Paired zero bins are undefined in Stable mode.
/// Legacy retains the source approximate logarithms and selected-scalar grouping.
/// Failure leaves output unchanged.
///
/// # Safety
/// Inputs must be initialized, readable single-allocation ranges, stable through
/// computation. Output must be aligned and exclusively writable for one scalar.
/// Input and output may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_log_itakura_saito_with_f64(
    source: *const f64,
    source_length: usize,
    reference: *const f64,
    reference_length: usize,
    policy: u32,
    output: *mut f64,
) -> u32 {
    let policy = match policy {
        0 => crate::distance::Policy::Stable,
        1 => crate::distance::Policy::Legacy,
        _ => return 3,
    };
    // SAFETY: Forward the documented caller input/output contracts.
    unsafe {
        pair_scalar_result(
            source,
            source_length,
            reference,
            reference_length,
            output,
            |source, reference| crate::distance::log_itakura_saito_with(source, reference, policy),
        )
    }
}
