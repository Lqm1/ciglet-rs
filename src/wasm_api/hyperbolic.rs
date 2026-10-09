use super::ApproximationPrecision;
use crate::approximation::{hyperbolic, lanes};
use wasm_bindgen::prelude::*;

macro_rules! hyperbolic_api {
    ($native:ident, $scalar:ident, $lanes:ident) => {
        /// Native binary32 hyperbolic formula with all three precision modes.
        #[wasm_bindgen]
        pub fn $scalar(value: f32, precision: ApproximationPrecision) -> Result<f32, JsValue> {
            hyperbolic::$native(value, precision.into()).map_err(|error| JsValue::from_str(error.0))
        }

        /// Complete four-lane native formula; any invalid lane rejects the call.
        #[wasm_bindgen]
        pub fn $lanes(
            values: &[f32],
            precision: ApproximationPrecision,
        ) -> Result<Vec<f32>, JsValue> {
            let values: [f32; 4] = values.try_into().map_err(|_| {
                JsValue::from_str("hyperbolic input must contain exactly four lanes")
            })?;
            lanes::hyperbolic::$native(values, precision.into())
                .map(|values| values.to_vec())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

hyperbolic_api!(sine, hyperbolic_sine, hyperbolic_sine_lanes);
hyperbolic_api!(cosine, hyperbolic_cosine, hyperbolic_cosine_lanes);
hyperbolic_api!(tangent, hyperbolic_tangent, hyperbolic_tangent_lanes);
hyperbolic_api!(sigmoid, hyperbolic_sigmoid, hyperbolic_sigmoid_lanes);
