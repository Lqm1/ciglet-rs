use super::*;
#[pyfunction(name = "band_to_cepstrum_f32")]
#[pyo3(signature = (bands, options))]
pub fn binding_band_to_cepstrum_f32(
    bands: Vec<f32>,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::cepstrum::band_to_cepstrum_f32(&bands, options.inner.as_ref().ok_or_else(consumed)?)
}
#[pyfunction(name = "band_to_cepstrum_legacy_f32")]
#[pyo3(signature = (bands, options))]
pub fn binding_band_to_cepstrum_legacy_f32(
    bands: Vec<f32>,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::cepstrum::band_to_cepstrum_legacy_f32(
        &bands,
        options.inner.as_ref().ok_or_else(consumed)?,
    )
}
#[pyfunction(name = "band_spectrogram_to_cepstra_f32")]
#[pyo3(signature = (features, bands, options))]
pub fn binding_band_spectrogram_to_cepstra_f32(
    features: Vec<f32>,
    bands: usize,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::cepstrum::band_spectrogram_to_cepstra_f32(
        &features,
        bands,
        options.inner.as_ref().ok_or_else(consumed)?,
    )
}
#[pyfunction(name = "band_to_cepstrum_f64")]
#[pyo3(signature = (bands, options))]
pub fn binding_band_to_cepstrum_f64(
    bands: Vec<f64>,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::cepstrum::band_to_cepstrum_f64(&bands, options.inner.as_ref().ok_or_else(consumed)?)
}
#[pyfunction(name = "band_to_cepstrum_legacy_f64")]
#[pyo3(signature = (bands, options))]
pub fn binding_band_to_cepstrum_legacy_f64(
    bands: Vec<f64>,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::cepstrum::band_to_cepstrum_legacy_f64(
        &bands,
        options.inner.as_ref().ok_or_else(consumed)?,
    )
}
#[pyfunction(name = "band_spectrogram_to_cepstra_f64")]
#[pyo3(signature = (features, bands, options))]
pub fn binding_band_spectrogram_to_cepstra_f64(
    features: Vec<f64>,
    bands: usize,
    options: pyo3::PyRef<'_, BandCepstralOptions>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::cepstrum::band_spectrogram_to_cepstra_f64(
        &features,
        bands,
        options.inner.as_ref().ok_or_else(consumed)?,
    )
}
