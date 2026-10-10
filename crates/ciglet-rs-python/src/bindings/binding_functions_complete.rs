use super::*;
#[pyfunction(name = "complete_spectrum_f32")]
#[pyo3(signature = (values, symmetry, policy))]
pub fn binding_complete_spectrum_f32(
    values: Vec<f32>,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::phase::complete_spectrum_f32(values, symmetry, policy)
}
#[pyfunction(name = "complete_spectrum_f64")]
#[pyo3(signature = (values, symmetry, policy))]
pub fn binding_complete_spectrum_f64(
    values: Vec<f64>,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::phase::complete_spectrum_f64(values, symmetry, policy)
}
