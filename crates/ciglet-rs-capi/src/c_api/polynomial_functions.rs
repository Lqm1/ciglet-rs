//! Typed polynomial value and root-report entry points.
use super::polynomial::{self, Diagnostics, Input, Output};
use crate::fourier::Complex;

/// Evaluate descending-order coefficients, preserving native power accumulation.
/// Missing complex coefficient planes mean zero components.
/// Both scalar component outputs are required.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_evaluate_f32(
    real: *const f32,
    imaginary: *const f32,
    coefficient_count: usize,
    argument_real: f32,
    argument_imaginary: f32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::evaluate(
            Input {
                real,
                imaginary,
                count: coefficient_count,
                real_only: false,
            },
            Complex::new(argument_real, argument_imaginary),
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: 1,
            },
        )
    }
}

/// Return root_count=coefficient_count-1 roots and all convergence diagnostics.
/// Missing complex coefficient planes mean zero components.
/// Converged is zero/one; nonconvergence is successful with its iteration/residual report.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_roots_f32(
    real: *const f32,
    imaginary: *const f32,
    coefficient_count: usize,
    output_real: *mut f32,
    output_imaginary: *mut f32,
    root_count: usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::roots(
            Input {
                real,
                imaginary,
                count: coefficient_count,
                real_only: false,
            },
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: root_count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}

/// Evaluate descending-order coefficients, preserving native power accumulation.
/// Nonempty real coefficients require a valid readable pointer.
/// Both scalar component outputs are required.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_evaluate_real_f32(
    real: *const f32,
    coefficient_count: usize,
    argument_real: f32,
    argument_imaginary: f32,
    output_real: *mut f32,
    output_imaginary: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::evaluate(
            Input {
                real,
                imaginary: std::ptr::null(),
                count: coefficient_count,
                real_only: true,
            },
            Complex::new(argument_real, argument_imaginary),
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: 1,
            },
        )
    }
}

/// Return root_count=coefficient_count-1 roots and all convergence diagnostics.
/// Nonempty real coefficients require a valid readable pointer.
/// Converged is zero/one; nonconvergence is successful with its iteration/residual report.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_roots_real_f32(
    real: *const f32,
    coefficient_count: usize,
    output_real: *mut f32,
    output_imaginary: *mut f32,
    root_count: usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::roots(
            Input {
                real,
                imaginary: std::ptr::null(),
                count: coefficient_count,
                real_only: true,
            },
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: root_count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}

/// Evaluate descending-order coefficients, preserving native power accumulation.
/// Missing complex coefficient planes mean zero components.
/// Both scalar component outputs are required.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_evaluate_f64(
    real: *const f64,
    imaginary: *const f64,
    coefficient_count: usize,
    argument_real: f64,
    argument_imaginary: f64,
    output_real: *mut f64,
    output_imaginary: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::evaluate(
            Input {
                real,
                imaginary,
                count: coefficient_count,
                real_only: false,
            },
            Complex::new(argument_real, argument_imaginary),
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: 1,
            },
        )
    }
}

/// Return root_count=coefficient_count-1 roots and all convergence diagnostics.
/// Missing complex coefficient planes mean zero components.
/// Converged is zero/one; nonconvergence is successful with its iteration/residual report.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_roots_f64(
    real: *const f64,
    imaginary: *const f64,
    coefficient_count: usize,
    output_real: *mut f64,
    output_imaginary: *mut f64,
    root_count: usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::roots(
            Input {
                real,
                imaginary,
                count: coefficient_count,
                real_only: false,
            },
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: root_count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}

/// Evaluate descending-order coefficients, preserving native power accumulation.
/// Nonempty real coefficients require a valid readable pointer.
/// Both scalar component outputs are required.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_evaluate_real_f64(
    real: *const f64,
    coefficient_count: usize,
    argument_real: f64,
    argument_imaginary: f64,
    output_real: *mut f64,
    output_imaginary: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::evaluate(
            Input {
                real,
                imaginary: std::ptr::null(),
                count: coefficient_count,
                real_only: true,
            },
            Complex::new(argument_real, argument_imaginary),
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: 1,
            },
        )
    }
}

/// Return root_count=coefficient_count-1 roots and all convergence diagnostics.
/// Nonempty real coefficients require a valid readable pointer.
/// Converged is zero/one; nonconvergence is successful with its iteration/residual report.
/// Errors retain every output. Root planes may be null only for zero roots.
///
/// # Safety
/// Present nonempty coefficient planes must be initialized, stable readable ranges.
/// Outputs must be aligned, exclusively writable and pairwise disjoint, including
/// diagnostics. Input/output may overlap after all input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_polynomial_roots_real_f64(
    real: *const f64,
    coefficient_count: usize,
    output_real: *mut f64,
    output_imaginary: *mut f64,
    root_count: usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented coefficient and output contracts.
    unsafe {
        polynomial::roots(
            Input {
                real,
                imaginary: std::ptr::null(),
                count: coefficient_count,
                real_only: true,
            },
            Output {
                real: output_real,
                imaginary: output_imaginary,
                count: root_count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}
