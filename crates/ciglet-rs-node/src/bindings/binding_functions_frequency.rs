use super::*;
#[napi(js_name = "frequency_to_mel_f32")]
pub fn binding_frequency_to_mel_f32(value: f64) -> napi::Result<f64> {
    (crate::api::frequency_distance::frequency_to_mel_f32(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "frequency_to_mel_f64")]
pub fn binding_frequency_to_mel_f64(value: f64) -> napi::Result<f64> {
    crate::api::frequency_distance::frequency_to_mel_f64(value)
}
#[napi(js_name = "frequency_to_bark_f32")]
pub fn binding_frequency_to_bark_f32(value: f64) -> napi::Result<f64> {
    (crate::api::frequency_distance::frequency_to_bark_f32(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "frequency_to_bark_f64")]
pub fn binding_frequency_to_bark_f64(value: f64) -> napi::Result<f64> {
    crate::api::frequency_distance::frequency_to_bark_f64(value)
}
