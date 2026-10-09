//! Centered instantaneous frequency from a modulated four-term cosine window.
use crate::{Error, statistics::Scalar, vector};

/// Owned impulse responses. All four arrays must have the same nonzero length.
#[derive(Debug, Clone, PartialEq)]
pub struct Kernels<T> {
    pub real: Vec<T>,
    pub imaginary: Vec<T>,
    pub derivative_real: Vec<T>,
    pub derivative_imaginary: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Scale before convolution and evaluate the quotient in double precision.
    Stable,
    /// Retain the original selected-scalar accumulation and quotient.
    Legacy,
}

/// Reusable detector; frequencies are ratios of the sampling rate.
#[derive(Debug, Clone, PartialEq)]
pub struct Detector<T> {
    center_frequency: T,
    kernels: Kernels<T>,
}

impl<T: Scalar> Detector<T> {
    /// Generate the original kernels, with length truncated from `4 / resolution`.
    pub fn new(center_frequency: T, resolution: T) -> Result<Self, Error> {
        vector::validate(&[center_frequency, resolution])?;
        if resolution <= T::zero() {
            return Err(Error("frequency resolution must be positive"));
        }
        let length = (T::from_f64(4.0).unwrap() / resolution).into().trunc();
        if !(1.0..=i32::MAX as f64).contains(&length) {
            return Err(Error("detector kernel length is out of range"));
        }
        let length = length as usize;
        let coefficients =
            [0.338946, 0.481973, 0.161054, 0.018027].map(|v| T::from_f64(v).unwrap());
        let omega: T = vector::scalar(std::f64::consts::TAU * center_frequency.into())?;
        let window_omega: T = vector::scalar(std::f64::consts::TAU / length as f64)?;
        let mut real = vector::buffer(length)?;
        let mut imaginary = vector::buffer(length)?;
        let mut derivative_real = vector::buffer(length)?;
        let mut derivative_imaginary = vector::buffer(length)?;
        for index in 0..length {
            let offset = T::from_i64(index as i64 - (length / 2) as i64).unwrap();
            let mut window = T::zero();
            let mut derivative = T::zero();
            for (order, coefficient) in coefficients.iter().copied().enumerate() {
                let order = T::from_usize(order).unwrap();
                let phase = (order * window_omega * offset).into();
                window = vector::scalar(window.into() + coefficient.into() * phase.cos())?;
                let slope = -window_omega * order * coefficient;
                derivative = vector::scalar(derivative.into() + slope.into() * phase.sin())?;
            }
            let phase = (omega * offset).into();
            let sine: T = vector::scalar(phase.sin())?;
            let cosine: T = vector::scalar(phase.cos())?;
            real.push(window * cosine);
            imaginary.push(window * sine);
            derivative_real.push(derivative * cosine - omega * window * sine);
            derivative_imaginary.push(omega * window * cosine + derivative * sine);
        }
        Self::from_kernels(
            center_frequency,
            Kernels {
                real,
                imaginary,
                derivative_real,
                derivative_imaginary,
            },
        )
    }

    /// Supply caller-designed filters, equivalent to configuring the C structure.
    pub fn from_kernels(center_frequency: T, kernels: Kernels<T>) -> Result<Self, Error> {
        vector::validate(&[center_frequency])?;
        validate_kernels(&kernels)?;
        Ok(Self {
            center_frequency,
            kernels,
        })
    }
    pub fn center_frequency(&self) -> T {
        self.center_frequency
    }
    pub fn kernels(&self) -> &Kernels<T> {
        &self.kernels
    }
    /// Edited filters are checked again before estimation.
    pub fn kernels_mut(&mut self) -> &mut Kernels<T> {
        &mut self.kernels
    }
    pub fn set_center_frequency(&mut self, frequency: T) -> Result<(), Error> {
        vector::validate(&[frequency])?;
        self.center_frequency = frequency;
        Ok(())
    }
    pub fn estimate(&self, samples: &[T]) -> Result<T, Error> {
        self.estimate_with(samples, Policy::Stable)
    }
    /// Estimate at the buffer center. A shorter buffer returns positive zero.
    /// Only participating samples are inspected. Zero response is undefined.
    pub fn estimate_with(&self, samples: &[T], policy: Policy) -> Result<T, Error> {
        let length = validate_kernels(&self.kernels)?;
        if samples.len() < length {
            return Ok(T::zero());
        }
        let start = samples.len() / 2 - length / 2;
        let samples = &samples[start..start + length];
        vector::validate(samples)?;
        let filters = [
            &self.kernels.real,
            &self.kernels.imaginary,
            &self.kernels.derivative_real,
            &self.kernels.derivative_imaginary,
        ];
        let frequency = match policy {
            Policy::Legacy => {
                let responses = filters.map(|filter| {
                    filter
                        .iter()
                        .zip(samples)
                        .fold(T::zero(), |sum, (&h, &x)| sum + h * x)
                });
                let [real, imaginary, derivative_real, derivative_imaginary] = responses;
                let quotient = (real * derivative_imaginary - imaginary * derivative_real)
                    / (real * real + imaginary * imaginary);
                quotient.into() / 2.0 / std::f64::consts::PI
            }
            Policy::Stable => {
                let sample_scale = samples
                    .iter()
                    .map(|&x| x.into().abs())
                    .fold(0.0_f64, f64::max);
                let filter_scale = filters
                    .iter()
                    .flat_map(|filter| filter.iter())
                    .map(|&h| h.into().abs())
                    .fold(0.0_f64, f64::max);
                if sample_scale == 0.0 || filter_scale == 0.0 {
                    return Err(Error("instantaneous frequency has zero response"));
                }
                let responses = filters.map(|filter| {
                    filter.iter().zip(samples).fold(0.0, |sum, (&h, &x)| {
                        sum + (h.into() / filter_scale) * (x.into() / sample_scale)
                    })
                });
                let [real, imaginary, derivative_real, derivative_imaginary] = responses;
                let scale = real.abs().max(imaginary.abs());
                if scale == 0.0 {
                    return Err(Error("instantaneous frequency has zero response"));
                }
                let real = real / scale;
                let imaginary = imaginary / scale;
                ((real * (derivative_imaginary / scale) - imaginary * (derivative_real / scale))
                    / (real * real + imaginary * imaginary))
                    / std::f64::consts::TAU
            }
        };
        vector::scalar(frequency)
    }
}

fn validate_kernels<T: Scalar>(kernels: &Kernels<T>) -> Result<usize, Error> {
    let length = kernels.real.len();
    if length == 0 {
        return Err(Error("detector kernels must not be empty"));
    }
    for filter in [
        &kernels.real,
        &kernels.imaginary,
        &kernels.derivative_real,
        &kernels.derivative_imaginary,
    ] {
        if filter.len() != length {
            return Err(Error("detector kernel lengths differ"));
        }
        vector::validate(filter)?;
    }
    Ok(length)
}
