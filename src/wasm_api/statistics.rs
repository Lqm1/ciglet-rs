use wasm_bindgen::prelude::*;

/// Native compensated/scaled moments or original selected-precision arithmetic.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum StatisticsPolicy {
    Stable,
    Legacy,
}

impl From<StatisticsPolicy> for crate::statistics::Policy {
    fn from(value: StatisticsPolicy) -> Self {
        match value {
            StatisticsPolicy::Stable => Self::Stable,
            StatisticsPolicy::Legacy => Self::Legacy,
        }
    }
}

macro_rules! plain_unary {
    ($scalar:ty, $name:ident, $method:ident) => {
        /// Native source-precision reduction, including IEEE nonfinite behavior.
        #[wasm_bindgen]
        pub fn $name(values: &[$scalar]) -> $scalar {
            crate::statistics::$method(values)
        }
    };
}
macro_rules! unary_moment {
    ($scalar:ty, $name:ident, $with:ident, $method:ident, $method_with:ident) => {
        /// Native stable moment with checked finite inputs and output range.
        #[wasm_bindgen]
        pub fn $name(values: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::statistics::$method(values).map_err(|error| JsValue::from_str(error.0))
        }
        /// Native moment with explicit stable or source-compatible arithmetic.
        #[wasm_bindgen]
        pub fn $with(values: &[$scalar], policy: StatisticsPolicy) -> Result<$scalar, JsValue> {
            crate::statistics::$method_with(values, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
macro_rules! paired_moment {
    ($scalar:ty, $name:ident, $with:ident, $method:ident, $method_with:ident) => {
        /// Native stable paired moment with complete shape and range validation.
        #[wasm_bindgen]
        pub fn $name(left: &[$scalar], right: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::statistics::$method(left, right).map_err(|error| JsValue::from_str(error.0))
        }
        /// Native paired moment with explicit stable or source-compatible arithmetic.
        #[wasm_bindgen]
        pub fn $with(
            left: &[$scalar],
            right: &[$scalar],
            policy: StatisticsPolicy,
        ) -> Result<$scalar, JsValue> {
            crate::statistics::$method_with(left, right, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
macro_rules! products {
    ($scalar:ty, $dot:ident, $cross:ident) => {
        /// Native sequential dot product, retaining selected-precision IEEE arithmetic.
        #[wasm_bindgen]
        pub fn $dot(left: &[$scalar], right: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::statistics::dot(left, right).map_err(|error| JsValue::from_str(error.0))
        }
        /// Nonnegative-lag unnormalized cross correlation with zero-filled excess lags.
        #[wasm_bindgen]
        pub fn $cross(
            left: &[$scalar],
            right: &[$scalar],
            lags: usize,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::statistics::cross_correlation(left, right, lags)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

plain_unary!(f32, sum_f32, sum);
plain_unary!(f64, sum_f64, sum);
plain_unary!(f32, sum_squares_f32, sum_squares);
plain_unary!(f64, sum_squares_f64, sum_squares);
unary_moment!(f32, mean_f32, mean_with_f32, mean, mean_with);
unary_moment!(f64, mean_f64, mean_with_f64, mean, mean_with);
unary_moment!(
    f32,
    variance_f32,
    variance_with_f32,
    variance,
    variance_with
);
unary_moment!(
    f64,
    variance_f64,
    variance_with_f64,
    variance,
    variance_with
);
paired_moment!(
    f32,
    covariance_f32,
    covariance_with_f32,
    covariance,
    covariance_with
);
paired_moment!(
    f64,
    covariance_f64,
    covariance_with_f64,
    covariance,
    covariance_with
);
paired_moment!(
    f32,
    correlation_f32,
    correlation_with_f32,
    correlation,
    correlation_with
);
paired_moment!(
    f64,
    correlation_f64,
    correlation_with_f64,
    correlation,
    correlation_with
);
products!(f32, dot_f32, cross_correlation_f32);
products!(f64, dot_f64, cross_correlation_f64);
