use super::*;
#[napi(js_name = "stft_analyze_regular_f32")]
pub fn binding_stft_analyze_regular_f32(
    signal: napi::bindgen_prelude::Float32Array,
    grid: &StftFrameGrid,
    precision_code: u32,
    options: &StftAnalysisOptions,
) -> napi::Result<StftAnalysisF32> {
    let storage_signal = signal.to_vec();
    (crate::api::stft::stft_analyze_regular_f32(
        &storage_signal,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        precision_code,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "stft_analyze_regular_legacy_f32")]
pub fn binding_stft_analyze_regular_legacy_f32(
    signal: napi::bindgen_prelude::Float32Array,
    grid: &StftFrameGrid,
    level: i32,
    options: &StftAnalysisOptions,
) -> napi::Result<StftAnalysisF32> {
    let storage_signal = signal.to_vec();
    (crate::api::stft::stft_analyze_regular_legacy_f32(
        &storage_signal,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        level,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "stft_synthesize_regular_f32")]
pub fn binding_stft_synthesize_regular_f32(
    magnitude: napi::bindgen_prelude::Float32Array,
    phase: napi::bindgen_prelude::Float32Array,
    grid: &StftFrameGrid,
    normalization: f64,
    nyquist_code: u32,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_magnitude = magnitude.to_vec();
    let storage_phase = phase.to_vec();
    (crate::api::stft::stft_synthesize_regular_f32(
        &storage_magnitude,
        &storage_phase,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        normalization as f32,
        nyquist_code,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "stft_analyze_regular_f64")]
pub fn binding_stft_analyze_regular_f64(
    signal: napi::bindgen_prelude::Float64Array,
    grid: &StftFrameGrid,
    precision_code: u32,
    options: &StftAnalysisOptions,
) -> napi::Result<StftAnalysisF64> {
    let storage_signal = signal.to_vec();
    (crate::api::stft::stft_analyze_regular_f64(
        &storage_signal,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        precision_code,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "stft_analyze_regular_legacy_f64")]
pub fn binding_stft_analyze_regular_legacy_f64(
    signal: napi::bindgen_prelude::Float64Array,
    grid: &StftFrameGrid,
    level: i32,
    options: &StftAnalysisOptions,
) -> napi::Result<StftAnalysisF64> {
    let storage_signal = signal.to_vec();
    (crate::api::stft::stft_analyze_regular_legacy_f64(
        &storage_signal,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        level,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "stft_synthesize_regular_f64")]
pub fn binding_stft_synthesize_regular_f64(
    magnitude: napi::bindgen_prelude::Float64Array,
    phase: napi::bindgen_prelude::Float64Array,
    grid: &StftFrameGrid,
    normalization: f64,
    nyquist_code: u32,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_magnitude = magnitude.to_vec();
    let storage_phase = phase.to_vec();
    (crate::api::stft::stft_synthesize_regular_f64(
        &storage_magnitude,
        &storage_phase,
        grid.inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        normalization,
        nyquist_code,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
