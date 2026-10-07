//! Autoregressive prediction with source-compatible Levinson recursion.
use crate::{
    Error,
    statistics::{self, Scalar},
    vector,
};

/// Owned prediction coefficients and the autocorrelation used to estimate them.
#[derive(Debug, Clone, PartialEq)]
pub struct Prediction<T> {
    pub coefficients: Vec<T>,
    pub autocorrelation: Vec<T>,
}

/// Estimate coefficients from nonnegative-lag autocorrelation.
/// The leading coefficient is one; the order is `autocorrelation.len() - 1`.
pub fn levinson<T: Scalar>(autocorrelation: &[T]) -> Result<Vec<T>, Error> {
    vector::validate(autocorrelation)?;
    let Some(&energy) = autocorrelation.first() else {
        return Err(Error(
            "prediction requires at least one autocorrelation lag",
        ));
    };
    if energy < T::zero() {
        return Err(Error("autocorrelation energy must be nonnegative"));
    }
    let n = autocorrelation.len();
    let mut coefficients = vector::buffer(n)?;
    coefficients.resize(n, T::zero());
    coefficients[0] = T::one();
    if n == 1 {
        return Ok(coefficients);
    }
    if energy == T::zero() {
        if autocorrelation.iter().any(|&value| value != T::zero()) {
            return Err(Error("zero energy requires zero autocorrelation"));
        }
        return Ok(coefficients);
    }
    let mut scratch = vector::buffer(n)?;
    scratch.resize(n, T::zero());
    let reflection: T = vector::scalar(-autocorrelation[1].into() / (energy.into() * 1.000001))?;
    coefficients[1] = reflection;
    let mut residual: T =
        vector::scalar(energy.into() * 1.000001 * (T::one() - reflection * reflection).into())?;
    if residual <= T::zero() {
        return Err(Error("autocorrelation is not positive definite"));
    }
    for order in 2..n {
        let mut sum = autocorrelation[order];
        for lag in 1..order {
            sum = sum + autocorrelation[lag] * coefficients[order - lag];
        }
        let reflection = -sum / residual;
        for lag in 1..order {
            scratch[lag] = coefficients[lag] + reflection * coefficients[order - lag];
        }
        coefficients[1..order].copy_from_slice(&scratch[1..order]);
        coefficients[order] = reflection;
        residual = vector::scalar(residual.into() * (1.0 - (reflection * reflection).into()))?;
        if residual <= T::zero() {
            return Err(Error("autocorrelation is not positive definite"));
        }
    }
    vector::validate(&coefficients)?;
    Ok(coefficients)
}

/// Unnormalized waveform autocorrelation followed by Levinson recursion.
pub fn from_samples<T: Scalar>(samples: &[T], order: usize) -> Result<Prediction<T>, Error> {
    vector::validate(samples)?;
    let count = order
        .checked_add(1)
        .filter(|&count| count <= samples.len())
        .ok_or(Error("prediction order must be smaller than sample count"))?;
    let autocorrelation = statistics::cross_correlation(samples, samples, count)?;
    let coefficients = levinson(&autocorrelation)?;
    Ok(Prediction {
        coefficients,
        autocorrelation,
    })
}

/// Source gain: square root of the sequential coefficient/autocorrelation dot.
pub fn gain<T: Scalar>(coefficients: &[T], autocorrelation: &[T]) -> Result<T, Error> {
    vector::validate(coefficients)?;
    vector::validate(autocorrelation)?;
    if coefficients.is_empty() || coefficients.len() != autocorrelation.len() {
        return Err(Error("prediction gain requires matching nonempty arrays"));
    }
    let energy = statistics::dot(coefficients, autocorrelation)?;
    if energy < T::zero() {
        return Err(Error("prediction residual energy must be nonnegative"));
    }
    vector::scalar(energy.into().sqrt())
}
