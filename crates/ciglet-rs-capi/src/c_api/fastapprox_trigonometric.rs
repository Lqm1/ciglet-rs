//! Checked bundled approximation bindings; native kernels retain source order.
use super::{fastapprox_types::*, scalar_value_result};
use crate::approximation;

/// Source `fastsin` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::sine(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastersin` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::sine(value, approximation::Precision::Fast)
        })
    }
}

/// Source `fastcos` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::cosine(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastercos` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::cosine(value, approximation::Precision::Fast)
        })
    }
}

/// Source `fasttan` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::tangent(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastertan` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::tangent(value, approximation::Precision::Fast)
        })
    }
}

/// Source `fasttanfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_full_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::tangent_full(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastertanfull` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_full_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::trigonometric::tangent_full(value, approximation::Precision::Fast)
        })
    }
}

/// Source `vfastsin` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::sine(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastersin` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::sine(value.values, approximation::Precision::Fast)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastsinfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_full_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::sine_full(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastersinfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_sine_full_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::sine_full(
                value.values,
                approximation::Precision::Fast,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastcos` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::cosine(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastercos` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::cosine(
                value.values,
                approximation::Precision::Fast,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastcosfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_full_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::cosine_full(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastercosfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_cosine_full_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::cosine_full(
                value.values,
                approximation::Precision::Fast,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasttan` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::tangent(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastertan` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::tangent(
                value.values,
                approximation::Precision::Fast,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasttanfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_full_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::tangent_full(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastertanfull` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_trigonometric_tangent_full_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::trigonometric::tangent_full(
                value.values,
                approximation::Precision::Fast,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}
