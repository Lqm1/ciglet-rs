use crate::cepstrum::{self, CepstralOptions};
use wasm_bindgen::prelude::*;

/// Complete mutable native cepstral configuration.
#[wasm_bindgen]
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
#[wasm_bindgen]
impl BandCepstralOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(order: usize, include_dc: bool) -> Self {
        Self { order, include_dc }
    }
    pub fn cloned(&self) -> Self {
        *self
    }
    pub fn coefficient_count(&self, bands: usize) -> Result<usize, JsValue> {
        self.native()
            .coefficient_count(bands)
            .map_err(|error| JsValue::from_str(error.0))
    }
}
macro_rules! cepstrum_api {
    ($scalar:ty, $selected:ident, $legacy:ident, $matrix:ident) => {
        #[wasm_bindgen]
        pub fn $selected(
            bands: &[$scalar],
            options: &BandCepstralOptions,
        ) -> Result<Vec<$scalar>, JsValue> {
            cepstrum::band_to_cepstrum(bands, options.native())
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $legacy(
            bands: &[$scalar],
            options: &BandCepstralOptions,
        ) -> Result<Vec<$scalar>, JsValue> {
            cepstrum::band_to_cepstrum_legacy(bands, options.native())
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $matrix(
            features: &[$scalar],
            bands: usize,
            options: &BandCepstralOptions,
        ) -> Result<Vec<$scalar>, JsValue> {
            cepstrum::band_spectrogram_to_cepstra(features, bands, options.native())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
cepstrum_api!(
    f32,
    band_to_cepstrum_f32,
    band_to_cepstrum_legacy_f32,
    band_spectrogram_to_cepstra_f32
);
cepstrum_api!(
    f64,
    band_to_cepstrum_f64,
    band_to_cepstrum_legacy_f64,
    band_spectrogram_to_cepstra_f64
);
