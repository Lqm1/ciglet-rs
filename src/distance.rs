//! Itakura-Saito mean divergence and source-compatible log-distance output.
use crate::{Error, approximation, statistics::Scalar, vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    Stable,
    /// Preserve source selected-scalar accumulation and bitwise approximate logs.
    Legacy,
}
fn validate<T: Scalar>(source: &[T], reference: &[T]) -> Result<(), Error> {
    if source.is_empty() || source.len() != reference.len() {
        return Err(Error("spectral distance requires matching nonempty arrays"));
    }
    vector::validate(source)?;
    vector::validate(reference)?;
    if source
        .iter()
        .chain(reference)
        .any(|&value| value < T::zero())
    {
        return Err(Error("spectral distance requires nonnegative spectra"));
    }
    Ok(())
}
fn bin_log(source: f64, reference: f64) -> f64 {
    if source == reference {
        return f64::NEG_INFINITY;
    }
    let difference = source - reference;
    if difference.abs() <= reference * 0.5 {
        let delta = difference / reference;
        // delta - ln(1 + delta), factored to retain adjacent-value distances.
        let mut coefficient = 0.5;
        let mut power = -delta;
        for denominator in 3..=64 {
            let next = coefficient + power / f64::from(denominator);
            if next == coefficient {
                break;
            }
            coefficient = next;
            power *= -delta;
        }
        return (delta * delta * coefficient).ln();
    }
    let ratio = source / reference;
    if ratio.is_finite() && ratio > 0.0 {
        return (ratio - ratio.ln() - 1.0).ln();
    }
    let log_ratio = source.ln() - reference.ln();
    if ratio == 0.0 {
        (-log_ratio - 1.0).ln()
    } else {
        log_ratio + (-(log_ratio + 1.0) * (-log_ratio).exp()).ln_1p()
    }
}
fn stable_log<T: Scalar>(source: &[T], reference: &[T]) -> Result<f64, Error> {
    if source
        .iter()
        .zip(reference)
        .any(|(&a, &b)| a == T::zero() && b == T::zero())
    {
        return Err(Error(
            "spectral distance is undefined for a paired zero bin",
        ));
    }
    if source
        .iter()
        .chain(reference)
        .any(|&value| value == T::zero())
    {
        return Ok(f64::INFINITY);
    }
    let mut maximum = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut compensation = 0.0;
    for (&a, &b) in source.iter().zip(reference) {
        let value = bin_log(a.into(), b.into());
        if value == f64::NEG_INFINITY {
            continue;
        }
        if !value.is_finite() {
            return Err(Error("spectral distance calculation failed"));
        }
        if value > maximum {
            let scale = (maximum - value).exp();
            sum *= scale;
            compensation *= scale;
            maximum = value;
        }
        let increment = (value - maximum).exp() - compensation;
        let next = sum + increment;
        compensation = (next - sum) - increment;
        sum = next;
    }
    Ok(if maximum == f64::NEG_INFINITY {
        maximum
    } else {
        maximum + (sum / source.len() as f64).ln()
    })
}
/// Mean divergence. Identical positive spectra return zero; a single zero
/// against a positive bin gives infinity. Paired zeros are undefined.
/// A finite mean exceeding the selected scalar range also returns infinity.
pub fn itakura_saito<T: Scalar>(source: &[T], reference: &[T]) -> Result<T, Error> {
    validate(source, reference)?;
    Ok(T::from_f64(stable_log(source, reference)?.exp()).unwrap())
}
/// Natural log of mean divergence, retaining finite extreme-ratio results.
/// Identical positive spectra return negative infinity.
pub fn log_itakura_saito<T: Scalar>(source: &[T], reference: &[T]) -> Result<T, Error> {
    log_itakura_saito_with(source, reference, Policy::Stable)
}
pub fn log_itakura_saito_with<T: Scalar>(
    source: &[T],
    reference: &[T],
    policy: Policy,
) -> Result<T, Error> {
    validate(source, reference)?;
    if policy == Policy::Stable {
        return Ok(T::from_f64(stable_log(source, reference)?).unwrap());
    }
    let mut sum = T::zero();
    for (&a, &b) in source.iter().zip(reference) {
        let ratio = a / b;
        let log = T::from_f32(approximation::logarithm_legacy_bits(ratio.into() as f32)).unwrap();
        let difference = ratio - log;
        let term = T::from_f64(difference.into() - 1.0).unwrap();
        sum = sum + term;
    }
    let mean = sum / T::from_usize(source.len()).unwrap();
    Ok(T::from_f32(approximation::logarithm_legacy_bits(mean.into() as f32)).unwrap())
}
