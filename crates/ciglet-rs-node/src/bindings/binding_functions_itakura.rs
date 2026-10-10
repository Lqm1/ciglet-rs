use super::*;
#[napi(js_name = "itakura_saito_f32")]
pub fn binding_itakura_saito_f32(
    source: napi::bindgen_prelude::Float32Array,
    reference: napi::bindgen_prelude::Float32Array,
) -> napi::Result<f64> {
    let storage_source = source.to_vec();
    let storage_reference = reference.to_vec();
    (crate::api::frequency_distance::itakura_saito_f32(&storage_source, &storage_reference))
        .map(|value| value as f64)
}
#[napi(js_name = "itakura_saito_f64")]
pub fn binding_itakura_saito_f64(
    source: napi::bindgen_prelude::Float64Array,
    reference: napi::bindgen_prelude::Float64Array,
) -> napi::Result<f64> {
    let storage_source = source.to_vec();
    let storage_reference = reference.to_vec();
    crate::api::frequency_distance::itakura_saito_f64(&storage_source, &storage_reference)
}
