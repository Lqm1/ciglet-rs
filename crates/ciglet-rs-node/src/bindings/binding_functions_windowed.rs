use super::*;
#[napi(js_name = "windowed_fir_f32")]
pub fn binding_windowed_fir_f32(
    length: f64,
    kind: FilterPassband,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::filtering::windowed_fir_f32(
        checked_usize(length)?,
        kind,
        lower as f32,
        upper as f32,
        window,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "windowed_fir_f64")]
pub fn binding_windowed_fir_f64(
    length: f64,
    kind: FilterPassband,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::filtering::windowed_fir_f64(checked_usize(length)?, kind, lower, upper, window))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
