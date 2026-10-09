//! Planar polynomial evaluation and complete root diagnostics.
use super::{boundary, disjoint_outputs, range};
use crate::{Error, fourier::Complex, polynomial, statistics::Scalar};
use std::panic::RefUnwindSafe;

#[derive(Clone, Copy)]
pub(super) struct Input<T> {
    pub real: *const T,
    pub imaginary: *const T,
    pub count: usize,
    pub real_only: bool,
}
#[derive(Clone, Copy)]
pub(super) struct Output<T> {
    pub real: *mut T,
    pub imaginary: *mut T,
    pub count: usize,
}
#[derive(Clone, Copy)]
pub(super) struct Diagnostics {
    pub iterations: *mut usize,
    pub converged: *mut u32,
    pub maximum_residual: *mut f64,
}
fn input_range<T>(input: Input<T>) -> Result<(), u32> {
    if input.real_only || !input.real.is_null() {
        range(input.real, input.count)?;
    }
    if !input.real_only && !input.imaginary.is_null() {
        range(input.imaginary, input.count)?;
    }
    if input.count > i32::MAX as usize {
        return Err(2);
    }
    Ok(())
}
unsafe fn complex_coefficients<T: Scalar>(input: Input<T>) -> Result<Vec<Complex<T>>, Error> {
    let mut result = crate::vector::buffer(input.count)?;
    for index in 0..input.count {
        // SAFETY: Each present plane was checked and caller provides initialized,
        // stable, single-allocation storage. Missing complex planes mean zero.
        let real = if input.real.is_null() {
            T::zero()
        } else {
            unsafe { input.real.add(index).read() }
        };
        let imaginary = if input.imaginary.is_null() {
            T::zero()
        } else {
            unsafe { input.imaginary.add(index).read() }
        };
        result.push(Complex::new(real, imaginary));
    }
    Ok(result)
}
pub(super) unsafe fn evaluate<T: Scalar + RefUnwindSafe>(
    input: Input<T>,
    argument: Complex<T>,
    output: Output<T>,
) -> u32 {
    if output.count != 1 {
        return 2;
    }
    if let Err(status) = input_range(input).and_then(|()| {
        disjoint_outputs(
            output.real.cast_const(),
            1,
            output.imaginary.cast_const(),
            1,
        )
    }) {
        return status;
    }
    let result = boundary(|| {
        if input.real_only {
            let values = if input.count == 0 {
                &[]
            } else {
                // SAFETY: Validated initialized stable real coefficient range.
                unsafe { std::slice::from_raw_parts(input.real, input.count) }
            };
            polynomial::evaluate_real(values, argument)
        } else {
            // SAFETY: Caller planes were validated before owned coefficient copies.
            polynomial::evaluate(&unsafe { complex_coefficients(input) }?, argument)
        }
    });
    match result {
        Ok(value) => {
            // SAFETY: Input borrows ended; outputs are distinct writable scalars.
            unsafe {
                output.real.write(value.re);
                output.imaginary.write(value.im);
            }
            0
        }
        Err(status) => status,
    }
}
fn outputs<T>(output: Output<T>, diagnostics: Diagnostics) -> Result<(), u32> {
    disjoint_outputs(
        output.real.cast_const(),
        output.count,
        output.imaginary.cast_const(),
        output.count,
    )?;
    for pointer in [output.real.cast_const(), output.imaginary.cast_const()] {
        disjoint_outputs(
            pointer,
            output.count,
            diagnostics.iterations.cast_const(),
            1,
        )?;
        disjoint_outputs(pointer, output.count, diagnostics.converged.cast_const(), 1)?;
        disjoint_outputs(
            pointer,
            output.count,
            diagnostics.maximum_residual.cast_const(),
            1,
        )?;
    }
    disjoint_outputs(
        diagnostics.iterations.cast_const(),
        1,
        diagnostics.converged.cast_const(),
        1,
    )?;
    disjoint_outputs(
        diagnostics.iterations.cast_const(),
        1,
        diagnostics.maximum_residual.cast_const(),
        1,
    )?;
    disjoint_outputs(
        diagnostics.converged.cast_const(),
        1,
        diagnostics.maximum_residual.cast_const(),
        1,
    )
}
pub(super) unsafe fn roots<T: Scalar + RefUnwindSafe>(
    input: Input<T>,
    output: Output<T>,
    diagnostics: Diagnostics,
) -> u32 {
    if input.count.checked_sub(1) != Some(output.count) {
        return if input.count == 0 { 3 } else { 2 };
    }
    if let Err(status) = input_range(input).and_then(|()| outputs(output, diagnostics)) {
        return status;
    }
    let result = boundary(|| {
        if input.real_only {
            // SAFETY: Validated nonempty initialized stable real coefficient range.
            polynomial::roots_real(unsafe { std::slice::from_raw_parts(input.real, input.count) })
        } else {
            // SAFETY: Caller planes were validated before owned coefficient copies.
            polynomial::roots(&unsafe { complex_coefficients(input) }?)
        }
    });
    match result {
        Ok(estimate) => {
            if estimate.values.len() != output.count {
                return 3;
            }
            // SAFETY: All input borrows ended. Pairwise-disjoint outputs are writable
            // and every result is complete before the first write, permitting input overlap.
            unsafe {
                for (index, value) in estimate.values.iter().enumerate() {
                    output.real.add(index).write(value.re);
                    output.imaginary.add(index).write(value.im);
                }
                diagnostics.iterations.write(estimate.iterations);
                diagnostics.converged.write(u32::from(estimate.converged));
                diagnostics
                    .maximum_residual
                    .write(estimate.maximum_residual);
            }
            0
        }
        Err(status) => status,
    }
}
