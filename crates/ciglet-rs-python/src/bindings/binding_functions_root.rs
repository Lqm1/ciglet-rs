use super::*;
#[pyfunction(name = "root_find_f32")]
#[pyo3(signature = (callback, start, end))]
pub fn binding_root_find_f32(callback: Py<PyAny>, start: f32, end: f32) -> pyo3::PyResult<f32> {
    crate::api::root::root_find_f32(&crate::callbacks::Function::new(callback)?, start, end)
}
#[pyfunction(name = "root_find_f64")]
#[pyo3(signature = (callback, start, end))]
pub fn binding_root_find_f64(callback: Py<PyAny>, start: f64, end: f64) -> pyo3::PyResult<f64> {
    crate::api::root::root_find_f64(&crate::callbacks::Function::new(callback)?, start, end)
}
