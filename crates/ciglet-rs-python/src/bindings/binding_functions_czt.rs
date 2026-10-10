use super::*;
#[pyfunction(name = "czt_f32")]
#[pyo3(signature = (length, angular_step, direction, real, imaginary))]
pub fn binding_czt_f32(
    length: usize,
    angular_step: f32,
    direction: u32,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
) -> pyo3::PyResult<ComplexValuesF32> {
    (crate::api::chirp_z::czt_f32(length, angular_step, direction, real, imaginary))
        .map(|value| ComplexValuesF32 { inner: Some(value) })
}
#[pyfunction(name = "czt_f64")]
#[pyo3(signature = (length, angular_step, direction, real, imaginary))]
pub fn binding_czt_f64(
    length: usize,
    angular_step: f64,
    direction: u32,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
) -> pyo3::PyResult<ComplexValuesF64> {
    (crate::api::chirp_z::czt_f64(length, angular_step, direction, real, imaginary))
        .map(|value| ComplexValuesF64 { inner: Some(value) })
}
