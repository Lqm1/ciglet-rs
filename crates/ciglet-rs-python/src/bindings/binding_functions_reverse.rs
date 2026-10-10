use super::*;
#[pyfunction(name = "reverse_f32")]
#[pyo3(signature = (values))]
pub fn binding_reverse_f32(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    Ok(crate::api::vector::reverse_f32(&values))
}
#[pyfunction(name = "reverse_f64")]
#[pyo3(signature = (values))]
pub fn binding_reverse_f64(values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    Ok(crate::api::vector::reverse_f64(&values))
}
