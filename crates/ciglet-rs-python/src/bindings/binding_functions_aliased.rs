use super::*;
#[pyfunction(name = "aliased_sinc_f32")]
#[pyo3(signature = (length, angle))]
pub fn binding_aliased_sinc_f32(length: f32, angle: f32) -> pyo3::PyResult<f32> {
    crate::api::sinc::aliased_sinc_f32(length, angle)
}
#[pyfunction(name = "aliased_sinc_with_f32")]
#[pyo3(signature = (length, angle, policy))]
pub fn binding_aliased_sinc_with_f32(
    length: f32,
    angle: f32,
    policy: SincPolicy,
) -> pyo3::PyResult<f32> {
    crate::api::sinc::aliased_sinc_with_f32(length, angle, policy)
}
#[pyfunction(name = "aliased_sinc_derivative_f32")]
#[pyo3(signature = (length, angle))]
pub fn binding_aliased_sinc_derivative_f32(length: f32, angle: f32) -> pyo3::PyResult<f32> {
    crate::api::sinc::aliased_sinc_derivative_f32(length, angle)
}
#[pyfunction(name = "aliased_sinc_derivative_with_f32")]
#[pyo3(signature = (length, angle, policy))]
pub fn binding_aliased_sinc_derivative_with_f32(
    length: f32,
    angle: f32,
    policy: SincPolicy,
) -> pyo3::PyResult<f32> {
    crate::api::sinc::aliased_sinc_derivative_with_f32(length, angle, policy)
}
#[pyfunction(name = "aliased_sinc_f64")]
#[pyo3(signature = (length, angle))]
pub fn binding_aliased_sinc_f64(length: f64, angle: f64) -> pyo3::PyResult<f64> {
    crate::api::sinc::aliased_sinc_f64(length, angle)
}
#[pyfunction(name = "aliased_sinc_with_f64")]
#[pyo3(signature = (length, angle, policy))]
pub fn binding_aliased_sinc_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> pyo3::PyResult<f64> {
    crate::api::sinc::aliased_sinc_with_f64(length, angle, policy)
}
#[pyfunction(name = "aliased_sinc_derivative_f64")]
#[pyo3(signature = (length, angle))]
pub fn binding_aliased_sinc_derivative_f64(length: f64, angle: f64) -> pyo3::PyResult<f64> {
    crate::api::sinc::aliased_sinc_derivative_f64(length, angle)
}
#[pyfunction(name = "aliased_sinc_derivative_with_f64")]
#[pyo3(signature = (length, angle, policy))]
pub fn binding_aliased_sinc_derivative_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> pyo3::PyResult<f64> {
    crate::api::sinc::aliased_sinc_derivative_with_f64(length, angle, policy)
}
