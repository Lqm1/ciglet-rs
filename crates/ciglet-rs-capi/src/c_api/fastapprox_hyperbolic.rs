//! Checked bundled approximation bindings; native kernels retain source order.
use super::{fastapprox_types::*, scalar_value_result};
use crate::approximation;

/// Source `fastsinh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sine_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::sine(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastersinh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sine_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::sine(value, approximation::Precision::Fast)
        })
    }
}

/// Source `fastcosh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_cosine_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::cosine(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastercosh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_cosine_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::cosine(value, approximation::Precision::Fast)
        })
    }
}

/// Source `fasttanh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_tangent_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::tangent(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastertanh` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_tangent_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::tangent(value, approximation::Precision::Fast)
        })
    }
}

/// Source `vfastsinh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sine_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::sine(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastersinh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sine_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::sine(value.values, approximation::Precision::Fast)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastcosh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_cosine_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::cosine(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastercosh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_cosine_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::cosine(value.values, approximation::Precision::Fast)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfasttanh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_tangent_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::tangent(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastertanh` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_tangent_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::tangent(value.values, approximation::Precision::Fast)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `fastsigmoid` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sigmoid_approximate_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::sigmoid(value, approximation::Precision::Approximate)
        })
    }
}

/// Source `fastersigmoid` using the audited native binary32 kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for f32.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sigmoid_fast_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::hyperbolic::sigmoid(value, approximation::Precision::Fast)
        })
    }
}

/// Source `vfastsigmoid` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sigmoid_approximate_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::sigmoid(
                value.values,
                approximation::Precision::Approximate,
            )
            .map(|values| CigletRsFloats4 { values })
        })
    }
}

/// Source `vfastersigmoid` using the audited native four-lane kernel.
/// Native domain checks and approved corrections apply; errors retain all output.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for CigletRsFloats4.
/// Inputs are copied by value; output may reuse the original source storage.
/// Existing caught-unwind/allocation limits apply. No native SIMD ABI is exposed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_fastapprox_hyperbolic_sigmoid_fast_lanes_f32(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Checked owned-result staging commits the complete result on success.
    unsafe {
        scalar_value_result(output, move || {
            approximation::lanes::hyperbolic::sigmoid(value.values, approximation::Precision::Fast)
                .map(|values| CigletRsFloats4 { values })
        })
    }
}
