use super::*;
#[napi(js_name = "window_f32")]
pub fn binding_window_f32(
    kind: WindowKind,
    length: f64,
    precision: WindowPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::window::window_f32(kind, checked_usize(length)?, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "window_f64")]
pub fn binding_window_f64(
    kind: WindowKind,
    length: f64,
    precision: WindowPrecision,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::window::window_f64(kind, checked_usize(length)?, precision))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "window_custom_f32_f32")]
pub fn binding_window_custom_f32_f32(
    kind: WindowKind,
    length: f64,
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::window::window_custom_f32_f32(
        kind,
        checked_usize(length)?,
        &crate::callbacks::Function::new(env, callback)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "window_custom_f32_f64")]
pub fn binding_window_custom_f32_f64(
    kind: WindowKind,
    length: f64,
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::window::window_custom_f32_f64(
        kind,
        checked_usize(length)?,
        &crate::callbacks::Function::new(env, callback)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "window_custom_f64_f32")]
pub fn binding_window_custom_f64_f32(
    kind: WindowKind,
    length: f64,
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::window::window_custom_f64_f32(
        kind,
        checked_usize(length)?,
        &crate::callbacks::Function::new(env, callback)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "window_custom_f64_f64")]
pub fn binding_window_custom_f64_f64(
    kind: WindowKind,
    length: f64,
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::window::window_custom_f64_f64(
        kind,
        checked_usize(length)?,
        &crate::callbacks::Function::new(env, callback)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
