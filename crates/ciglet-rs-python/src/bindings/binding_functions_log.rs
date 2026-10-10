use super::*;
#[pyfunction(name = "log_itakura_saito_f32")]
#[pyo3(signature = (source, reference))]
pub fn binding_log_itakura_saito_f32(source: Vec<f32>, reference: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::log_itakura_saito_f32(&source, &reference)
}
#[pyfunction(name = "log_itakura_saito_with_f32")]
#[pyo3(signature = (source, reference, policy))]
pub fn binding_log_itakura_saito_with_f32(
    source: Vec<f32>,
    reference: Vec<f32>,
    policy: DistancePolicy,
) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::log_itakura_saito_with_f32(&source, &reference, policy)
}
#[pyfunction(name = "log_itakura_saito_f64")]
#[pyo3(signature = (source, reference))]
pub fn binding_log_itakura_saito_f64(source: Vec<f64>, reference: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::log_itakura_saito_f64(&source, &reference)
}
#[pyfunction(name = "log_itakura_saito_with_f64")]
#[pyo3(signature = (source, reference, policy))]
pub fn binding_log_itakura_saito_with_f64(
    source: Vec<f64>,
    reference: Vec<f64>,
    policy: DistancePolicy,
) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::log_itakura_saito_with_f64(&source, &reference, policy)
}
