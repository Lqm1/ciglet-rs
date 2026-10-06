//! Reusable complex FFTs with ciglet's normalization and frequency ordering.
use std::sync::Arc;

pub use rustfft::num_complex::Complex;
use rustfft::{Fft, FftNum, FftPlanner};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Inverse,
}

/// Forward and inverse plans with a shared, reusable scratch buffer.
/// Supports f32 and f64. Inverse results are divided by the transform length.
pub struct FftPlan<T: FftNum> {
    length: usize,
    forward: Arc<dyn Fft<T>>,
    inverse: Arc<dyn Fft<T>>,
    scratch: Vec<Complex<T>>,
}

impl<T: FftNum> std::fmt::Debug for FftPlan<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FftPlan")
            .field("length", &self.length)
            .field("scratch_length", &self.scratch.len())
            .finish_non_exhaustive()
    }
}

impl<T: FftNum> FftPlan<T> {
    /// Plan a positive power-of-two length, the domain of upstream's Ooura FFT.
    pub fn new(length: usize) -> Result<Self, Error> {
        if !length.is_power_of_two() || length > (i32::MAX as usize) / 2 {
            return Err(Error(
                "FFT length must be a positive power of two within the legacy range",
            ));
        }
        let mut planner = FftPlanner::new();
        let forward = planner.plan_fft_forward(length);
        let inverse = planner.plan_fft_inverse(length);
        let scratch_length = forward
            .get_inplace_scratch_len()
            .max(inverse.get_inplace_scratch_len());
        Ok(Self {
            length,
            forward,
            inverse,
            scratch: zeros(scratch_length)?,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    /// Transform one interleaved complex buffer in place, without allocating.
    pub fn process(
        &mut self,
        values: &mut [Complex<T>],
        direction: Direction,
    ) -> Result<(), Error> {
        if values.len() != self.length {
            return Err(Error("FFT input length differs from the plan"));
        }
        match direction {
            Direction::Forward => self.forward.process_with_scratch(values, &mut self.scratch),
            Direction::Inverse => {
                self.inverse.process_with_scratch(values, &mut self.scratch);
                let divisor =
                    T::from_usize(self.length).ok_or(Error("FFT length cannot be represented"))?;
                for value in values {
                    value.re = value.re / divisor;
                    value.im = value.im / divisor;
                }
            }
        }
        Ok(())
    }

    /// Transform separate real/imaginary inputs. An absent input is all zeros,
    /// preserving the original NULL-input convention. Either component of the
    /// returned complex vector may be used or discarded independently.
    pub fn transform(
        &mut self,
        real: Option<&[T]>,
        imaginary: Option<&[T]>,
        direction: Direction,
    ) -> Result<Vec<Complex<T>>, Error> {
        if real.is_some_and(|values| values.len() != self.length)
            || imaginary.is_some_and(|values| values.len() != self.length)
        {
            return Err(Error("FFT component length differs from the plan"));
        }
        let mut values = zeros(self.length)?;
        for (index, value) in values.iter_mut().enumerate() {
            if let Some(real) = real {
                value.re = real[index];
            }
            if let Some(imaginary) = imaginary {
                value.im = imaginary[index];
            }
        }
        self.process(&mut values, direction)?;
        Ok(values)
    }
}

fn zeros<T: FftNum>(length: usize) -> Result<Vec<Complex<T>>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("FFT buffer allocation failed"))?;
    values.resize(length, Complex::new(T::zero(), T::zero()));
    Ok(values)
}

/// Move the DC bin to the center, preserving ciglet's odd-length ordering.
pub fn fft_shift<T: Clone>(values: &[T]) -> Vec<T> {
    let mut shifted = values.to_vec();
    fft_shift_in_place(&mut shifted);
    shifted
}

pub fn fft_shift_in_place<T>(values: &mut [T]) {
    values.rotate_right(values.len() / 2);
}
