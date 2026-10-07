//! Aliased DTFT sinc and its angular-frequency derivative.
use crate::{Error, statistics::Scalar, vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Evaluate removable integer-length poles and small-angle derivatives.
    Stable,
    /// Keep the source's separate 1e-4 and 1e-5 denominator plateaus.
    Legacy,
}

fn local(length: f64, angle: f64) -> Option<(f64, f64)> {
    if angle.abs() < 2e-4 {
        return Some((angle, 1.0));
    }
    if length.fract() != 0.0 {
        return None;
    }
    let turns = (angle / std::f64::consts::TAU).round();
    if turns.abs() > 4_503_599_627_370_496.0 {
        return None;
    }
    let reduced = angle - turns * std::f64::consts::TAU;
    if reduced.abs() >= 2e-4 {
        return None;
    }
    let sign = if length % 2.0 == 0.0 && (turns % 2.0).abs() == 1.0 {
        -1.0
    } else {
        1.0
    };
    Some((reduced, sign))
}
fn normalized_small(value: f64) -> f64 {
    let square = value * value;
    1.0 + square
        * (-1.0 / 6.0 + square * (1.0 / 120.0 + square * (-1.0 / 5040.0 + square / 362880.0)))
}
fn stable(length: f64, angle: f64, derivative: bool) -> f64 {
    if length == 0.0 || length.abs() == 1.0 {
        return if derivative { 0.0 } else { length };
    }
    let (angle, sign) = local(length, angle).unwrap_or((angle, 1.0));
    let product = length * angle;
    let value = if product.abs() <= 1e-3 && angle.abs() <= 1e-3 {
        if derivative {
            let p2 = product * product;
            let a2 = angle * angle;
            let correction = 1.0 - (3.0 * p2 - 7.0 * a2) / 120.0
                + (3.0 * p2 * p2 - 18.0 * p2 * a2 + 31.0 * a2 * a2) / 13440.0;
            (-length / 12.0) * (length * product - angle) * correction
        } else {
            length * (normalized_small(product * 0.5) / normalized_small(angle * 0.5))
        }
    } else {
        let numerator = (product * 0.5).sin();
        let denominator = (angle * 0.5).sin();
        if derivative {
            0.5 * (length * (product * 0.5).cos() - (angle * 0.5).cos() / denominator * numerator)
                / denominator
        } else {
            numerator / denominator
        }
    };
    sign * value
}

pub fn aliased<T: Scalar>(length: T, angle: T) -> Result<T, Error> {
    aliased_with(length, angle, Policy::Stable)
}
pub fn aliased_with<T: Scalar>(length: T, angle: T, policy: Policy) -> Result<T, Error> {
    vector::validate(&[length, angle])?;
    if policy == Policy::Stable {
        return vector::scalar(stable(length.into(), angle.into(), false));
    }
    let denominator: T = vector::scalar((angle.into() * 0.5).sin())?;
    if policy == Policy::Legacy && denominator.into().abs() < 1e-4 {
        return Ok(length);
    }
    let product = length * angle;
    vector::validate(&[product])?;
    vector::scalar((product.into() * 0.5).sin() / denominator.into())
}
pub fn derivative<T: Scalar>(length: T, angle: T) -> Result<T, Error> {
    derivative_with(length, angle, Policy::Stable)
}
pub fn derivative_with<T: Scalar>(length: T, angle: T, policy: Policy) -> Result<T, Error> {
    vector::validate(&[length, angle])?;
    if policy == Policy::Stable {
        return vector::scalar(stable(length.into(), angle.into(), true));
    }
    let two = T::from_f64(2.0).unwrap();
    let half = angle / two;
    let sinw: T = vector::scalar(half.into().sin())?;
    if policy == Policy::Legacy && sinw.into().abs() < 1e-5 {
        return Ok(T::zero());
    }
    let argument = length * angle / two;
    vector::validate(&[argument])?;
    let cosn: T = vector::scalar(argument.into().cos())?;
    let cosw: T = vector::scalar(half.into().cos())?;
    let sinn: T = vector::scalar(argument.into().sin())?;
    let difference = length * cosn - cosw / sinw * sinn;
    vector::scalar(0.5 * difference.into() / sinw.into())
}
