//! Centered moments with shared compensated binary-exponent accumulation.
use super::Scalar;
use crate::{
    Error,
    accumulation::{Scaled, Sum, scale},
    vector,
};

/// Two-part subtraction retains a small mean subtracted from a huge sample.
fn difference(left: f64, right: f64) -> [Scaled; 2] {
    let mut left = left;
    let mut right = right;
    let mut exponent = 0;
    if !(left - right).is_finite() {
        left *= 0.5;
        right *= 0.5;
        exponent = 1;
    }
    let high = left - right;
    let virtual_right = left - high;
    let virtual_left = high + virtual_right;
    let low = (left - virtual_left) + (virtual_right - right);
    [Scaled::new(high, exponent), Scaled::new(low, exponent)]
}

pub(super) fn validate<T: Scalar>(values: &[T]) -> Result<(), Error> {
    if values.is_empty() {
        return Err(Error("statistics require at least one value"));
    }
    vector::validate(values)
}

pub(super) fn mean<T: Scalar>(values: &[T]) -> f64 {
    let mut sum = Sum::new();
    for &value in values {
        sum.scalar(value.into(), 0);
    }
    sum.finish(values.len() as f64).value()
}

fn residual<T: Scalar>(values: &[T], mean: f64) -> Scaled {
    let mut sum = Sum::new();
    for &value in values {
        for part in difference(value.into(), mean) {
            sum.coefficient(part.fraction, part.exponent);
        }
    }
    sum.finish(1.0)
}

pub(super) fn product<T: Scalar>(left: &[T], right: &[T]) -> Scaled {
    let left_mean = mean(left);
    let right_mean = mean(right);
    let mut sum = Sum::new();
    for (&left, &right) in left.iter().zip(right) {
        for left in difference(left.into(), left_mean) {
            for right in difference(right.into(), right_mean) {
                sum.product(left, right);
            }
        }
    }
    // Correct the small residual means of the rounded centering values.
    let left_residual = residual(left, left_mean).average(left.len());
    let mut right_residual = residual(right, right_mean);
    right_residual.fraction = -right_residual.fraction;
    sum.product(left_residual, right_residual);
    sum.finish(left.len() as f64)
}

pub(super) fn correlation(covariance: Scaled, left: Scaled, right: Scaled) -> Result<f64, Error> {
    if left.fraction <= 0.0 || right.fraction <= 0.0 {
        return Err(Error("correlation requires nonconstant vectors"));
    }
    let exponent = left.exponent + right.exponent;
    let divisor = left.fraction.sqrt()
        * right.fraction.sqrt()
        * if exponent.rem_euclid(2) == 0 {
            1.0
        } else {
            std::f64::consts::SQRT_2
        };
    Ok(scale(
        covariance.fraction / divisor,
        covariance.exponent - exponent.div_euclid(2),
    )
    .clamp(-1.0, 1.0))
}
