use super::*;
#[napi(js_name = "convolve_f32")]
pub fn binding_convolve_f32(
    signal: napi::bindgen_prelude::Float32Array,
    impulse: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    let storage_impulse = impulse.to_vec();
    (crate::api::filtering::convolve_f32(&storage_signal, &storage_impulse))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "convolve_f64")]
pub fn binding_convolve_f64(
    signal: napi::bindgen_prelude::Float64Array,
    impulse: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    let storage_impulse = impulse.to_vec();
    (crate::api::filtering::convolve_f64(&storage_signal, &storage_impulse))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
