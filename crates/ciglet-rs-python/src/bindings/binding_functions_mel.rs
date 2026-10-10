use super::*;
#[pyfunction(name = "mel_to_frequency_f32")]
#[pyo3(signature = (value))]
pub fn binding_mel_to_frequency_f32(value: f32) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::mel_to_frequency_f32(value)
}
#[pyfunction(name = "mel_to_frequency_f64")]
#[pyo3(signature = (value))]
pub fn binding_mel_to_frequency_f64(value: f64) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::mel_to_frequency_f64(value)
}
#[pyfunction(name = "mel_space_f32")]
#[pyo3(signature = (minimum, maximum, intervals))]
pub fn binding_mel_space_f32(
    minimum: f32,
    maximum: f32,
    intervals: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::frequency_distance::mel_space_f32(minimum, maximum, intervals)
}
#[pyfunction(name = "mel_space_f64")]
#[pyo3(signature = (minimum, maximum, intervals))]
pub fn binding_mel_space_f64(
    minimum: f64,
    maximum: f64,
    intervals: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::frequency_distance::mel_space_f64(minimum, maximum, intervals)
}
