use super::*;
#[pyfunction(name = "bark_to_frequency_f32")]
#[pyo3(signature = (value))]
pub fn binding_bark_to_frequency_f32(value: f32) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::bark_to_frequency_f32(value)
}
#[pyfunction(name = "bark_to_frequency_f64")]
#[pyo3(signature = (value))]
pub fn binding_bark_to_frequency_f64(value: f64) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::bark_to_frequency_f64(value)
}
