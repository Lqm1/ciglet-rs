use crate::approximation::{self, Precision};
use wasm_bindgen::prelude::*;

/// All native scalar approximation precision levels.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ApproximationPrecision {
    Fast,
    Approximate,
    Exact,
}

impl From<ApproximationPrecision> for Precision {
    fn from(value: ApproximationPrecision) -> Self {
        match value {
            ApproximationPrecision::Fast => Self::Fast,
            ApproximationPrecision::Approximate => Self::Approximate,
            ApproximationPrecision::Exact => Self::Exact,
        }
    }
}

macro_rules! unary_modes {
    ($native:ident, $single:ident, $double:ident) => {
        #[wasm_bindgen]
        pub fn $single(value: f32, precision: ApproximationPrecision) -> Result<f32, JsValue> {
            approximation::$native(value, precision.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $double(value: f64, precision: ApproximationPrecision) -> Result<f64, JsValue> {
            approximation::$native(value, precision.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
unary_modes!(sine_with, approximation_sine_f32, approximation_sine_f64);
unary_modes!(
    cosine_with,
    approximation_cosine_f32,
    approximation_cosine_f64
);
unary_modes!(
    logarithm_with,
    approximation_logarithm_f32,
    approximation_logarithm_f64
);
unary_modes!(
    exponential_with,
    approximation_exponential_f32,
    approximation_exponential_f64
);

macro_rules! scalar_api {
    ($scalar:ty, $atan:ident, $fast_atan:ident, $min:ident, $max:ident) => {
        #[wasm_bindgen]
        pub fn $atan(
            y: $scalar,
            x: $scalar,
            precision: ApproximationPrecision,
        ) -> Result<$scalar, JsValue> {
            approximation::atan2_with(y, x, precision.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $fast_atan(y: $scalar, x: $scalar) -> Result<$scalar, JsValue> {
            approximation::atan2_fast(y, x).map_err(|error| JsValue::from_str(error.0))
        }
        /// Native strict comparison, retaining the second operand on ties and NaNs.
        #[wasm_bindgen]
        pub fn $min(first: $scalar, second: $scalar) -> $scalar {
            crate::scalar::minimum(first, second)
        }
        /// Native strict comparison, retaining the second operand on ties and NaNs.
        #[wasm_bindgen]
        pub fn $max(first: $scalar, second: $scalar) -> $scalar {
            crate::scalar::maximum(first, second)
        }
    };
}
scalar_api!(
    f32,
    approximation_atan2_f32,
    approximation_atan2_fast_f32,
    scalar_minimum_f32,
    scalar_maximum_f32
);
scalar_api!(
    f64,
    approximation_atan2_f64,
    approximation_atan2_fast_f64,
    scalar_minimum_f64,
    scalar_maximum_f64
);

macro_rules! unary_binary32 {
    ($native:ident, $export:ident) => {
        #[wasm_bindgen]
        pub fn $export(value: f32) -> Result<f32, JsValue> {
            approximation::$native(value).map_err(|error| JsValue::from_str(error.0))
        }
    };
}
unary_binary32!(sine_fast, approximation_sine_fast);
unary_binary32!(cosine_fast, approximation_cosine_fast);
unary_binary32!(sine, approximation_sine);
unary_binary32!(cosine, approximation_cosine);
unary_binary32!(logarithm2_fast, approximation_logarithm2_fast);
unary_binary32!(logarithm_fast, approximation_logarithm_fast);
unary_binary32!(exponential2_fast, approximation_exponential2_fast);
unary_binary32!(exponential_fast, approximation_exponential_fast);
unary_binary32!(logarithm2, approximation_logarithm2);
unary_binary32!(logarithm, approximation_logarithm);
unary_binary32!(exponential2, approximation_exponential2);
unary_binary32!(exponential, approximation_exponential);

#[wasm_bindgen]
pub fn approximation_power(value: f32, exponent: f32) -> Result<f32, JsValue> {
    approximation::power(value, exponent).map_err(|error| JsValue::from_str(error.0))
}

#[wasm_bindgen]
pub fn approximation_power_fast(value: f32, exponent: f32) -> Result<f32, JsValue> {
    approximation::power_fast(value, exponent).map_err(|error| JsValue::from_str(error.0))
}

/// Original finite sentinel; this is not IEEE infinity.
#[wasm_bindgen]
pub fn legacy_infinity_sentinel() -> f64 {
    crate::scalar::LEGACY_INFINITY_SENTINEL
}

/// Original compatibility epsilon, distinct from machine epsilon.
#[wasm_bindgen]
pub fn legacy_epsilon() -> f64 {
    crate::scalar::LEGACY_EPSILON
}
