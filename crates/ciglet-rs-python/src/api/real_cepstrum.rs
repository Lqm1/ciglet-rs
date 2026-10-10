use crate::real_cepstrum::{self, Lifter, NyquistPolicy, Plan};
use pyo3::PyErr as BindingError;
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum CepstralLifter {
    Causal = 0,
    Legacy = 1,
}
fn nyquist(value: u32) -> Result<NyquistPolicy, BindingError> {
    match value {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "unknown cepstral Nyquist policy",
        )),
    }
}
fn lifter(value: u32) -> Result<Lifter, BindingError> {
    match value {
        0 => Ok(Lifter::Causal),
        1 => Ok(Lifter::Legacy),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "unknown cepstral lifter",
        )),
    }
}
/// Complete reusable native real-cepstrum FFT plan and workspace.
pub struct RealCepstrumPlanF32 {
    plan: Plan<f32>,
}
impl RealCepstrumPlanF32 {
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
    pub fn analyze(&mut self, logs: &[f32], policy: u32) -> Result<Vec<f32>, BindingError> {
        self.plan
            .analyze(logs, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn reconstruct(
        &mut self,
        coefficients: &[f32],
        policy: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.plan
            .reconstruct(coefficients, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn minimum_phase_with(
        &mut self,
        logs: &[f32],
        policy: u32,
        lifter_code: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.plan
            .minimum_phase_with(logs, nyquist(policy)?, lifter(lifter_code)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn spectrogram_to_cepstra(
        &mut self,
        magnitudes: &[f32],
        count: usize,
        policy: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.plan
            .spectrogram_to_cepstra(magnitudes, count, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn cepstra_to_spectrogram(
        &mut self,
        coefficients: &[f32],
        frames: usize,
        count: usize,
        policy: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.plan
            .cepstra_to_spectrogram(coefficients, frames, count, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
pub fn real_cepstrum_from_log_spectrum_f32(logs: &[f32]) -> Result<Vec<f32>, BindingError> {
    real_cepstrum::from_log_spectrum(logs)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn real_cepstrum_to_log_spectrum_f32(
    coefficients: &[f32],
    length: usize,
) -> Result<Vec<f32>, BindingError> {
    real_cepstrum::to_log_spectrum(coefficients, length)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn real_cepstrum_minimum_phase_f32(logs: &[f32]) -> Result<Vec<f32>, BindingError> {
    real_cepstrum::minimum_phase(logs)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete reusable native real-cepstrum FFT plan and workspace.
pub struct RealCepstrumPlanF64 {
    plan: Plan<f64>,
}
impl RealCepstrumPlanF64 {
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
    pub fn analyze(&mut self, logs: &[f64], policy: u32) -> Result<Vec<f64>, BindingError> {
        self.plan
            .analyze(logs, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn reconstruct(
        &mut self,
        coefficients: &[f64],
        policy: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.plan
            .reconstruct(coefficients, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn minimum_phase_with(
        &mut self,
        logs: &[f64],
        policy: u32,
        lifter_code: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.plan
            .minimum_phase_with(logs, nyquist(policy)?, lifter(lifter_code)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn spectrogram_to_cepstra(
        &mut self,
        magnitudes: &[f64],
        count: usize,
        policy: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.plan
            .spectrogram_to_cepstra(magnitudes, count, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn cepstra_to_spectrogram(
        &mut self,
        coefficients: &[f64],
        frames: usize,
        count: usize,
        policy: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.plan
            .cepstra_to_spectrogram(coefficients, frames, count, nyquist(policy)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
pub fn real_cepstrum_from_log_spectrum_f64(logs: &[f64]) -> Result<Vec<f64>, BindingError> {
    real_cepstrum::from_log_spectrum(logs)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn real_cepstrum_to_log_spectrum_f64(
    coefficients: &[f64],
    length: usize,
) -> Result<Vec<f64>, BindingError> {
    real_cepstrum::to_log_spectrum(coefficients, length)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn real_cepstrum_minimum_phase_f64(logs: &[f64]) -> Result<Vec<f64>, BindingError> {
    real_cepstrum::minimum_phase(logs)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
