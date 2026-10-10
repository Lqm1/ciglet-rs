use crate::complex::{self, Complex, Precision};
use napi::Error as BindingError;
fn precision(value: u32) -> Result<Precision, BindingError> {
    match value {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(napi::Error::from_reason("invalid complex precision")),
    }
}
/// Complete owned native complex value, including arbitrary component bits.
pub struct ComplexF32 {
    value: Complex<f32>,
}
impl ComplexF32 {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Self {
            value: Complex::new(real, imaginary),
        }
    }
    /// Construct from two typed-array components without Number conversion.
    pub fn from_components(values: &[f32]) -> Result<Self, BindingError> {
        let [real, imaginary]: [f32; 2] = values.try_into().map_err(|_| {
            napi::Error::from_reason("complex value requires exactly two components")
        })?;
        Ok(Self::new(real, imaginary))
    }
    pub fn real(&self) -> f32 {
        self.value.re
    }
    pub fn set_real(&mut self, value: f32) {
        self.value.re = value;
    }
    pub fn imaginary(&self) -> f32 {
        self.value.im
    }
    pub fn set_imaginary(&mut self, value: f32) {
        self.value.im = value;
    }
    /// Independently owned real/imaginary storage retaining full payload bits.
    pub fn components(&self) -> Vec<f32> {
        vec![self.value.re, self.value.im]
    }
    /// Replace both native fields atomically using complete typed storage.
    pub fn set_components(&mut self, values: &[f32]) -> Result<(), BindingError> {
        self.value = Self::from_components(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            value: self.value.conj(),
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
    pub fn divide(&self, other: &Self) -> Result<Self, BindingError> {
        complex::divide(self.value, other.value)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn exponential(&self, mode: u32) -> Result<Self, BindingError> {
        complex::exponential(self.value, precision(mode)?)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn argument(&self, mode: u32) -> Result<f32, BindingError> {
        complex::argument(self.value, precision(mode)?)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
/// Complete owned native complex value, including arbitrary component bits.
pub struct ComplexF64 {
    value: Complex<f64>,
}
impl ComplexF64 {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self {
            value: Complex::new(real, imaginary),
        }
    }
    /// Construct from two typed-array components without Number conversion.
    pub fn from_components(values: &[f64]) -> Result<Self, BindingError> {
        let [real, imaginary]: [f64; 2] = values.try_into().map_err(|_| {
            napi::Error::from_reason("complex value requires exactly two components")
        })?;
        Ok(Self::new(real, imaginary))
    }
    pub fn real(&self) -> f64 {
        self.value.re
    }
    pub fn set_real(&mut self, value: f64) {
        self.value.re = value;
    }
    pub fn imaginary(&self) -> f64 {
        self.value.im
    }
    pub fn set_imaginary(&mut self, value: f64) {
        self.value.im = value;
    }
    /// Independently owned real/imaginary storage retaining full payload bits.
    pub fn components(&self) -> Vec<f64> {
        vec![self.value.re, self.value.im]
    }
    /// Replace both native fields atomically using complete typed storage.
    pub fn set_components(&mut self, values: &[f64]) -> Result<(), BindingError> {
        self.value = Self::from_components(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            value: self.value.conj(),
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
    pub fn divide(&self, other: &Self) -> Result<Self, BindingError> {
        complex::divide(self.value, other.value)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn exponential(&self, mode: u32) -> Result<Self, BindingError> {
        complex::exponential(self.value, precision(mode)?)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn argument(&self, mode: u32) -> Result<f64, BindingError> {
        complex::argument(self.value, precision(mode)?)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
