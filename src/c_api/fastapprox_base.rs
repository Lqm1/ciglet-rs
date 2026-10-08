//! Checked bundled approximation bindings; native kernels retain source order.
use super::{fastapprox_types::*, scalar_value_result};
use crate::approximation;

/// Source `fastpow2` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential2_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::exponential2(value)) }
}

/// Source `fastexp` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::exponential(value)) }
}

/// Source `fasterpow2` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential2_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::exponential2_fast(value)) }
}

/// Source `fasterexp` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::exponential_fast(value)) }
}

/// Source `vfastpow2` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential2_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::exponential2(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastexp` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::exponential(value.values).map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterpow2` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential2_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::exponential2_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterexp` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_exponential_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::exponential_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastlog2` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm2_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::logarithm2(value)) }
}

/// Source `fastlog` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::logarithm(value)) }
}

/// Source `fasterlog2` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm2_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::logarithm2_fast(value)) }
}

/// Source `fasterlog` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::logarithm_fast(value)) }
}

/// Source `vfastlog2` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm2_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::logarithm2(value.values).map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastlog` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::logarithm(value.values).map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterlog2` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm2_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::logarithm2_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterlog` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_logarithm_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::logarithm_fast(value.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastpow` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_power_f32(
    value: f32,
    exponent: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::power(value, exponent)) }
}

/// Source `fasterpow` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_power_fast_f32(
    value: f32,
    exponent: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::power_fast(value, exponent)) }
}

/// Source `vfastpow` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_power_lanes_f32(
    value: CigletRsFloats4,
    exponent: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::power(value.values, exponent.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasterpow` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_power_fast_lanes_f32(
    value: CigletRsFloats4,
    exponent: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::power_fast(value.values, exponent.values)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastsinfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_sine_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::sine(value)) }
}

/// Source `fastersinfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_sine_fast_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::sine_fast(value)) }
}

/// Source `fastcosfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_cosine_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::cosine(value)) }
}

/// Source `fastercosfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_cosine_fast_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe { scalar_value_result(output, move || approximation::cosine_fast(value)) }
}
