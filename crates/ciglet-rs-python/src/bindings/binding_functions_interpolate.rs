use super::*;
#[pyfunction(name = "interpolate_linear_f32")]
#[pyo3(signature = (coordinates, values, positions))]
pub fn binding_interpolate_linear_f32(
    coordinates: Vec<f32>,
    values: Vec<f32>,
    positions: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_linear_f32(&coordinates, &values, &positions)
}
#[pyfunction(name = "interpolate_uniform_f32")]
#[pyo3(signature = (start, end, values, positions))]
pub fn binding_interpolate_uniform_f32(
    start: f32,
    end: f32,
    values: Vec<f32>,
    positions: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_uniform_f32(start, end, &values, &positions)
}
#[pyfunction(name = "interpolate_sinc_f32")]
#[pyo3(signature = (start, end, values, positions))]
pub fn binding_interpolate_sinc_f32(
    start: f32,
    end: f32,
    values: Vec<f32>,
    positions: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_sinc_f32(start, end, &values, &positions)
}
#[pyfunction(name = "interpolate_sinc_with_f32")]
#[pyo3(signature = (start, end, values, positions, boundary))]
pub fn binding_interpolate_sinc_with_f32(
    start: f32,
    end: f32,
    values: Vec<f32>,
    positions: Vec<f32>,
    boundary: BoundaryPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_sinc_with_f32(start, end, &values, &positions, boundary)
}
#[pyfunction(name = "interpolate_sinc_with_kernel_f32")]
#[pyo3(signature = (start, end, values, positions, boundary, kernel))]
pub fn binding_interpolate_sinc_with_kernel_f32(
    start: f32,
    end: f32,
    values: Vec<f32>,
    positions: Vec<f32>,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_sinc_with_kernel_f32(
        start, end, &values, &positions, boundary, kernel,
    )
}
#[pyfunction(name = "interpolate_fill_missing_f32")]
#[pyo3(signature = (signal, blank))]
pub fn binding_interpolate_fill_missing_f32(
    signal: Vec<f32>,
    blank: f32,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::interpolation::interpolate_fill_missing_f32(&signal, blank)
}
#[pyfunction(name = "interpolate_linear_f64")]
#[pyo3(signature = (coordinates, values, positions))]
pub fn binding_interpolate_linear_f64(
    coordinates: Vec<f64>,
    values: Vec<f64>,
    positions: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_linear_f64(&coordinates, &values, &positions)
}
#[pyfunction(name = "interpolate_uniform_f64")]
#[pyo3(signature = (start, end, values, positions))]
pub fn binding_interpolate_uniform_f64(
    start: f64,
    end: f64,
    values: Vec<f64>,
    positions: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_uniform_f64(start, end, &values, &positions)
}
#[pyfunction(name = "interpolate_sinc_f64")]
#[pyo3(signature = (start, end, values, positions))]
pub fn binding_interpolate_sinc_f64(
    start: f64,
    end: f64,
    values: Vec<f64>,
    positions: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_sinc_f64(start, end, &values, &positions)
}
#[pyfunction(name = "interpolate_sinc_with_f64")]
#[pyo3(signature = (start, end, values, positions, boundary))]
pub fn binding_interpolate_sinc_with_f64(
    start: f64,
    end: f64,
    values: Vec<f64>,
    positions: Vec<f64>,
    boundary: BoundaryPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_sinc_with_f64(start, end, &values, &positions, boundary)
}
#[pyfunction(name = "interpolate_sinc_with_kernel_f64")]
#[pyo3(signature = (start, end, values, positions, boundary, kernel))]
pub fn binding_interpolate_sinc_with_kernel_f64(
    start: f64,
    end: f64,
    values: Vec<f64>,
    positions: Vec<f64>,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_sinc_with_kernel_f64(
        start, end, &values, &positions, boundary, kernel,
    )
}
#[pyfunction(name = "interpolate_fill_missing_f64")]
#[pyo3(signature = (signal, blank))]
pub fn binding_interpolate_fill_missing_f64(
    signal: Vec<f64>,
    blank: f64,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::interpolation::interpolate_fill_missing_f64(&signal, blank)
}
#[pyfunction(name = "interpolate_between_f32")]
#[pyo3(signature = (start, end, ratio))]
pub fn binding_interpolate_between_f32(start: f32, end: f32, ratio: f32) -> pyo3::PyResult<f32> {
    crate::api::vector::interpolate_between_f32(start, end, ratio)
}
#[pyfunction(name = "interpolate_between_f64")]
#[pyo3(signature = (start, end, ratio))]
pub fn binding_interpolate_between_f64(start: f64, end: f64, ratio: f64) -> pyo3::PyResult<f64> {
    crate::api::vector::interpolate_between_f64(start, end, ratio)
}
