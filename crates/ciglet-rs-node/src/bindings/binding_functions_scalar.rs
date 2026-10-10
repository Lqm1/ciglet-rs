use super::*;
#[napi(js_name = "scalar_minimum_f32")]
pub fn binding_scalar_minimum_f32(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::approximation::scalar_minimum_f32(first as f32, second as f32) as f64)
}
#[napi(js_name = "scalar_maximum_f32")]
pub fn binding_scalar_maximum_f32(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::approximation::scalar_maximum_f32(first as f32, second as f32) as f64)
}
#[napi(js_name = "scalar_minimum_f64")]
pub fn binding_scalar_minimum_f64(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::approximation::scalar_minimum_f64(first, second))
}
#[napi(js_name = "scalar_maximum_f64")]
pub fn binding_scalar_maximum_f64(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::approximation::scalar_maximum_f64(first, second))
}
#[napi(js_name = "scalar_math_constants")]
pub fn binding_scalar_math_constants() -> napi::Result<napi::bindgen_prelude::Float64Array> {
    Ok(<napi::bindgen_prelude::Float64Array>::from(
        crate::api::scalar::scalar_math_constants(),
    ))
}
#[napi(js_name = "scalar_add_f32")]
pub fn binding_scalar_add_f32(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::scalar::scalar_add_f32(first as f32, second as f32) as f64)
}
#[napi(js_name = "scalar_accumulate_square_f32")]
pub fn binding_scalar_accumulate_square_f32(accumulator: f64, value: f64) -> napi::Result<f64> {
    Ok(crate::api::scalar::scalar_accumulate_square_f32(accumulator as f32, value as f32) as f64)
}
#[napi(js_name = "scalar_fold_f32")]
pub fn binding_scalar_fold_f32(
    values: napi::bindgen_prelude::Float32Array,
    initial: f64,
    #[napi(
        ts_arg_type = "((accumulator: number, value: number, context: unknown) => number) | null | undefined"
    )]
    callback: Option<napi::bindgen_prelude::Unknown<'_>>,
    #[napi(ts_arg_type = "unknown")] context: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    (crate::api::scalar::scalar_fold_f32(
        &storage_values,
        initial as f32,
        callback
            .map(|value| crate::callbacks::Function::new(env, value))
            .transpose()?,
        &crate::callbacks::Value::new(env, context)?,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "scalar_add_f64")]
pub fn binding_scalar_add_f64(first: f64, second: f64) -> napi::Result<f64> {
    Ok(crate::api::scalar::scalar_add_f64(first, second))
}
#[napi(js_name = "scalar_accumulate_square_f64")]
pub fn binding_scalar_accumulate_square_f64(accumulator: f64, value: f64) -> napi::Result<f64> {
    Ok(crate::api::scalar::scalar_accumulate_square_f64(
        accumulator,
        value,
    ))
}
#[napi(js_name = "scalar_fold_f64")]
pub fn binding_scalar_fold_f64(
    values: napi::bindgen_prelude::Float64Array,
    initial: f64,
    #[napi(
        ts_arg_type = "((accumulator: number, value: number, context: unknown) => number) | null | undefined"
    )]
    callback: Option<napi::bindgen_prelude::Unknown<'_>>,
    #[napi(ts_arg_type = "unknown")] context: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<f64> {
    let storage_values = values.to_vec();
    crate::api::scalar::scalar_fold_f64(
        &storage_values,
        initial,
        callback
            .map(|value| crate::callbacks::Function::new(env, value))
            .transpose()?,
        &crate::callbacks::Value::new(env, context)?,
    )
}
