//! Upstream mel/Bark coordinates and equal-loudness weighting.
use crate::{Error, approximation};
use rustfft::FftNum;

fn scalar<T: FftNum + Into<f64>>(value: f64) -> Result<T, Error> {
    if !value.is_finite() {
        return Err(Error("frequency conversion is not finite"));
    }
    let result = T::from_f64(value).ok_or(Error("frequency value cannot be represented"))?;
    if !result.into().is_finite() {
        return Err(Error("frequency conversion exceeds scalar range"));
    }
    Ok(result)
}

pub fn frequency_to_mel<T: FftNum + Into<f64>>(frequency: T) -> Result<T, Error> {
    let logarithm = approximation::logarithm((1.0 + frequency.into() / 700.0) as f32)?;
    scalar(1125.0 * f64::from(logarithm))
}

pub fn mel_to_frequency<T: FftNum + Into<f64>>(mel: T) -> Result<T, Error> {
    let exponential = approximation::exponential((mel.into() / 1125.0) as f32)?;
    // C's subtraction is float-minus-double and therefore evaluated in f64.
    scalar(700.0 * (f64::from(exponential) - 1.0))
}

pub fn frequency_to_bark<T: FftNum + Into<f64>>(frequency: T) -> Result<T, Error> {
    scalar(6.0 * (frequency.into() / 600.0).asinh())
}
pub fn bark_to_frequency<T: FftNum + Into<f64>>(bark: T) -> Result<T, Error> {
    scalar(600.0 * (bark.into() / 6.0).sinh())
}
pub fn equal_loudness<T: FftNum + Into<f64>>(frequency: T) -> Result<T, Error> {
    let square = frequency * frequency;
    let fourth = square * square;
    let square: f64 = square.into();
    scalar(
        fourth.into() / (square + 1.6e5) / (square + 1.6e5) * (square + 1.44e6) / (square + 9.61e6),
    )
}

/// Returns intervals+1 mel-spaced points, retaining C's scalar interpolation
/// and approximate mel conversions rather than substituting exact ln/exp.
pub fn mel_space<T: FftNum + Into<f64>>(
    minimum: T,
    maximum: T,
    intervals: usize,
) -> Result<Vec<T>, Error> {
    if intervals == 0 || intervals >= i32::MAX as usize || minimum.into() > maximum.into() {
        return Err(Error(
            "mel spacing requires ordered bounds and positive bounded intervals",
        ));
    }
    let minimum = frequency_to_mel(minimum)?;
    let maximum = frequency_to_mel(maximum)?;
    let divisor =
        T::from_usize(intervals).ok_or(Error("mel interval count cannot be represented"))?;
    let mut points = Vec::new();
    points
        .try_reserve_exact(intervals + 1)
        .map_err(|_| Error("mel spacing allocation failed"))?;
    for index in 0..=intervals {
        let position = T::from_usize(index).ok_or(Error("mel index cannot be represented"))?;
        points.push(mel_to_frequency(
            position / divisor * (maximum - minimum) + minimum,
        )?);
    }
    Ok(points)
}
