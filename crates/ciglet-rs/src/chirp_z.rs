//! Reusable chirp-z transforms using FFT convolution.
use rustfft::FftNum;

use crate::{
    Error,
    fourier::{Complex, Direction, FftPlan},
};

/// A transform at frequencies k * angular_step, using ciglet's CZT convention.
/// Inverse plans negate the angular step and divide results by length. They
/// invert the forward transform on orthogonal DFT frequency grids.
#[derive(Debug)]
pub struct CztPlan<T: FftNum> {
    length: usize,
    direction: Direction,
    chirp: Vec<Complex<T>>,
    kernel: Vec<Complex<T>>,
    work: Vec<Complex<T>>,
    fft: FftPlan<T>,
}

impl<T: FftNum + Into<f64>> CztPlan<T> {
    pub fn new(length: usize, angular_step: T, direction: Direction) -> Result<Self, Error> {
        if length == 0 {
            return Err(Error("CZT length must be positive"));
        }
        let padded = length
            .checked_next_power_of_two()
            .and_then(|n| n.checked_mul(2))
            .filter(|&n| n <= i32::MAX as usize / 6)
            .ok_or(Error("CZT length exceeds the legacy buffer range"))?;
        let step: f64 = angular_step.into();
        if !step.is_finite() {
            return Err(Error("CZT angular step must be finite"));
        }
        let step = if direction == Direction::Inverse {
            -step
        } else {
            step
        };
        let mut chirp = buffer(length)?;
        let mut kernel = buffer(padded)?;
        chirp[0] = Complex::new(T::one(), T::zero());
        kernel[0] = chirp[0];
        for index in 1..length {
            // C computes this expression in double and stores the phase in FP_TYPE.
            let phase = T::from_f64(-0.5 * index as f64 * index as f64 * step)
                .ok_or(Error("CZT phase cannot be represented"))?;
            let phase: f64 = phase.into();
            if !phase.is_finite() {
                return Err(Error("CZT phase must remain finite"));
            }
            let value = Complex::new(
                T::from_f64(phase.cos()).ok_or(Error("CZT cosine cannot be represented"))?,
                T::from_f64(phase.sin()).ok_or(Error("CZT sine cannot be represented"))?,
            );
            chirp[index] = value;
            kernel[index] = Complex::new(value.re, -value.im);
            kernel[padded - index] = kernel[index];
        }
        let mut fft = FftPlan::new(padded)?;
        fft.process(&mut kernel, Direction::Forward)?;
        Ok(Self {
            length,
            direction,
            chirp,
            kernel,
            work: buffer(padded)?,
            fft,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Transform optional split components, returning an owned complex vector.
    pub fn transform(
        &mut self,
        real: Option<&[T]>,
        imaginary: Option<&[T]>,
    ) -> Result<Vec<Complex<T>>, Error> {
        let mut output = buffer(self.length)?;
        self.transform_into(real, imaginary, &mut output)?;
        Ok(output)
    }

    /// Reuse the cached kernel and work storage without allocating.
    /// Shape errors leave the caller's output unchanged.
    pub fn transform_into(
        &mut self,
        real: Option<&[T]>,
        imaginary: Option<&[T]>,
        output: &mut [Complex<T>],
    ) -> Result<(), Error> {
        if output.len() != self.length
            || real.is_some_and(|x| x.len() != self.length)
            || imaginary.is_some_and(|x| x.len() != self.length)
        {
            return Err(Error("CZT component length differs from the plan"));
        }
        self.work.fill(Complex::new(T::zero(), T::zero()));
        if let Some(imaginary) = imaginary {
            self.work[0].im = imaginary[0];
            for (index, &sample) in imaginary.iter().enumerate().skip(1) {
                self.work[index] = Complex::new(
                    -sample * self.chirp[index].im,
                    sample * self.chirp[index].re,
                );
            }
        }
        if let Some(real) = real {
            self.work[0].re = real[0];
            for (index, &sample) in real.iter().enumerate().skip(1) {
                self.work[index].re = self.work[index].re + sample * self.chirp[index].re;
                self.work[index].im = self.work[index].im + sample * self.chirp[index].im;
            }
        }
        self.fft.process(&mut self.work, Direction::Forward)?;
        for (value, &kernel) in self.work.iter_mut().zip(&self.kernel) {
            *value = *value * kernel;
        }
        self.fft.inverse_unnormalized(&mut self.work)?;
        let divisor = T::from_usize(self.work.len())
            .ok_or(Error("CZT padded length cannot be represented"))?;
        let inverse_divisor =
            T::from_usize(self.length).ok_or(Error("CZT length cannot be represented"))?;
        for ((out, &value), &chirp) in output.iter_mut().zip(&self.work).zip(&self.chirp) {
            out.re = (value.re * chirp.re - value.im * chirp.im) / divisor;
            out.im = (value.re * chirp.im + value.im * chirp.re) / divisor;
            if self.direction == Direction::Inverse {
                out.re = out.re / inverse_divisor;
                out.im = out.im / inverse_divisor;
            }
        }
        Ok(())
    }
}

fn buffer<T: FftNum>(length: usize) -> Result<Vec<Complex<T>>, Error> {
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(length)
        .map_err(|_| Error("CZT buffer allocation failed"))?;
    buffer.resize(length, Complex::new(T::zero(), T::zero()));
    Ok(buffer)
}
