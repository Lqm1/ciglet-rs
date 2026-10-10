use super::*;
#[pyfunction(name = "phase_wrap_f32")]
#[pyo3(signature = (angle))]
pub fn binding_phase_wrap_f32(angle: f32) -> pyo3::PyResult<f32> {
    crate::api::phase::phase_wrap_f32(angle)
}
#[pyfunction(name = "phase_unwrap_f32")]
#[pyo3(signature = (angles))]
pub fn binding_phase_unwrap_f32(angles: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::phase_unwrap_f32(&angles)
}
#[pyfunction(name = "phase_difference_f32")]
#[pyo3(signature = (first, second))]
pub fn binding_phase_difference_f32(first: f32, second: f32) -> pyo3::PyResult<f32> {
    crate::api::phase::phase_difference_f32(first, second)
}
#[pyfunction(name = "phase_wrap_f64")]
#[pyo3(signature = (angle))]
pub fn binding_phase_wrap_f64(angle: f64) -> pyo3::PyResult<f64> {
    crate::api::phase::phase_wrap_f64(angle)
}
#[pyfunction(name = "phase_unwrap_f64")]
#[pyo3(signature = (angles))]
pub fn binding_phase_unwrap_f64(angles: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::phase_unwrap_f64(&angles)
}
#[pyfunction(name = "phase_difference_f64")]
#[pyo3(signature = (first, second))]
pub fn binding_phase_difference_f64(first: f64, second: f64) -> pyo3::PyResult<f64> {
    crate::api::phase::phase_difference_f64(first, second)
}
