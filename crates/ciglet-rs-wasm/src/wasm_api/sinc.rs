use crate::sinc::{self, Policy};
use wasm_bindgen::prelude::*;

/// Native stable limits or the original separate denominator plateaus.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum SincPolicy {
    Stable,
    Legacy,
}
impl From<SincPolicy> for Policy {
    fn from(value: SincPolicy) -> Self {
        match value {
            SincPolicy::Stable => Self::Stable,
            SincPolicy::Legacy => Self::Legacy,
        }
    }
}

macro_rules! sinc_api {
    ($scalar:ty, $default:ident, $selected:ident, $derivative:ident, $selected_derivative:ident) => {
        #[wasm_bindgen]
        pub fn $default(length: $scalar, angle: $scalar) -> Result<$scalar, JsValue> {
            sinc::aliased(length, angle).map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $selected(
            length: $scalar,
            angle: $scalar,
            policy: SincPolicy,
        ) -> Result<$scalar, JsValue> {
            sinc::aliased_with(length, angle, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $derivative(length: $scalar, angle: $scalar) -> Result<$scalar, JsValue> {
            sinc::derivative(length, angle).map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $selected_derivative(
            length: $scalar,
            angle: $scalar,
            policy: SincPolicy,
        ) -> Result<$scalar, JsValue> {
            sinc::derivative_with(length, angle, policy.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
sinc_api!(
    f32,
    aliased_sinc_f32,
    aliased_sinc_with_f32,
    aliased_sinc_derivative_f32,
    aliased_sinc_derivative_with_f32
);
sinc_api!(
    f64,
    aliased_sinc_f64,
    aliased_sinc_with_f64,
    aliased_sinc_derivative_f64,
    aliased_sinc_derivative_with_f64
);
