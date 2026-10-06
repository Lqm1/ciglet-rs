//! Upstream window definitions with precise and level-two arithmetic.
use crate::{Error, approximation};
use rustfft::FftNum;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Precision {
    #[default]
    Accurate,
    /// Use upstream level-two approximations where the original does so.
    Approximate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    Boxcar,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    Nuttall98,
    MltSine,
}

impl Window {
    /// Generate precise f32 weights. Singleton windows are intentionally unity;
    /// empty windows remain empty. Use generate_with for fallible allocation.
    pub fn generate(self, length: usize) -> Vec<f32> {
        self.generate_with(length, Precision::Accurate)
            .expect("window generation failed")
    }

    /// Generate f32/f64 windows with the original precision and arithmetic.
    /// Hann/Hamming are symmetric; Blackman/Harris/Nuttall are periodic.
    /// Blackman retains accurate cosine in both modes, matching the upstream
    /// macro that defines blackman_2 using cos_3.
    pub fn generate_with<T: FftNum + Into<f64>>(
        self,
        length: usize,
        precision: Precision,
    ) -> Result<Vec<T>, Error> {
        if length > i32::MAX as usize {
            return Err(Error("window length exceeds the legacy range"));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(length)
            .map_err(|_| Error("window allocation failed"))?;
        if length == 1 {
            values.push(T::one());
            return Ok(values);
        }
        let cosine = |angle: f64| -> Result<f64, Error> {
            match precision {
                Precision::Accurate => Ok(angle.cos()),
                Precision::Approximate => Ok(approximation::cosine(angle as f32)? as f64),
            }
        };
        for index in 0..length {
            let value = match self {
                Self::Boxcar => T::one(),
                Self::Hann => {
                    let angle = 2.0 * PI * index as f64 / (length - 1) as f64;
                    // Integer 1 minus C's float approximation is evaluated in
                    // f32 before the double-precision multiplication by 0.5.
                    let difference = match precision {
                        Precision::Accurate => 1.0 - angle.cos(),
                        Precision::Approximate => {
                            (1.0f32 - approximation::cosine(angle as f32)?) as f64
                        }
                    };
                    scalar(0.5 * difference)?
                }
                Self::Hamming => {
                    scalar(0.54 - 0.46 * cosine(2.0 * PI * index as f64 / (length - 1) as f64)?)?
                }
                Self::MltSine => {
                    let angle = PI / length as f64 * (index as f64 + 0.5);
                    scalar(match precision {
                        Precision::Accurate => angle.sin(),
                        Precision::Approximate => approximation::sine(angle as f32)? as f64,
                    })?
                }
                Self::Blackman => {
                    cosine_terms(index, length, &[0.42, 0.5, 0.08], Precision::Accurate)?
                }
                Self::BlackmanHarris => cosine_terms(
                    index,
                    length,
                    &[0.35875, 0.48829, 0.14128, 0.01168],
                    precision,
                )?,
                Self::Nuttall98 => cosine_terms(
                    index,
                    length,
                    &[0.3635819, 0.4891775, 0.1365995, 0.0106411],
                    precision,
                )?,
            };
            values.push(value);
        }
        Ok(values)
    }
}

fn scalar<T: FftNum>(value: f64) -> Result<T, Error> {
    T::from_f64(value).ok_or(Error("window value cannot be represented"))
}

fn cosine_terms<T: FftNum + Into<f64>>(
    index: usize,
    length: usize,
    coefficients: &[f64],
    precision: Precision,
) -> Result<T, Error> {
    match precision {
        Precision::Accurate => {
            let mut sum: f64 = scalar::<T>(coefficients[0])?.into();
            for (order, &coefficient) in coefficients.iter().enumerate().skip(1) {
                let coefficient: f64 = scalar::<T>(coefficient)?.into();
                let product =
                    coefficient * (2.0 * order as f64 * PI * index as f64 / length as f64).cos();
                if order % 2 == 0 {
                    sum += product;
                } else {
                    sum -= product;
                }
            }
            scalar(sum)
        }
        Precision::Approximate => {
            let mut sum = scalar::<T>(coefficients[0])?;
            for (order, &coefficient) in coefficients.iter().enumerate().skip(1) {
                let angle = 2.0 * order as f64 * PI * index as f64 / length as f64;
                let cosine = scalar::<T>(approximation::cosine(angle as f32)? as f64)?;
                let product = scalar::<T>(coefficient)? * cosine;
                sum = if order % 2 == 0 {
                    sum + product
                } else {
                    sum - product
                };
            }
            Ok(sum)
        }
    }
}
