use super::*;
#[pyfunction(name = "special_complementary_error")]
#[pyo3(signature = (value))]
pub fn binding_special_complementary_error(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_complementary_error(value)
}
#[pyfunction(name = "special_complementary_error_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_complementary_error_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_complementary_error_lanes(&values)
}
#[pyfunction(name = "special_complementary_error_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_complementary_error_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_complementary_error_fast(value)
}
#[pyfunction(name = "special_complementary_error_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_complementary_error_fast_lanes(
    values: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_complementary_error_fast_lanes(&values)
}
#[pyfunction(name = "special_error")]
#[pyo3(signature = (value))]
pub fn binding_special_error(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_error(value)
}
#[pyfunction(name = "special_error_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_error_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_error_lanes(&values)
}
#[pyfunction(name = "special_error_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_error_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_error_fast(value)
}
#[pyfunction(name = "special_error_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_error_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_error_fast_lanes(&values)
}
#[pyfunction(name = "special_inverse_error")]
#[pyo3(signature = (value))]
pub fn binding_special_inverse_error(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_inverse_error(value)
}
#[pyfunction(name = "special_inverse_error_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_inverse_error_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_inverse_error_lanes(&values)
}
#[pyfunction(name = "special_inverse_error_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_inverse_error_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_inverse_error_fast(value)
}
#[pyfunction(name = "special_inverse_error_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_inverse_error_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_inverse_error_fast_lanes(&values)
}
#[pyfunction(name = "special_log_gamma")]
#[pyo3(signature = (value))]
pub fn binding_special_log_gamma(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_log_gamma(value)
}
#[pyfunction(name = "special_log_gamma_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_log_gamma_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_log_gamma_lanes(&values)
}
#[pyfunction(name = "special_log_gamma_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_log_gamma_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_log_gamma_fast(value)
}
#[pyfunction(name = "special_log_gamma_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_log_gamma_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_log_gamma_fast_lanes(&values)
}
#[pyfunction(name = "special_digamma")]
#[pyo3(signature = (value))]
pub fn binding_special_digamma(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_digamma(value)
}
#[pyfunction(name = "special_digamma_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_digamma_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_digamma_lanes(&values)
}
#[pyfunction(name = "special_digamma_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_digamma_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_digamma_fast(value)
}
#[pyfunction(name = "special_digamma_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_digamma_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_digamma_fast_lanes(&values)
}
#[pyfunction(name = "special_lambert_w")]
#[pyo3(signature = (value))]
pub fn binding_special_lambert_w(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_lambert_w(value)
}
#[pyfunction(name = "special_lambert_w_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_lambert_w_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_lambert_w_lanes(&values)
}
#[pyfunction(name = "special_lambert_w_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_lambert_w_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_lambert_w_fast(value)
}
#[pyfunction(name = "special_lambert_w_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_lambert_w_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_lambert_w_fast_lanes(&values)
}
#[pyfunction(name = "special_lambert_w_exp")]
#[pyo3(signature = (value))]
pub fn binding_special_lambert_w_exp(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_lambert_w_exp(value)
}
#[pyfunction(name = "special_lambert_w_exp_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_lambert_w_exp_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_lambert_w_exp_lanes(&values)
}
#[pyfunction(name = "special_lambert_w_exp_fast")]
#[pyo3(signature = (value))]
pub fn binding_special_lambert_w_exp_fast(value: f32) -> pyo3::PyResult<f32> {
    crate::api::special::special_lambert_w_exp_fast(value)
}
#[pyfunction(name = "special_lambert_w_exp_fast_lanes")]
#[pyo3(signature = (values))]
pub fn binding_special_lambert_w_exp_fast_lanes(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::special::special_lambert_w_exp_fast_lanes(&values)
}
