use super::*;
#[pyfunction(name = "polynomial_evaluate_f32")]
#[pyo3(signature = (coefficients, real, imaginary))]
pub fn binding_polynomial_evaluate_f32(
    coefficients: pyo3::PyRef<'_, ComplexValuesF32>,
    real: f32,
    imaginary: f32,
) -> pyo3::PyResult<ComplexValuesF32> {
    (crate::api::polynomial::polynomial_evaluate_f32(
        coefficients.inner.as_ref().ok_or_else(consumed)?,
        real,
        imaginary,
    ))
    .map(|value| ComplexValuesF32 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_evaluate_real_f32")]
#[pyo3(signature = (coefficients, real, imaginary))]
pub fn binding_polynomial_evaluate_real_f32(
    coefficients: Vec<f32>,
    real: f32,
    imaginary: f32,
) -> pyo3::PyResult<ComplexValuesF32> {
    (crate::api::polynomial::polynomial_evaluate_real_f32(&coefficients, real, imaginary))
        .map(|value| ComplexValuesF32 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_roots_f32")]
#[pyo3(signature = (coefficients))]
pub fn binding_polynomial_roots_f32(
    coefficients: pyo3::PyRef<'_, ComplexValuesF32>,
) -> pyo3::PyResult<PolynomialRootsF32> {
    (crate::api::polynomial::polynomial_roots_f32(
        coefficients.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| PolynomialRootsF32 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_roots_real_f32")]
#[pyo3(signature = (coefficients))]
pub fn binding_polynomial_roots_real_f32(
    coefficients: Vec<f32>,
) -> pyo3::PyResult<PolynomialRootsF32> {
    (crate::api::polynomial::polynomial_roots_real_f32(&coefficients))
        .map(|value| PolynomialRootsF32 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_evaluate_f64")]
#[pyo3(signature = (coefficients, real, imaginary))]
pub fn binding_polynomial_evaluate_f64(
    coefficients: pyo3::PyRef<'_, ComplexValuesF64>,
    real: f64,
    imaginary: f64,
) -> pyo3::PyResult<ComplexValuesF64> {
    (crate::api::polynomial::polynomial_evaluate_f64(
        coefficients.inner.as_ref().ok_or_else(consumed)?,
        real,
        imaginary,
    ))
    .map(|value| ComplexValuesF64 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_evaluate_real_f64")]
#[pyo3(signature = (coefficients, real, imaginary))]
pub fn binding_polynomial_evaluate_real_f64(
    coefficients: Vec<f64>,
    real: f64,
    imaginary: f64,
) -> pyo3::PyResult<ComplexValuesF64> {
    (crate::api::polynomial::polynomial_evaluate_real_f64(&coefficients, real, imaginary))
        .map(|value| ComplexValuesF64 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_roots_f64")]
#[pyo3(signature = (coefficients))]
pub fn binding_polynomial_roots_f64(
    coefficients: pyo3::PyRef<'_, ComplexValuesF64>,
) -> pyo3::PyResult<PolynomialRootsF64> {
    (crate::api::polynomial::polynomial_roots_f64(
        coefficients.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| PolynomialRootsF64 { inner: Some(value) })
}
#[pyfunction(name = "polynomial_roots_real_f64")]
#[pyo3(signature = (coefficients))]
pub fn binding_polynomial_roots_real_f64(
    coefficients: Vec<f64>,
) -> pyo3::PyResult<PolynomialRootsF64> {
    (crate::api::polynomial::polynomial_roots_real_f64(&coefficients))
        .map(|value| PolynomialRootsF64 { inner: Some(value) })
}
