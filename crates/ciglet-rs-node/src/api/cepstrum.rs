use crate::cepstrum::{self, CepstralOptions};
use napi::Error as BindingError;
/// Complete mutable native cepstral configuration.
#[derive(Clone, Copy)]
pub struct BandCepstralOptions {
    pub order: usize,
    pub include_dc: bool,
}
impl BandCepstralOptions {
    fn native(self) -> CepstralOptions {
        CepstralOptions {
            order: self.order,
            include_dc: self.include_dc,
        }
    }
}
impl BandCepstralOptions {
    pub fn new(order: usize, include_dc: bool) -> Self {
        Self { order, include_dc }
    }
    pub fn cloned(&self) -> Self {
        *self
    }
    pub fn coefficient_count(&self, bands: usize) -> Result<usize, BindingError> {
        self.native()
            .coefficient_count(bands)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
pub fn band_to_cepstrum_f32(
    bands: &[f32],
    options: &BandCepstralOptions,
) -> Result<Vec<f32>, BindingError> {
    cepstrum::band_to_cepstrum(bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn band_to_cepstrum_legacy_f32(
    bands: &[f32],
    options: &BandCepstralOptions,
) -> Result<Vec<f32>, BindingError> {
    cepstrum::band_to_cepstrum_legacy(bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn band_spectrogram_to_cepstra_f32(
    features: &[f32],
    bands: usize,
    options: &BandCepstralOptions,
) -> Result<Vec<f32>, BindingError> {
    cepstrum::band_spectrogram_to_cepstra(features, bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn band_to_cepstrum_f64(
    bands: &[f64],
    options: &BandCepstralOptions,
) -> Result<Vec<f64>, BindingError> {
    cepstrum::band_to_cepstrum(bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn band_to_cepstrum_legacy_f64(
    bands: &[f64],
    options: &BandCepstralOptions,
) -> Result<Vec<f64>, BindingError> {
    cepstrum::band_to_cepstrum_legacy(bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn band_spectrogram_to_cepstra_f64(
    features: &[f64],
    bands: usize,
    options: &BandCepstralOptions,
) -> Result<Vec<f64>, BindingError> {
    cepstrum::band_spectrogram_to_cepstra(features, bands, options.native())
        .map_err(|error| napi::Error::from_reason(error.0))
}
