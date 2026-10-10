use super::*;
#[pyfunction(name = "fir_f32")]
#[pyo3(signature = (length, cutoff, highpass, window))]
pub fn binding_fir_f32(
    length: usize,
    cutoff: f32,
    highpass: bool,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::fir_f32(length, cutoff, highpass, window)
}
#[pyfunction(name = "fir_f64")]
#[pyo3(signature = (length, cutoff, highpass, window))]
pub fn binding_fir_f64(
    length: usize,
    cutoff: f64,
    highpass: bool,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::fir_f64(length, cutoff, highpass, window)
}
