//! Checked bundled approximation bindings; native kernels retain source order.
use super::{fastapprox_types::*, scalar_value_result};
use crate::approximation;

/// Source `fasterfc` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_complementary_error_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::complementary_error(value)
        })
    }
}

/// Source `fastererfc` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_complementary_error_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::complementary_error_fast(value)
        })
    }
}

/// Source `fasterf` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_error_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::error(value)) }
}

/// Source `fastererf` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_error_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::error_fast(value)) }
}

/// Source `fastinverseerf` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_inverse_error_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::inverse_error(value)) }
}

/// Source `fasterinverseerf` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_inverse_error_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::inverse_error_fast(value)
        })
    }
}

/// Source `vfasterfc` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_complementary_error_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::complementary_error(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastererfc` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_complementary_error_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::complementary_error_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterf` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_error_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::error(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastererf` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_error_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::error_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastinverseerf` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_inverse_error_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::inverse_error(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterinverseerf` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_inverse_error_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::inverse_error_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastlgamma` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_log_gamma_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::log_gamma(value)) }
}

/// Source `fasterlgamma` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_log_gamma_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::log_gamma_fast(value)
        })
    }
}

/// Source `fastdigamma` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_digamma_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::digamma(value)) }
}

/// Source `fasterdigamma` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_digamma_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::digamma_fast(value)) }
}

/// Source `vfastlgamma` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_log_gamma_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::log_gamma(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterlgamma` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_log_gamma_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::log_gamma_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastdigamma` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_digamma_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::digamma(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterdigamma` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_digamma_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::digamma_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastlambertw` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::lambert_w(value)) }
}

/// Source `fasterlambertw` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::lambert_w_fast(value)
        })
    }
}

/// Source `fastlambertwexpx` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_exp_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::special::lambert_w_exp(value)) }
}

/// Source `fasterlambertwexpx` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_exp_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::special::lambert_w_exp_fast(value)
        })
    }
}

/// Source `vfastlambertw` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::lambert_w(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterlambertw` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::lambert_w_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastlambertwexpx` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_exp_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::lambert_w_exp(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterlambertwexpx` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_special_lambert_w_exp_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::special::lambert_w_exp_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}
