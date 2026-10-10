use super::*;
#[napi(js_name = "dct_f32")]
pub fn binding_dct_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::phase::dct_f32(&storage_values)).map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "dct_f64")]
pub fn binding_dct_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    (crate::api::phase::dct_f64(&storage_values)).map(<napi::bindgen_prelude::Float64Array>::from)
}
