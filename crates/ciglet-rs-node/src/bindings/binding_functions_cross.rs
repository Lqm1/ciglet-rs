use super::*;
#[napi(js_name = "cross_correlation_f32")]
pub fn binding_cross_correlation_f32(
    left: napi::bindgen_prelude::Float32Array,
    right: napi::bindgen_prelude::Float32Array,
    lags: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    (crate::api::statistics::cross_correlation_f32(
        &storage_left,
        &storage_right,
        checked_usize(lags)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "cross_correlation_f64")]
pub fn binding_cross_correlation_f64(
    left: napi::bindgen_prelude::Float64Array,
    right: napi::bindgen_prelude::Float64Array,
    lags: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_left = left.to_vec();
    let storage_right = right.to_vec();
    (crate::api::statistics::cross_correlation_f64(
        &storage_left,
        &storage_right,
        checked_usize(lags)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
