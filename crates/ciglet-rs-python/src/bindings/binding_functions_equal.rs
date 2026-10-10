use super::*;
#[pyfunction(name = "equal_loudness_f32")]
#[pyo3(signature = (value))]
pub fn binding_equal_loudness_f32(value: f32) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::equal_loudness_f32(value)
}
#[pyfunction(name = "equal_loudness_f64")]
#[pyo3(signature = (value))]
pub fn binding_equal_loudness_f64(value: f64) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::equal_loudness_f64(value)
}
