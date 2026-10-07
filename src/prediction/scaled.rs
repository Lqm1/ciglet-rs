//! Correlations whose powers can exceed the selected scalar's exponent range.
use super::{Prediction, levinson};
use crate::{
    Error,
    accumulation::{Scaled, Sum},
    statistics::Scalar,
    vector,
};

/// Prediction coefficients and individually scaled autocorrelation lags.
/// Each nonzero lag is represented as significand * 2^exponent. Significands
/// have absolute value in [1, 2); zero is represented by (0, 0).
#[derive(Debug, Clone)]
pub struct ScaledPrediction<T> {
    coefficients: Vec<T>,
    autocorrelation: Vec<Scaled>,
}

impl<T: Scalar> ScaledPrediction<T> {
    pub(crate) fn new(autocorrelation: Vec<Scaled>) -> Result<Self, Error> {
        let normalized = normalized(&autocorrelation)?;
        let mut coefficients = vector::buffer(normalized.len())?;
        for value in levinson(&normalized)? {
            coefficients.push(vector::scalar(value)?);
        }
        Ok(Self {
            coefficients,
            autocorrelation,
        })
    }

    pub fn coefficients(&self) -> &[T] {
        &self.coefficients
    }

    pub fn autocorrelation_parts(&self) -> impl ExactSizeIterator<Item = (f64, i32)> + '_ {
        self.autocorrelation
            .iter()
            .map(|value| (value.fraction, value.exponent))
    }

    /// Return ordinary correlations only if every nonzero lag survives conversion.
    pub fn into_prediction(self) -> Result<Prediction<T>, Error> {
        let mut autocorrelation = vector::buffer(self.autocorrelation.len())?;
        for value in self.autocorrelation {
            autocorrelation.push(convert(value)?);
        }
        Ok(Prediction {
            coefficients: self.coefficients,
            autocorrelation,
        })
    }

    /// Return the source gain without imposing a scalar exponent range.
    pub fn gain_parts(&self) -> Result<(f64, i32), Error> {
        let normalized = normalized(&self.autocorrelation)?;
        let mut sum = Sum::new();
        for (&coefficient, &correlation) in self.coefficients.iter().zip(&normalized) {
            sum.product(
                Scaled::new(coefficient.into(), 0),
                Scaled::new(correlation, 0),
            );
        }
        let result = sum.finish(1.0);
        if result.fraction < 0.0 {
            return Err(Error("prediction residual energy must be nonnegative"));
        }
        let energy = self.autocorrelation[0];
        let result = Scaled::new(
            result.fraction * energy.fraction,
            result.exponent + energy.exponent,
        );
        let root = Scaled::new(
            (result.fraction
                * if result.exponent.rem_euclid(2) == 0 {
                    1.0
                } else {
                    2.0
                })
            .sqrt(),
            result.exponent.div_euclid(2),
        );
        Ok((root.fraction, root.exponent))
    }

    /// A nonzero gain outside the selected scalar range returns an error.
    pub fn gain(&self) -> Result<T, Error> {
        let (fraction, exponent) = self.gain_parts()?;
        convert(Scaled::new(fraction, exponent))
    }
}

fn convert<T: Scalar>(value: Scaled) -> Result<T, Error> {
    let result = vector::scalar(value.value())?;
    if result == T::zero() && value.fraction != 0.0 {
        return Err(Error(
            "prediction value underflows; retain the scaled representation",
        ));
    }
    Ok(result)
}

fn normalized(values: &[Scaled]) -> Result<Vec<f64>, Error> {
    let Some(&energy) = values.first() else {
        return Err(Error(
            "prediction requires at least one autocorrelation lag",
        ));
    };
    if energy.fraction < 0.0 {
        return Err(Error("autocorrelation energy must be nonnegative"));
    }
    let mut output = vector::buffer(values.len())?;
    for &value in values {
        output.push(if energy.fraction == 0.0 {
            if value.fraction != 0.0 {
                return Err(Error("zero energy requires zero autocorrelation"));
            }
            0.0
        } else {
            value.divided_by(energy).value()
        });
    }
    Ok(output)
}

pub(super) fn waveform<T: Scalar>(
    samples: &[T],
    order: usize,
) -> Result<ScaledPrediction<T>, Error> {
    vector::validate(samples)?;
    let count = order
        .checked_add(1)
        .filter(|&count| count <= samples.len())
        .ok_or(Error("prediction order must be smaller than sample count"))?;
    let mut correlations = vector::buffer(count)?;
    for lag in 0..count {
        let mut sum = Sum::new();
        for (&left, &right) in samples.iter().zip(&samples[lag..]) {
            sum.product(Scaled::new(left.into(), 0), Scaled::new(right.into(), 0));
        }
        correlations.push(sum.finish(1.0));
    }
    ScaledPrediction::new(correlations)
}
