use super::*;
#[napi(js_name = "correlogram_compute_f32")]
pub fn binding_correlogram_compute_f32(
    samples: napi::bindgen_prelude::Float32Array,
    centers: napi::bindgen_prelude::Int32Array,
    lengths: napi::bindgen_prelude::Uint32Array,
    periods: f64,
    method_code: u32,
) -> napi::Result<Array2F32> {
    let storage_samples = samples.to_vec();
    let storage_centers = centers.to_vec();
    let storage_lengths = lengths.to_vec();
    (crate::api::correlogram::correlogram_compute_f32(
        &storage_samples,
        &storage_centers,
        &storage_lengths,
        checked_usize(periods)?,
        method_code,
    ))
    .map(|value| Array2F32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "correlogram_compute_with_f32")]
pub fn binding_correlogram_compute_with_f32(
    samples: napi::bindgen_prelude::Float32Array,
    centers: napi::bindgen_prelude::Int32Array,
    lengths: napi::bindgen_prelude::Uint32Array,
    periods: f64,
    method_code: u32,
    policy_code: u32,
) -> napi::Result<Array2F32> {
    let storage_samples = samples.to_vec();
    let storage_centers = centers.to_vec();
    let storage_lengths = lengths.to_vec();
    (crate::api::correlogram::correlogram_compute_with_f32(
        &storage_samples,
        &storage_centers,
        &storage_lengths,
        checked_usize(periods)?,
        method_code,
        policy_code,
    ))
    .map(|value| Array2F32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "correlogram_to_frequency_f32")]
pub fn binding_correlogram_to_frequency_f32(
    periods: &Array2F32,
    sampling_rate: i32,
    frequencies: napi::bindgen_prelude::Float32Array,
) -> napi::Result<Array2F32> {
    let storage_frequencies = frequencies.to_vec();
    (crate::api::correlogram::correlogram_to_frequency_f32(
        periods
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        sampling_rate,
        &storage_frequencies,
    ))
    .map(|value| Array2F32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "correlogram_compute_f64")]
pub fn binding_correlogram_compute_f64(
    samples: napi::bindgen_prelude::Float64Array,
    centers: napi::bindgen_prelude::Int32Array,
    lengths: napi::bindgen_prelude::Uint32Array,
    periods: f64,
    method_code: u32,
) -> napi::Result<Array2F64> {
    let storage_samples = samples.to_vec();
    let storage_centers = centers.to_vec();
    let storage_lengths = lengths.to_vec();
    (crate::api::correlogram::correlogram_compute_f64(
        &storage_samples,
        &storage_centers,
        &storage_lengths,
        checked_usize(periods)?,
        method_code,
    ))
    .map(|value| Array2F64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "correlogram_compute_with_f64")]
pub fn binding_correlogram_compute_with_f64(
    samples: napi::bindgen_prelude::Float64Array,
    centers: napi::bindgen_prelude::Int32Array,
    lengths: napi::bindgen_prelude::Uint32Array,
    periods: f64,
    method_code: u32,
    policy_code: u32,
) -> napi::Result<Array2F64> {
    let storage_samples = samples.to_vec();
    let storage_centers = centers.to_vec();
    let storage_lengths = lengths.to_vec();
    (crate::api::correlogram::correlogram_compute_with_f64(
        &storage_samples,
        &storage_centers,
        &storage_lengths,
        checked_usize(periods)?,
        method_code,
        policy_code,
    ))
    .map(|value| Array2F64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "correlogram_to_frequency_f64")]
pub fn binding_correlogram_to_frequency_f64(
    periods: &Array2F64,
    sampling_rate: i32,
    frequencies: napi::bindgen_prelude::Float64Array,
) -> napi::Result<Array2F64> {
    let storage_frequencies = frequencies.to_vec();
    (crate::api::correlogram::correlogram_to_frequency_f64(
        periods
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        sampling_rate,
        &storage_frequencies,
    ))
    .map(|value| Array2F64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
