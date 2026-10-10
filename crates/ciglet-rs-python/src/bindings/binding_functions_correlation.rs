use super::*;
#[pyfunction(name = "correlation_f32")]
#[pyo3(signature = (left, right))]
pub fn binding_correlation_f32(left: Vec<f32>, right: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::statistics::correlation_f32(&left, &right)
}
#[pyfunction(name = "correlation_with_f32")]
#[pyo3(signature = (left, right, policy))]
pub fn binding_correlation_with_f32(
    left: Vec<f32>,
    right: Vec<f32>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f32> {
    crate::api::statistics::correlation_with_f32(&left, &right, policy)
}
#[pyfunction(name = "correlation_f64")]
#[pyo3(signature = (left, right))]
pub fn binding_correlation_f64(left: Vec<f64>, right: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::statistics::correlation_f64(&left, &right)
}
#[pyfunction(name = "correlation_with_f64")]
#[pyo3(signature = (left, right, policy))]
pub fn binding_correlation_with_f64(
    left: Vec<f64>,
    right: Vec<f64>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f64> {
    crate::api::statistics::correlation_with_f64(&left, &right, policy)
}
