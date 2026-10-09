//! Complex arithmetic using the RustFFT re-export of num-complex.
pub use crate::approximation::Precision;
pub use crate::fourier::Complex;
use crate::{Error, approximation, statistics::Scalar, vector};

/// Source division order, with scaled fallback for intermediate range loss.
pub fn divide<T: Scalar>(numerator: Complex<T>, divisor: Complex<T>) -> Result<Complex<T>, Error> {
    vector::validate(&[numerator.re, numerator.im, divisor.re, divisor.im])?;
    if divisor.re == T::zero() && divisor.im == T::zero() {
        return Err(Error("complex division requires a nonzero divisor"));
    }
    let denominator = divisor.re * divisor.re + divisor.im * divisor.im;
    let products = [
        numerator.re * divisor.re,
        numerator.im * divisor.im,
        numerator.im * divisor.re,
        numerator.re * divisor.im,
    ];
    let factors = [
        (numerator.re, divisor.re),
        (numerator.im, divisor.im),
        (numerator.im, divisor.re),
        (numerator.re, divisor.im),
    ];
    let underflow = products
        .iter()
        .zip(factors)
        .any(|(&product, (a, b))| product == T::zero() && a != T::zero() && b != T::zero());
    let ordinary = Complex::new(
        (products[0] + products[1]) / denominator,
        (products[2] - products[3]) / denominator,
    );
    if denominator > T::zero()
        && denominator.into().is_finite()
        && !underflow
        && ordinary.re.into().is_finite()
        && ordinary.im.into().is_finite()
    {
        return Ok(ordinary);
    }
    let scale = divisor.re.into().abs().max(divisor.im.into().abs());
    let real = divisor.re.into() / scale;
    let imaginary = divisor.im.into() / scale;
    let denominator = real * real + imaginary * imaginary;
    let scaled = |value: T| {
        let quotient = value.into() / scale;
        if quotient.is_finite() {
            quotient / denominator
        } else {
            (value.into() / denominator) / scale
        }
    };
    let a = scaled(numerator.re);
    let b = scaled(numerator.im);
    Ok(Complex::new(
        vector::scalar(a * real + b * imaginary)?,
        vector::scalar(b * real - a * imaginary)?,
    ))
}

/// Complex exponential retaining the source magnitude and trig rounding stages.
pub fn exponential<T: Scalar>(
    value: Complex<T>,
    precision: Precision,
) -> Result<Complex<T>, Error> {
    vector::validate(&[value.re, value.im])?;
    let magnitude: T = approximation::exponential_with(value.re, precision)?;
    let result = if precision == Precision::Exact {
        Complex::new(
            vector::scalar(magnitude.into() * value.im.into().cos())?,
            vector::scalar(magnitude.into() * value.im.into().sin())?,
        )
    } else {
        let angle = value.im.into() as f32;
        let (cosine, sine) = if precision == Precision::Fast {
            (
                approximation::cosine_fast(angle)?,
                approximation::sine_fast(angle)?,
            )
        } else {
            (approximation::cosine(angle)?, approximation::sine(angle)?)
        };
        Complex::new(
            magnitude * T::from_f32(cosine).unwrap(),
            magnitude * T::from_f32(sine).unwrap(),
        )
    };
    vector::validate(&[result.re, result.im])?;
    Ok(result)
}

pub fn argument<T: Scalar>(value: Complex<T>, precision: Precision) -> Result<T, Error> {
    approximation::atan2_with(value.im, value.re, precision)
}
