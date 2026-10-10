use super::*;
#[napi(js_name = "aliased_sinc_f32")]
pub fn binding_aliased_sinc_f32(length: f64, angle: f64) -> napi::Result<f64> {
    (crate::api::sinc::aliased_sinc_f32(length as f32, angle as f32)).map(|value| value as f64)
}
#[napi(js_name = "aliased_sinc_with_f32")]
pub fn binding_aliased_sinc_with_f32(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> napi::Result<f64> {
    (crate::api::sinc::aliased_sinc_with_f32(length as f32, angle as f32, policy))
        .map(|value| value as f64)
}
#[napi(js_name = "aliased_sinc_derivative_f32")]
pub fn binding_aliased_sinc_derivative_f32(length: f64, angle: f64) -> napi::Result<f64> {
    (crate::api::sinc::aliased_sinc_derivative_f32(length as f32, angle as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "aliased_sinc_derivative_with_f32")]
pub fn binding_aliased_sinc_derivative_with_f32(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> napi::Result<f64> {
    (crate::api::sinc::aliased_sinc_derivative_with_f32(length as f32, angle as f32, policy))
        .map(|value| value as f64)
}
#[napi(js_name = "aliased_sinc_f64")]
pub fn binding_aliased_sinc_f64(length: f64, angle: f64) -> napi::Result<f64> {
    crate::api::sinc::aliased_sinc_f64(length, angle)
}
#[napi(js_name = "aliased_sinc_with_f64")]
pub fn binding_aliased_sinc_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> napi::Result<f64> {
    crate::api::sinc::aliased_sinc_with_f64(length, angle, policy)
}
#[napi(js_name = "aliased_sinc_derivative_f64")]
pub fn binding_aliased_sinc_derivative_f64(length: f64, angle: f64) -> napi::Result<f64> {
    crate::api::sinc::aliased_sinc_derivative_f64(length, angle)
}
#[napi(js_name = "aliased_sinc_derivative_with_f64")]
pub fn binding_aliased_sinc_derivative_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> napi::Result<f64> {
    crate::api::sinc::aliased_sinc_derivative_with_f64(length, angle, policy)
}
