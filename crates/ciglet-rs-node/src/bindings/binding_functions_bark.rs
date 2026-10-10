use super::*;
#[napi(js_name = "bark_to_frequency_f32")]
pub fn binding_bark_to_frequency_f32(value: f64) -> napi::Result<f64> {
    (crate::api::frequency_distance::bark_to_frequency_f32(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "bark_to_frequency_f64")]
pub fn binding_bark_to_frequency_f64(value: f64) -> napi::Result<f64> {
    crate::api::frequency_distance::bark_to_frequency_f64(value)
}
