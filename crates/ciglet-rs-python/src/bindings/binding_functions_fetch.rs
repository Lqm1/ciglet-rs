use super::*;
#[pyfunction(name = "fetch_frame_f32")]
#[pyo3(signature = (input, center, length))]
pub fn binding_fetch_frame_f32(
    input: Vec<f32>,
    center: isize,
    length: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::frame_extrema::fetch_frame_f32(&input, center, length)
}
#[pyfunction(name = "fetch_frame_f64")]
#[pyo3(signature = (input, center, length))]
pub fn binding_fetch_frame_f64(
    input: Vec<f64>,
    center: isize,
    length: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::frame_extrema::fetch_frame_f64(&input, center, length)
}
