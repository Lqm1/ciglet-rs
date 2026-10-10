use super::*;
#[pyfunction(name = "inverse_dft_f32")]
#[pyo3(signature = (length, real, imaginary))]
pub fn binding_inverse_dft_f32(
    length: usize,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
) -> pyo3::PyResult<ComplexValuesF32> {
    (crate::api::fourier::inverse_dft_f32(length, real, imaginary))
        .map(|value| ComplexValuesF32 { inner: Some(value) })
}
#[pyfunction(name = "inverse_dft_f64")]
#[pyo3(signature = (length, real, imaginary))]
pub fn binding_inverse_dft_f64(
    length: usize,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
) -> pyo3::PyResult<ComplexValuesF64> {
    (crate::api::fourier::inverse_dft_f64(length, real, imaginary))
        .map(|value| ComplexValuesF64 { inner: Some(value) })
}
