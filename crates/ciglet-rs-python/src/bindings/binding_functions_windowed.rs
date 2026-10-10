use super::*;
#[pyfunction(name = "windowed_fir_f32")]
#[pyo3(signature = (length, kind, lower, upper, window))]
pub fn binding_windowed_fir_f32(
    length: usize,
    kind: FilterPassband,
    lower: f32,
    upper: f32,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::windowed_fir_f32(length, kind, lower, upper, window)
}
#[pyfunction(name = "windowed_fir_f64")]
#[pyo3(signature = (length, kind, lower, upper, window))]
pub fn binding_windowed_fir_f64(
    length: usize,
    kind: FilterPassband,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::windowed_fir_f64(length, kind, lower, upper, window)
}
