use super::*;
#[napi(js_name = "mean_f32")]
pub fn binding_mean_f32(values: napi::bindgen_prelude::Float32Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::statistics::mean_f32(&storage_values)).map(|value| value as f64)
}
#[napi(js_name = "mean_with_f32")]
pub fn binding_mean_with_f32(
    values: napi::bindgen_prelude::Float32Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::statistics::mean_with_f32(&storage_values, policy)).map(|value| value as f64)
}
#[napi(js_name = "mean_f64")]
pub fn binding_mean_f64(values: napi::bindgen_prelude::Float64Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::statistics::mean_f64(&storage_values)
}
#[napi(js_name = "mean_with_f64")]
pub fn binding_mean_with_f64(
    values: napi::bindgen_prelude::Float64Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::statistics::mean_with_f64(&storage_values, policy)
}
