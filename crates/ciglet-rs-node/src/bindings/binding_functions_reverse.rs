use super::*;
#[napi(js_name = "reverse_f32")]
pub fn binding_reverse_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    Ok(<napi::bindgen_prelude::Float32Array>::from(
        crate::api::vector::reverse_f32(&storage_values),
    ))
}
#[napi(js_name = "reverse_f64")]
pub fn binding_reverse_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    Ok(<napi::bindgen_prelude::Float64Array>::from(
        crate::api::vector::reverse_f64(&storage_values),
    ))
}
