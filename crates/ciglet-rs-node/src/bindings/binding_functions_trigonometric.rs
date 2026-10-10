use super::*;
#[napi(js_name = "trigonometric_sine")]
pub fn binding_trigonometric_sine(
    angle: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::trigonometric::trigonometric_sine(angle as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "trigonometric_cosine")]
pub fn binding_trigonometric_cosine(
    angle: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::trigonometric::trigonometric_cosine(angle as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "trigonometric_tangent")]
pub fn binding_trigonometric_tangent(
    angle: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::trigonometric::trigonometric_tangent(angle as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "trigonometric_tangent_full")]
pub fn binding_trigonometric_tangent_full(
    angle: f64,
    precision: ApproximationPrecision,
) -> napi::Result<f64> {
    (crate::api::trigonometric::trigonometric_tangent_full(angle as f32, precision))
        .map(|value| value as f64)
}
#[napi(js_name = "trigonometric_sine_lanes")]
pub fn binding_trigonometric_sine_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_sine_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "trigonometric_cosine_lanes")]
pub fn binding_trigonometric_cosine_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_cosine_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "trigonometric_tangent_lanes")]
pub fn binding_trigonometric_tangent_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_tangent_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "trigonometric_sine_full_lanes")]
pub fn binding_trigonometric_sine_full_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_sine_full_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "trigonometric_cosine_full_lanes")]
pub fn binding_trigonometric_cosine_full_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_cosine_full_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "trigonometric_tangent_full_lanes")]
pub fn binding_trigonometric_tangent_full_lanes(
    angles: napi::bindgen_prelude::Float32Array,
    precision: ApproximationPrecision,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_angles = angles.to_vec();
    (crate::api::trigonometric::trigonometric_tangent_full_lanes(&storage_angles, precision))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
