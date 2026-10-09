use wasm_bindgen::prelude::*;

/// Scaled stable divergence or original approximate-log arithmetic.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum DistancePolicy {
    Stable,
    Legacy,
}
impl From<DistancePolicy> for crate::distance::Policy {
    fn from(value: DistancePolicy) -> Self {
        match value {
            DistancePolicy::Stable => Self::Stable,
            DistancePolicy::Legacy => Self::Legacy,
        }
    }
}

macro_rules! frequency_scalar {
    ($method:ident, $single:ident, $double:ident) => {
        /// Native frequency conversion with original staging and domain checks.
        #[wasm_bindgen]
        pub fn $single(value: f32) -> Result<f32, JsValue> {
            crate::frequency::$method(value).map_err(|error| JsValue::from_str(error.0))
        }
        /// Native frequency conversion with original staging and domain checks.
        #[wasm_bindgen]
        pub fn $double(value: f64) -> Result<f64, JsValue> {
            crate::frequency::$method(value).map_err(|error| JsValue::from_str(error.0))
        }
    };
}
frequency_scalar!(frequency_to_mel, frequency_to_mel_f32, frequency_to_mel_f64);
frequency_scalar!(mel_to_frequency, mel_to_frequency_f32, mel_to_frequency_f64);
frequency_scalar!(
    frequency_to_bark,
    frequency_to_bark_f32,
    frequency_to_bark_f64
);
frequency_scalar!(
    bark_to_frequency,
    bark_to_frequency_f32,
    bark_to_frequency_f64
);
frequency_scalar!(equal_loudness, equal_loudness_f32, equal_loudness_f64);

macro_rules! frequency_distance_api {
    ($scalar:ty, $space:ident, $distance:ident, $log:ident, $with:ident) => {
        /// Native approximate Mel grid with intervals plus one independently owned points.
        #[wasm_bindgen]
        pub fn $space(
            minimum: $scalar,
            maximum: $scalar,
            intervals: usize,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::frequency::mel_space(minimum, maximum, intervals)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Stable mean divergence, retaining native zero/infinity semantics.
        #[wasm_bindgen]
        pub fn $distance(source: &[$scalar], reference: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::distance::itakura_saito(source, reference)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Stable log mean divergence with finite extreme-ratio output.
        #[wasm_bindgen]
        pub fn $log(source: &[$scalar], reference: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::distance::log_itakura_saito(source, reference)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Complete stable or original selected-precision approximate-log distance.
        #[wasm_bindgen]
        pub fn $with(
            source: &[$scalar],
            reference: &[$scalar],
            policy: DistancePolicy,
        ) -> Result<$scalar, JsValue> {
            crate::distance::log_itakura_saito_with(source, reference, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
frequency_distance_api!(
    f32,
    mel_space_f32,
    itakura_saito_f32,
    log_itakura_saito_f32,
    log_itakura_saito_with_f32
);
frequency_distance_api!(
    f64,
    mel_space_f64,
    itakura_saito_f64,
    log_itakura_saito_f64,
    log_itakura_saito_with_f64
);
