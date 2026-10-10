use super::*;
#[pyfunction(name = "frequency_to_mel_f32")]
#[pyo3(signature = (value))]
pub fn binding_frequency_to_mel_f32(value: f32) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::frequency_to_mel_f32(value)
}
#[pyfunction(name = "frequency_to_mel_f64")]
#[pyo3(signature = (value))]
pub fn binding_frequency_to_mel_f64(value: f64) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::frequency_to_mel_f64(value)
}
#[pyfunction(name = "frequency_to_bark_f32")]
#[pyo3(signature = (value))]
pub fn binding_frequency_to_bark_f32(value: f32) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::frequency_to_bark_f32(value)
}
#[pyfunction(name = "frequency_to_bark_f64")]
#[pyo3(signature = (value))]
pub fn binding_frequency_to_bark_f64(value: f64) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::frequency_to_bark_f64(value)
}
