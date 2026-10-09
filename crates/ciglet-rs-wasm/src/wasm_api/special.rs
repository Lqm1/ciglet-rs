use crate::approximation::{lanes, special};
use wasm_bindgen::prelude::*;

macro_rules! special_api {
    ($native:ident, $scalar:ident, $lanes:ident) => {
        /// Native binary32 special-function kernel, retaining its validated domain.
        #[wasm_bindgen]
        pub fn $scalar(value: f32) -> Result<f32, JsValue> {
            special::$native(value).map_err(|error| JsValue::from_str(error.0))
        }

        /// Complete independently owned four-lane output; no partial result escapes.
        #[wasm_bindgen]
        pub fn $lanes(values: &[f32]) -> Result<Vec<f32>, JsValue> {
            let values: [f32; 4] = values.try_into().map_err(|_| {
                JsValue::from_str("special-function input must contain exactly four lanes")
            })?;
            lanes::special::$native(values)
                .map(|values| values.to_vec())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

special_api!(
    complementary_error,
    special_complementary_error,
    special_complementary_error_lanes
);
special_api!(
    complementary_error_fast,
    special_complementary_error_fast,
    special_complementary_error_fast_lanes
);
special_api!(error, special_error, special_error_lanes);
special_api!(error_fast, special_error_fast, special_error_fast_lanes);
special_api!(
    inverse_error,
    special_inverse_error,
    special_inverse_error_lanes
);
special_api!(
    inverse_error_fast,
    special_inverse_error_fast,
    special_inverse_error_fast_lanes
);
special_api!(log_gamma, special_log_gamma, special_log_gamma_lanes);
special_api!(
    log_gamma_fast,
    special_log_gamma_fast,
    special_log_gamma_fast_lanes
);
special_api!(digamma, special_digamma, special_digamma_lanes);
special_api!(
    digamma_fast,
    special_digamma_fast,
    special_digamma_fast_lanes
);
special_api!(lambert_w, special_lambert_w, special_lambert_w_lanes);
special_api!(
    lambert_w_fast,
    special_lambert_w_fast,
    special_lambert_w_fast_lanes
);
special_api!(
    lambert_w_exp,
    special_lambert_w_exp,
    special_lambert_w_exp_lanes
);
special_api!(
    lambert_w_exp_fast,
    special_lambert_w_exp_fast,
    special_lambert_w_exp_fast_lanes
);
