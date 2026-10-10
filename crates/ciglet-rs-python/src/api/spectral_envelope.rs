use crate::spectral_envelope::{self, Envelope, NyquistPolicy, Plan, SmoothingPolicy};
use pyo3::PyErr as BindingError;
fn nyquist(code: u32) -> Result<NyquistPolicy, BindingError> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid envelope Nyquist policy",
        )),
    }
}
fn smoothing(code: u32) -> Result<SmoothingPolicy, BindingError> {
    match code {
        0 => Ok(SmoothingPolicy::Stable),
        1 => Ok(SmoothingPolicy::Legacy),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid envelope smoothing policy",
        )),
    }
}
/// Both complete mutable native output fields in natural-log units.
pub struct SpectralEnvelopeF32 {
    envelope: Envelope<f32>,
}
impl SpectralEnvelopeF32 {
    pub fn new(log_spectrum: Vec<f32>, cepstrum: Vec<f32>) -> Self {
        Self {
            envelope: Envelope {
                log_spectrum,
                cepstrum,
            },
        }
    }
    pub fn log_spectrum(&self) -> Vec<f32> {
        self.envelope.log_spectrum.clone()
    }
    pub fn set_log_spectrum(&mut self, values: Vec<f32>) {
        self.envelope.log_spectrum = values;
    }
    pub fn cepstrum(&self) -> Vec<f32> {
        self.envelope.cepstrum.clone()
    }
    pub fn set_cepstrum(&mut self, values: Vec<f32>) {
        self.envelope.cepstrum = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            envelope: self.envelope.clone(),
        }
    }
}
pub struct SpectralEnvelopePlanF32 {
    plan: Plan<f32>,
}
impl SpectralEnvelopePlanF32 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        Plan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn bins(&self) -> usize {
        self.plan.bins()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    pub fn estimate(
        &mut self,
        spectrum: &[f32],
        fundamental: f32,
        harmonics: Option<usize>,
    ) -> Result<SpectralEnvelopeF32, BindingError> {
        self.plan
            .estimate(spectrum, fundamental, harmonics)
            .map(|envelope| SpectralEnvelopeF32 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn estimate_with(
        &mut self,
        spectrum: &[f32],
        fundamental: f32,
        harmonics: Option<usize>,
        nyquist_code: u32,
    ) -> Result<SpectralEnvelopeF32, BindingError> {
        self.plan
            .estimate_with(spectrum, fundamental, harmonics, nyquist(nyquist_code)?)
            .map(|envelope| SpectralEnvelopeF32 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn estimate_with_smoothing(
        &mut self,
        spectrum: &[f32],
        fundamental: f32,
        harmonics: Option<usize>,
        nyquist_code: u32,
        smoothing_code: u32,
    ) -> Result<SpectralEnvelopeF32, BindingError> {
        self.plan
            .estimate_with_smoothing(
                spectrum,
                fundamental,
                harmonics,
                nyquist(nyquist_code)?,
                smoothing(smoothing_code)?,
            )
            .map(|envelope| SpectralEnvelopeF32 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
pub fn spectral_envelope_estimate_f32(
    spectrum: &[f32],
    fft_length: usize,
    fundamental: f32,
    harmonics: Option<usize>,
) -> Result<SpectralEnvelopeF32, BindingError> {
    spectral_envelope::estimate(spectrum, fft_length, fundamental, harmonics)
        .map(|envelope| SpectralEnvelopeF32 { envelope })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Both complete mutable native output fields in natural-log units.
pub struct SpectralEnvelopeF64 {
    envelope: Envelope<f64>,
}
impl SpectralEnvelopeF64 {
    pub fn new(log_spectrum: Vec<f64>, cepstrum: Vec<f64>) -> Self {
        Self {
            envelope: Envelope {
                log_spectrum,
                cepstrum,
            },
        }
    }
    pub fn log_spectrum(&self) -> Vec<f64> {
        self.envelope.log_spectrum.clone()
    }
    pub fn set_log_spectrum(&mut self, values: Vec<f64>) {
        self.envelope.log_spectrum = values;
    }
    pub fn cepstrum(&self) -> Vec<f64> {
        self.envelope.cepstrum.clone()
    }
    pub fn set_cepstrum(&mut self, values: Vec<f64>) {
        self.envelope.cepstrum = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            envelope: self.envelope.clone(),
        }
    }
}
pub struct SpectralEnvelopePlanF64 {
    plan: Plan<f64>,
}
impl SpectralEnvelopePlanF64 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        Plan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn bins(&self) -> usize {
        self.plan.bins()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    pub fn estimate(
        &mut self,
        spectrum: &[f64],
        fundamental: f64,
        harmonics: Option<usize>,
    ) -> Result<SpectralEnvelopeF64, BindingError> {
        self.plan
            .estimate(spectrum, fundamental, harmonics)
            .map(|envelope| SpectralEnvelopeF64 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn estimate_with(
        &mut self,
        spectrum: &[f64],
        fundamental: f64,
        harmonics: Option<usize>,
        nyquist_code: u32,
    ) -> Result<SpectralEnvelopeF64, BindingError> {
        self.plan
            .estimate_with(spectrum, fundamental, harmonics, nyquist(nyquist_code)?)
            .map(|envelope| SpectralEnvelopeF64 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn estimate_with_smoothing(
        &mut self,
        spectrum: &[f64],
        fundamental: f64,
        harmonics: Option<usize>,
        nyquist_code: u32,
        smoothing_code: u32,
    ) -> Result<SpectralEnvelopeF64, BindingError> {
        self.plan
            .estimate_with_smoothing(
                spectrum,
                fundamental,
                harmonics,
                nyquist(nyquist_code)?,
                smoothing(smoothing_code)?,
            )
            .map(|envelope| SpectralEnvelopeF64 { envelope })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
pub fn spectral_envelope_estimate_f64(
    spectrum: &[f64],
    fft_length: usize,
    fundamental: f64,
    harmonics: Option<usize>,
) -> Result<SpectralEnvelopeF64, BindingError> {
    spectral_envelope::estimate(spectrum, fft_length, fundamental, harmonics)
        .map(|envelope| SpectralEnvelopeF64 { envelope })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
