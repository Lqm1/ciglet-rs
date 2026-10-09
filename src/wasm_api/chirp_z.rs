use super::fourier::{ComplexValuesF32, ComplexValuesF64, fft_direction};
use crate::{chirp_z::CztPlan, fourier::Complex};
use wasm_bindgen::prelude::*;

macro_rules! czt_api {
    ($scalar:ty, $owner:ident, $values:ident, $function:ident) => {
        /// Reusable complete native Chirp-Z plan, including cached work storage.
        #[wasm_bindgen]
        pub struct $owner {
            plan: CztPlan<$scalar>,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(
                length: usize,
                angular_step: $scalar,
                direction: u32,
            ) -> Result<Self, JsValue> {
                CztPlan::new(length, angular_step, fft_direction(direction)?)
                    .map(|plan| Self { plan })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.plan.len()
            }
            pub fn is_empty(&self) -> bool {
                self.plan.is_empty()
            }
            /// Missing split planes represent zero; present planes match length.
            pub fn transform(
                &mut self,
                real: Option<Vec<$scalar>>,
                imaginary: Option<Vec<$scalar>>,
            ) -> Result<$values, JsValue> {
                self.plan
                    .transform(real.as_deref(), imaginary.as_deref())
                    .map(|values| $values { values })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            /// Commit alternating real/imaginary values to caller storage only
            /// after successful native computation. Shape errors are atomic.
            pub fn transform_into(
                &mut self,
                real: Option<Vec<$scalar>>,
                imaginary: Option<Vec<$scalar>>,
                output: &mut [$scalar],
            ) -> Result<(), JsValue> {
                if output.len() != self.plan.len() * 2 {
                    return Err(JsValue::from_str("CZT output length differs from the plan"));
                }
                let mut values = Vec::new();
                values
                    .try_reserve_exact(self.plan.len())
                    .map_err(|_| JsValue::from_str("CZT output allocation failed"))?;
                values.resize(self.plan.len(), Complex::new(0.0, 0.0));
                self.plan
                    .transform_into(real.as_deref(), imaginary.as_deref(), &mut values)
                    .map_err(|error| JsValue::from_str(error.0))?;
                for (pair, value) in output.chunks_exact_mut(2).zip(values) {
                    pair[0] = value.re;
                    pair[1] = value.im;
                }
                Ok(())
            }
        }
        #[wasm_bindgen]
        pub fn $function(
            length: usize,
            angular_step: $scalar,
            direction: u32,
            real: Option<Vec<$scalar>>,
            imaginary: Option<Vec<$scalar>>,
        ) -> Result<$values, JsValue> {
            $owner::new(length, angular_step, direction)?.transform(real, imaginary)
        }
    };
}
czt_api!(f32, CztPlanF32, ComplexValuesF32, czt_f32);
czt_api!(f64, CztPlanF64, ComplexValuesF64, czt_f64);
