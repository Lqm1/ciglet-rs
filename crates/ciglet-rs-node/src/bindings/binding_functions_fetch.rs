use super::*;
#[napi(js_name = "fetch_frame_f32")]
pub fn binding_fetch_frame_f32(
    input: napi::bindgen_prelude::Float32Array,
    center: i64,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_input = input.to_vec();
    (crate::api::frame_extrema::fetch_frame_f32(
        &storage_input,
        isize::try_from(center)
            .map_err(|_| napi::Error::from_reason("integer is outside isize range"))?,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "fetch_frame_f64")]
pub fn binding_fetch_frame_f64(
    input: napi::bindgen_prelude::Float64Array,
    center: i64,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_input = input.to_vec();
    (crate::api::frame_extrema::fetch_frame_f64(
        &storage_input,
        isize::try_from(center)
            .map_err(|_| napi::Error::from_reason("integer is outside isize range"))?,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
