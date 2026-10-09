use super::fourier::{ComplexValuesF32, ComplexValuesF64};
use crate::{
    fourier::Complex,
    polynomial::{self, RootEstimate},
};
use wasm_bindgen::prelude::*;

macro_rules! polynomial_api {
    ($scalar:ty, $values:ident, $report:ident, $evaluate:ident, $evaluate_real:ident, $roots:ident, $roots_real:ident) => {
        /// Complete mutable native root report, including unconverged estimates.
        #[wasm_bindgen]
        pub struct $report {
            estimate: RootEstimate<$scalar>,
        }

        #[wasm_bindgen]
        impl $report {
            #[wasm_bindgen(constructor)]
            pub fn new(
                values: &$values,
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
            #[wasm_bindgen(getter)]
            pub fn values(&self) -> $values {
                $values {
                    values: self.estimate.values.clone(),
                }
            }
            #[wasm_bindgen(setter)]
            pub fn set_values(&mut self, values: &$values) {
                self.estimate.values.clone_from(&values.values);
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
            /// Typed storage retains arbitrary binary64 residual payload bits.
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
        pub fn $evaluate(
            coefficients: &$values,
            real: $scalar,
            imaginary: $scalar,
        ) -> Result<$values, JsValue> {
            polynomial::evaluate(&coefficients.values, Complex::new(real, imaginary))
                .map(|value| $values {
                    values: vec![value],
                })
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $evaluate_real(
            coefficients: &[$scalar],
            real: $scalar,
            imaginary: $scalar,
        ) -> Result<$values, JsValue> {
            polynomial::evaluate_real(coefficients, Complex::new(real, imaginary))
                .map(|value| $values {
                    values: vec![value],
                })
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $roots(coefficients: &$values) -> Result<$report, JsValue> {
            polynomial::roots(&coefficients.values)
                .map(|estimate| $report { estimate })
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $roots_real(coefficients: &[$scalar]) -> Result<$report, JsValue> {
            polynomial::roots_real(coefficients)
                .map(|estimate| $report { estimate })
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
polynomial_api!(
    f32,
    ComplexValuesF32,
    PolynomialRootsF32,
    polynomial_evaluate_f32,
    polynomial_evaluate_real_f32,
    polynomial_roots_f32,
    polynomial_roots_real_f32
);
polynomial_api!(
    f64,
    ComplexValuesF64,
    PolynomialRootsF64,
    polynomial_evaluate_f64,
    polynomial_evaluate_real_f64,
    polynomial_roots_f64,
    polynomial_roots_real_f64
);
