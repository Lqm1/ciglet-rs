use super::*;
#[pyfunction(name = "approximation_sine_f32")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_sine_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_sine_f32(value, precision)
}
#[pyfunction(name = "approximation_sine_f64")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_sine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_sine_f64(value, precision)
}
#[pyfunction(name = "approximation_cosine_f32")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_cosine_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_cosine_f32(value, precision)
}
#[pyfunction(name = "approximation_cosine_f64")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_cosine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_cosine_f64(value, precision)
}
#[pyfunction(name = "approximation_logarithm_f32")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_logarithm_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_logarithm_f32(value, precision)
}
#[pyfunction(name = "approximation_logarithm_f64")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_logarithm_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_logarithm_f64(value, precision)
}
#[pyfunction(name = "approximation_exponential_f32")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_exponential_f32(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_exponential_f32(value, precision)
}
#[pyfunction(name = "approximation_exponential_f64")]
#[pyo3(signature = (value, precision))]
pub fn binding_approximation_exponential_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_exponential_f64(value, precision)
}
#[pyfunction(name = "approximation_atan2_f32")]
#[pyo3(signature = (y, x, precision))]
pub fn binding_approximation_atan2_f32(
    y: f32,
    x: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_atan2_f32(y, x, precision)
}
#[pyfunction(name = "approximation_atan2_fast_f32")]
#[pyo3(signature = (y, x))]
pub fn binding_approximation_atan2_fast_f32(y: f32, x: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_atan2_fast_f32(y, x)
}
#[pyfunction(name = "approximation_atan2_f64")]
#[pyo3(signature = (y, x, precision))]
pub fn binding_approximation_atan2_f64(
    y: f64,
    x: f64,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_atan2_f64(y, x, precision)
}
#[pyfunction(name = "approximation_atan2_fast_f64")]
#[pyo3(signature = (y, x))]
pub fn binding_approximation_atan2_fast_f64(y: f64, x: f64) -> pyo3::PyResult<f64> {
    crate::api::approximation::approximation_atan2_fast_f64(y, x)
}
#[pyfunction(name = "approximation_sine_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_sine_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_sine_fast(value)
}
#[pyfunction(name = "approximation_cosine_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_cosine_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_cosine_fast(value)
}
#[pyfunction(name = "approximation_sine")]
#[pyo3(signature = (value))]
pub fn binding_approximation_sine(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_sine(value)
}
#[pyfunction(name = "approximation_cosine")]
#[pyo3(signature = (value))]
pub fn binding_approximation_cosine(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_cosine(value)
}
#[pyfunction(name = "approximation_logarithm2_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_logarithm2_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_logarithm2_fast(value)
}
#[pyfunction(name = "approximation_logarithm_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_logarithm_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_logarithm_fast(value)
}
#[pyfunction(name = "approximation_exponential2_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_exponential2_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_exponential2_fast(value)
}
#[pyfunction(name = "approximation_exponential_fast")]
#[pyo3(signature = (value))]
pub fn binding_approximation_exponential_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_exponential_fast(value)
}
#[pyfunction(name = "approximation_logarithm2")]
#[pyo3(signature = (value))]
pub fn binding_approximation_logarithm2(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_logarithm2(value)
}
#[pyfunction(name = "approximation_logarithm")]
#[pyo3(signature = (value))]
pub fn binding_approximation_logarithm(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_logarithm(value)
}
#[pyfunction(name = "approximation_exponential2")]
#[pyo3(signature = (value))]
pub fn binding_approximation_exponential2(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_exponential2(value)
}
#[pyfunction(name = "approximation_exponential")]
#[pyo3(signature = (value))]
pub fn binding_approximation_exponential(value: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_exponential(value)
}
#[pyfunction(name = "approximation_power")]
#[pyo3(signature = (value, exponent))]
pub fn binding_approximation_power(value: f32, exponent: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_power(value, exponent)
}
#[pyfunction(name = "approximation_power_fast")]
#[pyo3(signature = (value, exponent))]
pub fn binding_approximation_power_fast(value: f32, exponent: f32) -> pyo3::PyResult<f32> {
    crate::api::approximation::approximation_power_fast(value, exponent)
}
