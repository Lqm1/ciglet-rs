use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn scalar_math_constants() -> Vec<f64> {
    vec![
        crate::scalar::LEGACY_EPSILON,
        crate::scalar::LEGACY_INFINITY_SENTINEL,
        std::f64::consts::PI,
        std::f64::consts::FRAC_PI_2,
    ]
}

macro_rules! scalar_api {
    ($scalar:ty, $add:ident, $square:ident, $fold:ident) => {
        #[wasm_bindgen]
        pub fn $add(first: $scalar, second: $scalar) -> $scalar {
            first + second
        }
        #[wasm_bindgen]
        pub fn $square(accumulator: $scalar, value: $scalar) -> $scalar {
            accumulator + value * value
        }
        #[wasm_bindgen]
        pub fn $fold(
            values: &[$scalar],
            initial: $scalar,
            callback: Option<Function>,
            context: &JsValue,
        ) -> Result<$scalar, JsValue> {
            if values.is_empty() {
                return Ok(initial);
            }
            let callback = callback
                .ok_or_else(|| JsValue::from_str("nonempty scalar fold requires a callback"))?;
            values
                .iter()
                .copied()
                .try_fold(initial, |accumulator, value| {
                    callback
                        .call3(
                            &JsValue::UNDEFINED,
                            &JsValue::from_f64(accumulator as f64),
                            &JsValue::from_f64(value as f64),
                            context,
                        )?
                        .as_f64()
                        .map(|result| result as $scalar)
                        .ok_or_else(|| {
                            JsValue::from_str("scalar fold callback must return a number")
                        })
                })
        }
    };
}
scalar_api!(
    f32,
    scalar_add_f32,
    scalar_accumulate_square_f32,
    scalar_fold_f32
);
scalar_api!(
    f64,
    scalar_add_f64,
    scalar_accumulate_square_f64,
    scalar_fold_f64
);
