use super::*;
#[pyfunction(name = "dot_f32")]
#[pyo3(signature = (left, right))]
pub fn binding_dot_f32(left: Vec<f32>, right: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::statistics::dot_f32(&left, &right)
}
#[pyfunction(name = "dot_f64")]
#[pyo3(signature = (left, right))]
pub fn binding_dot_f64(left: Vec<f64>, right: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::statistics::dot_f64(&left, &right)
}
