use crate::{Error, fourier::Complex, phase, statistics::Scalar};
use napi::Error as BindingError;
/// Complete spectrum mirroring choices.
#[derive(Clone, Copy, Debug)]
#[napi_derive::napi]
pub enum SpectrumSymmetry {
    Symmetric,
    Antisymmetric,
}
/// Native preserved Nyquist or original substituted-bin behavior.
#[derive(Clone, Copy, Debug)]
#[napi_derive::napi]
pub enum SpectrumNyquist {
    Preserve,
    Legacy,
}
fn components<T: Scalar>(real: &[T], imaginary: &[T]) -> Result<Vec<Complex<T>>, Error> {
    if real.len() != imaginary.len() {
        return Err(Error("complex component lengths differ"));
    }
    let mut values = Vec::new();
    values
        .try_reserve_exact(real.len())
        .map_err(|_| Error("complex input allocation failed"))?;
    values.extend(
        real.iter()
            .zip(imaginary)
            .map(|(&re, &im)| Complex::new(re, im)),
    );
    Ok(values)
}
/// Original orthonormal DCT-II, retaining approximate binary32 cosine.
pub fn dct_f32(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    crate::cosine::dct(values).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native phase wrapping, including source rounding at half turns.
pub fn phase_wrap_f32(angle: f32) -> Result<f32, BindingError> {
    phase::wrap(angle).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native single-turn unwrapping; returned arrays own their storage.
pub fn phase_unwrap_f32(angles: &[f32]) -> Result<Vec<f32>, BindingError> {
    phase::unwrap(angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Directed phase difference, preserving the original integer range.
pub fn phase_difference_f32(first: f32, second: f32) -> Result<f32, BindingError> {
    phase::difference(first, second).map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete complex magnitudes; scalar aliases use one-element planes.
pub fn magnitude_f32(real: &[f32], imaginary: &[f32]) -> Result<Vec<f32>, BindingError> {
    components(real, imaginary)
        .and_then(|values| phase::magnitude(&values))
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete complex arguments; scalar aliases use one-element planes.
pub fn argument_f32(real: &[f32], imaginary: &[f32]) -> Result<Vec<f32>, BindingError> {
    components(real, imaginary)
        .and_then(|values| phase::argument(&values))
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Polar real components with original approximate trigonometry.
pub fn polar_real_f32(magnitudes: &[f32], angles: &[f32]) -> Result<Vec<f32>, BindingError> {
    phase::polar_real(magnitudes, angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Polar imaginary components with original approximate trigonometry.
pub fn polar_imaginary_f32(magnitudes: &[f32], angles: &[f32]) -> Result<Vec<f32>, BindingError> {
    phase::polar_imaginary(magnitudes, angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Copy and complete the spectrum. Omitted policy preserves Nyquist;
/// Legacy retains the original even-bin substitution. Input is unchanged.
pub fn complete_spectrum_f32(
    mut values: Vec<f32>,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> Result<Vec<f32>, BindingError> {
    let symmetry = match symmetry {
        SpectrumSymmetry::Symmetric => phase::Symmetry::Symmetric,
        SpectrumSymmetry::Antisymmetric => phase::Symmetry::Antisymmetric,
    };
    let policy = match policy.unwrap_or(SpectrumNyquist::Preserve) {
        SpectrumNyquist::Preserve => phase::NyquistPolicy::Preserve,
        SpectrumNyquist::Legacy => phase::NyquistPolicy::Legacy,
    };
    phase::complete_spectrum_with(&mut values, symmetry, policy)
        .map_err(|error| napi::Error::from_reason(error.0))?;
    Ok(values)
}
/// Original orthonormal DCT-II, retaining approximate binary32 cosine.
pub fn dct_f64(values: &[f64]) -> Result<Vec<f64>, BindingError> {
    crate::cosine::dct(values).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native phase wrapping, including source rounding at half turns.
pub fn phase_wrap_f64(angle: f64) -> Result<f64, BindingError> {
    phase::wrap(angle).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native single-turn unwrapping; returned arrays own their storage.
pub fn phase_unwrap_f64(angles: &[f64]) -> Result<Vec<f64>, BindingError> {
    phase::unwrap(angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Directed phase difference, preserving the original integer range.
pub fn phase_difference_f64(first: f64, second: f64) -> Result<f64, BindingError> {
    phase::difference(first, second).map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete complex magnitudes; scalar aliases use one-element planes.
pub fn magnitude_f64(real: &[f64], imaginary: &[f64]) -> Result<Vec<f64>, BindingError> {
    components(real, imaginary)
        .and_then(|values| phase::magnitude(&values))
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete complex arguments; scalar aliases use one-element planes.
pub fn argument_f64(real: &[f64], imaginary: &[f64]) -> Result<Vec<f64>, BindingError> {
    components(real, imaginary)
        .and_then(|values| phase::argument(&values))
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Polar real components with original approximate trigonometry.
pub fn polar_real_f64(magnitudes: &[f64], angles: &[f64]) -> Result<Vec<f64>, BindingError> {
    phase::polar_real(magnitudes, angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Polar imaginary components with original approximate trigonometry.
pub fn polar_imaginary_f64(magnitudes: &[f64], angles: &[f64]) -> Result<Vec<f64>, BindingError> {
    phase::polar_imaginary(magnitudes, angles).map_err(|error| napi::Error::from_reason(error.0))
}
/// Copy and complete the spectrum. Omitted policy preserves Nyquist;
/// Legacy retains the original even-bin substitution. Input is unchanged.
pub fn complete_spectrum_f64(
    mut values: Vec<f64>,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> Result<Vec<f64>, BindingError> {
    let symmetry = match symmetry {
        SpectrumSymmetry::Symmetric => phase::Symmetry::Symmetric,
        SpectrumSymmetry::Antisymmetric => phase::Symmetry::Antisymmetric,
    };
    let policy = match policy.unwrap_or(SpectrumNyquist::Preserve) {
        SpectrumNyquist::Preserve => phase::NyquistPolicy::Preserve,
        SpectrumNyquist::Legacy => phase::NyquistPolicy::Legacy,
    };
    phase::complete_spectrum_with(&mut values, symmetry, policy)
        .map_err(|error| napi::Error::from_reason(error.0))?;
    Ok(values)
}
