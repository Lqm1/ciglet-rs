use super::*;
#[napi(js_name = "interpolate_linear_f32")]
pub fn binding_interpolate_linear_f32(
    coordinates: napi::bindgen_prelude::Float32Array,
    values: napi::bindgen_prelude::Float32Array,
    positions: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_coordinates = coordinates.to_vec();
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_linear_f32(
        &storage_coordinates,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_uniform_f32")]
pub fn binding_interpolate_uniform_f32(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float32Array,
    positions: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_uniform_f32(
        start as f32,
        end as f32,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_sinc_f32")]
pub fn binding_interpolate_sinc_f32(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float32Array,
    positions: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_f32(
        start as f32,
        end as f32,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_sinc_with_f32")]
pub fn binding_interpolate_sinc_with_f32(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float32Array,
    positions: napi::bindgen_prelude::Float32Array,
    boundary: BoundaryPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_with_f32(
        start as f32,
        end as f32,
        &storage_values,
        &storage_positions,
        boundary,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_sinc_with_kernel_f32")]
pub fn binding_interpolate_sinc_with_kernel_f32(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float32Array,
    positions: napi::bindgen_prelude::Float32Array,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_with_kernel_f32(
        start as f32,
        end as f32,
        &storage_values,
        &storage_positions,
        boundary,
        kernel,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_fill_missing_f32")]
pub fn binding_interpolate_fill_missing_f32(
    signal: napi::bindgen_prelude::Float32Array,
    blank: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::interpolation::interpolate_fill_missing_f32(&storage_signal, blank as f32))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "interpolate_linear_f64")]
pub fn binding_interpolate_linear_f64(
    coordinates: napi::bindgen_prelude::Float64Array,
    values: napi::bindgen_prelude::Float64Array,
    positions: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_coordinates = coordinates.to_vec();
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_linear_f64(
        &storage_coordinates,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_uniform_f64")]
pub fn binding_interpolate_uniform_f64(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float64Array,
    positions: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_uniform_f64(
        start,
        end,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_sinc_f64")]
pub fn binding_interpolate_sinc_f64(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float64Array,
    positions: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_f64(
        start,
        end,
        &storage_values,
        &storage_positions,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_sinc_with_f64")]
pub fn binding_interpolate_sinc_with_f64(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float64Array,
    positions: napi::bindgen_prelude::Float64Array,
    boundary: BoundaryPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_with_f64(
        start,
        end,
        &storage_values,
        &storage_positions,
        boundary,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_sinc_with_kernel_f64")]
pub fn binding_interpolate_sinc_with_kernel_f64(
    start: f64,
    end: f64,
    values: napi::bindgen_prelude::Float64Array,
    positions: napi::bindgen_prelude::Float64Array,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    let storage_positions = positions.to_vec();
    (crate::api::interpolation::interpolate_sinc_with_kernel_f64(
        start,
        end,
        &storage_values,
        &storage_positions,
        boundary,
        kernel,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_fill_missing_f64")]
pub fn binding_interpolate_fill_missing_f64(
    signal: napi::bindgen_prelude::Float64Array,
    blank: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::interpolation::interpolate_fill_missing_f64(&storage_signal, blank))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "interpolate_between_f32")]
pub fn binding_interpolate_between_f32(start: f64, end: f64, ratio: f64) -> napi::Result<f64> {
    (crate::api::vector::interpolate_between_f32(start as f32, end as f32, ratio as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "interpolate_between_f64")]
pub fn binding_interpolate_between_f64(start: f64, end: f64, ratio: f64) -> napi::Result<f64> {
    crate::api::vector::interpolate_between_f64(start, end, ratio)
}
