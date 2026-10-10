use super::*;
#[napi(js_name = "dot_f32")]
pub fn binding_dot_f32(
    left: napi::bindgen_prelude::Float32Array,
    right: napi::bindgen_prelude::Float32Array,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    (crate::api::statistics::dot_f32(&storage_left, &storage_right)).map(|value| value as f64)
}
#[napi(js_name = "dot_f64")]
pub fn binding_dot_f64(
    left: napi::bindgen_prelude::Float64Array,
    right: napi::bindgen_prelude::Float64Array,
) -> napi::Result<f64> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    crate::api::statistics::dot_f64(&storage_left, &storage_right)
}
