use super::*;
#[napi(js_name = "resample_f32")]
pub fn binding_resample_f32(
    signal: napi::bindgen_prelude::Float32Array,
    ratio: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_f32(&storage_signal, ratio as f32))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "resample_with_f32")]
pub fn binding_resample_with_f32(
    signal: napi::bindgen_prelude::Float32Array,
    ratio: f64,
    boundary: BoundaryPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_with_f32(&storage_signal, ratio as f32, boundary))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "resample_with_kernel_f32")]
pub fn binding_resample_with_kernel_f32(
    signal: napi::bindgen_prelude::Float32Array,
    ratio: f64,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_with_kernel_f32(
        &storage_signal,
        ratio as f32,
        boundary,
        kernel,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "resample_output_length_f32")]
pub fn binding_resample_output_length_f32(count: f64, ratio: f64) -> napi::Result<f64> {
    (crate::api::resampling::resample_output_length_f32(checked_usize(count)?, ratio as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "resample_f64")]
pub fn binding_resample_f64(
    signal: napi::bindgen_prelude::Float64Array,
    ratio: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_f64(&storage_signal, ratio))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "resample_with_f64")]
pub fn binding_resample_with_f64(
    signal: napi::bindgen_prelude::Float64Array,
    ratio: f64,
    boundary: BoundaryPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_with_f64(&storage_signal, ratio, boundary))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "resample_with_kernel_f64")]
pub fn binding_resample_with_kernel_f64(
    signal: napi::bindgen_prelude::Float64Array,
    ratio: f64,
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    (crate::api::resampling::resample_with_kernel_f64(&storage_signal, ratio, boundary, kernel))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "resample_output_length_f64")]
pub fn binding_resample_output_length_f64(count: f64, ratio: f64) -> napi::Result<f64> {
    (crate::api::resampling::resample_output_length_f64(checked_usize(count)?, ratio))
        .map(|value| value as f64)
}
