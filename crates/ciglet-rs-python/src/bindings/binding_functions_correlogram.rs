use super::*;
#[pyfunction(name = "correlogram_compute_f32")]
#[pyo3(signature = (samples, centers, lengths, periods, method_code))]
pub fn binding_correlogram_compute_f32(
    samples: Vec<f32>,
    centers: Vec<i32>,
    lengths: Vec<u32>,
    periods: usize,
    method_code: u32,
) -> pyo3::PyResult<Array2F32> {
    (crate::api::correlogram::correlogram_compute_f32(
        &samples,
        &centers,
        &lengths,
        periods,
        method_code,
    ))
    .map(|value| Array2F32 { inner: Some(value) })
}
#[pyfunction(name = "correlogram_compute_with_f32")]
#[pyo3(signature = (samples, centers, lengths, periods, method_code, policy_code))]
pub fn binding_correlogram_compute_with_f32(
    samples: Vec<f32>,
    centers: Vec<i32>,
    lengths: Vec<u32>,
    periods: usize,
    method_code: u32,
    policy_code: u32,
) -> pyo3::PyResult<Array2F32> {
    (crate::api::correlogram::correlogram_compute_with_f32(
        &samples,
        &centers,
        &lengths,
        periods,
        method_code,
        policy_code,
    ))
    .map(|value| Array2F32 { inner: Some(value) })
}
#[pyfunction(name = "correlogram_to_frequency_f32")]
#[pyo3(signature = (periods, sampling_rate, frequencies))]
pub fn binding_correlogram_to_frequency_f32(
    periods: pyo3::PyRef<'_, Array2F32>,
    sampling_rate: i32,
    frequencies: Vec<f32>,
) -> pyo3::PyResult<Array2F32> {
    (crate::api::correlogram::correlogram_to_frequency_f32(
        periods.inner.as_ref().ok_or_else(consumed)?,
        sampling_rate,
        &frequencies,
    ))
    .map(|value| Array2F32 { inner: Some(value) })
}
#[pyfunction(name = "correlogram_compute_f64")]
#[pyo3(signature = (samples, centers, lengths, periods, method_code))]
pub fn binding_correlogram_compute_f64(
    samples: Vec<f64>,
    centers: Vec<i32>,
    lengths: Vec<u32>,
    periods: usize,
    method_code: u32,
) -> pyo3::PyResult<Array2F64> {
    (crate::api::correlogram::correlogram_compute_f64(
        &samples,
        &centers,
        &lengths,
        periods,
        method_code,
    ))
    .map(|value| Array2F64 { inner: Some(value) })
}
#[pyfunction(name = "correlogram_compute_with_f64")]
#[pyo3(signature = (samples, centers, lengths, periods, method_code, policy_code))]
pub fn binding_correlogram_compute_with_f64(
    samples: Vec<f64>,
    centers: Vec<i32>,
    lengths: Vec<u32>,
    periods: usize,
    method_code: u32,
    policy_code: u32,
) -> pyo3::PyResult<Array2F64> {
    (crate::api::correlogram::correlogram_compute_with_f64(
        &samples,
        &centers,
        &lengths,
        periods,
        method_code,
        policy_code,
    ))
    .map(|value| Array2F64 { inner: Some(value) })
}
#[pyfunction(name = "correlogram_to_frequency_f64")]
#[pyo3(signature = (periods, sampling_rate, frequencies))]
pub fn binding_correlogram_to_frequency_f64(
    periods: pyo3::PyRef<'_, Array2F64>,
    sampling_rate: i32,
    frequencies: Vec<f64>,
) -> pyo3::PyResult<Array2F64> {
    (crate::api::correlogram::correlogram_to_frequency_f64(
        periods.inner.as_ref().ok_or_else(consumed)?,
        sampling_rate,
        &frequencies,
    ))
    .map(|value| Array2F64 { inner: Some(value) })
}
