use crate::approximation::lanes;
use pyo3::PyErr as BindingError;
fn four<T: Copy>(values: &[T]) -> Result<[T; 4], BindingError> {
    values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("input must contain exactly four lanes")
    })
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_exponential2(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::exponential2(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_exponential2_fast(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::exponential2_fast(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_exponential(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::exponential(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_exponential_fast(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::exponential_fast(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_logarithm2(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::logarithm2(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_logarithm2_fast(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::logarithm2_fast(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_logarithm(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::logarithm(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned native four-lane approximation.
pub fn lanes_logarithm_fast(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::logarithm_fast(four(values)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Retain a separate exponent for each of the four native lanes.
pub fn lanes_power(bases: &[f32], exponents: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::power(four(bases)?, four(exponents)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Retain a separate exponent for each of the four native lanes.
pub fn lanes_power_fast(bases: &[f32], exponents: &[f32]) -> Result<Vec<f32>, BindingError> {
    lanes::power_fast(four(bases)?, four(exponents)?)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn lanes_to_integers(values: &[f32]) -> Result<Vec<i32>, BindingError> {
    Ok(lanes::to_integers(four(values)?).to_vec())
}
pub fn lanes_from_integers(values: &[i32]) -> Result<Vec<f32>, BindingError> {
    Ok(lanes::from_integers(four(values)?).to_vec())
}
pub fn lanes_to_bits(values: &[f32]) -> Result<Vec<u32>, BindingError> {
    Ok(lanes::to_bits(four(values)?).to_vec())
}
pub fn lanes_from_bits(values: &[u32]) -> Result<Vec<f32>, BindingError> {
    Ok(lanes::from_bits(four(values)?).to_vec())
}
pub fn lanes_absolute(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    Ok(lanes::absolute(four(values)?).to_vec())
}
/// Each signed 64-bit BigInt contributes its low word followed by its high word.
pub fn lanes_from_integer64(first: i64, second: i64) -> Vec<i32> {
    lanes::from_integer64_lanes([first, second]).to_vec()
}
