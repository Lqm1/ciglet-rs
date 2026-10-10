use super::*;
#[pyfunction(name = "peak_f32")]
#[pyo3(signature = (signal, start, end, direction))]
pub fn binding_peak_f32(
    signal: Vec<f32>,
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> pyo3::PyResult<usize> {
    crate::api::frame_extrema::peak_f32(&signal, start, end, direction)
}
#[pyfunction(name = "peak_f64")]
#[pyo3(signature = (signal, start, end, direction))]
pub fn binding_peak_f64(
    signal: Vec<f64>,
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> pyo3::PyResult<usize> {
    crate::api::frame_extrema::peak_f64(&signal, start, end, direction)
}
