//! Scalar time-varying Kalman filtering and backward smoothing.
use crate::{Error, approximation, filtering::zeros};
use rustfft::FftNum;

#[derive(Debug, Clone, PartialEq)]
pub struct Estimate<T> {
    pub means: Vec<T>,
    pub variances: Vec<T>,
    pub log_likelihood: T,
}

fn validate<T: FftNum + Into<f64>>(values: &[T], variances: &[&[T]]) -> Result<(), Error> {
    if values.len() > i32::MAX as usize
        || values.iter().any(|value| !(*value).into().is_finite())
        || variances.iter().any(|variance| {
            variance.len() != values.len()
                || variance.iter().any(|value| {
                    let value = (*value).into();
                    !value.is_finite() || value < 0.0
                })
        })
    {
        return Err(Error(
            "Kalman values require matching finite arrays and nonnegative variances",
        ));
    }
    Ok(())
}

/// Use the first measurement as the initial estimate, as in kalmanf1d.
/// Empty arrays return an empty estimate with zero log likelihood.
pub fn filter<T: FftNum + Into<f64>>(
    measurements: &[T],
    process_variances: &[T],
    measurement_variances: &[T],
) -> Result<Estimate<T>, Error> {
    filter_with_initial(
        measurements,
        process_variances,
        measurement_variances,
        measurements.first().copied().unwrap_or_else(T::zero),
    )
}

/// Filter with an explicit initial mean, as in cig_kalmanf1d. The initial
/// predicted variance is process_variances[0]; later steps add process noise.
/// Likelihood retains the source's f32 approximate logarithm and f64 expression
/// evaluation, rounding the running sum into T after every measurement.
pub fn filter_with_initial<T: FftNum + Into<f64>>(
    measurements: &[T],
    process_variances: &[T],
    measurement_variances: &[T],
    initial_mean: T,
) -> Result<Estimate<T>, Error> {
    validate(measurements, &[process_variances, measurement_variances])?;
    if !initial_mean.into().is_finite() {
        return Err(Error("Kalman initial mean must be finite"));
    }
    let mut means = zeros(measurements.len())?;
    let mut variances = zeros(measurements.len())?;
    let mut likelihood = T::zero();
    let mut predicted_mean = initial_mean;
    for index in 0..measurements.len() {
        let predicted_variance = if index == 0 {
            process_variances[0]
        } else {
            variances[index - 1] + process_variances[index]
        };
        let error = measurements[index] - predicted_mean;
        let innovation_variance = predicted_variance + measurement_variances[index];
        if !error.into().is_finite()
            || !innovation_variance.into().is_finite()
            || innovation_variance.into() <= 0.0
        {
            return Err(Error("Kalman innovation requires finite positive variance"));
        }
        let gain = predicted_variance / innovation_variance;
        let mean = predicted_mean + gain * error;
        let variance = predicted_variance - gain * predicted_variance;
        let log_argument = std::f64::consts::TAU * innovation_variance.into();
        let narrow = log_argument as f32;
        // The original double build still calls a float approximation. Outside
        // its positive finite domain, use a wide logarithm to avoid the source
        // bit approximation's nonsensical zero/infinity results.
        let logarithm = if narrow.is_finite() && narrow > 0.0 {
            f64::from(approximation::logarithm(narrow)?)
        } else {
            std::f64::consts::TAU.ln() + innovation_variance.into().ln()
        };
        let error = error.into();
        let increment = -0.5 * logarithm - 0.5 * error * error / innovation_variance.into();
        let total = likelihood.into() + increment;
        likelihood = T::from_f64(total).ok_or(Error("Kalman likelihood conversion failed"))?;
        if !mean.into().is_finite()
            || !variance.into().is_finite()
            || !likelihood.into().is_finite()
        {
            return Err(Error("Kalman estimate or likelihood overflows"));
        }
        means[index] = mean;
        variances[index] = variance;
        predicted_mean = mean;
    }
    Ok(Estimate {
        means,
        variances,
        log_likelihood: likelihood,
    })
}

/// Smooth filtered means backward using their posterior and process variances.
/// Zero posterior/process uncertainty has zero backward gain rather than 0/0.
pub fn smooth<T: FftNum + Into<f64>>(
    means: &[T],
    posterior_variances: &[T],
    process_variances: &[T],
) -> Result<Vec<T>, Error> {
    validate(means, &[posterior_variances, process_variances])?;
    let mut output = zeros(means.len())?;
    let Some(&last) = means.last() else {
        return Ok(output);
    };
    output[means.len() - 1] = last;
    for next in (1..means.len()).rev() {
        let variance = posterior_variances[next - 1] + process_variances[next];
        if !variance.into().is_finite() {
            return Err(Error("Kalman smoothing variance overflows"));
        }
        let gain = if variance == T::zero() {
            T::zero()
        } else {
            posterior_variances[next - 1] / variance
        };
        let value = means[next - 1] + gain * (output[next] - means[next - 1]);
        if !value.into().is_finite() {
            return Err(Error("Kalman smoothed mean overflows"));
        }
        output[next - 1] = value;
    }
    Ok(output)
}
