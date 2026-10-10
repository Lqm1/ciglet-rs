use super::*;
#[napi(js_name = "approximation_sine_f32")]
pub fn binding_approximation_sine_f32(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::approximation::approximation_sine_f32(value as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_sine_f64")]
pub fn binding_approximation_sine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    crate::api::approximation::approximation_sine_f64(value, precision)
}
#[napi(js_name = "approximation_cosine_f32")]
pub fn binding_approximation_cosine_f32(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::approximation::approximation_cosine_f32(value as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_cosine_f64")]
pub fn binding_approximation_cosine_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    crate::api::approximation::approximation_cosine_f64(value, precision)
}
#[napi(js_name = "approximation_logarithm_f32")]
pub fn binding_approximation_logarithm_f32(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::approximation::approximation_logarithm_f32(value as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_logarithm_f64")]
pub fn binding_approximation_logarithm_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    crate::api::approximation::approximation_logarithm_f64(value, precision)
}
#[napi(js_name = "approximation_exponential_f32")]
pub fn binding_approximation_exponential_f32(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::approximation::approximation_exponential_f32(value as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_exponential_f64")]
pub fn binding_approximation_exponential_f64(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    crate::api::approximation::approximation_exponential_f64(value, precision)
}
#[napi(js_name = "approximation_atan2_f32")]
pub fn binding_approximation_atan2_f32(
    y: f64,
    x: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::approximation::approximation_atan2_f32(y as f32, x as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_atan2_fast_f32")]
pub fn binding_approximation_atan2_fast_f32(y: f64, x: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_atan2_fast_f32(y as f32, x as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_atan2_f64")]
pub fn binding_approximation_atan2_f64(
    y: f64,
    x: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    crate::api::approximation::approximation_atan2_f64(y, x, precision)
}
#[napi(js_name = "approximation_atan2_fast_f64")]
pub fn binding_approximation_atan2_fast_f64(y: f64, x: f64) -> napi::Result<f64> {
    crate::api::approximation::approximation_atan2_fast_f64(y, x)
}
#[napi(js_name = "approximation_sine_fast")]
pub fn binding_approximation_sine_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_sine_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_cosine_fast")]
pub fn binding_approximation_cosine_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_cosine_fast(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_sine")]
pub fn binding_approximation_sine(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_sine(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_cosine")]
pub fn binding_approximation_cosine(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_cosine(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_logarithm2_fast")]
pub fn binding_approximation_logarithm2_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_logarithm2_fast(value as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_logarithm_fast")]
pub fn binding_approximation_logarithm_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_logarithm_fast(value as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_exponential2_fast")]
pub fn binding_approximation_exponential2_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_exponential2_fast(value as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_exponential_fast")]
pub fn binding_approximation_exponential_fast(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_exponential_fast(value as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_logarithm2")]
pub fn binding_approximation_logarithm2(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_logarithm2(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_logarithm")]
pub fn binding_approximation_logarithm(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_logarithm(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_exponential2")]
pub fn binding_approximation_exponential2(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_exponential2(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_exponential")]
pub fn binding_approximation_exponential(value: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_exponential(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "approximation_power")]
pub fn binding_approximation_power(value: f64, exponent: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_power(value as f32, exponent as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "approximation_power_fast")]
pub fn binding_approximation_power_fast(value: f64, exponent: f64) -> napi::Result<f64> {
    (crate::api::approximation::approximation_power_fast(value as f32, exponent as f32))
        .map(|value| value as f64)
}
