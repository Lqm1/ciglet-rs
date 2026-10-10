use crate::approximation::{self, Precision};
use pyo3::PyErr as BindingError;
/// All native scalar approximation precision levels.
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum ApproximationPrecision {
    Fast,
    Approximate,
    Exact,
}
impl From<ApproximationPrecision> for Precision {
    fn from(value: ApproximationPrecision) -> Self {
        match value {
            ApproximationPrecision::Fast => Self::Fast,
            ApproximationPrecision::Approximate => Self::Approximate,
            ApproximationPrecision::Exact => Self::Exact,
        }
    }
}
pub fn approximation_sine_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    approximation::sine_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_sine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> Result<f64, BindingError> {
    approximation::sine_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_cosine_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    approximation::cosine_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_cosine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> Result<f64, BindingError> {
    approximation::cosine_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    approximation::logarithm_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> Result<f64, BindingError> {
    approximation::logarithm_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    approximation::exponential_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> Result<f64, BindingError> {
    approximation::exponential_with(value, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_atan2_f32(
    y: f32,
    x: f32,
    precision: ApproximationPrecision,
) -> Result<f32, BindingError> {
    approximation::atan2_with(y, x, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_atan2_fast_f32(y: f32, x: f32) -> Result<f32, BindingError> {
    approximation::atan2_fast(y, x)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native strict comparison, retaining the second operand on ties and NaNs.
pub fn scalar_minimum_f32(first: f32, second: f32) -> f32 {
    crate::scalar::minimum(first, second)
}
/// Native strict comparison, retaining the second operand on ties and NaNs.
pub fn scalar_maximum_f32(first: f32, second: f32) -> f32 {
    crate::scalar::maximum(first, second)
}
pub fn approximation_atan2_f64(
    y: f64,
    x: f64,
    precision: ApproximationPrecision,
) -> Result<f64, BindingError> {
    approximation::atan2_with(y, x, precision.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_atan2_fast_f64(y: f64, x: f64) -> Result<f64, BindingError> {
    approximation::atan2_fast(y, x)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native strict comparison, retaining the second operand on ties and NaNs.
pub fn scalar_minimum_f64(first: f64, second: f64) -> f64 {
    crate::scalar::minimum(first, second)
}
/// Native strict comparison, retaining the second operand on ties and NaNs.
pub fn scalar_maximum_f64(first: f64, second: f64) -> f64 {
    crate::scalar::maximum(first, second)
}
pub fn approximation_sine_fast(value: f32) -> Result<f32, BindingError> {
    approximation::sine_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_cosine_fast(value: f32) -> Result<f32, BindingError> {
    approximation::cosine_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_sine(value: f32) -> Result<f32, BindingError> {
    approximation::sine(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_cosine(value: f32) -> Result<f32, BindingError> {
    approximation::cosine(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm2_fast(value: f32) -> Result<f32, BindingError> {
    approximation::logarithm2_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm_fast(value: f32) -> Result<f32, BindingError> {
    approximation::logarithm_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential2_fast(value: f32) -> Result<f32, BindingError> {
    approximation::exponential2_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential_fast(value: f32) -> Result<f32, BindingError> {
    approximation::exponential_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm2(value: f32) -> Result<f32, BindingError> {
    approximation::logarithm2(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_logarithm(value: f32) -> Result<f32, BindingError> {
    approximation::logarithm(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential2(value: f32) -> Result<f32, BindingError> {
    approximation::exponential2(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_exponential(value: f32) -> Result<f32, BindingError> {
    approximation::exponential(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_power(value: f32, exponent: f32) -> Result<f32, BindingError> {
    approximation::power(value, exponent)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn approximation_power_fast(value: f32, exponent: f32) -> Result<f32, BindingError> {
    approximation::power_fast(value, exponent)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Original finite sentinel; this is not IEEE infinity.
pub fn legacy_infinity_sentinel() -> f64 {
    crate::scalar::LEGACY_INFINITY_SENTINEL
}
/// Original compatibility epsilon, distinct from machine epsilon.
pub fn legacy_epsilon() -> f64 {
    crate::scalar::LEGACY_EPSILON
}
