use super::*;
#[napi(js_name = "mel_to_frequency_f32")]
pub fn binding_mel_to_frequency_f32(value: f64) -> napi::Result<f64> {
    (crate::api::frequency_distance::mel_to_frequency_f32(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "mel_to_frequency_f64")]
pub fn binding_mel_to_frequency_f64(value: f64) -> napi::Result<f64> {
    crate::api::frequency_distance::mel_to_frequency_f64(value)
}
#[napi(js_name = "mel_space_f32")]
pub fn binding_mel_space_f32(
    minimum: f64,
    maximum: f64,
    intervals: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::frequency_distance::mel_space_f32(
        minimum as f32,
        maximum as f32,
        checked_usize(intervals)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "mel_space_f64")]
pub fn binding_mel_space_f64(
    minimum: f64,
    maximum: f64,
    intervals: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::frequency_distance::mel_space_f64(minimum, maximum, checked_usize(intervals)?))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
