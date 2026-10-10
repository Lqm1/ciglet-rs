use super::*;
#[pyfunction(name = "mean_f32")]
#[pyo3(signature = (values))]
pub fn binding_mean_f32(values: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::statistics::mean_f32(&values)
}
#[pyfunction(name = "mean_with_f32")]
#[pyo3(signature = (values, policy))]
pub fn binding_mean_with_f32(values: Vec<f32>, policy: StatisticsPolicy) -> pyo3::PyResult<f32> {
    crate::api::statistics::mean_with_f32(&values, policy)
}
#[pyfunction(name = "mean_f64")]
#[pyo3(signature = (values))]
pub fn binding_mean_f64(values: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::statistics::mean_f64(&values)
}
#[pyfunction(name = "mean_with_f64")]
#[pyo3(signature = (values, policy))]
pub fn binding_mean_with_f64(values: Vec<f64>, policy: StatisticsPolicy) -> pyo3::PyResult<f64> {
    crate::api::statistics::mean_with_f64(&values, policy)
}
