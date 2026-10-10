use super::*;
#[napi(js_name = "polynomial_evaluate_f32")]
pub fn binding_polynomial_evaluate_f32(
    coefficients: &ComplexValuesF32,
    real: f64,
    imaginary: f64,
) -> napi::Result<ComplexValuesF32> {
    (crate::api::polynomial::polynomial_evaluate_f32(
        coefficients
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        real as f32,
        imaginary as f32,
    ))
    .map(|value| ComplexValuesF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "polynomial_evaluate_real_f32")]
pub fn binding_polynomial_evaluate_real_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
    real: f64,
    imaginary: f64,
) -> napi::Result<ComplexValuesF32> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::polynomial::polynomial_evaluate_real_f32(
        &storage_coefficients,
        real as f32,
        imaginary as f32,
    ))
    .map(|value| ComplexValuesF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "polynomial_roots_f32")]
pub fn binding_polynomial_roots_f32(
    coefficients: &ComplexValuesF32,
) -> napi::Result<PolynomialRootsF32> {
    (crate::api::polynomial::polynomial_roots_f32(
        coefficients
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| PolynomialRootsF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "polynomial_roots_real_f32")]
pub fn binding_polynomial_roots_real_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
) -> napi::Result<PolynomialRootsF32> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::polynomial::polynomial_roots_real_f32(&storage_coefficients)).map(|value| {
        PolynomialRootsF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
#[napi(js_name = "polynomial_evaluate_f64")]
pub fn binding_polynomial_evaluate_f64(
    coefficients: &ComplexValuesF64,
    real: f64,
    imaginary: f64,
) -> napi::Result<ComplexValuesF64> {
    (crate::api::polynomial::polynomial_evaluate_f64(
        coefficients
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        real,
        imaginary,
    ))
    .map(|value| ComplexValuesF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "polynomial_evaluate_real_f64")]
pub fn binding_polynomial_evaluate_real_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
    real: f64,
    imaginary: f64,
) -> napi::Result<ComplexValuesF64> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::polynomial::polynomial_evaluate_real_f64(&storage_coefficients, real, imaginary))
        .map(|value| ComplexValuesF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
}
#[napi(js_name = "polynomial_roots_f64")]
pub fn binding_polynomial_roots_f64(
    coefficients: &ComplexValuesF64,
) -> napi::Result<PolynomialRootsF64> {
    (crate::api::polynomial::polynomial_roots_f64(
        coefficients
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| PolynomialRootsF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "polynomial_roots_real_f64")]
pub fn binding_polynomial_roots_real_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
) -> napi::Result<PolynomialRootsF64> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::polynomial::polynomial_roots_real_f64(&storage_coefficients)).map(|value| {
        PolynomialRootsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
