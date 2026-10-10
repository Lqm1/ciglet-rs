use super::*;
#[napi(js_name = "polar_real_f32")]
pub fn binding_polar_real_f32(
    magnitudes: napi::bindgen_prelude::Float32Array,
    angles: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_magnitudes = magnitudes.to_vec();
    let storage_angles = angles.to_vec();
    (crate::api::phase::polar_real_f32(&storage_magnitudes, &storage_angles))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "polar_imaginary_f32")]
pub fn binding_polar_imaginary_f32(
    magnitudes: napi::bindgen_prelude::Float32Array,
    angles: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_magnitudes = magnitudes.to_vec();
    let storage_angles = angles.to_vec();
    (crate::api::phase::polar_imaginary_f32(&storage_magnitudes, &storage_angles))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "polar_real_f64")]
pub fn binding_polar_real_f64(
    magnitudes: napi::bindgen_prelude::Float64Array,
    angles: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_magnitudes = magnitudes.to_vec();
    let storage_angles = angles.to_vec();
    (crate::api::phase::polar_real_f64(&storage_magnitudes, &storage_angles))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "polar_imaginary_f64")]
pub fn binding_polar_imaginary_f64(
    magnitudes: napi::bindgen_prelude::Float64Array,
    angles: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_magnitudes = magnitudes.to_vec();
    let storage_angles = angles.to_vec();
    (crate::api::phase::polar_imaginary_f64(&storage_magnitudes, &storage_angles))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
