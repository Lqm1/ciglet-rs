use super::fourier::{ComplexValuesF32, ComplexValuesF64};
use crate::{
    fourier::Complex,
    polynomial::{self, RootEstimate},
};
use pyo3::PyErr as BindingError;
/// Complete mutable native root report, including unconverged estimates.
pub struct PolynomialRootsF32 {
    estimate: RootEstimate<f32>,
}
impl PolynomialRootsF32 {
    pub fn new(
        values: &ComplexValuesF32,
        iterations: usize,
        converged: bool,
        maximum_residual: f64,
    ) -> Self {
        Self {
            estimate: RootEstimate {
                values: values.values.clone(),
                iterations,
                converged,
                maximum_residual,
            },
        }
    }
    pub fn values(&self) -> ComplexValuesF32 {
        ComplexValuesF32 {
            values: self.estimate.values.clone(),
        }
    }
    pub fn set_values(&mut self, values: &ComplexValuesF32) {
        self.estimate.values.clone_from(&values.values);
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
    /// Typed storage retains arbitrary binary64 residual payload bits.
    pub fn residual_storage(&self) -> Vec<f64> {
        vec![self.estimate.maximum_residual]
    }
    pub fn set_residual_storage(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("residual requires exactly one value")
        })?;
        self.estimate.maximum_residual = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            estimate: self.estimate.clone(),
        }
    }
}
pub fn polynomial_evaluate_f32(
    coefficients: &ComplexValuesF32,
    real: f32,
    imaginary: f32,
) -> Result<ComplexValuesF32, BindingError> {
    polynomial::evaluate(&coefficients.values, Complex::new(real, imaginary))
        .map(|value| ComplexValuesF32 {
            values: vec![value],
        })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_evaluate_real_f32(
    coefficients: &[f32],
    real: f32,
    imaginary: f32,
) -> Result<ComplexValuesF32, BindingError> {
    polynomial::evaluate_real(coefficients, Complex::new(real, imaginary))
        .map(|value| ComplexValuesF32 {
            values: vec![value],
        })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_roots_f32(
    coefficients: &ComplexValuesF32,
) -> Result<PolynomialRootsF32, BindingError> {
    polynomial::roots(&coefficients.values)
        .map(|estimate| PolynomialRootsF32 { estimate })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_roots_real_f32(coefficients: &[f32]) -> Result<PolynomialRootsF32, BindingError> {
    polynomial::roots_real(coefficients)
        .map(|estimate| PolynomialRootsF32 { estimate })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete mutable native root report, including unconverged estimates.
pub struct PolynomialRootsF64 {
    estimate: RootEstimate<f64>,
}
impl PolynomialRootsF64 {
    pub fn new(
        values: &ComplexValuesF64,
        iterations: usize,
        converged: bool,
        maximum_residual: f64,
    ) -> Self {
        Self {
            estimate: RootEstimate {
                values: values.values.clone(),
                iterations,
                converged,
                maximum_residual,
            },
        }
    }
    pub fn values(&self) -> ComplexValuesF64 {
        ComplexValuesF64 {
            values: self.estimate.values.clone(),
        }
    }
    pub fn set_values(&mut self, values: &ComplexValuesF64) {
        self.estimate.values.clone_from(&values.values);
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
    /// Typed storage retains arbitrary binary64 residual payload bits.
    pub fn residual_storage(&self) -> Vec<f64> {
        vec![self.estimate.maximum_residual]
    }
    pub fn set_residual_storage(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("residual requires exactly one value")
        })?;
        self.estimate.maximum_residual = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            estimate: self.estimate.clone(),
        }
    }
}
pub fn polynomial_evaluate_f64(
    coefficients: &ComplexValuesF64,
    real: f64,
    imaginary: f64,
) -> Result<ComplexValuesF64, BindingError> {
    polynomial::evaluate(&coefficients.values, Complex::new(real, imaginary))
        .map(|value| ComplexValuesF64 {
            values: vec![value],
        })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_evaluate_real_f64(
    coefficients: &[f64],
    real: f64,
    imaginary: f64,
) -> Result<ComplexValuesF64, BindingError> {
    polynomial::evaluate_real(coefficients, Complex::new(real, imaginary))
        .map(|value| ComplexValuesF64 {
            values: vec![value],
        })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_roots_f64(
    coefficients: &ComplexValuesF64,
) -> Result<PolynomialRootsF64, BindingError> {
    polynomial::roots(&coefficients.values)
        .map(|estimate| PolynomialRootsF64 { estimate })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn polynomial_roots_real_f64(coefficients: &[f64]) -> Result<PolynomialRootsF64, BindingError> {
    polynomial::roots_real(coefficients)
        .map(|estimate| PolynomialRootsF64 { estimate })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
