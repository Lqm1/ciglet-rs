use super::array::{Array2F32, Array2F64};
use crate::correlogram::{self, Frame, Method, Policy};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum CorrelogramMethod {
    Autocorrelation = 0,
    AbsoluteDifference = 1,
    SquaredDifference = 2,
    Yin = 3,
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum CorrelogramPolicy {
    Stable = 0,
    Legacy = 1,
}
fn method(value: u32) -> Result<Method, JsValue> {
    i32::try_from(value)
        .map_err(|_| JsValue::from_str("unknown correlogram method code"))?
        .try_into()
        .map_err(|error: crate::Error| JsValue::from_str(error.0))
}
fn policy(value: u32) -> Result<Policy, JsValue> {
    match value {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(JsValue::from_str("unknown correlogram policy code")),
    }
}
fn frames(centers: &[i32], lengths: &[u32]) -> Result<Vec<Frame>, JsValue> {
    if centers.len() != lengths.len() {
        return Err(JsValue::from_str(
            "correlogram frame arrays must have matching lengths",
        ));
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(centers.len())
        .map_err(|_| JsValue::from_str("correlogram frame allocation failed"))?;
    output.extend(centers.iter().zip(lengths).map(|(&center, &length)| Frame {
        center,
        length: length as usize,
    }));
    Ok(output)
}
/// Complete native frame configuration. Validation occurs during computation.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct CorrelogramFrame {
    pub center: i32,
    pub length: usize,
}
#[wasm_bindgen]
impl CorrelogramFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(center: i32, length: usize) -> Self {
        Self { center, length }
    }
    pub fn cloned(&self) -> Self {
        *self
    }
    pub fn compute_f32(
        &self,
        samples: &[f32],
        periods: usize,
        method_code: u32,
        policy_code: u32,
    ) -> Result<Array2F32, JsValue> {
        correlogram_compute_with_f32(
            samples,
            &[self.center],
            &[self.length as u32],
            periods,
            method_code,
            policy_code,
        )
    }
    pub fn compute_f64(
        &self,
        samples: &[f64],
        periods: usize,
        method_code: u32,
        policy_code: u32,
    ) -> Result<Array2F64, JsValue> {
        correlogram_compute_with_f64(
            samples,
            &[self.center],
            &[self.length as u32],
            periods,
            method_code,
            policy_code,
        )
    }
}
macro_rules! correlogram_api {
    ($scalar:ty, $array:ident, $compute:ident, $with:ident, $frequency:ident) => {
        #[wasm_bindgen]
        pub fn $compute(
            samples: &[$scalar],
            centers: &[i32],
            lengths: &[u32],
            periods: usize,
            method_code: u32,
        ) -> Result<$array, JsValue> {
            correlogram::compute(
                samples,
                &frames(centers, lengths)?,
                periods,
                method(method_code)?,
            )
            .map(|value| $array { value })
            .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $with(
            samples: &[$scalar],
            centers: &[i32],
            lengths: &[u32],
            periods: usize,
            method_code: u32,
            policy_code: u32,
        ) -> Result<$array, JsValue> {
            correlogram::compute_with(
                samples,
                &frames(centers, lengths)?,
                periods,
                method(method_code)?,
                policy(policy_code)?,
            )
            .map(|value| $array { value })
            .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $frequency(
            periods: &$array,
            sampling_rate: i32,
            frequencies: &[$scalar],
        ) -> Result<$array, JsValue> {
            correlogram::to_frequency(&periods.value, sampling_rate, frequencies)
                .map(|value| $array { value })
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
correlogram_api!(
    f32,
    Array2F32,
    correlogram_compute_f32,
    correlogram_compute_with_f32,
    correlogram_to_frequency_f32
);
correlogram_api!(
    f64,
    Array2F64,
    correlogram_compute_f64,
    correlogram_compute_with_f64,
    correlogram_to_frequency_f64
);
