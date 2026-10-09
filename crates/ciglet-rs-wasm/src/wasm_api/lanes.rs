use crate::approximation::lanes;
use wasm_bindgen::prelude::*;

fn four<T: Copy>(values: &[T]) -> Result<[T; 4], JsValue> {
    values
        .try_into()
        .map_err(|_| JsValue::from_str("input must contain exactly four lanes"))
}

macro_rules! unary_api {
    ($native:ident, $export:ident) => {
        /// Complete independently owned native four-lane approximation.
        #[wasm_bindgen]
        pub fn $export(values: &[f32]) -> Result<Vec<f32>, JsValue> {
            lanes::$native(four(values)?)
                .map(|values| values.to_vec())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
unary_api!(exponential2, lanes_exponential2);
unary_api!(exponential2_fast, lanes_exponential2_fast);
unary_api!(exponential, lanes_exponential);
unary_api!(exponential_fast, lanes_exponential_fast);
unary_api!(logarithm2, lanes_logarithm2);
unary_api!(logarithm2_fast, lanes_logarithm2_fast);
unary_api!(logarithm, lanes_logarithm);
unary_api!(logarithm_fast, lanes_logarithm_fast);

macro_rules! power_api {
    ($native:ident, $export:ident) => {
        /// Retain a separate exponent for each of the four native lanes.
        #[wasm_bindgen]
        pub fn $export(bases: &[f32], exponents: &[f32]) -> Result<Vec<f32>, JsValue> {
            lanes::$native(four(bases)?, four(exponents)?)
                .map(|values| values.to_vec())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
power_api!(power, lanes_power);
power_api!(power_fast, lanes_power_fast);

#[wasm_bindgen]
pub fn lanes_to_integers(values: &[f32]) -> Result<Vec<i32>, JsValue> {
    Ok(lanes::to_integers(four(values)?).to_vec())
}

#[wasm_bindgen]
pub fn lanes_from_integers(values: &[i32]) -> Result<Vec<f32>, JsValue> {
    Ok(lanes::from_integers(four(values)?).to_vec())
}

#[wasm_bindgen]
pub fn lanes_to_bits(values: &[f32]) -> Result<Vec<u32>, JsValue> {
    Ok(lanes::to_bits(four(values)?).to_vec())
}

#[wasm_bindgen]
pub fn lanes_from_bits(values: &[u32]) -> Result<Vec<f32>, JsValue> {
    Ok(lanes::from_bits(four(values)?).to_vec())
}

#[wasm_bindgen]
pub fn lanes_absolute(values: &[f32]) -> Result<Vec<f32>, JsValue> {
    Ok(lanes::absolute(four(values)?).to_vec())
}

/// Each signed 64-bit BigInt contributes its low word followed by its high word.
#[wasm_bindgen]
pub fn lanes_from_integer64(first: i64, second: i64) -> Vec<i32> {
    lanes::from_integer64_lanes([first, second]).to_vec()
}
