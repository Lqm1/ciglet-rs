use crate::real_cepstrum::{self, Lifter, NyquistPolicy, Plan};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum CepstralLifter {
    Causal = 0,
    Legacy = 1,
}
fn nyquist(value: u32) -> Result<NyquistPolicy, JsValue> {
    match value {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(JsValue::from_str("unknown cepstral Nyquist policy")),
    }
}
fn lifter(value: u32) -> Result<Lifter, JsValue> {
    match value {
        0 => Ok(Lifter::Causal),
        1 => Ok(Lifter::Legacy),
        _ => Err(JsValue::from_str("unknown cepstral lifter")),
    }
}
macro_rules! real_cepstrum_api {
    ($scalar:ty, $owner:ident, $from:ident, $to:ident, $minimum:ident) => {
        /// Complete reusable native real-cepstrum FFT plan and workspace.
        #[wasm_bindgen]
        pub struct $owner {
            plan: Plan<$scalar>,
        }
        #[wasm_bindgen]
        impl $owner {
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
            pub fn analyze(
                &mut self,
                logs: &[$scalar],
                policy: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .analyze(logs, nyquist(policy)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn reconstruct(
                &mut self,
                coefficients: &[$scalar],
                policy: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .reconstruct(coefficients, nyquist(policy)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn minimum_phase_with(
                &mut self,
                logs: &[$scalar],
                policy: u32,
                lifter_code: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .minimum_phase_with(logs, nyquist(policy)?, lifter(lifter_code)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn spectrogram_to_cepstra(
                &mut self,
                magnitudes: &[$scalar],
                count: usize,
                policy: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .spectrogram_to_cepstra(magnitudes, count, nyquist(policy)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cepstra_to_spectrogram(
                &mut self,
                coefficients: &[$scalar],
                frames: usize,
                count: usize,
                policy: u32,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .cepstra_to_spectrogram(coefficients, frames, count, nyquist(policy)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
        #[wasm_bindgen]
        pub fn $from(logs: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            real_cepstrum::from_log_spectrum(logs).map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $to(coefficients: &[$scalar], length: usize) -> Result<Vec<$scalar>, JsValue> {
            real_cepstrum::to_log_spectrum(coefficients, length)
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $minimum(logs: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            real_cepstrum::minimum_phase(logs).map_err(|error| JsValue::from_str(error.0))
        }
    };
}
real_cepstrum_api!(
    f32,
    RealCepstrumPlanF32,
    real_cepstrum_from_log_spectrum_f32,
    real_cepstrum_to_log_spectrum_f32,
    real_cepstrum_minimum_phase_f32
);
real_cepstrum_api!(
    f64,
    RealCepstrumPlanF64,
    real_cepstrum_from_log_spectrum_f64,
    real_cepstrum_to_log_spectrum_f64,
    real_cepstrum_minimum_phase_f64
);
