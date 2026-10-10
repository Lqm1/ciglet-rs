use super::*;
#[pyfunction(name = "real_cepstrum_from_log_spectrum_f32")]
#[pyo3(signature = (logs))]
pub fn binding_real_cepstrum_from_log_spectrum_f32(logs: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::real_cepstrum::real_cepstrum_from_log_spectrum_f32(&logs)
}
#[pyfunction(name = "real_cepstrum_to_log_spectrum_f32")]
#[pyo3(signature = (coefficients, length))]
pub fn binding_real_cepstrum_to_log_spectrum_f32(
    coefficients: Vec<f32>,
    length: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::real_cepstrum::real_cepstrum_to_log_spectrum_f32(&coefficients, length)
}
#[pyfunction(name = "real_cepstrum_minimum_phase_f32")]
#[pyo3(signature = (logs))]
pub fn binding_real_cepstrum_minimum_phase_f32(logs: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::real_cepstrum::real_cepstrum_minimum_phase_f32(&logs)
}
#[pyfunction(name = "real_cepstrum_from_log_spectrum_f64")]
#[pyo3(signature = (logs))]
pub fn binding_real_cepstrum_from_log_spectrum_f64(logs: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::real_cepstrum::real_cepstrum_from_log_spectrum_f64(&logs)
}
#[pyfunction(name = "real_cepstrum_to_log_spectrum_f64")]
#[pyo3(signature = (coefficients, length))]
pub fn binding_real_cepstrum_to_log_spectrum_f64(
    coefficients: Vec<f64>,
    length: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::real_cepstrum::real_cepstrum_to_log_spectrum_f64(&coefficients, length)
}
#[pyfunction(name = "real_cepstrum_minimum_phase_f64")]
#[pyo3(signature = (logs))]
pub fn binding_real_cepstrum_minimum_phase_f64(logs: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::real_cepstrum::real_cepstrum_minimum_phase_f64(&logs)
}
