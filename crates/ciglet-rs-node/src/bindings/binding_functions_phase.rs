use super::*;
#[napi(js_name = "phase_wrap_f32")]
pub fn binding_phase_wrap_f32(angle: f64) -> napi::Result<f64> {
    (crate::api::phase::phase_wrap_f32(angle as f32)).map(|value| value as f64)
}
#[napi(js_name = "phase_unwrap_f32")]
pub fn binding_phase_unwrap_f32(
    angles: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::phase::phase_unwrap_f32(&storage_angles))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "phase_difference_f32")]
pub fn binding_phase_difference_f32(first: f64, second: f64) -> napi::Result<f64> {
    (crate::api::phase::phase_difference_f32(first as f32, second as f32)).map(|value| value as f64)
}
#[napi(js_name = "phase_wrap_f64")]
pub fn binding_phase_wrap_f64(angle: f64) -> napi::Result<f64> {
    crate::api::phase::phase_wrap_f64(angle)
}
#[napi(js_name = "phase_unwrap_f64")]
pub fn binding_phase_unwrap_f64(
    angles: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_angles = angles.to_vec();
    (crate::api::phase::phase_unwrap_f64(&storage_angles))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "phase_difference_f64")]
pub fn binding_phase_difference_f64(first: f64, second: f64) -> napi::Result<f64> {
    crate::api::phase::phase_difference_f64(first, second)
}
