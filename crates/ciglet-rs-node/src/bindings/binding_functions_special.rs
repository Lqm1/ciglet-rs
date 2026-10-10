use super::*;
#[napi(js_name = "special_complementary_error")]
pub fn binding_special_complementary_error(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_complementary_error(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_complementary_error_lanes")]
pub fn binding_special_complementary_error_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_complementary_error_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_complementary_error_fast")]
pub fn binding_special_complementary_error_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_complementary_error_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_complementary_error_fast_lanes")]
pub fn binding_special_complementary_error_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_complementary_error_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_error")]
pub fn binding_special_error(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_error(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_error_lanes")]
pub fn binding_special_error_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_error_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_error_fast")]
pub fn binding_special_error_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_error_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_error_fast_lanes")]
pub fn binding_special_error_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_error_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_inverse_error")]
pub fn binding_special_inverse_error(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_inverse_error(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_inverse_error_lanes")]
pub fn binding_special_inverse_error_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_inverse_error_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_inverse_error_fast")]
pub fn binding_special_inverse_error_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_inverse_error_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_inverse_error_fast_lanes")]
pub fn binding_special_inverse_error_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_inverse_error_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_log_gamma")]
pub fn binding_special_log_gamma(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_log_gamma(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_log_gamma_lanes")]
pub fn binding_special_log_gamma_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_log_gamma_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_log_gamma_fast")]
pub fn binding_special_log_gamma_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_log_gamma_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_log_gamma_fast_lanes")]
pub fn binding_special_log_gamma_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_log_gamma_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_digamma")]
pub fn binding_special_digamma(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_digamma(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_digamma_lanes")]
pub fn binding_special_digamma_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_digamma_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_digamma_fast")]
pub fn binding_special_digamma_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_digamma_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_digamma_fast_lanes")]
pub fn binding_special_digamma_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_digamma_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_lambert_w")]
pub fn binding_special_lambert_w(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_lambert_w(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_lambert_w_lanes")]
pub fn binding_special_lambert_w_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_lambert_w_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_lambert_w_fast")]
pub fn binding_special_lambert_w_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_lambert_w_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_lambert_w_fast_lanes")]
pub fn binding_special_lambert_w_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_lambert_w_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_lambert_w_exp")]
pub fn binding_special_lambert_w_exp(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_lambert_w_exp(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_lambert_w_exp_lanes")]
pub fn binding_special_lambert_w_exp_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_lambert_w_exp_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "special_lambert_w_exp_fast")]
pub fn binding_special_lambert_w_exp_fast(value: f64) -> napi::Result<f64> {
    (crate::api::special::special_lambert_w_exp_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "special_lambert_w_exp_fast_lanes")]
pub fn binding_special_lambert_w_exp_fast_lanes(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::special::special_lambert_w_exp_fast_lanes(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
