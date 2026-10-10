use super::*;
#[napi(js_name = "hyperbolic_sine")]
pub fn binding_hyperbolic_sine(value: f64, precision: ApproximationPrecision) -> napi::Result<f64> {
    (crate::api::hyperbolic::hyperbolic_sine(value as f32, precision)).map(|value| value as f64)
}
#[napi(js_name = "hyperbolic_sine_lanes")]
pub fn binding_hyperbolic_sine_lanes(
    values: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::hyperbolic::hyperbolic_sine_lanes(&storage_values, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "hyperbolic_cosine")]
pub fn binding_hyperbolic_cosine(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::hyperbolic::hyperbolic_cosine(value as f32, precision)).map(|value| value as f64)
}
#[napi(js_name = "hyperbolic_cosine_lanes")]
pub fn binding_hyperbolic_cosine_lanes(
    values: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::hyperbolic::hyperbolic_cosine_lanes(&storage_values, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "hyperbolic_tangent")]
pub fn binding_hyperbolic_tangent(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::hyperbolic::hyperbolic_tangent(value as f32, precision)).map(|value| value as f64)
}
#[napi(js_name = "hyperbolic_tangent_lanes")]
pub fn binding_hyperbolic_tangent_lanes(
    values: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::hyperbolic::hyperbolic_tangent_lanes(&storage_values, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "hyperbolic_sigmoid")]
pub fn binding_hyperbolic_sigmoid(
    value: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::hyperbolic::hyperbolic_sigmoid(value as f32, precision)).map(|value| value as f64)
}
#[napi(js_name = "hyperbolic_sigmoid_lanes")]
pub fn binding_hyperbolic_sigmoid_lanes(
    values: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::hyperbolic::hyperbolic_sigmoid_lanes(&storage_values, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
