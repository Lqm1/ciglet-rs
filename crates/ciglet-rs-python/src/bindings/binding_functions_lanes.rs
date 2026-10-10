use super::*;
#[pyfunction(name = "lanes_exponential2")]
#[pyo3(signature = (values))]
pub fn binding_lanes_exponential2(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_exponential2(&values)
}
#[pyfunction(name = "lanes_exponential2_fast")]
#[pyo3(signature = (values))]
pub fn binding_lanes_exponential2_fast(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_exponential2_fast(&values)
}
#[pyfunction(name = "lanes_exponential")]
#[pyo3(signature = (values))]
pub fn binding_lanes_exponential(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_exponential(&values)
}
#[pyfunction(name = "lanes_exponential_fast")]
#[pyo3(signature = (values))]
pub fn binding_lanes_exponential_fast(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_exponential_fast(&values)
}
#[pyfunction(name = "lanes_logarithm2")]
#[pyo3(signature = (values))]
pub fn binding_lanes_logarithm2(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_logarithm2(&values)
}
#[pyfunction(name = "lanes_logarithm2_fast")]
#[pyo3(signature = (values))]
pub fn binding_lanes_logarithm2_fast(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_logarithm2_fast(&values)
}
#[pyfunction(name = "lanes_logarithm")]
#[pyo3(signature = (values))]
pub fn binding_lanes_logarithm(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_logarithm(&values)
}
#[pyfunction(name = "lanes_logarithm_fast")]
#[pyo3(signature = (values))]
pub fn binding_lanes_logarithm_fast(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_logarithm_fast(&values)
}
#[pyfunction(name = "lanes_power")]
#[pyo3(signature = (bases, exponents))]
pub fn binding_lanes_power(bases: Vec<f32>, exponents: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_power(&bases, &exponents)
}
#[pyfunction(name = "lanes_power_fast")]
#[pyo3(signature = (bases, exponents))]
pub fn binding_lanes_power_fast(bases: Vec<f32>, exponents: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_power_fast(&bases, &exponents)
}
#[pyfunction(name = "lanes_to_integers")]
#[pyo3(signature = (values))]
pub fn binding_lanes_to_integers(values: Vec<f32>) -> pyo3::PyResult<Vec<i32>> {
    crate::api::lanes::lanes_to_integers(&values)
}
#[pyfunction(name = "lanes_from_integers")]
#[pyo3(signature = (values))]
pub fn binding_lanes_from_integers(values: Vec<i32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_from_integers(&values)
}
#[pyfunction(name = "lanes_to_bits")]
#[pyo3(signature = (values))]
pub fn binding_lanes_to_bits(values: Vec<f32>) -> pyo3::PyResult<Vec<u32>> {
    crate::api::lanes::lanes_to_bits(&values)
}
#[pyfunction(name = "lanes_from_bits")]
#[pyo3(signature = (values))]
pub fn binding_lanes_from_bits(values: Vec<u32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_from_bits(&values)
}
#[pyfunction(name = "lanes_absolute")]
#[pyo3(signature = (values))]
pub fn binding_lanes_absolute(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::lanes::lanes_absolute(&values)
}
#[pyfunction(name = "lanes_from_integer64")]
#[pyo3(signature = (first, second))]
pub fn binding_lanes_from_integer64(first: i64, second: i64) -> pyo3::PyResult<Vec<i32>> {
    Ok(crate::api::lanes::lanes_from_integer64(first, second))
}
