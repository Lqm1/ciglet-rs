use super::*;
#[pyfunction(name = "dct_f32")]
#[pyo3(signature = (values))]
pub fn binding_dct_f32(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::dct_f32(&values)
}
#[pyfunction(name = "dct_f64")]
#[pyo3(signature = (values))]
pub fn binding_dct_f64(values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::dct_f64(&values)
}
