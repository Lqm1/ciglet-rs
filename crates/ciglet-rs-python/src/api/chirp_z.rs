use super::fourier::{ComplexValuesF32, ComplexValuesF64, fft_direction};
use crate::{chirp_z::CztPlan, fourier::Complex};
use pyo3::PyErr as BindingError;
/// Reusable complete native Chirp-Z plan, including cached work storage.
pub struct CztPlanF32 {
    plan: CztPlan<f32>,
}
impl CztPlanF32 {
    pub fn new(length: usize, angular_step: f32, direction: u32) -> Result<Self, BindingError> {
        CztPlan::new(length, angular_step, fft_direction(direction)?)
            .map(|plan| Self { plan })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    /// Missing split planes represent zero; present planes match length.
    pub fn transform(
        &mut self,
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
    ) -> Result<ComplexValuesF32, BindingError> {
        self.plan
            .transform(real.as_deref(), imaginary.as_deref())
            .map(|values| ComplexValuesF32 { values })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    /// Commit alternating real/imaginary values to caller storage only
    /// after successful native computation. Shape errors are atomic.
    pub fn transform_into(
        &mut self,
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
        output: &mut [f32],
    ) -> Result<(), BindingError> {
        if output.len() != self.plan.len() * 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "CZT output length differs from the plan",
            ));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.plan.len())
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("CZT output allocation failed"))?;
        values.resize(self.plan.len(), Complex::new(0.0, 0.0));
        self.plan
            .transform_into(real.as_deref(), imaginary.as_deref(), &mut values)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))?;
        for (pair, value) in output.chunks_exact_mut(2).zip(values) {
            pair[0] = value.re;
            pair[1] = value.im;
        }
        Ok(())
    }
}
pub fn czt_f32(
    length: usize,
    angular_step: f32,
    direction: u32,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
) -> Result<ComplexValuesF32, BindingError> {
    CztPlanF32::new(length, angular_step, direction)?.transform(real, imaginary)
}
/// Reusable complete native Chirp-Z plan, including cached work storage.
pub struct CztPlanF64 {
    plan: CztPlan<f64>,
}
impl CztPlanF64 {
    pub fn new(length: usize, angular_step: f64, direction: u32) -> Result<Self, BindingError> {
        CztPlan::new(length, angular_step, fft_direction(direction)?)
            .map(|plan| Self { plan })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn length(&self) -> usize {
        self.plan.len()
    }
    pub fn is_empty(&self) -> bool {
        self.plan.is_empty()
    }
    /// Missing split planes represent zero; present planes match length.
    pub fn transform(
        &mut self,
        real: Option<Vec<f64>>,
        imaginary: Option<Vec<f64>>,
    ) -> Result<ComplexValuesF64, BindingError> {
        self.plan
            .transform(real.as_deref(), imaginary.as_deref())
            .map(|values| ComplexValuesF64 { values })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    /// Commit alternating real/imaginary values to caller storage only
    /// after successful native computation. Shape errors are atomic.
    pub fn transform_into(
        &mut self,
        real: Option<Vec<f64>>,
        imaginary: Option<Vec<f64>>,
        output: &mut [f64],
    ) -> Result<(), BindingError> {
        if output.len() != self.plan.len() * 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "CZT output length differs from the plan",
            ));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.plan.len())
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("CZT output allocation failed"))?;
        values.resize(self.plan.len(), Complex::new(0.0, 0.0));
        self.plan
            .transform_into(real.as_deref(), imaginary.as_deref(), &mut values)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))?;
        for (pair, value) in output.chunks_exact_mut(2).zip(values) {
            pair[0] = value.re;
            pair[1] = value.im;
        }
        Ok(())
    }
}
pub fn czt_f64(
    length: usize,
    angular_step: f64,
    direction: u32,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
) -> Result<ComplexValuesF64, BindingError> {
    CztPlanF64::new(length, angular_step, direction)?.transform(real, imaginary)
}
