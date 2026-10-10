use super::*;
#[pyfunction(name = "window_f32")]
#[pyo3(signature = (kind, length, precision))]
pub fn binding_window_f32(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::window::window_f32(kind, length, precision)
}
#[pyfunction(name = "window_f64")]
#[pyo3(signature = (kind, length, precision))]
pub fn binding_window_f64(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::window::window_f64(kind, length, precision)
}
#[pyfunction(name = "window_custom_f32_f32")]
#[pyo3(signature = (kind, length, callback))]
pub fn binding_window_custom_f32_f32(
    kind: WindowKind,
    length: usize,
    callback: Py<PyAny>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::window::window_custom_f32_f32(
        kind,
        length,
        &crate::callbacks::Function::new(callback)?,
    )
}
#[pyfunction(name = "window_custom_f32_f64")]
#[pyo3(signature = (kind, length, callback))]
pub fn binding_window_custom_f32_f64(
    kind: WindowKind,
    length: usize,
    callback: Py<PyAny>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::window::window_custom_f32_f64(
        kind,
        length,
        &crate::callbacks::Function::new(callback)?,
    )
}
#[pyfunction(name = "window_custom_f64_f32")]
#[pyo3(signature = (kind, length, callback))]
pub fn binding_window_custom_f64_f32(
    kind: WindowKind,
    length: usize,
    callback: Py<PyAny>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::window::window_custom_f64_f32(
        kind,
        length,
        &crate::callbacks::Function::new(callback)?,
    )
}
#[pyfunction(name = "window_custom_f64_f64")]
#[pyo3(signature = (kind, length, callback))]
pub fn binding_window_custom_f64_f64(
    kind: WindowKind,
    length: usize,
    callback: Py<PyAny>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::window::window_custom_f64_f64(
        kind,
        length,
        &crate::callbacks::Function::new(callback)?,
    )
}
