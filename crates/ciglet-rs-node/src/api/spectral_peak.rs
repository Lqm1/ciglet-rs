use crate::spectral_peak::{self, Peak};
use napi::Error as BindingError;
/// Complete native peak value with independently mutable fields.
pub struct SpectralPeakF32 {
    value: Peak<f32>,
}
impl SpectralPeakF32 {
    pub fn new(bin: f32, magnitude: f32) -> Self {
        Self {
            value: Peak { bin, magnitude },
        }
    }
    pub fn from_values(values: &[f32]) -> Result<Self, BindingError> {
        let [bin, magnitude]: [f32; 2] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("peak requires exactly two values"))?;
        Ok(Self::new(bin, magnitude))
    }
    pub fn bin(&self) -> f32 {
        self.value.bin
    }
    pub fn set_bin(&mut self, value: f32) {
        self.value.bin = value;
    }
    pub fn magnitude(&self) -> f32 {
        self.value.magnitude
    }
    pub fn set_magnitude(&mut self, value: f32) {
        self.value.magnitude = value;
    }
    /// Independent typed storage preserves all field bits.
    pub fn values(&self) -> Vec<f32> {
        vec![self.value.bin, self.value.magnitude]
    }
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        self.value = Self::from_values(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
}
pub fn spectral_peak_interpolate_f32(
    magnitudes: &[f32],
    bin: usize,
) -> Result<SpectralPeakF32, BindingError> {
    spectral_peak::interpolate(magnitudes, bin)
        .map(|value| SpectralPeakF32 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete native peak value with independently mutable fields.
pub struct SpectralPeakF64 {
    value: Peak<f64>,
}
impl SpectralPeakF64 {
    pub fn new(bin: f64, magnitude: f64) -> Self {
        Self {
            value: Peak { bin, magnitude },
        }
    }
    pub fn from_values(values: &[f64]) -> Result<Self, BindingError> {
        let [bin, magnitude]: [f64; 2] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("peak requires exactly two values"))?;
        Ok(Self::new(bin, magnitude))
    }
    pub fn bin(&self) -> f64 {
        self.value.bin
    }
    pub fn set_bin(&mut self, value: f64) {
        self.value.bin = value;
    }
    pub fn magnitude(&self) -> f64 {
        self.value.magnitude
    }
    pub fn set_magnitude(&mut self, value: f64) {
        self.value.magnitude = value;
    }
    /// Independent typed storage preserves all field bits.
    pub fn values(&self) -> Vec<f64> {
        vec![self.value.bin, self.value.magnitude]
    }
    pub fn set_values(&mut self, values: &[f64]) -> Result<(), BindingError> {
        self.value = Self::from_values(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
}
pub fn spectral_peak_interpolate_f64(
    magnitudes: &[f64],
    bin: usize,
) -> Result<SpectralPeakF64, BindingError> {
    spectral_peak::interpolate(magnitudes, bin)
        .map(|value| SpectralPeakF64 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
