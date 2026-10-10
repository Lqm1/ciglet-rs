use super::*;
#[napi(js_name = "fir_f32")]
pub fn binding_fir_f32(
    length: f64,
    cutoff: f64,
    highpass: bool,
    window: WindowKind,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::filtering::fir_f32(checked_usize(length)?, cutoff as f32, highpass, window))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "fir_f64")]
pub fn binding_fir_f64(
    length: f64,
    cutoff: f64,
    highpass: bool,
    window: WindowKind,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::filtering::fir_f64(checked_usize(length)?, cutoff, highpass, window))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
