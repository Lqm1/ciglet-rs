use super::*;
#[napi(js_name = "correlation_f32")]
pub fn binding_correlation_f32(
    left: napi::bindgen_prelude::Float32Array,
    right: napi::bindgen_prelude::Float32Array,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    (crate::api::statistics::correlation_f32(&storage_left, &storage_right))
        .map(|value| value as f64)
}
#[napi(js_name = "correlation_with_f32")]
pub fn binding_correlation_with_f32(
    left: napi::bindgen_prelude::Float32Array,
    right: napi::bindgen_prelude::Float32Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    (crate::api::statistics::correlation_with_f32(&storage_left, &storage_right, policy))
        .map(|value| value as f64)
}
#[napi(js_name = "correlation_f64")]
pub fn binding_correlation_f64(
    left: napi::bindgen_prelude::Float64Array,
    right: napi::bindgen_prelude::Float64Array,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    crate::api::statistics::correlation_f64(&storage_left, &storage_right)
}
#[napi(js_name = "correlation_with_f64")]
pub fn binding_correlation_with_f64(
    left: napi::bindgen_prelude::Float64Array,
    right: napi::bindgen_prelude::Float64Array,
    policy: StatisticsPolicy,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    crate::api::statistics::correlation_with_f64(&storage_left, &storage_right, policy)
}
