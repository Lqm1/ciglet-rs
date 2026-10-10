use super::*;
#[napi(js_name = "root_find_f32")]
pub fn binding_root_find_f32(
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    start: f64,
    end: f64,
    env: napi::Env,
) -> napi::Result<f64> {
    (crate::api::root::root_find_f32(
        &crate::callbacks::Function::new(env, callback)?,
        start as f32,
        end as f32,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "root_find_f64")]
pub fn binding_root_find_f64(
    #[napi(ts_arg_type = "(value: number) => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    start: f64,
    end: f64,
    env: napi::Env,
) -> napi::Result<f64> {
    crate::api::root::root_find_f64(&crate::callbacks::Function::new(env, callback)?, start, end)
}
