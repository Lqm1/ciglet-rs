use pyo3::PyErr as BindingError;
/// Adjacent differences with the original first sample retained.
pub fn difference_f32(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    crate::vector::difference(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Sequential source-precision sums, preserving the first zero sign.
pub fn cumulative_sum_f32(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    crate::vector::cumulative_sum(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Reverse complete scalar storage, including nonfinite payloads.
pub fn reverse_f32(values: &[f32]) -> Vec<f32> {
    crate::vector::reverse(values)
}
/// Inclusive native linear grid, with empty/singleton and range fallbacks.
pub fn linear_grid_f32(start: f32, end: f32, count: usize) -> Result<Vec<f32>, BindingError> {
    crate::vector::linear_grid(start, end, count)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native scalar interpolation or extrapolation, with finite fallback.
pub fn interpolate_between_f32(start: f32, end: f32, ratio: f32) -> Result<f32, BindingError> {
    crate::interpolation::between(start, end, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Adjacent differences with the original first sample retained.
pub fn difference_f64(values: &[f64]) -> Result<Vec<f64>, BindingError> {
    crate::vector::difference(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Sequential source-precision sums, preserving the first zero sign.
pub fn cumulative_sum_f64(values: &[f64]) -> Result<Vec<f64>, BindingError> {
    crate::vector::cumulative_sum(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Reverse complete scalar storage, including nonfinite payloads.
pub fn reverse_f64(values: &[f64]) -> Vec<f64> {
    crate::vector::reverse(values)
}
/// Inclusive native linear grid, with empty/singleton and range fallbacks.
pub fn linear_grid_f64(start: f64, end: f64, count: usize) -> Result<Vec<f64>, BindingError> {
    crate::vector::linear_grid(start, end, count)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native scalar interpolation or extrapolation, with finite fallback.
pub fn interpolate_between_f64(start: f64, end: f64, ratio: f64) -> Result<f64, BindingError> {
    crate::interpolation::between(start, end, ratio)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Checked signed 32-bit arithmetic progression in an independent Int32Array.
pub fn integer_sequence(start: i32, step: i32, count: usize) -> Result<Vec<i32>, BindingError> {
    crate::vector::integer_sequence(start, step, count)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
