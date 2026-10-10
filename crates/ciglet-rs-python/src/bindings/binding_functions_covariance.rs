use super::*;
#[pyfunction(name = "covariance_f32")]
#[pyo3(signature = (left, right))]
pub fn binding_covariance_f32(left: Vec<f32>, right: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::statistics::covariance_f32(&left, &right)
}
#[pyfunction(name = "covariance_with_f32")]
#[pyo3(signature = (left, right, policy))]
pub fn binding_covariance_with_f32(
    left: Vec<f32>,
    right: Vec<f32>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f32> {
    crate::api::statistics::covariance_with_f32(&left, &right, policy)
}
#[pyfunction(name = "covariance_f64")]
#[pyo3(signature = (left, right))]
pub fn binding_covariance_f64(left: Vec<f64>, right: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::statistics::covariance_f64(&left, &right)
}
#[pyfunction(name = "covariance_with_f64")]
#[pyo3(signature = (left, right, policy))]
pub fn binding_covariance_with_f64(
    left: Vec<f64>,
    right: Vec<f64>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f64> {
    crate::api::statistics::covariance_with_f64(&left, &right, policy)
}
