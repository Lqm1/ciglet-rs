use crate::glottal::{Model, Precision, Prepared, Spectrum};
use napi::Error as BindingError;
fn precision(code: u32) -> Result<Precision, BindingError> {
    match code {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(napi::Error::from_reason("invalid LF precision")),
    }
}
/// All mutable native model fields, including intermediate invalid configurations.
#[derive(Clone)]
pub struct GlottalModelF32 {
    pub period: f32,
    pub closure: f32,
    pub peak: f32,
    pub return_time: f32,
    pub excitation: f32,
}
impl GlottalModelF32 {
    pub(super) fn native(&self) -> Model<f32> {
        Model {
            period: self.period,
            closure: self.closure,
            peak: self.peak,
            return_time: self.return_time,
            excitation: self.excitation,
        }
    }
    fn from_native(model: Model<f32>) -> Self {
        Self {
            period: model.period,
            closure: model.closure,
            peak: model.peak,
            return_time: model.return_time,
            excitation: model.excitation,
        }
    }
}
impl GlottalModelF32 {
    pub fn new(period: f32, closure: f32, peak: f32, return_time: f32, excitation: f32) -> Self {
        Self {
            period,
            closure,
            peak,
            return_time,
            excitation,
        }
    }
    pub fn from_shape(shape: f32, period: f32, excitation: f32) -> Result<Self, BindingError> {
        Model::from_shape(shape, period, excitation)
            .map(Self::from_native)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn from_components(values: &[f32]) -> Result<Self, BindingError> {
        let [period, closure, peak, return_time, excitation]: [f32; 5] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("LF model requires exactly five fields"))?;
        Ok(Self::new(period, closure, peak, return_time, excitation))
    }
    pub fn components(&self) -> Vec<f32> {
        vec![
            self.period,
            self.closure,
            self.peak,
            self.return_time,
            self.excitation,
        ]
    }
    pub fn set_components(&mut self, values: &[f32]) -> Result<(), BindingError> {
        *self = Self::from_components(values)?;
        Ok(())
    }
    pub fn prepare(&self) -> Result<GlottalPreparedF32, BindingError> {
        self.native()
            .prepare()
            .map(|prepared| GlottalPreparedF32 { prepared })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum(&self, frequencies: &[f32]) -> Result<GlottalSpectrumF32, BindingError> {
        self.native()
            .spectrum(frequencies)
            .map(|spectrum| GlottalSpectrumF32 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> Result<Vec<f32>, BindingError> {
        self.native()
            .sampled_period(sampling_rate, count)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
pub struct GlottalPreparedF32 {
    prepared: Prepared<f32>,
}
impl GlottalPreparedF32 {
    pub fn model(&self) -> GlottalModelF32 {
        GlottalModelF32::from_native(self.prepared.model())
    }
    pub fn spectrum(&self, frequencies: &[f32]) -> Result<GlottalSpectrumF32, BindingError> {
        self.prepared
            .spectrum(frequencies)
            .map(|spectrum| GlottalSpectrumF32 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum_with(
        &self,
        frequencies: &[f32],
        code: u32,
    ) -> Result<GlottalSpectrumF32, BindingError> {
        self.prepared
            .spectrum_with(frequencies, precision(code)?)
            .map(|spectrum| GlottalSpectrumF32 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> Result<Vec<f32>, BindingError> {
        self.prepared
            .sampled_period(sampling_rate, count)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            prepared: self.prepared.clone(),
        }
    }
}
pub struct GlottalSpectrumF32 {
    spectrum: Spectrum<f32>,
}
impl GlottalSpectrumF32 {
    pub fn new(magnitude: Vec<f32>, phase: Vec<f32>) -> Self {
        Self {
            spectrum: Spectrum { magnitude, phase },
        }
    }
    pub fn magnitude(&self) -> Vec<f32> {
        self.spectrum.magnitude.clone()
    }
    pub fn set_magnitude(&mut self, values: Vec<f32>) {
        self.spectrum.magnitude = values;
    }
    pub fn phase(&self) -> Vec<f32> {
        self.spectrum.phase.clone()
    }
    pub fn set_phase(&mut self, values: Vec<f32>) {
        self.spectrum.phase = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            spectrum: self.spectrum.clone(),
        }
    }
}
/// All mutable native model fields, including intermediate invalid configurations.
#[derive(Clone)]
pub struct GlottalModelF64 {
    pub period: f64,
    pub closure: f64,
    pub peak: f64,
    pub return_time: f64,
    pub excitation: f64,
}
impl GlottalModelF64 {
    pub(super) fn native(&self) -> Model<f64> {
        Model {
            period: self.period,
            closure: self.closure,
            peak: self.peak,
            return_time: self.return_time,
            excitation: self.excitation,
        }
    }
    fn from_native(model: Model<f64>) -> Self {
        Self {
            period: model.period,
            closure: model.closure,
            peak: model.peak,
            return_time: model.return_time,
            excitation: model.excitation,
        }
    }
}
impl GlottalModelF64 {
    pub fn new(period: f64, closure: f64, peak: f64, return_time: f64, excitation: f64) -> Self {
        Self {
            period,
            closure,
            peak,
            return_time,
            excitation,
        }
    }
    pub fn from_shape(shape: f64, period: f64, excitation: f64) -> Result<Self, BindingError> {
        Model::from_shape(shape, period, excitation)
            .map(Self::from_native)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn from_components(values: &[f64]) -> Result<Self, BindingError> {
        let [period, closure, peak, return_time, excitation]: [f64; 5] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("LF model requires exactly five fields"))?;
        Ok(Self::new(period, closure, peak, return_time, excitation))
    }
    pub fn components(&self) -> Vec<f64> {
        vec![
            self.period,
            self.closure,
            self.peak,
            self.return_time,
            self.excitation,
        ]
    }
    pub fn set_components(&mut self, values: &[f64]) -> Result<(), BindingError> {
        *self = Self::from_components(values)?;
        Ok(())
    }
    pub fn prepare(&self) -> Result<GlottalPreparedF64, BindingError> {
        self.native()
            .prepare()
            .map(|prepared| GlottalPreparedF64 { prepared })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum(&self, frequencies: &[f64]) -> Result<GlottalSpectrumF64, BindingError> {
        self.native()
            .spectrum(frequencies)
            .map(|spectrum| GlottalSpectrumF64 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> Result<Vec<f64>, BindingError> {
        self.native()
            .sampled_period(sampling_rate, count)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
pub struct GlottalPreparedF64 {
    prepared: Prepared<f64>,
}
impl GlottalPreparedF64 {
    pub fn model(&self) -> GlottalModelF64 {
        GlottalModelF64::from_native(self.prepared.model())
    }
    pub fn spectrum(&self, frequencies: &[f64]) -> Result<GlottalSpectrumF64, BindingError> {
        self.prepared
            .spectrum(frequencies)
            .map(|spectrum| GlottalSpectrumF64 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn spectrum_with(
        &self,
        frequencies: &[f64],
        code: u32,
    ) -> Result<GlottalSpectrumF64, BindingError> {
        self.prepared
            .spectrum_with(frequencies, precision(code)?)
            .map(|spectrum| GlottalSpectrumF64 { spectrum })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> Result<Vec<f64>, BindingError> {
        self.prepared
            .sampled_period(sampling_rate, count)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            prepared: self.prepared.clone(),
        }
    }
}
pub struct GlottalSpectrumF64 {
    spectrum: Spectrum<f64>,
}
impl GlottalSpectrumF64 {
    pub fn new(magnitude: Vec<f64>, phase: Vec<f64>) -> Self {
        Self {
            spectrum: Spectrum { magnitude, phase },
        }
    }
    pub fn magnitude(&self) -> Vec<f64> {
        self.spectrum.magnitude.clone()
    }
    pub fn set_magnitude(&mut self, values: Vec<f64>) {
        self.spectrum.magnitude = values;
    }
    pub fn phase(&self) -> Vec<f64> {
        self.spectrum.phase.clone()
    }
    pub fn set_phase(&mut self, values: Vec<f64>) {
        self.spectrum.phase = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            spectrum: self.spectrum.clone(),
        }
    }
}
