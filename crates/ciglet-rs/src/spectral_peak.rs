//! Quadratic interpolation of a three-bin magnitude-spectrum peak.
use crate::{Error, statistics::Scalar, vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Peak<T> {
    /// Fractional FFT-bin position, without conversion to hertz.
    pub bin: T,
    /// Interpolated value capped at the center value plus 0.2.
    pub magnitude: T,
}

/// Interpolate an interior bin using the upstream mixed-precision polynomial.
/// A flat or displaced parabola retains the original bin and magnitude.
/// Values may be signed, as when the caller supplies log magnitudes.
pub fn interpolate<T: Scalar>(magnitudes: &[T], bin: usize) -> Result<Peak<T>, Error> {
    if bin == 0 || bin >= magnitudes.len().saturating_sub(1) {
        return Err(Error("peak interpolation requires both neighboring bins"));
    }
    vector::validate(&magnitudes[bin - 1..=bin + 1])?;
    if bin > i32::MAX as usize {
        return Err(Error("peak bin exceeds the supported range"));
    }
    let a = magnitudes[bin - 1];
    let b = magnitudes[bin];
    let c = magnitudes[bin + 1];
    // C rounds the sum in the selected scalar before promotion by 2.0.
    let quadratic: T = vector::scalar((a + c).into() / 2.0 - b.into())?;
    let linear = c - b - quadratic;
    vector::validate(&[linear])?;
    let offset: T = if quadratic == T::zero() {
        T::zero()
    } else {
        let ratio = -linear / quadratic;
        // An infinite ratio is outside the permitted one-bin displacement.
        let candidate = ratio.into() * 0.5;
        if candidate.abs() < 1.0 {
            vector::scalar(candidate)?
        } else {
            T::zero()
        }
    };
    let bin = T::from_usize(bin).unwrap() + offset;
    let value = quadratic * offset * offset + linear * offset + b;
    vector::validate(&[bin, value])?;
    let magnitude = if value.into() > b.into() + 0.2 {
        vector::scalar(b.into() + 0.2)?
    } else {
        value
    };
    Ok(Peak { bin, magnitude })
}
