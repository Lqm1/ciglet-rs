use super::*;
#[pyfunction(name = "bandpass_fir_f32")]
#[pyo3(signature = (length, lower, upper, window))]
pub fn binding_bandpass_fir_f32(
    length: usize,
    lower: f32,
    upper: f32,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::bandpass_fir_f32(length, lower, upper, window)
}
#[pyfunction(name = "bandpass_fir_f64")]
#[pyo3(signature = (length, lower, upper, window))]
pub fn binding_bandpass_fir_f64(
    length: usize,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::bandpass_fir_f64(length, lower, upper, window)
}
