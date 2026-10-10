use super::*;
#[napi(js_name = "variance_f32")]
pub fn binding_variance_f32(values: napi::bindgen_prelude::Float32Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::statistics::variance_f32(&storage_values)).map(|value| value as f64)
}
#[napi(js_name = "variance_with_f32")]
pub fn binding_variance_with_f32(
    values: napi::bindgen_prelude::Float32Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::statistics::variance_with_f32(&storage_values, policy)).map(|value| value as f64)
}
#[napi(js_name = "variance_f64")]
pub fn binding_variance_f64(values: napi::bindgen_prelude::Float64Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::statistics::variance_f64(&storage_values)
}
#[napi(js_name = "variance_with_f64")]
pub fn binding_variance_with_f64(
    values: napi::bindgen_prelude::Float64Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::statistics::variance_with_f64(&storage_values, policy)
}
