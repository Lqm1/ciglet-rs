use super::ApproximationPrecision;
use crate::approximation::{lanes, trigonometric};
use wasm_bindgen::prelude::*;

macro_rules! scalar_api {
    ($native:ident, $export:ident) => {
        /// Native binary32 polynomial or full-range tangent in every precision mode.
        #[wasm_bindgen]
        pub fn $export(angle: f32, precision: ApproximationPrecision) -> Result<f32, JsValue> {
            trigonometric::$native(angle, precision.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
scalar_api!(sine, trigonometric_sine);
scalar_api!(cosine, trigonometric_cosine);
scalar_api!(tangent, trigonometric_tangent);
scalar_api!(tangent_full, trigonometric_tangent_full);

macro_rules! lane_api {
    ($native:ident, $export:ident) => {
        /// Complete four-lane native expression, retaining vector-specific branches.
        #[wasm_bindgen]
        pub fn $export(
            angles: &[f32],
            precision: ApproximationPrecision,
        ) -> Result<Vec<f32>, JsValue> {
            let angles: [f32; 4] = angles.try_into().map_err(|_| {
                JsValue::from_str("trigonometric input must contain exactly four lanes")
            })?;
            lanes::trigonometric::$native(angles, precision.into())
                .map(|values| values.to_vec())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
lane_api!(sine, trigonometric_sine_lanes);
lane_api!(cosine, trigonometric_cosine_lanes);
lane_api!(tangent, trigonometric_tangent_lanes);
lane_api!(sine_full, trigonometric_sine_full_lanes);
lane_api!(cosine_full, trigonometric_cosine_full_lanes);
lane_api!(tangent_full, trigonometric_tangent_full_lanes);
