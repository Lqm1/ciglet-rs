use super::*;
#[pyfunction(name = "resample_f32")]
#[pyo3(signature = (signal, ratio))]
pub fn binding_resample_f32(signal: Vec<f32>, ratio: f32) -> pyo3::PyResult<Vec<f32>> {
    crate::api::resampling::resample_f32(&signal, ratio)
}
#[pyfunction(name = "resample_with_f32")]
#[pyo3(signature = (signal, ratio, boundary))]
pub fn binding_resample_with_f32(
    signal: Vec<f32>,
    ratio: f32,
    boundary: BoundaryPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::resampling::resample_with_f32(&signal, ratio, boundary)
}
#[pyfunction(name = "resample_with_kernel_f32")]
#[pyo3(signature = (signal, ratio, boundary, kernel))]
pub fn binding_resample_with_kernel_f32(
    signal: Vec<f32>,
    ratio: f32,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::resampling::resample_with_kernel_f32(&signal, ratio, boundary, kernel)
}
#[pyfunction(name = "resample_output_length_f32")]
#[pyo3(signature = (count, ratio))]
pub fn binding_resample_output_length_f32(count: usize, ratio: f32) -> pyo3::PyResult<usize> {
    crate::api::resampling::resample_output_length_f32(count, ratio)
}
#[pyfunction(name = "resample_f64")]
#[pyo3(signature = (signal, ratio))]
pub fn binding_resample_f64(signal: Vec<f64>, ratio: f64) -> pyo3::PyResult<Vec<f64>> {
    crate::api::resampling::resample_f64(&signal, ratio)
}
#[pyfunction(name = "resample_with_f64")]
#[pyo3(signature = (signal, ratio, boundary))]
pub fn binding_resample_with_f64(
    signal: Vec<f64>,
    ratio: f64,
    boundary: BoundaryPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::resampling::resample_with_f64(&signal, ratio, boundary)
}
#[pyfunction(name = "resample_with_kernel_f64")]
#[pyo3(signature = (signal, ratio, boundary, kernel))]
pub fn binding_resample_with_kernel_f64(
    signal: Vec<f64>,
    ratio: f64,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::resampling::resample_with_kernel_f64(&signal, ratio, boundary, kernel)
}
#[pyfunction(name = "resample_output_length_f64")]
#[pyo3(signature = (count, ratio))]
pub fn binding_resample_output_length_f64(count: usize, ratio: f64) -> pyo3::PyResult<usize> {
    crate::api::resampling::resample_output_length_f64(count, ratio)
}
