use super::*;
#[pyfunction(name = "legacy_infinity_sentinel")]
#[pyo3(signature = ())]
pub fn binding_legacy_infinity_sentinel() -> pyo3::PyResult<f64> {
    Ok(crate::api::approximation::legacy_infinity_sentinel())
}
#[pyfunction(name = "legacy_epsilon")]
#[pyo3(signature = ())]
pub fn binding_legacy_epsilon() -> pyo3::PyResult<f64> {
    Ok(crate::api::approximation::legacy_epsilon())
}
