use super::*;
#[napi(js_name = "extremum_f32")]
pub fn binding_extremum_f32(
    signal: napi::bindgen_prelude::Float32Array,
    start: f64,
    end: f64,
    direction: ExtremumDirection,
) -> napi::Result<f64> {
    let storage_signal = signal.to_vec();
    (crate::api::frame_extrema::extremum_f32(
        &storage_signal,
        checked_usize(start)?,
        checked_usize(end)?,
        direction,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "extremum_f64")]
pub fn binding_extremum_f64(
    signal: napi::bindgen_prelude::Float64Array,
    start: f64,
    end: f64,
    direction: ExtremumDirection,
) -> napi::Result<f64> {
    let storage_signal = signal.to_vec();
    (crate::api::frame_extrema::extremum_f64(
        &storage_signal,
        checked_usize(start)?,
        checked_usize(end)?,
        direction,
    ))
    .map(|value| value as f64)
}
