use super::*;
#[pyfunction(name = "stft_analyze_regular_f32")]
#[pyo3(signature = (signal, grid, precision_code, options))]
pub fn binding_stft_analyze_regular_f32(
    signal: Vec<f32>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    precision_code: u32,
    options: pyo3::PyRef<'_, StftAnalysisOptions>,
) -> pyo3::PyResult<StftAnalysisF32> {
    (crate::api::stft::stft_analyze_regular_f32(
        &signal,
        grid.inner.as_ref().ok_or_else(consumed)?,
        precision_code,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF32 { inner: Some(value) })
}
#[pyfunction(name = "stft_analyze_regular_legacy_f32")]
#[pyo3(signature = (signal, grid, level, options))]
pub fn binding_stft_analyze_regular_legacy_f32(
    signal: Vec<f32>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    level: i32,
    options: pyo3::PyRef<'_, StftAnalysisOptions>,
) -> pyo3::PyResult<StftAnalysisF32> {
    (crate::api::stft::stft_analyze_regular_legacy_f32(
        &signal,
        grid.inner.as_ref().ok_or_else(consumed)?,
        level,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF32 { inner: Some(value) })
}
#[pyfunction(name = "stft_synthesize_regular_f32")]
#[pyo3(signature = (magnitude, phase, grid, normalization, nyquist_code))]
pub fn binding_stft_synthesize_regular_f32(
    magnitude: Vec<f32>,
    phase: Vec<f32>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    normalization: f32,
    nyquist_code: u32,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::stft::stft_synthesize_regular_f32(
        &magnitude,
        &phase,
        grid.inner.as_ref().ok_or_else(consumed)?,
        normalization,
        nyquist_code,
    )
}
#[pyfunction(name = "stft_analyze_regular_f64")]
#[pyo3(signature = (signal, grid, precision_code, options))]
pub fn binding_stft_analyze_regular_f64(
    signal: Vec<f64>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    precision_code: u32,
    options: pyo3::PyRef<'_, StftAnalysisOptions>,
) -> pyo3::PyResult<StftAnalysisF64> {
    (crate::api::stft::stft_analyze_regular_f64(
        &signal,
        grid.inner.as_ref().ok_or_else(consumed)?,
        precision_code,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF64 { inner: Some(value) })
}
#[pyfunction(name = "stft_analyze_regular_legacy_f64")]
#[pyo3(signature = (signal, grid, level, options))]
pub fn binding_stft_analyze_regular_legacy_f64(
    signal: Vec<f64>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    level: i32,
    options: pyo3::PyRef<'_, StftAnalysisOptions>,
) -> pyo3::PyResult<StftAnalysisF64> {
    (crate::api::stft::stft_analyze_regular_legacy_f64(
        &signal,
        grid.inner.as_ref().ok_or_else(consumed)?,
        level,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| StftAnalysisF64 { inner: Some(value) })
}
#[pyfunction(name = "stft_synthesize_regular_f64")]
#[pyo3(signature = (magnitude, phase, grid, normalization, nyquist_code))]
pub fn binding_stft_synthesize_regular_f64(
    magnitude: Vec<f64>,
    phase: Vec<f64>,
    grid: pyo3::PyRef<'_, StftFrameGrid>,
    normalization: f64,
    nyquist_code: u32,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::stft::stft_synthesize_regular_f64(
        &magnitude,
        &phase,
        grid.inner.as_ref().ok_or_else(consumed)?,
        normalization,
        nyquist_code,
    )
}
