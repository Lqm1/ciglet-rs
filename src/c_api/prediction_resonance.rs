//! Complete sorted resonance poles and native root convergence diagnostics.
use super::{boundary, disjoint_outputs, polynomial::Diagnostics, range};
use crate::{prediction, statistics::Scalar};
use std::panic::RefUnwindSafe;

#[derive(Clone, Copy)]
pub(super) struct Output<T> {
    pub frequencies: *mut T,
    pub real: *mut T,
    pub imaginary: *mut T,
    pub capacity: usize,
    pub count: *mut usize,
}
fn outputs<T>(output: Output<T>, diagnostics: Diagnostics) -> Result<(), u32> {
    range(output.count.cast_const(), 1)?;
    range(diagnostics.iterations.cast_const(), 1)?;
    range(diagnostics.converged.cast_const(), 1)?;
    range(diagnostics.maximum_residual.cast_const(), 1)?;
    let planes = [
        output.frequencies.cast_const(),
        output.real.cast_const(),
        output.imaginary.cast_const(),
    ];
    let scalars = [
        (output.count.cast_const().cast::<u8>(), size_of::<usize>()),
        (
            diagnostics.iterations.cast_const().cast::<u8>(),
            size_of::<usize>(),
        ),
        (
            diagnostics.converged.cast_const().cast::<u8>(),
            size_of::<u32>(),
        ),
        (
            diagnostics.maximum_residual.cast_const().cast::<u8>(),
            size_of::<f64>(),
        ),
    ];
    for (index, &plane) in planes.iter().enumerate() {
        for &other in &planes[index + 1..] {
            disjoint_outputs(plane, output.capacity, other, output.capacity)?;
        }
        for &(scalar, bytes) in &scalars {
            disjoint_outputs(plane, output.capacity, scalar, bytes)?;
        }
    }
    // Typed scalar validation above established real alignment; byte ranges
    // permit pairwise mixed usize/u32/f64 storage checks without creating borrows.
    for (index, &(pointer, bytes)) in scalars.iter().enumerate() {
        for &(other, other_bytes) in &scalars[index + 1..] {
            disjoint_outputs(pointer, bytes, other, other_bytes)?;
        }
    }
    Ok(())
}
pub(super) unsafe fn estimate<T: Scalar + RefUnwindSafe>(
    input: *const T,
    input_count: usize,
    output: Output<T>,
    diagnostics: Diagnostics,
) -> u32 {
    if let Err(status) = range(input, input_count).and_then(|()| outputs(output, diagnostics)) {
        return status;
    }
    let result = boundary(|| {
        let coefficients = if input_count == 0 {
            &[]
        } else {
            // SAFETY: Initialized stable readable single-allocation coefficient range.
            unsafe { std::slice::from_raw_parts(input, input_count) }
        };
        prediction::resonances(coefficients)
    });
    match result {
        Ok(estimate) => {
            let count = estimate.resonances.len();
            if count > output.capacity {
                return 2;
            }
            // SAFETY: Input borrowing ended and the complete owned report is staged.
            // Every output range is pairwise disjoint and exclusively writable.
            // Only populated elements are written; unused capacity remains unchanged.
            unsafe {
                for (index, resonance) in estimate.resonances.into_iter().enumerate() {
                    output
                        .frequencies
                        .add(index)
                        .write(resonance.frequency_nyquist);
                    output.real.add(index).write(resonance.pole.re);
                    output.imaginary.add(index).write(resonance.pole.im);
                }
                output.count.write(count);
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
