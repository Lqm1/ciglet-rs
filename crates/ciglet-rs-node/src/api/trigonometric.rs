use super::ApproximationPrecision;
use crate::approximation::{lanes, trigonometric};
use napi::Error as BindingError;
/// Native binary32 polynomial or full-range tangent in every precision mode.
pub fn trigonometric_sine(
    angle: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    trigonometric::sine(angle, precision.into()).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native binary32 polynomial or full-range tangent in every precision mode.
pub fn trigonometric_cosine(
    angle: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    trigonometric::cosine(angle, precision.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native binary32 polynomial or full-range tangent in every precision mode.
pub fn trigonometric_tangent(
    angle: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    trigonometric::tangent(angle, precision.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native binary32 polynomial or full-range tangent in every precision mode.
pub fn trigonometric_tangent_full(
    angle: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    trigonometric::tangent_full(angle, precision.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_sine_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::sine(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_cosine_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::cosine(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_tangent_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::tangent(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_sine_full_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::sine_full(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_cosine_full_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::cosine_full(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete four-lane native expression, retaining vector-specific branches.
pub fn trigonometric_tangent_full_lanes(
    angles: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let angles: [f32; 4] = angles.try_into().map_err(|_| {
        napi::Error::from_reason("trigonometric input must contain exactly four lanes")
    })?;
    lanes::trigonometric::tangent_full(angles, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| napi::Error::from_reason(error.0))
}
