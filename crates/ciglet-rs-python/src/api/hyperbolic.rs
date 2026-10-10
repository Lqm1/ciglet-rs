use super::ApproximationPrecision;
use crate::approximation::{hyperbolic, lanes};
use pyo3::PyErr as BindingError;
/// Native binary32 hyperbolic formula with all three precision modes.
pub fn hyperbolic_sine(value: f32, precision: ApproximationPrecision) -> Result<f32, BindingError> {
    hyperbolic::sine(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete four-lane native formula; any invalid lane rejects the call.
pub fn hyperbolic_sine_lanes(
    values: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("hyperbolic input must contain exactly four lanes")
    })?;
    lanes::hyperbolic::sine(values, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 hyperbolic formula with all three precision modes.
pub fn hyperbolic_cosine(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    hyperbolic::cosine(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete four-lane native formula; any invalid lane rejects the call.
pub fn hyperbolic_cosine_lanes(
    values: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("hyperbolic input must contain exactly four lanes")
    })?;
    lanes::hyperbolic::cosine(values, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 hyperbolic formula with all three precision modes.
pub fn hyperbolic_tangent(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    hyperbolic::tangent(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete four-lane native formula; any invalid lane rejects the call.
pub fn hyperbolic_tangent_lanes(
    values: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("hyperbolic input must contain exactly four lanes")
    })?;
    lanes::hyperbolic::tangent(values, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 hyperbolic formula with all three precision modes.
pub fn hyperbolic_sigmoid(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    hyperbolic::sigmoid(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete four-lane native formula; any invalid lane rejects the call.
pub fn hyperbolic_sigmoid_lanes(
    values: &[f32],
    precision: ApproximationPrecision,
) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("hyperbolic input must contain exactly four lanes")
    })?;
    lanes::hyperbolic::sigmoid(values, precision.into())
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
