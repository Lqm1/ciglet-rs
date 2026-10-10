use super::*;
#[napi(js_name = "sum_f32")]
pub fn binding_sum_f32(values: napi::bindgen_prelude::Float32Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    Ok(crate::api::statistics::sum_f32(&storage_values) as f64)
}
#[napi(js_name = "sum_f64")]
pub fn binding_sum_f64(values: napi::bindgen_prelude::Float64Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    Ok(crate::api::statistics::sum_f64(&storage_values))
}
#[napi(js_name = "sum_squares_f32")]
pub fn binding_sum_squares_f32(values: napi::bindgen_prelude::Float32Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    Ok(crate::api::statistics::sum_squares_f32(&storage_values) as f64)
}
#[napi(js_name = "sum_squares_f64")]
pub fn binding_sum_squares_f64(values: napi::bindgen_prelude::Float64Array) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    Ok(crate::api::statistics::sum_squares_f64(&storage_values))
}
