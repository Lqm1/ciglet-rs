//! Checked output boundary for the complete complex arithmetic family.
use super::{complex::*, scalar_value_result};
use crate::complex;

/// Construct a complex value, preserving IEEE fields and signed zeros.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_new_f32(
    real: f32,
    imaginary: f32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe { scalar_value_result(output, move || Ok(CigletRsComplexF32 { real, imaginary })) }
}

/// Conjugate a complex value, preserving native IEEE arithmetic.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_conjugate_f32(
    value: CigletRsComplexF32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF32::from(
                complex::Complex::from(value).conj(),
            ))
        })
    }
}

/// Add two complex values with native scalar rounding.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_add_f32(
    left: CigletRsComplexF32,
    right: CigletRsComplexF32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF32::from(
                complex::Complex::from(left) + complex::Complex::from(right),
            ))
        })
    }
}

/// Subtract two complex values with native scalar rounding.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_subtract_f32(
    left: CigletRsComplexF32,
    right: CigletRsComplexF32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF32::from(
                complex::Complex::from(left) - complex::Complex::from(right),
            ))
        })
    }
}

/// Multiply two complex values in the native operation order.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_multiply_f32(
    left: CigletRsComplexF32,
    right: CigletRsComplexF32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF32::from(
                complex::Complex::from(left) * complex::Complex::from(right),
            ))
        })
    }
}

/// Checked division retains the native scaled fallback for range loss.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_divide_f32(
    numerator: CigletRsComplexF32,
    divisor: CigletRsComplexF32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::divide(numerator.into(), divisor.into()).map(CigletRsComplexF32::from)
        })
    }
}

/// Complex exponential with Fast=0, Approximate=1 or Exact=2. Source c_exp selects 1.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_exponential_f32(
    value: CigletRsComplexF32,
    mode: u32,
    output: *mut CigletRsComplexF32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::exponential(value.into(), precision(mode)?).map(CigletRsComplexF32::from)
        })
    }
}

/// Complex argument with Fast=0, Approximate=1 or Exact=2.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_argument_f32(
    value: CigletRsComplexF32,
    mode: u32,
    output: *mut f32,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::argument(value.into(), precision(mode)?)
        })
    }
}

/// Construct a complex value, preserving IEEE fields and signed zeros.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_new_f64(
    real: f64,
    imaginary: f64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe { scalar_value_result(output, move || Ok(CigletRsComplexF64 { real, imaginary })) }
}

/// Conjugate a complex value, preserving native IEEE arithmetic.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_conjugate_f64(
    value: CigletRsComplexF64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF64::from(
                complex::Complex::from(value).conj(),
            ))
        })
    }
}

/// Add two complex values with native scalar rounding.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_add_f64(
    left: CigletRsComplexF64,
    right: CigletRsComplexF64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF64::from(
                complex::Complex::from(left) + complex::Complex::from(right),
            ))
        })
    }
}

/// Subtract two complex values with native scalar rounding.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_subtract_f64(
    left: CigletRsComplexF64,
    right: CigletRsComplexF64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF64::from(
                complex::Complex::from(left) - complex::Complex::from(right),
            ))
        })
    }
}

/// Multiply two complex values in the native operation order.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_multiply_f64(
    left: CigletRsComplexF64,
    right: CigletRsComplexF64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            Ok(CigletRsComplexF64::from(
                complex::Complex::from(left) * complex::Complex::from(right),
            ))
        })
    }
}

/// Checked division retains the native scaled fallback for range loss.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_divide_f64(
    numerator: CigletRsComplexF64,
    divisor: CigletRsComplexF64,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::divide(numerator.into(), divisor.into()).map(CigletRsComplexF64::from)
        })
    }
}

/// Complex exponential with Fast=0, Approximate=1 or Exact=2. Source c_exp selects 1.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_exponential_f64(
    value: CigletRsComplexF64,
    mode: u32,
    output: *mut CigletRsComplexF64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::exponential(value.into(), precision(mode)?).map(CigletRsComplexF64::from)
        })
    }
}

/// Complex argument with Fast=0, Approximate=1 or Exact=2.
/// Native validation is retained; errors leave the output unchanged.
/// Construction and ordinary operators retain successful nonfinite IEEE results.
///
/// # Safety
/// Output points to aligned, exclusively writable storage for its declared type.
/// Descriptors are copied by value; output may reuse the source value's storage.
/// Existing panic/allocation limits apply; selectors and null/alignment are checked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_complex_argument_f64(
    value: CigletRsComplexF64,
    mode: u32,
    output: *mut f64,
) -> u32 {
    // SAFETY: The shared helper checks the scalar slot and commits only on success.
    unsafe {
        scalar_value_result(output, move || {
            complex::argument(value.into(), precision(mode)?)
        })
    }
}
