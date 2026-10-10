use super::*;
#[pyfunction(name = "sinusoids_f32")]
#[pyo3(signature = (frequencies, amplitudes, phases, sample_rate, length))]
pub fn binding_sinusoids_f32(
    frequencies: Vec<f32>,
    amplitudes: Vec<f32>,
    phases: Vec<f32>,
    sample_rate: u32,
    length: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::synthesis::sinusoids_f32(&frequencies, &amplitudes, &phases, sample_rate, length)
}
#[pyfunction(name = "sinusoids_f64")]
#[pyo3(signature = (frequencies, amplitudes, phases, sample_rate, length))]
pub fn binding_sinusoids_f64(
    frequencies: Vec<f64>,
    amplitudes: Vec<f64>,
    phases: Vec<f64>,
    sample_rate: u32,
    length: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::synthesis::sinusoids_f64(&frequencies, &amplitudes, &phases, sample_rate, length)
}
