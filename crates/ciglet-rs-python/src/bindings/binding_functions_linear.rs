use super::*;
#[pyfunction(name = "linear_grid_f32")]
#[pyo3(signature = (start, end, count))]
pub fn binding_linear_grid_f32(start: f32, end: f32, count: usize) -> pyo3::PyResult<Vec<f32>> {
    crate::api::vector::linear_grid_f32(start, end, count)
}
#[pyfunction(name = "linear_grid_f64")]
#[pyo3(signature = (start, end, count))]
pub fn binding_linear_grid_f64(start: f64, end: f64, count: usize) -> pyo3::PyResult<Vec<f64>> {
    crate::api::vector::linear_grid_f64(start, end, count)
}
