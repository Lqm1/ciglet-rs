use super::*;
#[pyfunction(name = "magnitude_f32")]
#[pyo3(signature = (real, imaginary))]
pub fn binding_magnitude_f32(real: Vec<f32>, imaginary: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::magnitude_f32(&real, &imaginary)
}
#[pyfunction(name = "magnitude_f64")]
#[pyo3(signature = (real, imaginary))]
pub fn binding_magnitude_f64(real: Vec<f64>, imaginary: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::magnitude_f64(&real, &imaginary)
}
