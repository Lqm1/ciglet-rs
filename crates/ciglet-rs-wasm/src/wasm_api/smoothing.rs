use wasm_bindgen::prelude::*;

/// Compensated/scaled sample-cell windows or original cumulative-sum arithmetic.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SmoothingPolicy {
    Stable,
    Legacy,
}
impl From<SmoothingPolicy> for crate::smoothing::Policy {
    fn from(value: SmoothingPolicy) -> Self {
        match value {
            SmoothingPolicy::Stable => Self::Stable,
            SmoothingPolicy::Legacy => Self::Legacy,
        }
    }
}

macro_rules! smoothing_api {
    ($scalar:ty, $median:ident, $average:ident, $average_with:ident, $rms:ident, $rms_with:ident) => {
        /// Finite median filtering with native progressive endpoint windows.
        #[wasm_bindgen]
        pub fn $median(signal: &[$scalar], width: usize) -> Result<Vec<$scalar>, JsValue> {
            crate::median::filter(signal, width).map_err(|error| JsValue::from_str(error.0))
        }

        /// Stable moving average with fractional half-width and endpoint replication.
        #[wasm_bindgen]
        pub fn $average(signal: &[$scalar], half_width: $scalar) -> Result<Vec<$scalar>, JsValue> {
            crate::smoothing::moving_average(signal, half_width)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Explicit compensated sample-cell or original cumulative-grid averaging.
        #[wasm_bindgen]
        pub fn $average_with(
            signal: &[$scalar],
            half_width: $scalar,
            policy: SmoothingPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::smoothing::moving_average_with(signal, half_width, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Stable scaled-square RMS over native integer half-width windows.
        #[wasm_bindgen]
        pub fn $rms(signal: &[$scalar], half_width: usize) -> Result<Vec<$scalar>, JsValue> {
            crate::smoothing::moving_rms(signal, half_width)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Explicit scaled-square or original square-average-square-root arithmetic.
        #[wasm_bindgen]
        pub fn $rms_with(
            signal: &[$scalar],
            half_width: usize,
            policy: SmoothingPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::smoothing::moving_rms_with(signal, half_width, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

smoothing_api!(
    f32,
    median_filter_f32,
    moving_average_f32,
    moving_average_with_f32,
    moving_rms_f32,
    moving_rms_with_f32
);
smoothing_api!(
    f64,
    median_filter_f64,
    moving_average_f64,
    moving_average_with_f64,
    moving_rms_f64,
    moving_rms_with_f64
);
