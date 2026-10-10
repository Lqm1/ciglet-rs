use crate::instantaneous_frequency::{Detector, Kernels, Policy};
use napi::Error as BindingError;
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum InstantaneousFrequencyPolicy {
    Stable = 0,
    Legacy = 1,
}
fn policy(value: u32) -> Result<Policy, BindingError> {
    match value {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(napi::Error::from_reason(
            "invalid instantaneous frequency policy",
        )),
    }
}
/// Complete mutable impulse responses, including temporarily invalid edits.
pub struct InstantaneousFrequencyKernelsF32 {
    kernels: Kernels<f32>,
}
impl InstantaneousFrequencyKernelsF32 {
    pub fn new(
        real: Vec<f32>,
        imaginary: Vec<f32>,
        derivative_real: Vec<f32>,
        derivative_imaginary: Vec<f32>,
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
    pub fn real(&self) -> Vec<f32> {
        self.kernels.real.clone()
    }
    pub fn set_real(&mut self, values: Vec<f32>) {
        self.kernels.real = values;
    }
    pub fn imaginary(&self) -> Vec<f32> {
        self.kernels.imaginary.clone()
    }
    pub fn set_imaginary(&mut self, values: Vec<f32>) {
        self.kernels.imaginary = values;
    }
    pub fn derivative_real(&self) -> Vec<f32> {
        self.kernels.derivative_real.clone()
    }
    pub fn set_derivative_real(&mut self, values: Vec<f32>) {
        self.kernels.derivative_real = values;
    }
    pub fn derivative_imaginary(&self) -> Vec<f32> {
        self.kernels.derivative_imaginary.clone()
    }
    pub fn set_derivative_imaginary(&mut self, values: Vec<f32>) {
        self.kernels.derivative_imaginary = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            kernels: self.kernels.clone(),
        }
    }
}
/// Reusable native centered instantaneous-frequency detector.
pub struct InstantaneousFrequencyDetectorF32 {
    detector: Detector<f32>,
}
impl InstantaneousFrequencyDetectorF32 {
    pub fn new(center_frequency: f32, resolution: f32) -> Result<Self, BindingError> {
        Detector::new(center_frequency, resolution)
            .map(|detector| Self { detector })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn from_kernels(
        center_frequency: f32,
        kernels: &InstantaneousFrequencyKernelsF32,
    ) -> Result<Self, BindingError> {
        Detector::from_kernels(center_frequency, kernels.kernels.clone())
            .map(|detector| Self { detector })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn center_frequency(&self) -> f32 {
        self.detector.center_frequency()
    }
    pub fn set_center_frequency(&mut self, value: f32) -> Result<(), BindingError> {
        self.detector
            .set_center_frequency(value)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn kernels(&self) -> InstantaneousFrequencyKernelsF32 {
        InstantaneousFrequencyKernelsF32 {
            kernels: self.detector.kernels().clone(),
        }
    }
    /// Full replacement mirrors unrestricted native kernels_mut access.
    pub fn set_kernels(&mut self, kernels: &InstantaneousFrequencyKernelsF32) {
        self.detector.kernels_mut().clone_from(&kernels.kernels);
    }
    pub fn estimate(&self, samples: &[f32]) -> Result<f32, BindingError> {
        self.detector
            .estimate(samples)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_with(&self, samples: &[f32], policy_code: u32) -> Result<f32, BindingError> {
        self.detector
            .estimate_with(samples, policy(policy_code)?)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            detector: self.detector.clone(),
        }
    }
}
/// Complete mutable impulse responses, including temporarily invalid edits.
pub struct InstantaneousFrequencyKernelsF64 {
    kernels: Kernels<f64>,
}
impl InstantaneousFrequencyKernelsF64 {
    pub fn new(
        real: Vec<f64>,
        imaginary: Vec<f64>,
        derivative_real: Vec<f64>,
        derivative_imaginary: Vec<f64>,
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
    pub fn real(&self) -> Vec<f64> {
        self.kernels.real.clone()
    }
    pub fn set_real(&mut self, values: Vec<f64>) {
        self.kernels.real = values;
    }
    pub fn imaginary(&self) -> Vec<f64> {
        self.kernels.imaginary.clone()
    }
    pub fn set_imaginary(&mut self, values: Vec<f64>) {
        self.kernels.imaginary = values;
    }
    pub fn derivative_real(&self) -> Vec<f64> {
        self.kernels.derivative_real.clone()
    }
    pub fn set_derivative_real(&mut self, values: Vec<f64>) {
        self.kernels.derivative_real = values;
    }
    pub fn derivative_imaginary(&self) -> Vec<f64> {
        self.kernels.derivative_imaginary.clone()
    }
    pub fn set_derivative_imaginary(&mut self, values: Vec<f64>) {
        self.kernels.derivative_imaginary = values;
    }
    pub fn cloned(&self) -> Self {
        Self {
            kernels: self.kernels.clone(),
        }
    }
}
/// Reusable native centered instantaneous-frequency detector.
pub struct InstantaneousFrequencyDetectorF64 {
    detector: Detector<f64>,
}
impl InstantaneousFrequencyDetectorF64 {
    pub fn new(center_frequency: f64, resolution: f64) -> Result<Self, BindingError> {
        Detector::new(center_frequency, resolution)
            .map(|detector| Self { detector })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn from_kernels(
        center_frequency: f64,
        kernels: &InstantaneousFrequencyKernelsF64,
    ) -> Result<Self, BindingError> {
        Detector::from_kernels(center_frequency, kernels.kernels.clone())
            .map(|detector| Self { detector })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn center_frequency(&self) -> f64 {
        self.detector.center_frequency()
    }
    pub fn set_center_frequency(&mut self, value: f64) -> Result<(), BindingError> {
        self.detector
            .set_center_frequency(value)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn kernels(&self) -> InstantaneousFrequencyKernelsF64 {
        InstantaneousFrequencyKernelsF64 {
            kernels: self.detector.kernels().clone(),
        }
    }
    /// Full replacement mirrors unrestricted native kernels_mut access.
    pub fn set_kernels(&mut self, kernels: &InstantaneousFrequencyKernelsF64) {
        self.detector.kernels_mut().clone_from(&kernels.kernels);
    }
    pub fn estimate(&self, samples: &[f64]) -> Result<f64, BindingError> {
        self.detector
            .estimate(samples)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn estimate_with(&self, samples: &[f64], policy_code: u32) -> Result<f64, BindingError> {
        self.detector
            .estimate_with(samples, policy(policy_code)?)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            detector: self.detector.clone(),
        }
    }
}
