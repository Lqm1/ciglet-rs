use super::*;
#[napi(js_name = "maximum_f32")]
pub fn binding_maximum_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<Option<f64>> {
    let storage_values = values.to_vec();
    Ok((crate::api::ordering::maximum_f32(&storage_values)).map(|value| value as f64))
}
#[napi(js_name = "maximum_f64")]
pub fn binding_maximum_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<Option<f64>> {
    let storage_values = values.to_vec();
    Ok(crate::api::ordering::maximum_f64(&storage_values))
}
