//! Phase and Cartesian/polar helpers with the source's precision conventions.
pub use crate::stft::NyquistPolicy;
use crate::{Error, approximation, fourier::Complex, statistics::Scalar, vector};
use std::f64::consts::PI;

pub fn wrap<T: Scalar>(angle: T) -> Result<T, Error> {
    vector::validate(&[angle])?;
    let x = angle.into();
    vector::scalar(x - (x / 2.0 / PI).round() * 2.0 * PI)
}
/// Preserve source's single-turn correction for every jump beyond pi.
pub fn unwrap<T: Scalar>(angles: &[T]) -> Result<Vec<T>, Error> {
    vector::validate(angles)?;
    let mut output = vector::buffer(angles.len())?;
    if let Some(&first) = angles.first() {
        output.push(first);
    }
    for (i, pair) in angles.windows(2).enumerate() {
        let accumulated = output[i] + pair[1];
        let value = if (pair[1] - pair[0]).into().abs() > PI {
            let direction = if pair[1] > pair[0] { 1.0 } else { -1.0 };
            vector::scalar(accumulated.into() - (pair[0].into() + 2.0 * PI * direction))?
        } else {
            accumulated - pair[0]
        };
        output.push(value);
    }
    vector::validate(&output)?;
    Ok(output)
}
/// Source directed circular difference, retaining shift rounding and [-pi, pi).
pub fn difference<T: Scalar>(mut first: T, second: T) -> Result<T, Error> {
    vector::validate(&[first, second])?;
    if second > first {
        let turns = (second - first).into() / 2.0 / PI + 1.0;
        if !turns.is_finite() || turns >= 2_147_483_648.0 {
            return Err(Error("phase shift exceeds the source integer range"));
        }
        first = vector::scalar(first.into() + turns.trunc() * 2.0 * PI)?;
    }
    vector::scalar(((first - second).into() + PI) % (PI * 2.0) - PI)
}
/// Source square/sum/sqrt grouping, with hypot fallback for finite range loss.
pub fn magnitude_of<T: Scalar>(value: Complex<T>) -> Result<T, Error> {
    vector::validate(&[value.re, value.im])?;
    let square = value.re * value.re + value.im * value.im;
    let result = if !square.into().is_finite()
        || (square == T::zero() && (value.re != T::zero() || value.im != T::zero()))
    {
        value.re.into().hypot(value.im.into())
    } else {
        square.into().sqrt()
    };
    vector::scalar(result)
}
pub fn argument_of<T: Scalar>(value: Complex<T>) -> Result<T, Error> {
    vector::validate(&[value.re, value.im])?;
    vector::scalar(value.im.into().atan2(value.re.into()))
}
pub fn magnitude<T: Scalar>(values: &[Complex<T>]) -> Result<Vec<T>, Error> {
    let mut output = vector::buffer(values.len())?;
    for &value in values {
        output.push(magnitude_of(value)?);
    }
    Ok(output)
}
pub fn argument<T: Scalar>(values: &[Complex<T>]) -> Result<Vec<T>, Error> {
    let mut output = vector::buffer(values.len())?;
    for &value in values {
        output.push(argument_of(value)?);
    }
    Ok(output)
}
fn polar_component<T: Scalar>(
    magnitudes: &[T],
    angles: &[T],
    trig: fn(f32) -> Result<f32, Error>,
) -> Result<Vec<T>, Error> {
    if magnitudes.len() != angles.len() {
        return Err(Error("polar arrays have different lengths"));
    }
    vector::validate(magnitudes)?;
    vector::validate(angles)?;
    let mut output = vector::buffer(angles.len())?;
    for (&magnitude, &angle) in magnitudes.iter().zip(angles) {
        output.push(magnitude * T::from_f32(trig(angle.into() as f32)?).unwrap());
    }
    vector::validate(&output)?;
    Ok(output)
}
pub fn polar_real<T: Scalar>(magnitudes: &[T], angles: &[T]) -> Result<Vec<T>, Error> {
    polar_component(magnitudes, angles, approximation::cosine)
}
pub fn polar_imaginary<T: Scalar>(magnitudes: &[T], angles: &[T]) -> Result<Vec<T>, Error> {
    polar_component(magnitudes, angles, approximation::sine)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symmetry {
    Symmetric,
    Antisymmetric,
}

/// Mirror the upper spectrum, retaining supplied DC/Nyquist values.
pub fn complete_spectrum<T: Scalar>(values: &mut [T], symmetry: Symmetry) -> Result<(), Error> {
    complete_spectrum_with(values, symmetry, NyquistPolicy::Preserve)
}
/// Legacy substitutes the preceding value for an even-length Nyquist bin,
/// including its original positive substitution for antisymmetric completion.
pub fn complete_spectrum_with<T: Scalar>(
    values: &mut [T],
    symmetry: Symmetry,
    policy: NyquistPolicy,
) -> Result<(), Error> {
    if values.is_empty() {
        return Ok(());
    }
    let n = values.len();
    if n > i32::MAX as usize {
        return Err(Error("spectrum length exceeds the supported range"));
    }
    let legacy = policy == NyquistPolicy::Legacy && n.is_multiple_of(2);
    vector::validate(&values[..if legacy { n / 2 } else { n / 2 + 1 }])?;
    if legacy {
        values[n / 2] = values[n / 2 - 1];
    }
    for i in n / 2 + 1..n {
        values[i] = match symmetry {
            Symmetry::Symmetric => values[n - i],
            Symmetry::Antisymmetric => -values[n - i],
        };
    }
    Ok(())
}
