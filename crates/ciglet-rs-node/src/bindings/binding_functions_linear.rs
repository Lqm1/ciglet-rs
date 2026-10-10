use super::*;
#[napi(js_name = "linear_grid_f32")]
pub fn binding_linear_grid_f32(
    start: f64,
    end: f64,
    count: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::vector::linear_grid_f32(start as f32, end as f32, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "linear_grid_f64")]
pub fn binding_linear_grid_f64(
    start: f64,
    end: f64,
    count: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::vector::linear_grid_f64(start, end, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
