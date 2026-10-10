use super::*;
#[napi(js_name = "band_to_cepstrum_f32")]
pub fn binding_band_to_cepstrum_f32(
    bands: napi::bindgen_prelude::Float32Array,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_bands = bands.to_vec();
    (crate::api::cepstrum::band_to_cepstrum_f32(
        &storage_bands,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "band_to_cepstrum_legacy_f32")]
pub fn binding_band_to_cepstrum_legacy_f32(
    bands: napi::bindgen_prelude::Float32Array,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_bands = bands.to_vec();
    (crate::api::cepstrum::band_to_cepstrum_legacy_f32(
        &storage_bands,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "band_spectrogram_to_cepstra_f32")]
pub fn binding_band_spectrogram_to_cepstra_f32(
    features: napi::bindgen_prelude::Float32Array,
    bands: f64,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_features = features.to_vec();
    (crate::api::cepstrum::band_spectrogram_to_cepstra_f32(
        &storage_features,
        checked_usize(bands)?,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "band_to_cepstrum_f64")]
pub fn binding_band_to_cepstrum_f64(
    bands: napi::bindgen_prelude::Float64Array,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_bands = bands.to_vec();
    (crate::api::cepstrum::band_to_cepstrum_f64(
        &storage_bands,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "band_to_cepstrum_legacy_f64")]
pub fn binding_band_to_cepstrum_legacy_f64(
    bands: napi::bindgen_prelude::Float64Array,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_bands = bands.to_vec();
    (crate::api::cepstrum::band_to_cepstrum_legacy_f64(
        &storage_bands,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "band_spectrogram_to_cepstra_f64")]
pub fn binding_band_spectrogram_to_cepstra_f64(
    features: napi::bindgen_prelude::Float64Array,
    bands: f64,
    options: &BandCepstralOptions,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_features = features.to_vec();
    (crate::api::cepstrum::band_spectrogram_to_cepstra_f64(
        &storage_features,
        checked_usize(bands)?,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
