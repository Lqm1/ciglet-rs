use crate::spectral_envelope::{self, Envelope, NyquistPolicy, Plan, SmoothingPolicy};
use wasm_bindgen::prelude::*;

fn nyquist(code: u32) -> Result<NyquistPolicy, JsValue> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(JsValue::from_str("invalid envelope Nyquist policy")),
    }
}
fn smoothing(code: u32) -> Result<SmoothingPolicy, JsValue> {
    match code {
        0 => Ok(SmoothingPolicy::Stable),
        1 => Ok(SmoothingPolicy::Legacy),
        _ => Err(JsValue::from_str("invalid envelope smoothing policy")),
    }
}
macro_rules! envelope_api {
    ($scalar:ty, $result:ident, $plan:ident, $estimate:ident) => {
        /// Both complete mutable native output fields in natural-log units.
        #[wasm_bindgen]
        pub struct $result {
            envelope: Envelope<$scalar>,
        }
        #[wasm_bindgen]
        impl $result {
            #[wasm_bindgen(constructor)]
            pub fn new(log_spectrum: Vec<$scalar>, cepstrum: Vec<$scalar>) -> Self {
                Self {
                    envelope: Envelope {
                        log_spectrum,
                        cepstrum,
                    },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn log_spectrum(&self) -> Vec<$scalar> {
                self.envelope.log_spectrum.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_log_spectrum(&mut self, values: Vec<$scalar>) {
                self.envelope.log_spectrum = values;
            }
            #[wasm_bindgen(getter)]
            pub fn cepstrum(&self) -> Vec<$scalar> {
                self.envelope.cepstrum.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_cepstrum(&mut self, values: Vec<$scalar>) {
                self.envelope.cepstrum = values;
            }
            pub fn cloned(&self) -> Self {
                Self {
                    envelope: self.envelope.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub struct $plan {
            plan: Plan<$scalar>,
        }
        #[wasm_bindgen]
        impl $plan {
            #[wasm_bindgen(constructor)]
            pub fn new(length: usize) -> Result<Self, JsValue> {
                Plan::new(length)
                    .map(|plan| Self { plan })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.plan.len()
            }
            #[wasm_bindgen(getter)]
            pub fn bins(&self) -> usize {
                self.plan.bins()
            }
            pub fn is_empty(&self) -> bool {
                self.plan.is_empty()
            }
            pub fn estimate(
                &mut self,
                spectrum: &[$scalar],
                fundamental: $scalar,
                harmonics: Option<usize>,
            ) -> Result<$result, JsValue> {
                self.plan
                    .estimate(spectrum, fundamental, harmonics)
                    .map(|envelope| $result { envelope })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn estimate_with(
                &mut self,
                spectrum: &[$scalar],
                fundamental: $scalar,
                harmonics: Option<usize>,
                nyquist_code: u32,
            ) -> Result<$result, JsValue> {
                self.plan
                    .estimate_with(spectrum, fundamental, harmonics, nyquist(nyquist_code)?)
                    .map(|envelope| $result { envelope })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn estimate_with_smoothing(
                &mut self,
                spectrum: &[$scalar],
                fundamental: $scalar,
                harmonics: Option<usize>,
                nyquist_code: u32,
                smoothing_code: u32,
            ) -> Result<$result, JsValue> {
                self.plan
                    .estimate_with_smoothing(
                        spectrum,
                        fundamental,
                        harmonics,
                        nyquist(nyquist_code)?,
                        smoothing(smoothing_code)?,
                    )
                    .map(|envelope| $result { envelope })
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
        #[wasm_bindgen]
        pub fn $estimate(
            spectrum: &[$scalar],
            fft_length: usize,
            fundamental: $scalar,
            harmonics: Option<usize>,
        ) -> Result<$result, JsValue> {
            spectral_envelope::estimate(spectrum, fft_length, fundamental, harmonics)
                .map(|envelope| $result { envelope })
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
envelope_api!(
    f32,
    SpectralEnvelopeF32,
    SpectralEnvelopePlanF32,
    spectral_envelope_estimate_f32
);
envelope_api!(
    f64,
    SpectralEnvelopeF64,
    SpectralEnvelopePlanF64,
    spectral_envelope_estimate_f64
);
