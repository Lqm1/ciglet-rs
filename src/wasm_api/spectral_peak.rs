use crate::spectral_peak::{self, Peak};
use wasm_bindgen::prelude::*;

macro_rules! peak_api {
    ($scalar:ty, $owner:ident, $interpolate:ident) => {
        /// Complete native peak value with independently mutable fields.
        #[wasm_bindgen]
        pub struct $owner {
            value: Peak<$scalar>,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(bin: $scalar, magnitude: $scalar) -> Self {
                Self {
                    value: Peak { bin, magnitude },
                }
            }
            pub fn from_values(values: &[$scalar]) -> Result<Self, JsValue> {
                let [bin, magnitude]: [$scalar; 2] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("peak requires exactly two values"))?;
                Ok(Self::new(bin, magnitude))
            }
            #[wasm_bindgen(getter)]
            pub fn bin(&self) -> $scalar {
                self.value.bin
            }
            #[wasm_bindgen(setter)]
            pub fn set_bin(&mut self, value: $scalar) {
                self.value.bin = value;
            }
            #[wasm_bindgen(getter)]
            pub fn magnitude(&self) -> $scalar {
                self.value.magnitude
            }
            #[wasm_bindgen(setter)]
            pub fn set_magnitude(&mut self, value: $scalar) {
                self.value.magnitude = value;
            }
            /// Independent typed storage preserves all field bits.
            pub fn values(&self) -> Vec<$scalar> {
                vec![self.value.bin, self.value.magnitude]
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
        pub fn $interpolate(magnitudes: &[$scalar], bin: usize) -> Result<$owner, JsValue> {
            spectral_peak::interpolate(magnitudes, bin)
                .map(|value| $owner { value })
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
peak_api!(f32, SpectralPeakF32, spectral_peak_interpolate_f32);
peak_api!(f64, SpectralPeakF64, spectral_peak_interpolate_f64);
