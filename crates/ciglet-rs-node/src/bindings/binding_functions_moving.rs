use super::*;
#[napi(js_name = "moving_average_f32")]
pub fn binding_moving_average_f32(
    signal: napi::bindgen_prelude::Float32Array,
    half_width: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_average_f32(&storage_signal, half_width as f32))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "moving_average_with_f32")]
pub fn binding_moving_average_with_f32(
    signal: napi::bindgen_prelude::Float32Array,
    half_width: f64,
    policy: SmoothingPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_average_with_f32(&storage_signal, half_width as f32, policy))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "moving_rms_f32")]
pub fn binding_moving_rms_f32(
    signal: napi::bindgen_prelude::Float32Array,
    half_width: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_rms_f32(&storage_signal, checked_usize(half_width)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "moving_rms_with_f32")]
pub fn binding_moving_rms_with_f32(
    signal: napi::bindgen_prelude::Float32Array,
    half_width: f64,
    policy: SmoothingPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_rms_with_f32(
        &storage_signal,
        checked_usize(half_width)?,
        policy,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "moving_average_f64")]
pub fn binding_moving_average_f64(
    signal: napi::bindgen_prelude::Float64Array,
    half_width: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_average_f64(&storage_signal, half_width))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "moving_average_with_f64")]
pub fn binding_moving_average_with_f64(
    signal: napi::bindgen_prelude::Float64Array,
    half_width: f64,
    policy: SmoothingPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_average_with_f64(&storage_signal, half_width, policy))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "moving_rms_f64")]
pub fn binding_moving_rms_f64(
    signal: napi::bindgen_prelude::Float64Array,
    half_width: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_rms_f64(&storage_signal, checked_usize(half_width)?))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "moving_rms_with_f64")]
pub fn binding_moving_rms_with_f64(
    signal: napi::bindgen_prelude::Float64Array,
    half_width: f64,
    policy: SmoothingPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::moving_rms_with_f64(
        &storage_signal,
        checked_usize(half_width)?,
        policy,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
