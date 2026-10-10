use super::*;
#[pyfunction(name = "polar_real_f32")]
#[pyo3(signature = (magnitudes, angles))]
pub fn binding_polar_real_f32(magnitudes: Vec<f32>, angles: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::polar_real_f32(&magnitudes, &angles)
}
#[pyfunction(name = "polar_imaginary_f32")]
#[pyo3(signature = (magnitudes, angles))]
pub fn binding_polar_imaginary_f32(
    magnitudes: Vec<f32>,
    angles: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::polar_imaginary_f32(&magnitudes, &angles)
}
#[pyfunction(name = "polar_real_f64")]
#[pyo3(signature = (magnitudes, angles))]
pub fn binding_polar_real_f64(magnitudes: Vec<f64>, angles: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::polar_real_f64(&magnitudes, &angles)
}
#[pyfunction(name = "polar_imaginary_f64")]
#[pyo3(signature = (magnitudes, angles))]
pub fn binding_polar_imaginary_f64(
    magnitudes: Vec<f64>,
    angles: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::polar_imaginary_f64(&magnitudes, &angles)
}
