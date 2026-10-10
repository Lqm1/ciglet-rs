use super::*;
#[pyfunction(name = "cumulative_sum_f32")]
#[pyo3(signature = (values))]
pub fn binding_cumulative_sum_f32(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::vector::cumulative_sum_f32(&values)
}
#[pyfunction(name = "cumulative_sum_f64")]
#[pyo3(signature = (values))]
pub fn binding_cumulative_sum_f64(values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::vector::cumulative_sum_f64(&values)
}
