use super::*;
#[napi(js_name = "sinusoid_f32")]
pub fn binding_sinusoid_f32(
    tone: &SinusoidF32,
    sample_rate: u32,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::synthesis::sinusoid_f32(
        tone.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        sample_rate,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "sinusoid_f64")]
pub fn binding_sinusoid_f64(
    tone: &SinusoidF64,
    sample_rate: u32,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::synthesis::sinusoid_f64(
        tone.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        sample_rate,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
