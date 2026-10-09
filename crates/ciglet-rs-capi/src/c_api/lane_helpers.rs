//! Complete portable conversion/bit helpers, using natural C array values.
use super::{fastapprox_types::*, scalar_value_result};
use crate::approximation::lanes;

/// Native `to_integers` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsIntegers4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_to_integers(
    value: CigletRsFloats4,
    output: *mut CigletRsIntegers4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsIntegers4 {
                values: lanes::to_integers(value.values),
            })
        })
    }
}

/// Native `from_integers` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsFloats4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_from_integers(
    value: CigletRsIntegers4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsFloats4 {
                values: lanes::from_integers(value.values),
            })
        })
    }
}

/// Native `to_bits` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsBits4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_to_bits(
    value: CigletRsFloats4,
    output: *mut CigletRsBits4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsBits4 {
                values: lanes::to_bits(value.values),
            })
        })
    }
}

/// Native `from_bits` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsFloats4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_from_bits(
    value: CigletRsBits4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsFloats4 {
                values: lanes::from_bits(value.values),
            })
        })
    }
}

/// Native `absolute` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsFloats4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_absolute(
    value: CigletRsFloats4,
    output: *mut CigletRsFloats4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsFloats4 {
                values: lanes::absolute(value.values),
            })
        })
    }
}

/// Native `from_integer64_lanes` retaining lane order and documented IEEE conversion rules.
///
/// # Safety
/// Output is aligned, exclusively writable CigletRsIntegers4 storage.
/// By-value inputs permit source-storage reuse. Native panic limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lanes_from_integer64_lanes(
    value: CigletRsIntegers64x2,
    output: *mut CigletRsIntegers4,
) -> u32 {
    // SAFETY: Shared helper checks the output slot before the complete value write.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsIntegers4 {
                values: lanes::from_integer64_lanes(value.values),
            })
        })
    }
}
