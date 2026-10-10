use super::*;
#[napi(js_name = "forward_backward_filter_f32")]
pub fn binding_forward_backward_filter_f32(
    signal: napi::bindgen_prelude::Float32Array,
    numerator: napi::bindgen_prelude::Float32Array,
    denominator: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    let storage_numerator = numerator.to_vec();
    let storage_denominator = denominator.to_vec();
    (crate::api::filtering::forward_backward_filter_f32(
        &storage_signal,
        &storage_numerator,
        &storage_denominator,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "forward_backward_filter_with_f32")]
pub fn binding_forward_backward_filter_with_f32(
    signal: napi::bindgen_prelude::Float32Array,
    numerator: napi::bindgen_prelude::Float32Array,
    denominator: napi::bindgen_prelude::Float32Array,
    startup: FilterStartup,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_signal = signal.to_vec();
    let storage_numerator = numerator.to_vec();
    let storage_denominator = denominator.to_vec();
    (crate::api::filtering::forward_backward_filter_with_f32(
        &storage_signal,
        &storage_numerator,
        &storage_denominator,
        startup,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "forward_backward_filter_f64")]
pub fn binding_forward_backward_filter_f64(
    signal: napi::bindgen_prelude::Float64Array,
    numerator: napi::bindgen_prelude::Float64Array,
    denominator: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    let storage_numerator = numerator.to_vec();
    let storage_denominator = denominator.to_vec();
    (crate::api::filtering::forward_backward_filter_f64(
        &storage_signal,
        &storage_numerator,
        &storage_denominator,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "forward_backward_filter_with_f64")]
pub fn binding_forward_backward_filter_with_f64(
    signal: napi::bindgen_prelude::Float64Array,
    numerator: napi::bindgen_prelude::Float64Array,
    denominator: napi::bindgen_prelude::Float64Array,
    startup: FilterStartup,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_signal = signal.to_vec();
    let storage_numerator = numerator.to_vec();
    let storage_denominator = denominator.to_vec();
    (crate::api::filtering::forward_backward_filter_with_f64(
        &storage_signal,
        &storage_numerator,
        &storage_denominator,
        startup,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
