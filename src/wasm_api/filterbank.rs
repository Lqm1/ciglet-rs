use crate::filterbank::{Compression, FilterBank, MelOptions};
use wasm_bindgen::prelude::*;

/// Native filterbank output compression.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum FilterBankCompression {
    Logarithmic,
    Plp,
}

impl From<FilterBankCompression> for Compression {
    fn from(value: FilterBankCompression) -> Self {
        match value {
            FilterBankCompression::Logarithmic => Self::Logarithmic,
            FilterBankCompression::Plp => Self::Plp,
        }
    }
}

// Check owner method arguments inside Rust so a conversion exception cannot
// bypass destruction of wasm-bindgen's borrowed-owner guard.
fn compression(value: u32) -> Result<Compression, JsValue> {
    match value {
        0 => Ok(FilterBankCompression::Logarithmic.into()),
        1 => Ok(FilterBankCompression::Plp.into()),
        _ => Err(JsValue::from_str("invalid filterbank compression")),
    }
}

macro_rules! filterbank_api {
    ($scalar:ty, $options:ident, $bank:ident) => {
        /// Complete mutable Mel configuration in the selected scalar precision.
        #[wasm_bindgen]
        #[derive(Clone, Copy)]
        pub struct $options {
            pub minimum_hz: $scalar,
            pub maximum_hz: $scalar,
            pub scale: $scalar,
            pub minimum_width_hz: $scalar,
        }

        #[wasm_bindgen]
        impl $options {
            #[wasm_bindgen(constructor)]
            pub fn new(
                minimum_hz: $scalar,
                maximum_hz: $scalar,
                scale: $scalar,
                minimum_width_hz: $scalar,
            ) -> Self {
                Self {
                    minimum_hz,
                    maximum_hz,
                    scale,
                    minimum_width_hz,
                }
            }
        }

        /// Independently owned responses, half-open supports and Nyquist metadata.
        #[wasm_bindgen]
        pub struct $bank {
            bank: FilterBank<$scalar>,
        }

        #[wasm_bindgen]
        impl $bank {
            /// Create a zero-filled custom bank, including empty dimensions.
            #[wasm_bindgen(constructor)]
            pub fn new(bins: usize, nyquist_hz: $scalar, channels: usize) -> Result<Self, JsValue> {
                FilterBank::empty(bins, nyquist_hz, channels)
                    .map(|bank| Self { bank })
                    .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn mel(
                bins: usize,
                nyquist_hz: $scalar,
                channels: usize,
                options: &$options,
            ) -> Result<Self, JsValue> {
                FilterBank::mel(
                    bins,
                    nyquist_hz,
                    channels,
                    MelOptions {
                        minimum_hz: options.minimum_hz,
                        maximum_hz: options.maximum_hz,
                        scale: options.scale,
                        minimum_width_hz: options.minimum_width_hz,
                    },
                )
                .map(|bank| Self { bank })
                .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn mel_default(
                bins: usize,
                nyquist_hz: $scalar,
                channels: usize,
                minimum_hz: $scalar,
                maximum_hz: $scalar,
            ) -> Result<Self, JsValue> {
                FilterBank::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz)
                    .map(|bank| Self { bank })
                    .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn plp(bins: usize, nyquist_hz: $scalar, channels: usize) -> Result<Self, JsValue> {
                FilterBank::plp(bins, nyquist_hz, channels)
                    .map(|bank| Self { bank })
                    .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn cloned(&self) -> Self {
                Self {
                    bank: self.bank.clone(),
                }
            }

            #[wasm_bindgen(getter)]
            pub fn bins(&self) -> usize {
                self.bank.bins()
            }

            #[wasm_bindgen(getter)]
            pub fn channels(&self) -> usize {
                self.bank.channels()
            }

            #[wasm_bindgen(getter)]
            pub fn nyquist_hz(&self) -> $scalar {
                self.bank.nyquist_hz()
            }

            /// Metadata changes do not change responses or application results.
            pub fn set_nyquist_hz(&mut self, value: $scalar) {
                self.bank.set_nyquist_hz(value);
            }

            /// Copy a complete response, including inactive scalar payloads.
            pub fn response(&self, channel: usize) -> Option<Vec<$scalar>> {
                self.bank.response(channel).map(<[$scalar]>::to_vec)
            }

            /// Return the half-open start and end, or undefined for a missing channel.
            pub fn support(&self, channel: usize) -> Option<Vec<u32>> {
                self.bank
                    .support(channel)
                    .map(|range| vec![range.start as u32, range.end as u32])
            }

            /// Atomically replace a complete response and its half-open support.
            pub fn set_channel(
                &mut self,
                channel: usize,
                response: &[$scalar],
                start: usize,
                end: usize,
            ) -> Result<(), JsValue> {
                self.bank
                    .set_channel(channel, response, start..end)
                    .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn apply(
                &self,
                spectrum: &[$scalar],
                compression_kind: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.bank
                    .apply(spectrum, compression(compression_kind)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }

            pub fn apply_spectrogram(
                &self,
                spectra: &[$scalar],
                compression_kind: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.bank
                    .apply_spectrogram(spectra, compression(compression_kind)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }

            /// Explicit frame counts retain zero-bin and zero-channel behavior.
            pub fn apply_frames(
                &self,
                spectra: &[$scalar],
                frames: usize,
                compression_kind: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.bank
                    .apply_frames(spectra, frames, compression(compression_kind)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
    };
}

filterbank_api!(f32, MelOptionsF32, FilterBankF32);
filterbank_api!(f64, MelOptionsF64, FilterBankF64);
