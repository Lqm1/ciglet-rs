use super::*;
#[pyfunction(name = "fft_f32")]
#[pyo3(signature = (length, real, imaginary, direction))]
pub fn binding_fft_f32(
    length: usize,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
    direction: u32,
) -> pyo3::PyResult<ComplexValuesF32> {
    (crate::api::fourier::fft_f32(length, real, imaginary, direction))
        .map(|value| ComplexValuesF32 { inner: Some(value) })
}
#[pyfunction(name = "fft_shift_f32")]
#[pyo3(signature = (values))]
pub fn binding_fft_shift_f32(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    Ok(crate::api::fourier::fft_shift_f32(&values))
}
#[pyfunction(name = "fft_f64")]
#[pyo3(signature = (length, real, imaginary, direction))]
pub fn binding_fft_f64(
    length: usize,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
    direction: u32,
) -> pyo3::PyResult<ComplexValuesF64> {
    (crate::api::fourier::fft_f64(length, real, imaginary, direction))
        .map(|value| ComplexValuesF64 { inner: Some(value) })
}
#[pyfunction(name = "fft_shift_f64")]
#[pyo3(signature = (values))]
pub fn binding_fft_shift_f64(values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    Ok(crate::api::fourier::fft_shift_f64(&values))
}
