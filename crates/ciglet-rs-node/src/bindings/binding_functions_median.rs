use super::*;
#[napi(js_name = "median_f32")]
pub fn binding_median_f32(values: napi::bindgen_prelude::Float32Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::ordering::median_f32(&storage_values)).map(|value| value as f64)
}
#[napi(js_name = "median_f64")]
pub fn binding_median_f64(values: napi::bindgen_prelude::Float64Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::ordering::median_f64(&storage_values)
}
#[napi(js_name = "median_filter_f32")]
pub fn binding_median_filter_f32(
    signal: napi::bindgen_prelude::Float32Array,
    width: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::median_filter_f32(&storage_signal, checked_usize(width)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "median_filter_f64")]
pub fn binding_median_filter_f64(
    signal: napi::bindgen_prelude::Float64Array,
    width: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::smoothing::median_filter_f64(&storage_signal, checked_usize(width)?))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
