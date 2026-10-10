use super::*;
#[pyfunction(name = "filter_f32")]
#[pyo3(signature = (signal, numerator, denominator))]
pub fn binding_filter_f32(
    signal: Vec<f32>,
    numerator: Vec<f32>,
    denominator: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::filter_f32(&signal, &numerator, &denominator)
}
#[pyfunction(name = "filter_with_f32")]
#[pyo3(signature = (signal, numerator, denominator, startup))]
pub fn binding_filter_with_f32(
    signal: Vec<f32>,
    numerator: Vec<f32>,
    denominator: Vec<f32>,
    startup: FilterStartup,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::filtering::filter_with_f32(&signal, &numerator, &denominator, startup)
}
#[pyfunction(name = "filter_f64")]
#[pyo3(signature = (signal, numerator, denominator))]
pub fn binding_filter_f64(
    signal: Vec<f64>,
    numerator: Vec<f64>,
    denominator: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::filter_f64(&signal, &numerator, &denominator)
}
#[pyfunction(name = "filter_with_f64")]
#[pyo3(signature = (signal, numerator, denominator, startup))]
pub fn binding_filter_with_f64(
    signal: Vec<f64>,
    numerator: Vec<f64>,
    denominator: Vec<f64>,
    startup: FilterStartup,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::filtering::filter_with_f64(&signal, &numerator, &denominator, startup)
}
