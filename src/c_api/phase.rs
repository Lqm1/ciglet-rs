//! DCT and phase adapters with checked options and staged caller outputs.
use super::{generated_result, pair_result, range};
use crate::{Error, phase, statistics::Scalar};

type PairTransform<T> = fn(&[T], &[T]) -> Result<Vec<T>, Error>;

pub(super) unsafe fn paired<T: Scalar + std::panic::RefUnwindSafe>(
    first: *const T,
    first_length: usize,
    second: *const T,
    second_length: usize,
    output: *mut T,
    output_length: usize,
    compute: PairTransform<T>,
) -> u32 {
    if output_length != first_length {
        return 2;
    }
    if let Err(status) = range(output.cast_const(), output_length) {
        return status;
    }
    // SAFETY: Forward the initialized, stable caller input contracts.
    let result = unsafe { pair_result(first, first_length, second, second_length, compute) };
    match result {
        Ok(values) => {
            if values.len() != output_length {
                return 3;
            }
            if output_length != 0 {
                // SAFETY: Input borrows have ended. The owned result is independent
                // of caller output, which is exclusively writable.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, output_length) };
            }
            0
        }
        Err(status) => status,
    }
}

pub(super) fn magnitude<T: Scalar>(real: &[T], imaginary: &[T]) -> Result<Vec<T>, Error> {
    components(real, imaginary, phase::magnitude_of)
}
pub(super) fn argument<T: Scalar>(real: &[T], imaginary: &[T]) -> Result<Vec<T>, Error> {
    components(real, imaginary, phase::argument_of)
}
fn components<T: Scalar>(
    real: &[T],
    imaginary: &[T],
    compute: fn(crate::fourier::Complex<T>) -> Result<T, Error>,
) -> Result<Vec<T>, Error> {
    let mut values = crate::vector::buffer(real.len())?;
    for (&re, &im) in real.iter().zip(imaginary) {
        values.push(compute(crate::fourier::Complex::new(re, im))?);
    }
    Ok(values)
}

pub(super) unsafe fn complete<T: Scalar + std::panic::RefUnwindSafe>(
    input: *const T,
    length: usize,
    symmetry: u32,
    nyquist: u32,
    output: *mut T,
    output_length: usize,
) -> u32 {
    if length != output_length {
        return 2;
    }
    if let Err(status) = range(input, length) {
        return status;
    }
    let symmetry = match symmetry {
        0 => phase::Symmetry::Symmetric,
        1 => phase::Symmetry::Antisymmetric,
        _ => return 3,
    };
    let nyquist = match nyquist {
        0 => phase::NyquistPolicy::Preserve,
        1 => phase::NyquistPolicy::Legacy,
        _ => return 3,
    };
    // SAFETY: Caller output contract is forwarded. The complete owned result
    // is built before any output write, so input/output overlap is supported.
    unsafe {
        generated_result(output, length, || {
            let input = if length == 0 {
                &[]
            } else {
                // SAFETY: Input is an initialized, stable single-allocation range.
                std::slice::from_raw_parts(input, length)
            };
            let mut values = crate::vector::buffer(length)?;
            values.extend_from_slice(input);
            phase::complete_spectrum_with(&mut values, symmetry, nyquist)?;
            Ok(values)
        })
    }
}
