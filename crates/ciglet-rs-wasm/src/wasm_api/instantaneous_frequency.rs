use crate::instantaneous_frequency::{Detector, Kernels, Policy};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum InstantaneousFrequencyPolicy {
    Stable = 0,
    Legacy = 1,
}

fn policy(value: u32) -> Result<Policy, JsValue> {
    match value {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(JsValue::from_str("invalid instantaneous frequency policy")),
    }
}

macro_rules! detector_api {
    ($scalar:ty, $kernels:ident, $detector:ident) => {
        /// Complete mutable impulse responses, including temporarily invalid edits.
        #[wasm_bindgen]
        pub struct $kernels {
            kernels: Kernels<$scalar>,
        }

        #[wasm_bindgen]
        impl $kernels {
            #[wasm_bindgen(constructor)]
            pub fn new(
                real: Vec<$scalar>,
                imaginary: Vec<$scalar>,
                derivative_real: Vec<$scalar>,
                derivative_imaginary: Vec<$scalar>,
            ) -> Self {
                Self {
                    kernels: Kernels {
                        real,
                        imaginary,
                        derivative_real,
                        derivative_imaginary,
                    },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn real(&self) -> Vec<$scalar> {
                self.kernels.real.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_real(&mut self, values: Vec<$scalar>) {
                self.kernels.real = values;
            }
            #[wasm_bindgen(getter)]
            pub fn imaginary(&self) -> Vec<$scalar> {
                self.kernels.imaginary.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_imaginary(&mut self, values: Vec<$scalar>) {
                self.kernels.imaginary = values;
            }
            #[wasm_bindgen(getter)]
            pub fn derivative_real(&self) -> Vec<$scalar> {
                self.kernels.derivative_real.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_derivative_real(&mut self, values: Vec<$scalar>) {
                self.kernels.derivative_real = values;
            }
            #[wasm_bindgen(getter)]
            pub fn derivative_imaginary(&self) -> Vec<$scalar> {
                self.kernels.derivative_imaginary.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_derivative_imaginary(&mut self, values: Vec<$scalar>) {
                self.kernels.derivative_imaginary = values;
            }
            pub fn cloned(&self) -> Self {
                Self {
                    kernels: self.kernels.clone(),
                }
            }
        }

        /// Reusable native centered instantaneous-frequency detector.
        #[wasm_bindgen]
        pub struct $detector {
            detector: Detector<$scalar>,
        }

        #[wasm_bindgen]
        impl $detector {
            #[wasm_bindgen(constructor)]
            pub fn new(center_frequency: $scalar, resolution: $scalar) -> Result<Self, JsValue> {
                Detector::new(center_frequency, resolution)
                    .map(|detector| Self { detector })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn from_kernels(
                center_frequency: $scalar,
                kernels: &$kernels,
            ) -> Result<Self, JsValue> {
                Detector::from_kernels(center_frequency, kernels.kernels.clone())
                    .map(|detector| Self { detector })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn center_frequency(&self) -> $scalar {
                self.detector.center_frequency()
            }
            pub fn set_center_frequency(&mut self, value: $scalar) -> Result<(), JsValue> {
                self.detector
                    .set_center_frequency(value)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn kernels(&self) -> $kernels {
                $kernels {
                    kernels: self.detector.kernels().clone(),
                }
            }
            /// Full replacement mirrors unrestricted native kernels_mut access.
            #[wasm_bindgen(setter)]
            pub fn set_kernels(&mut self, kernels: &$kernels) {
                self.detector.kernels_mut().clone_from(&kernels.kernels);
            }
            pub fn estimate(&self, samples: &[$scalar]) -> Result<$scalar, JsValue> {
                self.detector
                    .estimate(samples)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn estimate_with(
                &self,
                samples: &[$scalar],
                policy_code: u32,
            ) -> Result<$scalar, JsValue> {
                self.detector
                    .estimate_with(samples, policy(policy_code)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                Self {
                    detector: self.detector.clone(),
                }
            }
        }
    };
}
detector_api!(
    f32,
    InstantaneousFrequencyKernelsF32,
    InstantaneousFrequencyDetectorF32
);
detector_api!(
    f64,
    InstantaneousFrequencyKernelsF64,
    InstantaneousFrequencyDetectorF64
);
