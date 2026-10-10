use super::*;
#[pyfunction(name = "cross_correlation_f32")]
#[pyo3(signature = (left, right, lags))]
pub fn binding_cross_correlation_f32(
    left: Vec<f32>,
    right: Vec<f32>,
    lags: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::statistics::cross_correlation_f32(&left, &right, lags)
}
#[pyfunction(name = "cross_correlation_f64")]
#[pyo3(signature = (left, right, lags))]
pub fn binding_cross_correlation_f64(
    left: Vec<f64>,
    right: Vec<f64>,
    lags: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::statistics::cross_correlation_f64(&left, &right, lags)
}
