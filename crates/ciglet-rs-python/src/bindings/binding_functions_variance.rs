use super::*;
#[pyfunction(name = "variance_f32")]
#[pyo3(signature = (values))]
pub fn binding_variance_f32(values: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::statistics::variance_f32(&values)
}
#[pyfunction(name = "variance_with_f32")]
#[pyo3(signature = (values, policy))]
pub fn binding_variance_with_f32(
    values: Vec<f32>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f32> {
    crate::api::statistics::variance_with_f32(&values, policy)
}
#[pyfunction(name = "variance_f64")]
#[pyo3(signature = (values))]
pub fn binding_variance_f64(values: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::statistics::variance_f64(&values)
}
#[pyfunction(name = "variance_with_f64")]
#[pyo3(signature = (values, policy))]
pub fn binding_variance_with_f64(
    values: Vec<f64>,
    policy: StatisticsPolicy,
) -> pyo3::PyResult<f64> {
    crate::api::statistics::variance_with_f64(&values, policy)
}
