use super::*;
#[napi(js_name = "magnitude_f32")]
pub fn binding_magnitude_f32(
    real: napi::bindgen_prelude::Float32Array,
    imaginary: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_real = real.to_vec();
    let storage_imaginary = imaginary.to_vec();
    (crate::api::phase::magnitude_f32(&storage_real, &storage_imaginary))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "magnitude_f64")]
pub fn binding_magnitude_f64(
    real: napi::bindgen_prelude::Float64Array,
    imaginary: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_real = real.to_vec();
    let storage_imaginary = imaginary.to_vec();
    (crate::api::phase::magnitude_f64(&storage_real, &storage_imaginary))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
