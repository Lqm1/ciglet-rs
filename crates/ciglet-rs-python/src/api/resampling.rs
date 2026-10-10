use super::{BoundaryPolicy, KernelPolicy};
use pyo3::PyErr as BindingError;
/// Antialiased Lanczos resampling with corrected native defaults.
pub fn resample_f32(signal: &[f32], ratio: f32) -> Result<Vec<f32>, BindingError> {
    crate::resampling::resample(signal, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Select sample-zero handling while retaining original kernel arithmetic.
pub fn resample_with_f32(
    signal: &[f32],
    ratio: f32,
    boundary: BoundaryPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::resampling::resample_with(signal, ratio, boundary.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Select sample-zero handling and kernel arithmetic independently.
pub fn resample_with_kernel_f32(
    signal: &[f32],
    ratio: f32,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::resampling::resample_with_kernel(signal, ratio, boundary.into(), kernel.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Compute the native checked output length without allocating a signal.
pub fn resample_output_length_f32(count: usize, ratio: f32) -> Result<usize, BindingError> {
    crate::resampling::output_length(count, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Antialiased Lanczos resampling with corrected native defaults.
pub fn resample_f64(signal: &[f64], ratio: f64) -> Result<Vec<f64>, BindingError> {
    crate::resampling::resample(signal, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Select sample-zero handling while retaining original kernel arithmetic.
pub fn resample_with_f64(
    signal: &[f64],
    ratio: f64,
    boundary: BoundaryPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::resampling::resample_with(signal, ratio, boundary.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Select sample-zero handling and kernel arithmetic independently.
pub fn resample_with_kernel_f64(
    signal: &[f64],
    ratio: f64,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::resampling::resample_with_kernel(signal, ratio, boundary.into(), kernel.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Compute the native checked output length without allocating a signal.
pub fn resample_output_length_f64(count: usize, ratio: f64) -> Result<usize, BindingError> {
    crate::resampling::output_length(count, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
