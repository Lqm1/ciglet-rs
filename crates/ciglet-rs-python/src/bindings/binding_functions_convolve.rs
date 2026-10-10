use super::*;
#[pyfunction(name = "convolve_f32")]
#[pyo3(signature = (signal, impulse))]
pub fn binding_convolve_f32(signal: Vec<f32>, impulse: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::convolve_f32(&signal, &impulse)
}
#[pyfunction(name = "convolve_f64")]
#[pyo3(signature = (signal, impulse))]
pub fn binding_convolve_f64(signal: Vec<f64>, impulse: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::convolve_f64(&signal, &impulse)
}
