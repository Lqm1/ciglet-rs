use super::*;
#[pyfunction(name = "moving_average_f32")]
#[pyo3(signature = (signal, half_width))]
pub fn binding_moving_average_f32(signal: Vec<f32>, half_width: f32) -> pyo3::PyResult<Vec<f32>> {
    crate::api::smoothing::moving_average_f32(&signal, half_width)
}
#[pyfunction(name = "moving_average_with_f32")]
#[pyo3(signature = (signal, half_width, policy))]
pub fn binding_moving_average_with_f32(
    signal: Vec<f32>,
    half_width: f32,
    policy: SmoothingPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::smoothing::moving_average_with_f32(&signal, half_width, policy)
}
#[pyfunction(name = "moving_rms_f32")]
#[pyo3(signature = (signal, half_width))]
pub fn binding_moving_rms_f32(signal: Vec<f32>, half_width: usize) -> pyo3::PyResult<Vec<f32>> {
    crate::api::smoothing::moving_rms_f32(&signal, half_width)
}
#[pyfunction(name = "moving_rms_with_f32")]
#[pyo3(signature = (signal, half_width, policy))]
pub fn binding_moving_rms_with_f32(
    signal: Vec<f32>,
    half_width: usize,
    policy: SmoothingPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::smoothing::moving_rms_with_f32(&signal, half_width, policy)
}
#[pyfunction(name = "moving_average_f64")]
#[pyo3(signature = (signal, half_width))]
pub fn binding_moving_average_f64(signal: Vec<f64>, half_width: f64) -> pyo3::PyResult<Vec<f64>> {
    crate::api::smoothing::moving_average_f64(&signal, half_width)
}
#[pyfunction(name = "moving_average_with_f64")]
#[pyo3(signature = (signal, half_width, policy))]
pub fn binding_moving_average_with_f64(
    signal: Vec<f64>,
    half_width: f64,
    policy: SmoothingPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::smoothing::moving_average_with_f64(&signal, half_width, policy)
}
#[pyfunction(name = "moving_rms_f64")]
#[pyo3(signature = (signal, half_width))]
pub fn binding_moving_rms_f64(signal: Vec<f64>, half_width: usize) -> pyo3::PyResult<Vec<f64>> {
    crate::api::smoothing::moving_rms_f64(&signal, half_width)
}
#[pyfunction(name = "moving_rms_with_f64")]
#[pyo3(signature = (signal, half_width, policy))]
pub fn binding_moving_rms_with_f64(
    signal: Vec<f64>,
    half_width: usize,
    policy: SmoothingPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::smoothing::moving_rms_with_f64(&signal, half_width, policy)
}
