use super::*;
#[pyfunction(name = "sum_f32")]
#[pyo3(signature = (values))]
pub fn binding_sum_f32(values: Vec<f32>) -> pyo3::PyResult<f32> {
    Ok(crate::api::statistics::sum_f32(&values))
}
#[pyfunction(name = "sum_f64")]
#[pyo3(signature = (values))]
pub fn binding_sum_f64(values: Vec<f64>) -> pyo3::PyResult<f64> {
    Ok(crate::api::statistics::sum_f64(&values))
}
#[pyfunction(name = "sum_squares_f32")]
#[pyo3(signature = (values))]
pub fn binding_sum_squares_f32(values: Vec<f32>) -> pyo3::PyResult<f32> {
    Ok(crate::api::statistics::sum_squares_f32(&values))
}
#[pyfunction(name = "sum_squares_f64")]
#[pyo3(signature = (values))]
pub fn binding_sum_squares_f64(values: Vec<f64>) -> pyo3::PyResult<f64> {
    Ok(crate::api::statistics::sum_squares_f64(&values))
}
