use crate::kalman::{self, Estimate};
use wasm_bindgen::prelude::*;

macro_rules! kalman_api {
    ($scalar:ty, $owner:ident, $filter:ident, $initial:ident, $smooth:ident) => {
        /// Owned complete native estimate with independently mutable public fields.
        #[wasm_bindgen]
        pub struct $owner {
            value: Estimate<$scalar>,
        }

        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(means: &[$scalar], variances: &[$scalar], log_likelihood: $scalar) -> Self {
                Self {
                    value: Estimate {
                        means: means.to_vec(),
                        variances: variances.to_vec(),
                        log_likelihood,
                    },
                }
            }

            #[wasm_bindgen(getter)]
            pub fn means(&self) -> Vec<$scalar> {
                self.value.means.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_means(&mut self, values: &[$scalar]) {
                self.value.means = values.to_vec();
            }
            #[wasm_bindgen(getter)]
            pub fn variances(&self) -> Vec<$scalar> {
                self.value.variances.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_variances(&mut self, values: &[$scalar]) {
                self.value.variances = values.to_vec();
            }
            #[wasm_bindgen(getter)]
            pub fn log_likelihood(&self) -> $scalar {
                self.value.log_likelihood
            }
            #[wasm_bindgen(setter)]
            pub fn set_log_likelihood(&mut self, value: $scalar) {
                self.value.log_likelihood = value;
            }
            /// Preserve every native scalar bit without JavaScript Number conversion.
            pub fn likelihood_value(&self) -> Vec<$scalar> {
                vec![self.value.log_likelihood]
            }
            pub fn set_likelihood_value(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [value]: [$scalar; 1] = values.try_into().map_err(|_| {
                    JsValue::from_str("Kalman likelihood requires exactly one value")
                })?;
                self.value.log_likelihood = value;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                Self {
                    value: self.value.clone(),
                }
            }
        }

        #[wasm_bindgen]
        pub fn $filter(
            measurements: &[$scalar],
            process_variances: &[$scalar],
            measurement_variances: &[$scalar],
        ) -> Result<$owner, JsValue> {
            kalman::filter(measurements, process_variances, measurement_variances)
                .map(|value| $owner { value })
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $initial(
            measurements: &[$scalar],
            process_variances: &[$scalar],
            measurement_variances: &[$scalar],
            initial_mean: $scalar,
        ) -> Result<$owner, JsValue> {
            kalman::filter_with_initial(
                measurements,
                process_variances,
                measurement_variances,
                initial_mean,
            )
            .map(|value| $owner { value })
            .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $smooth(
            means: &[$scalar],
            posterior_variances: &[$scalar],
            process_variances: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            kalman::smooth(means, posterior_variances, process_variances)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
kalman_api!(
    f32,
    KalmanEstimateF32,
    kalman_filter_f32,
    kalman_filter_with_initial_f32,
    kalman_smooth_f32
);
kalman_api!(
    f64,
    KalmanEstimateF64,
    kalman_filter_f64,
    kalman_filter_with_initial_f64,
    kalman_smooth_f64
);
