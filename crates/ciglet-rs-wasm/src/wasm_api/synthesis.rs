use crate::synthesis::{self, Sinusoid};
use wasm_bindgen::prelude::*;

macro_rules! synthesis_api {
    ($scalar:ty, $owner:ident, $single:ident, $multiple:ident) => {
        /// Complete native sinusoid configuration, including arbitrary field bits.
        #[wasm_bindgen]
        pub struct $owner {
            value: Sinusoid<$scalar>,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(frequency_hz: $scalar, amplitude: $scalar, phase_radians: $scalar) -> Self {
                Self {
                    value: Sinusoid {
                        frequency_hz,
                        amplitude,
                        phase_radians,
                    },
                }
            }
            pub fn from_values(values: &[$scalar]) -> Result<Self, JsValue> {
                let [frequency, amplitude, phase]: [$scalar; 3] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("sinusoid requires exactly three values"))?;
                Ok(Self::new(frequency, amplitude, phase))
            }
            #[wasm_bindgen(getter)]
            pub fn frequency_hz(&self) -> $scalar {
                self.value.frequency_hz
            }
            #[wasm_bindgen(setter)]
            pub fn set_frequency_hz(&mut self, value: $scalar) {
                self.value.frequency_hz = value;
            }
            #[wasm_bindgen(getter)]
            pub fn amplitude(&self) -> $scalar {
                self.value.amplitude
            }
            #[wasm_bindgen(setter)]
            pub fn set_amplitude(&mut self, value: $scalar) {
                self.value.amplitude = value;
            }
            #[wasm_bindgen(getter)]
            pub fn phase_radians(&self) -> $scalar {
                self.value.phase_radians
            }
            #[wasm_bindgen(setter)]
            pub fn set_phase_radians(&mut self, value: $scalar) {
                self.value.phase_radians = value;
            }
            pub fn values(&self) -> Vec<$scalar> {
                vec![
                    self.value.frequency_hz,
                    self.value.amplitude,
                    self.value.phase_radians,
                ]
            }
            pub fn set_values(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                self.value = Self::from_values(values)?.value;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                Self { value: self.value }
            }
        }
        #[wasm_bindgen]
        pub fn $single(
            tone: &$owner,
            sample_rate: u32,
            length: usize,
        ) -> Result<Vec<$scalar>, JsValue> {
            synthesis::sinusoid(tone.value, sample_rate, length)
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $multiple(
            frequencies: &[$scalar],
            amplitudes: &[$scalar],
            phases: &[$scalar],
            sample_rate: u32,
            length: usize,
        ) -> Result<Vec<$scalar>, JsValue> {
            if frequencies.len() != amplitudes.len() || frequencies.len() != phases.len() {
                return Err(JsValue::from_str(
                    "sinusoid arrays must have matching lengths",
                ));
            }
            let mut tones = Vec::new();
            tones
                .try_reserve_exact(frequencies.len())
                .map_err(|_| JsValue::from_str("sinusoid configuration allocation failed"))?;
            tones.extend(frequencies.iter().zip(amplitudes).zip(phases).map(
                |((&frequency_hz, &amplitude), &phase_radians)| Sinusoid {
                    frequency_hz,
                    amplitude,
                    phase_radians,
                },
            ));
            synthesis::sinusoids(&tones, sample_rate, length)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
synthesis_api!(f32, SinusoidF32, sinusoid_f32, sinusoids_f32);
synthesis_api!(f64, SinusoidF64, sinusoid_f64, sinusoids_f64);
