use crate::{
    fourier::Complex,
    prediction::{
        self, ArithmeticPolicy, NyquistPolicy, Prediction, Resonance, ResonanceEstimate,
        ScaledPrediction, SpectrumPlan,
    },
};
use napi::Error as BindingError;
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum PredictionArithmetic {
    Stable = 0,
    Legacy = 1,
}
fn arithmetic(code: u32) -> Result<ArithmeticPolicy, BindingError> {
    match code {
        0 => Ok(ArithmeticPolicy::Stable),
        1 => Ok(ArithmeticPolicy::Legacy),
        _ => Err(napi::Error::from_reason(
            "invalid prediction arithmetic policy",
        )),
    }
}
fn nyquist(code: u32) -> Result<NyquistPolicy, BindingError> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(napi::Error::from_reason(
            "invalid prediction Nyquist policy",
        )),
    }
}
/// Complete copied iterator output, with separate binary64 significands and i32 exponents.
pub struct PredictionCorrelationParts {
    fractions: Vec<f64>,
    exponents: Vec<i32>,
}
impl PredictionCorrelationParts {
    pub fn fractions(&self) -> Vec<f64> {
        self.fractions.clone()
    }
    pub fn exponents(&self) -> Vec<i32> {
        self.exponents.clone()
    }
    pub fn length(&self) -> usize {
        self.fractions.len()
    }
}
pub struct PredictionGainParts {
    pub fraction: f64,
    pub exponent: i32,
}
impl PredictionGainParts {
    pub fn fraction_storage(&self) -> Vec<f64> {
        vec![self.fraction]
    }
}
pub struct PredictionF32 {
    prediction: Prediction<f32>,
}
impl PredictionF32 {
    pub fn new(coefficients: Vec<f32>, autocorrelation: Vec<f32>) -> Self {
        Self {
            prediction: Prediction {
                coefficients,
                autocorrelation,
            },
        }
    }
    pub fn coefficients(&self) -> Vec<f32> {
        self.prediction.coefficients.clone()
    }
    pub fn set_coefficients(&mut self, values: Vec<f32>) {
        self.prediction.coefficients = values;
    }
    pub fn autocorrelation(&self) -> Vec<f32> {
        self.prediction.autocorrelation.clone()
    }
    pub fn set_autocorrelation(&mut self, values: Vec<f32>) {
        self.prediction.autocorrelation = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            prediction: self.prediction.clone(),
        }
    }
}
pub struct ScaledPredictionF32 {
    prediction: ScaledPrediction<f32>,
}
impl ScaledPredictionF32 {
    pub fn coefficients(&self) -> Vec<f32> {
        self.prediction.coefficients().to_vec()
    }
    pub fn autocorrelation_parts(&self) -> PredictionCorrelationParts {
        let (fractions, exponents) = self.prediction.autocorrelation_parts().unzip();
        PredictionCorrelationParts {
            fractions,
            exponents,
        }
    }
    pub fn into_prediction(self) -> Result<PredictionF32, BindingError> {
        self.prediction
            .into_prediction()
            .map(|prediction| PredictionF32 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn gain_parts(&self) -> Result<PredictionGainParts, BindingError> {
        self.prediction
            .gain_parts()
            .map(|(fraction, exponent)| PredictionGainParts { fraction, exponent })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn gain(&self) -> Result<f32, BindingError> {
        self.prediction
            .gain()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            prediction: self.prediction.clone(),
        }
    }
}
pub struct PredictionSpectrumPlanF32 {
    plan: SpectrumPlan<f32>,
}
impl PredictionSpectrumPlanF32 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        SpectrumPlan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    pub fn estimate(
        &mut self,
        magnitudes: &[f32],
        order: usize,
        code: u32,
    ) -> Result<PredictionF32, BindingError> {
        self.plan
            .estimate(magnitudes, order, nyquist(code)?)
            .map(|prediction| PredictionF32 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_with_arithmetic(
        &mut self,
        magnitudes: &[f32],
        order: usize,
        nyquist_code: u32,
        arithmetic_code: u32,
    ) -> Result<PredictionF32, BindingError> {
        self.plan
            .estimate_with_arithmetic(
                magnitudes,
                order,
                nyquist(nyquist_code)?,
                arithmetic(arithmetic_code)?,
            )
            .map(|prediction| PredictionF32 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_scaled(
        &mut self,
        magnitudes: &[f32],
        order: usize,
        code: u32,
    ) -> Result<ScaledPredictionF32, BindingError> {
        self.plan
            .estimate_scaled(magnitudes, order, nyquist(code)?)
            .map(|prediction| ScaledPredictionF32 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum(&mut self, coefficients: &[f32], gain: f32) -> Result<Vec<f32>, BindingError> {
        self.plan
            .spectrum(coefficients, gain)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
pub struct PredictionResonanceF32 {
    resonance: Resonance<f32>,
}
impl PredictionResonanceF32 {
    pub fn new(frequency_nyquist: f32, real: f32, imaginary: f32) -> Self {
        Self {
            resonance: Resonance {
                frequency_nyquist,
                pole: Complex::new(real, imaginary),
            },
        }
    }
    pub fn from_components(values: &[f32]) -> Result<Self, BindingError> {
        let [frequency, real, imaginary]: [f32; 3] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("resonance requires exactly three fields"))?;
        Ok(Self::new(frequency, real, imaginary))
    }
    pub fn components(&self) -> Vec<f32> {
        vec![
            self.resonance.frequency_nyquist,
            self.resonance.pole.re,
            self.resonance.pole.im,
        ]
    }
    pub fn set_components(&mut self, values: &[f32]) -> Result<(), BindingError> {
        *self = Self::from_components(values)?;
        Ok(())
    }
    pub fn frequency_nyquist(&self) -> f32 {
        self.resonance.frequency_nyquist
    }
    pub fn set_frequency_nyquist(&mut self, value: f32) {
        self.resonance.frequency_nyquist = value;
    }
    pub fn pole(&self) -> Vec<f32> {
        vec![self.resonance.pole.re, self.resonance.pole.im]
    }
    pub fn set_pole(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [real, imaginary]: [f32; 2] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("pole requires exactly two fields"))?;
        self.resonance.pole = Complex::new(real, imaginary);
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            resonance: self.resonance.clone(),
        }
    }
}
pub struct PredictionResonanceEstimateF32 {
    estimate: ResonanceEstimate<f32>,
}
impl PredictionResonanceEstimateF32 {
    fn unpack(values: &[f32]) -> Result<Vec<Resonance<f32>>, BindingError> {
        if !values.len().is_multiple_of(3) {
            return Err(napi::Error::from_reason(
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
impl PredictionResonanceEstimateF32 {
    pub fn new(
        resonances: &[f32],
        iterations: usize,
        converged: bool,
        maximum_residual: f64,
    ) -> Result<Self, BindingError> {
        Ok(Self {
            estimate: ResonanceEstimate {
                resonances: Self::unpack(resonances)?,
                iterations,
                converged,
                maximum_residual,
            },
        })
    }
    pub fn resonances(&self) -> Vec<f32> {
        self.estimate
            .resonances
            .iter()
            .flat_map(|r| [r.frequency_nyquist, r.pole.re, r.pole.im])
            .collect()
    }
    pub fn set_resonances(&mut self, values: &[f32]) -> Result<(), BindingError> {
        self.estimate.resonances = Self::unpack(values)?;
        Ok(())
    }
    pub fn resonance(&self, index: usize) -> Option<PredictionResonanceF32> {
        self.estimate
            .resonances
            .get(index)
            .cloned()
            .map(|resonance| PredictionResonanceF32 { resonance })
    }
    pub fn iterations(&self) -> usize {
        self.estimate.iterations
    }
    pub fn set_iterations(&mut self, value: usize) {
        self.estimate.iterations = value;
    }
    pub fn converged(&self) -> bool {
        self.estimate.converged
    }
    pub fn set_converged(&mut self, value: bool) {
        self.estimate.converged = value;
    }
    pub fn maximum_residual(&self) -> f64 {
        self.estimate.maximum_residual
    }
    pub fn set_maximum_residual(&mut self, value: f64) {
        self.estimate.maximum_residual = value;
    }
    pub fn residual_storage(&self) -> Vec<f64> {
        vec![self.estimate.maximum_residual]
    }
    pub fn set_residual_storage(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("residual requires exactly one value"))?;
        self.estimate.maximum_residual = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            estimate: self.estimate.clone(),
        }
    }
}
pub fn prediction_levinson_f32(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    prediction::levinson(values).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_levinson_with_f32(values: &[f32], code: u32) -> Result<Vec<f32>, BindingError> {
    prediction::levinson_with(values, arithmetic(code)?).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_f32(
    values: &[f32],
    order: usize,
) -> Result<PredictionF32, BindingError> {
    prediction::from_samples(values, order)
        .map(|prediction| PredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_with_f32(
    values: &[f32],
    order: usize,
    code: u32,
) -> Result<PredictionF32, BindingError> {
    prediction::from_samples_with(values, order, arithmetic(code)?)
        .map(|prediction| PredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_scaled_f32(
    values: &[f32],
    order: usize,
) -> Result<ScaledPredictionF32, BindingError> {
    prediction::from_samples_scaled(values, order)
        .map(|prediction| ScaledPredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_f32(
    values: &[f32],
    order: usize,
) -> Result<PredictionF32, BindingError> {
    prediction::from_magnitude(values, order)
        .map(|prediction| PredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_with_f32(
    values: &[f32],
    order: usize,
    code: u32,
) -> Result<PredictionF32, BindingError> {
    prediction::from_magnitude_with(values, order, nyquist(code)?)
        .map(|prediction| PredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_with_arithmetic_f32(
    values: &[f32],
    order: usize,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> Result<PredictionF32, BindingError> {
    prediction::from_magnitude_with_arithmetic(
        values,
        order,
        nyquist(nyquist_code)?,
        arithmetic(arithmetic_code)?,
    )
    .map(|prediction| PredictionF32 { prediction })
    .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_scaled_f32(
    values: &[f32],
    order: usize,
    code: u32,
) -> Result<ScaledPredictionF32, BindingError> {
    prediction::from_magnitude_scaled(values, order, nyquist(code)?)
        .map(|prediction| ScaledPredictionF32 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_gain_f32(coefficients: &[f32], correlation: &[f32]) -> Result<f32, BindingError> {
    prediction::gain(coefficients, correlation).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_gain_with_f32(
    coefficients: &[f32],
    correlation: &[f32],
    code: u32,
) -> Result<f32, BindingError> {
    prediction::gain_with(coefficients, correlation, arithmetic(code)?)
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_resonances_f32(
    coefficients: &[f32],
) -> Result<PredictionResonanceEstimateF32, BindingError> {
    prediction::resonances(coefficients)
        .map(|estimate| PredictionResonanceEstimateF32 { estimate })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub struct PredictionF64 {
    prediction: Prediction<f64>,
}
impl PredictionF64 {
    pub fn new(coefficients: Vec<f64>, autocorrelation: Vec<f64>) -> Self {
        Self {
            prediction: Prediction {
                coefficients,
                autocorrelation,
            },
        }
    }
    pub fn coefficients(&self) -> Vec<f64> {
        self.prediction.coefficients.clone()
    }
    pub fn set_coefficients(&mut self, values: Vec<f64>) {
        self.prediction.coefficients = values;
    }
    pub fn autocorrelation(&self) -> Vec<f64> {
        self.prediction.autocorrelation.clone()
    }
    pub fn set_autocorrelation(&mut self, values: Vec<f64>) {
        self.prediction.autocorrelation = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            prediction: self.prediction.clone(),
        }
    }
}
pub struct ScaledPredictionF64 {
    prediction: ScaledPrediction<f64>,
}
impl ScaledPredictionF64 {
    pub fn coefficients(&self) -> Vec<f64> {
        self.prediction.coefficients().to_vec()
    }
    pub fn autocorrelation_parts(&self) -> PredictionCorrelationParts {
        let (fractions, exponents) = self.prediction.autocorrelation_parts().unzip();
        PredictionCorrelationParts {
            fractions,
            exponents,
        }
    }
    pub fn into_prediction(self) -> Result<PredictionF64, BindingError> {
        self.prediction
            .into_prediction()
            .map(|prediction| PredictionF64 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn gain_parts(&self) -> Result<PredictionGainParts, BindingError> {
        self.prediction
            .gain_parts()
            .map(|(fraction, exponent)| PredictionGainParts { fraction, exponent })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn gain(&self) -> Result<f64, BindingError> {
        self.prediction
            .gain()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            prediction: self.prediction.clone(),
        }
    }
}
pub struct PredictionSpectrumPlanF64 {
    plan: SpectrumPlan<f64>,
}
impl PredictionSpectrumPlanF64 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        SpectrumPlan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    pub fn estimate(
        &mut self,
        magnitudes: &[f64],
        order: usize,
        code: u32,
    ) -> Result<PredictionF64, BindingError> {
        self.plan
            .estimate(magnitudes, order, nyquist(code)?)
            .map(|prediction| PredictionF64 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_with_arithmetic(
        &mut self,
        magnitudes: &[f64],
        order: usize,
        nyquist_code: u32,
        arithmetic_code: u32,
    ) -> Result<PredictionF64, BindingError> {
        self.plan
            .estimate_with_arithmetic(
                magnitudes,
                order,
                nyquist(nyquist_code)?,
                arithmetic(arithmetic_code)?,
            )
            .map(|prediction| PredictionF64 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_scaled(
        &mut self,
        magnitudes: &[f64],
        order: usize,
        code: u32,
    ) -> Result<ScaledPredictionF64, BindingError> {
        self.plan
            .estimate_scaled(magnitudes, order, nyquist(code)?)
            .map(|prediction| ScaledPredictionF64 { prediction })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum(&mut self, coefficients: &[f64], gain: f64) -> Result<Vec<f64>, BindingError> {
        self.plan
            .spectrum(coefficients, gain)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
pub struct PredictionResonanceF64 {
    resonance: Resonance<f64>,
}
impl PredictionResonanceF64 {
    pub fn new(frequency_nyquist: f64, real: f64, imaginary: f64) -> Self {
        Self {
            resonance: Resonance {
                frequency_nyquist,
                pole: Complex::new(real, imaginary),
            },
        }
    }
    pub fn from_components(values: &[f64]) -> Result<Self, BindingError> {
        let [frequency, real, imaginary]: [f64; 3] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("resonance requires exactly three fields"))?;
        Ok(Self::new(frequency, real, imaginary))
    }
    pub fn components(&self) -> Vec<f64> {
        vec![
            self.resonance.frequency_nyquist,
            self.resonance.pole.re,
            self.resonance.pole.im,
        ]
    }
    pub fn set_components(&mut self, values: &[f64]) -> Result<(), BindingError> {
        *self = Self::from_components(values)?;
        Ok(())
    }
    pub fn frequency_nyquist(&self) -> f64 {
        self.resonance.frequency_nyquist
    }
    pub fn set_frequency_nyquist(&mut self, value: f64) {
        self.resonance.frequency_nyquist = value;
    }
    pub fn pole(&self) -> Vec<f64> {
        vec![self.resonance.pole.re, self.resonance.pole.im]
    }
    pub fn set_pole(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [real, imaginary]: [f64; 2] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("pole requires exactly two fields"))?;
        self.resonance.pole = Complex::new(real, imaginary);
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            resonance: self.resonance.clone(),
        }
    }
}
pub struct PredictionResonanceEstimateF64 {
    estimate: ResonanceEstimate<f64>,
}
impl PredictionResonanceEstimateF64 {
    fn unpack(values: &[f64]) -> Result<Vec<Resonance<f64>>, BindingError> {
        if !values.len().is_multiple_of(3) {
            return Err(napi::Error::from_reason(
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
impl PredictionResonanceEstimateF64 {
    pub fn new(
        resonances: &[f64],
        iterations: usize,
        converged: bool,
        maximum_residual: f64,
    ) -> Result<Self, BindingError> {
        Ok(Self {
            estimate: ResonanceEstimate {
                resonances: Self::unpack(resonances)?,
                iterations,
                converged,
                maximum_residual,
            },
        })
    }
    pub fn resonances(&self) -> Vec<f64> {
        self.estimate
            .resonances
            .iter()
            .flat_map(|r| [r.frequency_nyquist, r.pole.re, r.pole.im])
            .collect()
    }
    pub fn set_resonances(&mut self, values: &[f64]) -> Result<(), BindingError> {
        self.estimate.resonances = Self::unpack(values)?;
        Ok(())
    }
    pub fn resonance(&self, index: usize) -> Option<PredictionResonanceF64> {
        self.estimate
            .resonances
            .get(index)
            .cloned()
            .map(|resonance| PredictionResonanceF64 { resonance })
    }
    pub fn iterations(&self) -> usize {
        self.estimate.iterations
    }
    pub fn set_iterations(&mut self, value: usize) {
        self.estimate.iterations = value;
    }
    pub fn converged(&self) -> bool {
        self.estimate.converged
    }
    pub fn set_converged(&mut self, value: bool) {
        self.estimate.converged = value;
    }
    pub fn maximum_residual(&self) -> f64 {
        self.estimate.maximum_residual
    }
    pub fn set_maximum_residual(&mut self, value: f64) {
        self.estimate.maximum_residual = value;
    }
    pub fn residual_storage(&self) -> Vec<f64> {
        vec![self.estimate.maximum_residual]
    }
    pub fn set_residual_storage(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("residual requires exactly one value"))?;
        self.estimate.maximum_residual = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            estimate: self.estimate.clone(),
        }
    }
}
pub fn prediction_levinson_f64(values: &[f64]) -> Result<Vec<f64>, BindingError> {
    prediction::levinson(values).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_levinson_with_f64(values: &[f64], code: u32) -> Result<Vec<f64>, BindingError> {
    prediction::levinson_with(values, arithmetic(code)?).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_f64(
    values: &[f64],
    order: usize,
) -> Result<PredictionF64, BindingError> {
    prediction::from_samples(values, order)
        .map(|prediction| PredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_with_f64(
    values: &[f64],
    order: usize,
    code: u32,
) -> Result<PredictionF64, BindingError> {
    prediction::from_samples_with(values, order, arithmetic(code)?)
        .map(|prediction| PredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_samples_scaled_f64(
    values: &[f64],
    order: usize,
) -> Result<ScaledPredictionF64, BindingError> {
    prediction::from_samples_scaled(values, order)
        .map(|prediction| ScaledPredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_f64(
    values: &[f64],
    order: usize,
) -> Result<PredictionF64, BindingError> {
    prediction::from_magnitude(values, order)
        .map(|prediction| PredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_with_f64(
    values: &[f64],
    order: usize,
    code: u32,
) -> Result<PredictionF64, BindingError> {
    prediction::from_magnitude_with(values, order, nyquist(code)?)
        .map(|prediction| PredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_with_arithmetic_f64(
    values: &[f64],
    order: usize,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> Result<PredictionF64, BindingError> {
    prediction::from_magnitude_with_arithmetic(
        values,
        order,
        nyquist(nyquist_code)?,
        arithmetic(arithmetic_code)?,
    )
    .map(|prediction| PredictionF64 { prediction })
    .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_from_magnitude_scaled_f64(
    values: &[f64],
    order: usize,
    code: u32,
) -> Result<ScaledPredictionF64, BindingError> {
    prediction::from_magnitude_scaled(values, order, nyquist(code)?)
        .map(|prediction| ScaledPredictionF64 { prediction })
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_gain_f64(coefficients: &[f64], correlation: &[f64]) -> Result<f64, BindingError> {
    prediction::gain(coefficients, correlation).map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_gain_with_f64(
    coefficients: &[f64],
    correlation: &[f64],
    code: u32,
) -> Result<f64, BindingError> {
    prediction::gain_with(coefficients, correlation, arithmetic(code)?)
        .map_err(|e| napi::Error::from_reason(e.0))
}
pub fn prediction_resonances_f64(
    coefficients: &[f64],
) -> Result<PredictionResonanceEstimateF64, BindingError> {
    prediction::resonances(coefficients)
        .map(|estimate| PredictionResonanceEstimateF64 { estimate })
        .map_err(|e| napi::Error::from_reason(e.0))
}
