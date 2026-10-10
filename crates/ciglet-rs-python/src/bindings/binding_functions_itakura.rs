use super::*;
#[pyfunction(name = "itakura_saito_f32")]
#[pyo3(signature = (source, reference))]
pub fn binding_itakura_saito_f32(source: Vec<f32>, reference: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::frequency_distance::itakura_saito_f32(&source, &reference)
}
#[pyfunction(name = "itakura_saito_f64")]
#[pyo3(signature = (source, reference))]
pub fn binding_itakura_saito_f64(source: Vec<f64>, reference: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::frequency_distance::itakura_saito_f64(&source, &reference)
}
