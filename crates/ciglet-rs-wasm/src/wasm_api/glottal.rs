use crate::glottal::{Model, Precision, Prepared, Spectrum};
use wasm_bindgen::prelude::*;

fn precision(code: u32) -> Result<Precision, JsValue> {
    match code {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(JsValue::from_str("invalid LF precision")),
    }
}

macro_rules! glottal_api {
    ($scalar:ty, $model:ident, $prepared:ident, $spectrum:ident) => {
        /// All mutable native model fields, including intermediate invalid configurations.
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $model {
            pub period: $scalar,
            pub closure: $scalar,
            pub peak: $scalar,
            pub return_time: $scalar,
            pub excitation: $scalar,
        }
        impl $model {
            fn native(&self) -> Model<$scalar> {
                Model {
                    period: self.period,
                    closure: self.closure,
                    peak: self.peak,
                    return_time: self.return_time,
                    excitation: self.excitation,
                }
            }
            fn from_native(model: Model<$scalar>) -> Self {
                Self {
                    period: model.period,
                    closure: model.closure,
                    peak: model.peak,
                    return_time: model.return_time,
                    excitation: model.excitation,
                }
            }
        }
        #[wasm_bindgen]
        impl $model {
            #[wasm_bindgen(constructor)]
            pub fn new(
                period: $scalar,
                closure: $scalar,
                peak: $scalar,
                return_time: $scalar,
                excitation: $scalar,
            ) -> Self {
                Self {
                    period,
                    closure,
                    peak,
                    return_time,
                    excitation,
                }
            }
            pub fn from_shape(
                shape: $scalar,
                period: $scalar,
                excitation: $scalar,
            ) -> Result<Self, JsValue> {
                Model::from_shape(shape, period, excitation)
                    .map(Self::from_native)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn from_components(values: &[$scalar]) -> Result<Self, JsValue> {
                let [period, closure, peak, return_time, excitation]: [$scalar; 5] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("LF model requires exactly five fields"))?;
                Ok(Self::new(period, closure, peak, return_time, excitation))
            }
            pub fn components(&self) -> Vec<$scalar> {
                vec![
                    self.period,
                    self.closure,
                    self.peak,
                    self.return_time,
                    self.excitation,
                ]
            }
            pub fn set_components(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                *self = Self::from_components(values)?;
                Ok(())
            }
            pub fn prepare(&self) -> Result<$prepared, JsValue> {
                self.native()
                    .prepare()
                    .map(|prepared| $prepared { prepared })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn spectrum(&self, frequencies: &[$scalar]) -> Result<$spectrum, JsValue> {
                self.native()
                    .spectrum(frequencies)
                    .map(|spectrum| $spectrum { spectrum })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn sampled_period(
                &self,
                sampling_rate: i32,
                count: usize,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.native()
                    .sampled_period(sampling_rate, count)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        pub struct $prepared {
            prepared: Prepared<$scalar>,
        }
        #[wasm_bindgen]
        impl $prepared {
            pub fn model(&self) -> $model {
                $model::from_native(self.prepared.model())
            }
            pub fn spectrum(&self, frequencies: &[$scalar]) -> Result<$spectrum, JsValue> {
                self.prepared
                    .spectrum(frequencies)
                    .map(|spectrum| $spectrum { spectrum })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn spectrum_with(
                &self,
                frequencies: &[$scalar],
                code: u32,
            ) -> Result<$spectrum, JsValue> {
                self.prepared
                    .spectrum_with(frequencies, precision(code)?)
                    .map(|spectrum| $spectrum { spectrum })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn sampled_period(
                &self,
                sampling_rate: i32,
                count: usize,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.prepared
                    .sampled_period(sampling_rate, count)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                Self {
                    prepared: self.prepared.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub struct $spectrum {
            spectrum: Spectrum<$scalar>,
        }
        #[wasm_bindgen]
        impl $spectrum {
            #[wasm_bindgen(constructor)]
            pub fn new(magnitude: Vec<$scalar>, phase: Vec<$scalar>) -> Self {
                Self {
                    spectrum: Spectrum { magnitude, phase },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn magnitude(&self) -> Vec<$scalar> {
                self.spectrum.magnitude.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_magnitude(&mut self, values: Vec<$scalar>) {
                self.spectrum.magnitude = values;
            }
            #[wasm_bindgen(getter)]
            pub fn phase(&self) -> Vec<$scalar> {
                self.spectrum.phase.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_phase(&mut self, values: Vec<$scalar>) {
                self.spectrum.phase = values;
            }
            pub fn cloned(&self) -> Self {
                Self {
                    spectrum: self.spectrum.clone(),
                }
            }
        }
    };
}
glottal_api!(f32, GlottalModelF32, GlottalPreparedF32, GlottalSpectrumF32);
glottal_api!(f64, GlottalModelF64, GlottalPreparedF64, GlottalSpectrumF64);
