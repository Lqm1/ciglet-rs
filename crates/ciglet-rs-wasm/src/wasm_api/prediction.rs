use crate::{
    fourier::Complex,
    prediction::{
        self, ArithmeticPolicy, NyquistPolicy, Prediction, Resonance, ResonanceEstimate,
        ScaledPrediction, SpectrumPlan,
    },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum PredictionArithmetic {
    Stable = 0,
    Legacy = 1,
}
fn arithmetic(code: u32) -> Result<ArithmeticPolicy, JsValue> {
    match code {
        0 => Ok(ArithmeticPolicy::Stable),
        1 => Ok(ArithmeticPolicy::Legacy),
        _ => Err(JsValue::from_str("invalid prediction arithmetic policy")),
    }
}
fn nyquist(code: u32) -> Result<NyquistPolicy, JsValue> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(JsValue::from_str("invalid prediction Nyquist policy")),
    }
}

/// Complete copied iterator output, with separate binary64 significands and i32 exponents.
#[wasm_bindgen]
pub struct PredictionCorrelationParts {
    fractions: Vec<f64>,
    exponents: Vec<i32>,
}
#[wasm_bindgen]
impl PredictionCorrelationParts {
    #[wasm_bindgen(getter)]
    pub fn fractions(&self) -> Vec<f64> {
        self.fractions.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn exponents(&self) -> Vec<i32> {
        self.exponents.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.fractions.len()
    }
}
#[wasm_bindgen]
pub struct PredictionGainParts {
    pub fraction: f64,
    pub exponent: i32,
}
#[wasm_bindgen]
impl PredictionGainParts {
    pub fn fraction_storage(&self) -> Vec<f64> {
        vec![self.fraction]
    }
}

macro_rules! prediction_api {
    ($scalar:ty, $ordinary:ident, $scaled:ident, $plan:ident, $resonance:ident, $estimate:ident,
     $levinson:ident, $levinson_with:ident, $samples:ident, $samples_with:ident, $samples_scaled:ident,
     $magnitude:ident, $magnitude_with:ident, $magnitude_arithmetic:ident, $magnitude_scaled:ident,
     $gain:ident, $gain_with:ident, $resonances:ident) => {
        #[wasm_bindgen]
        pub struct $ordinary {
            prediction: Prediction<$scalar>,
        }
        #[wasm_bindgen]
        impl $ordinary {
            #[wasm_bindgen(constructor)]
            pub fn new(coefficients: Vec<$scalar>, autocorrelation: Vec<$scalar>) -> Self {
                Self {
                    prediction: Prediction {
                        coefficients,
                        autocorrelation,
                    },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn coefficients(&self) -> Vec<$scalar> {
                self.prediction.coefficients.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_coefficients(&mut self, values: Vec<$scalar>) {
                self.prediction.coefficients = values;
            }
            #[wasm_bindgen(getter)]
            pub fn autocorrelation(&self) -> Vec<$scalar> {
                self.prediction.autocorrelation.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_autocorrelation(&mut self, values: Vec<$scalar>) {
                self.prediction.autocorrelation = values;
            }
            pub fn cloned(&self) -> Self {
                Self {
                    prediction: self.prediction.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub struct $scaled {
            prediction: ScaledPrediction<$scalar>,
        }
        #[wasm_bindgen]
        impl $scaled {
            #[wasm_bindgen(getter)]
            pub fn coefficients(&self) -> Vec<$scalar> {
                self.prediction.coefficients().to_vec()
            }
            pub fn autocorrelation_parts(&self) -> PredictionCorrelationParts {
                let (fractions, exponents) = self.prediction.autocorrelation_parts().unzip();
                PredictionCorrelationParts {
                    fractions,
                    exponents,
                }
            }
            pub fn into_prediction(self) -> Result<$ordinary, JsValue> {
                self.prediction
                    .into_prediction()
                    .map(|prediction| $ordinary { prediction })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn gain_parts(&self) -> Result<PredictionGainParts, JsValue> {
                self.prediction
                    .gain_parts()
                    .map(|(fraction, exponent)| PredictionGainParts { fraction, exponent })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn gain(&self) -> Result<$scalar, JsValue> {
                self.prediction
                    .gain()
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                Self {
                    prediction: self.prediction.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub struct $plan {
            plan: SpectrumPlan<$scalar>,
        }
        #[wasm_bindgen]
        impl $plan {
            #[wasm_bindgen(constructor)]
            pub fn new(length: usize) -> Result<Self, JsValue> {
                SpectrumPlan::new(length)
                    .map(|plan| Self { plan })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.plan.len()
            }
            pub fn is_empty(&self) -> bool {
                self.plan.is_empty()
            }
            pub fn estimate(
                &mut self,
                magnitudes: &[$scalar],
                order: usize,
                code: u32,
            ) -> Result<$ordinary, JsValue> {
                self.plan
                    .estimate(magnitudes, order, nyquist(code)?)
                    .map(|prediction| $ordinary { prediction })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn estimate_with_arithmetic(
                &mut self,
                magnitudes: &[$scalar],
                order: usize,
                nyquist_code: u32,
                arithmetic_code: u32,
            ) -> Result<$ordinary, JsValue> {
                self.plan
                    .estimate_with_arithmetic(
                        magnitudes,
                        order,
                        nyquist(nyquist_code)?,
                        arithmetic(arithmetic_code)?,
                    )
                    .map(|prediction| $ordinary { prediction })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn estimate_scaled(
                &mut self,
                magnitudes: &[$scalar],
                order: usize,
                code: u32,
            ) -> Result<$scaled, JsValue> {
                self.plan
                    .estimate_scaled(magnitudes, order, nyquist(code)?)
                    .map(|prediction| $scaled { prediction })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn spectrum(
                &mut self,
                coefficients: &[$scalar],
                gain: $scalar,
            ) -> Result<Vec<$scalar>, JsValue> {
                self.plan
                    .spectrum(coefficients, gain)
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
        #[wasm_bindgen]
        pub struct $resonance {
            resonance: Resonance<$scalar>,
        }
        #[wasm_bindgen]
        impl $resonance {
            #[wasm_bindgen(constructor)]
            pub fn new(frequency_nyquist: $scalar, real: $scalar, imaginary: $scalar) -> Self {
                Self {
                    resonance: Resonance {
                        frequency_nyquist,
                        pole: Complex::new(real, imaginary),
                    },
                }
            }
            pub fn from_components(values: &[$scalar]) -> Result<Self, JsValue> {
                let [frequency, real, imaginary]: [$scalar; 3] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("resonance requires exactly three fields"))?;
                Ok(Self::new(frequency, real, imaginary))
            }
            pub fn components(&self) -> Vec<$scalar> {
                vec![
                    self.resonance.frequency_nyquist,
                    self.resonance.pole.re,
                    self.resonance.pole.im,
                ]
            }
            pub fn set_components(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                *self = Self::from_components(values)?;
                Ok(())
            }
            #[wasm_bindgen(getter)]
            pub fn frequency_nyquist(&self) -> $scalar {
                self.resonance.frequency_nyquist
            }
            #[wasm_bindgen(setter)]
            pub fn set_frequency_nyquist(&mut self, value: $scalar) {
                self.resonance.frequency_nyquist = value;
            }
            pub fn pole(&self) -> Vec<$scalar> {
                vec![self.resonance.pole.re, self.resonance.pole.im]
            }
            pub fn set_pole(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [real, imaginary]: [$scalar; 2] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("pole requires exactly two fields"))?;
                self.resonance.pole = Complex::new(real, imaginary);
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                Self {
                    resonance: self.resonance.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub struct $estimate {
            estimate: ResonanceEstimate<$scalar>,
        }
        impl $estimate {
            fn unpack(values: &[$scalar]) -> Result<Vec<Resonance<$scalar>>, JsValue> {
                if !values.len().is_multiple_of(3) {
                    return Err(JsValue::from_str(
                        "resonances require frequency/real/imaginary triples",
                    ));
                }
                Ok(values
                    .chunks_exact(3)
                    .map(|v| Resonance {
                        frequency_nyquist: v[0],
                        pole: Complex::new(v[1], v[2]),
                    })
                    .collect())
            }
        }
        #[wasm_bindgen]
        impl $estimate {
            #[wasm_bindgen(constructor)]
            pub fn new(
                resonances: &[$scalar],
                iterations: usize,
                converged: bool,
                maximum_residual: f64,
            ) -> Result<Self, JsValue> {
                Ok(Self {
                    estimate: ResonanceEstimate {
                        resonances: Self::unpack(resonances)?,
                        iterations,
                        converged,
                        maximum_residual,
                    },
                })
            }
            #[wasm_bindgen(getter)]
            pub fn resonances(&self) -> Vec<$scalar> {
                self.estimate
                    .resonances
                    .iter()
                    .flat_map(|r| [r.frequency_nyquist, r.pole.re, r.pole.im])
                    .collect()
            }
            #[wasm_bindgen(setter)]
            pub fn set_resonances(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                self.estimate.resonances = Self::unpack(values)?;
                Ok(())
            }
            pub fn resonance(&self, index: usize) -> Option<$resonance> {
                self.estimate
                    .resonances
                    .get(index)
                    .cloned()
                    .map(|resonance| $resonance { resonance })
            }
            #[wasm_bindgen(getter)]
            pub fn iterations(&self) -> usize {
                self.estimate.iterations
            }
            #[wasm_bindgen(setter)]
            pub fn set_iterations(&mut self, value: usize) {
                self.estimate.iterations = value;
            }
            #[wasm_bindgen(getter)]
            pub fn converged(&self) -> bool {
                self.estimate.converged
            }
            #[wasm_bindgen(setter)]
            pub fn set_converged(&mut self, value: bool) {
                self.estimate.converged = value;
            }
            #[wasm_bindgen(getter)]
            pub fn maximum_residual(&self) -> f64 {
                self.estimate.maximum_residual
            }
            #[wasm_bindgen(setter)]
            pub fn set_maximum_residual(&mut self, value: f64) {
                self.estimate.maximum_residual = value;
            }
            pub fn residual_storage(&self) -> Vec<f64> {
                vec![self.estimate.maximum_residual]
            }
            pub fn set_residual_storage(&mut self, values: &[f64]) -> Result<(), JsValue> {
                let [value]: [f64; 1] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("residual requires exactly one value"))?;
                self.estimate.maximum_residual = value;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                Self {
                    estimate: self.estimate.clone(),
                }
            }
        }
        #[wasm_bindgen]
        pub fn $levinson(values: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            prediction::levinson(values).map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $levinson_with(values: &[$scalar], code: u32) -> Result<Vec<$scalar>, JsValue> {
            prediction::levinson_with(values, arithmetic(code)?).map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $samples(values: &[$scalar], order: usize) -> Result<$ordinary, JsValue> {
            prediction::from_samples(values, order)
                .map(|prediction| $ordinary { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $samples_with(
            values: &[$scalar],
            order: usize,
            code: u32,
        ) -> Result<$ordinary, JsValue> {
            prediction::from_samples_with(values, order, arithmetic(code)?)
                .map(|prediction| $ordinary { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $samples_scaled(values: &[$scalar], order: usize) -> Result<$scaled, JsValue> {
            prediction::from_samples_scaled(values, order)
                .map(|prediction| $scaled { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $magnitude(values: &[$scalar], order: usize) -> Result<$ordinary, JsValue> {
            prediction::from_magnitude(values, order)
                .map(|prediction| $ordinary { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $magnitude_with(
            values: &[$scalar],
            order: usize,
            code: u32,
        ) -> Result<$ordinary, JsValue> {
            prediction::from_magnitude_with(values, order, nyquist(code)?)
                .map(|prediction| $ordinary { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $magnitude_arithmetic(
            values: &[$scalar],
            order: usize,
            nyquist_code: u32,
            arithmetic_code: u32,
        ) -> Result<$ordinary, JsValue> {
            prediction::from_magnitude_with_arithmetic(
                values,
                order,
                nyquist(nyquist_code)?,
                arithmetic(arithmetic_code)?,
            )
            .map(|prediction| $ordinary { prediction })
            .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $magnitude_scaled(
            values: &[$scalar],
            order: usize,
            code: u32,
        ) -> Result<$scaled, JsValue> {
            prediction::from_magnitude_scaled(values, order, nyquist(code)?)
                .map(|prediction| $scaled { prediction })
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $gain(
            coefficients: &[$scalar],
            correlation: &[$scalar],
        ) -> Result<$scalar, JsValue> {
            prediction::gain(coefficients, correlation).map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $gain_with(
            coefficients: &[$scalar],
            correlation: &[$scalar],
            code: u32,
        ) -> Result<$scalar, JsValue> {
            prediction::gain_with(coefficients, correlation, arithmetic(code)?)
                .map_err(|e| JsValue::from_str(e.0))
        }
        #[wasm_bindgen]
        pub fn $resonances(coefficients: &[$scalar]) -> Result<$estimate, JsValue> {
            prediction::resonances(coefficients)
                .map(|estimate| $estimate { estimate })
                .map_err(|e| JsValue::from_str(e.0))
        }
    };
}
prediction_api!(
    f32,
    PredictionF32,
    ScaledPredictionF32,
    PredictionSpectrumPlanF32,
    PredictionResonanceF32,
    PredictionResonanceEstimateF32,
    prediction_levinson_f32,
    prediction_levinson_with_f32,
    prediction_from_samples_f32,
    prediction_from_samples_with_f32,
    prediction_from_samples_scaled_f32,
    prediction_from_magnitude_f32,
    prediction_from_magnitude_with_f32,
    prediction_from_magnitude_with_arithmetic_f32,
    prediction_from_magnitude_scaled_f32,
    prediction_gain_f32,
    prediction_gain_with_f32,
    prediction_resonances_f32
);
prediction_api!(
    f64,
    PredictionF64,
    ScaledPredictionF64,
    PredictionSpectrumPlanF64,
    PredictionResonanceF64,
    PredictionResonanceEstimateF64,
    prediction_levinson_f64,
    prediction_levinson_with_f64,
    prediction_from_samples_f64,
    prediction_from_samples_with_f64,
    prediction_from_samples_scaled_f64,
    prediction_from_magnitude_f64,
    prediction_from_magnitude_with_f64,
    prediction_from_magnitude_with_arithmetic_f64,
    prediction_from_magnitude_scaled_f64,
    prediction_gain_f64,
    prediction_gain_with_f64,
    prediction_resonances_f64
);
