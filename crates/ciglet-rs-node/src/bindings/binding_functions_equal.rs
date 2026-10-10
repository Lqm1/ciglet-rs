use super::*;
#[napi(js_name = "equal_loudness_f32")]
pub fn binding_equal_loudness_f32(value: f64) -> napi::Result<f64> {
    (crate::api::frequency_distance::equal_loudness_f32(value as f32)).map(|value| value as f64)
}
#[napi(js_name = "equal_loudness_f64")]
pub fn binding_equal_loudness_f64(value: f64) -> napi::Result<f64> {
    crate::api::frequency_distance::equal_loudness_f64(value)
}
