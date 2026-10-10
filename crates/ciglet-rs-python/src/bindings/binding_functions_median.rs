use super::*;
#[pyfunction(name = "median_f32")]
#[pyo3(signature = (values))]
pub fn binding_median_f32(values: Vec<f32>) -> pyo3::PyResult<f32> {
    crate::api::ordering::median_f32(&values)
}
#[pyfunction(name = "median_f64")]
#[pyo3(signature = (values))]
pub fn binding_median_f64(values: Vec<f64>) -> pyo3::PyResult<f64> {
    crate::api::ordering::median_f64(&values)
}
#[pyfunction(name = "median_filter_f32")]
#[pyo3(signature = (signal, width))]
pub fn binding_median_filter_f32(signal: Vec<f32>, width: usize) -> pyo3::PyResult<Vec<f32>> {
    crate::api::smoothing::median_filter_f32(&signal, width)
}
#[pyfunction(name = "median_filter_f64")]
#[pyo3(signature = (signal, width))]
pub fn binding_median_filter_f64(signal: Vec<f64>, width: usize) -> pyo3::PyResult<Vec<f64>> {
    crate::api::smoothing::median_filter_f64(&signal, width)
}
