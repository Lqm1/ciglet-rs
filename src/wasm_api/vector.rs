use wasm_bindgen::prelude::*;

macro_rules! vector_api {
    ($scalar:ty, $difference:ident, $sum:ident, $reverse:ident, $grid:ident, $between:ident) => {
        /// Adjacent differences with the original first sample retained.
        #[wasm_bindgen]
        pub fn $difference(values: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            crate::vector::difference(values).map_err(|error| JsValue::from_str(error.0))
        }

        /// Sequential source-precision sums, preserving the first zero sign.
        #[wasm_bindgen]
        pub fn $sum(values: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            crate::vector::cumulative_sum(values).map_err(|error| JsValue::from_str(error.0))
        }

        /// Reverse complete scalar storage, including nonfinite payloads.
        #[wasm_bindgen]
        pub fn $reverse(values: &[$scalar]) -> Vec<$scalar> {
            crate::vector::reverse(values)
        }

        /// Inclusive native linear grid, with empty/singleton and range fallbacks.
        #[wasm_bindgen]
        pub fn $grid(start: $scalar, end: $scalar, count: usize) -> Result<Vec<$scalar>, JsValue> {
            crate::vector::linear_grid(start, end, count)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Native scalar interpolation or extrapolation, with finite fallback.
        #[wasm_bindgen]
        pub fn $between(start: $scalar, end: $scalar, ratio: $scalar) -> Result<$scalar, JsValue> {
            crate::interpolation::between(start, end, ratio)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

vector_api!(
    f32,
    difference_f32,
    cumulative_sum_f32,
    reverse_f32,
    linear_grid_f32,
    interpolate_between_f32
);
vector_api!(
    f64,
    difference_f64,
    cumulative_sum_f64,
    reverse_f64,
    linear_grid_f64,
    interpolate_between_f64
);

/// Checked signed 32-bit arithmetic progression in an independent Int32Array.
#[wasm_bindgen]
pub fn integer_sequence(start: i32, step: i32, count: usize) -> Result<Vec<i32>, JsValue> {
    crate::vector::integer_sequence(start, step, count).map_err(|error| JsValue::from_str(error.0))
}
