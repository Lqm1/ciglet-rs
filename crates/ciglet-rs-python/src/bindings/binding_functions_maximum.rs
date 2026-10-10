use super::*;
#[pyfunction(name = "maximum_f32")]
#[pyo3(signature = (values))]
pub fn binding_maximum_f32(values: Vec<f32>) -> pyo3::PyResult<Option<f32>> {
    Ok(crate::api::ordering::maximum_f32(&values))
}
#[pyfunction(name = "maximum_f64")]
#[pyo3(signature = (values))]
pub fn binding_maximum_f64(values: Vec<f64>) -> pyo3::PyResult<Option<f64>> {
    Ok(crate::api::ordering::maximum_f64(&values))
}
