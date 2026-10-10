use super::*;
#[napi(js_name = "prediction_levinson_f32")]
pub fn binding_prediction_levinson_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_levinson_f32(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "prediction_levinson_with_f32")]
pub fn binding_prediction_levinson_with_f32(
    values: napi::bindgen_prelude::Float32Array,
    code: u32,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_levinson_with_f32(&storage_values, code))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "prediction_from_samples_f32")]
pub fn binding_prediction_from_samples_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
) -> napi::Result<PredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_f32(&storage_values, checked_usize(order)?))
        .map(|value| PredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
}
#[napi(js_name = "prediction_from_samples_with_f32")]
pub fn binding_prediction_from_samples_with_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
    code: u32,
) -> napi::Result<PredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_with_f32(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| PredictionF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_samples_scaled_f32")]
pub fn binding_prediction_from_samples_scaled_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
) -> napi::Result<ScaledPredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_scaled_f32(
        &storage_values,
        checked_usize(order)?,
    ))
    .map(|value| ScaledPredictionF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_f32")]
pub fn binding_prediction_from_magnitude_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
) -> napi::Result<PredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_f32(&storage_values, checked_usize(order)?))
        .map(|value| PredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
}
#[napi(js_name = "prediction_from_magnitude_with_f32")]
pub fn binding_prediction_from_magnitude_with_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
    code: u32,
) -> napi::Result<PredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_with_f32(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| PredictionF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_with_arithmetic_f32")]
pub fn binding_prediction_from_magnitude_with_arithmetic_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> napi::Result<PredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_with_arithmetic_f32(
        &storage_values,
        checked_usize(order)?,
        nyquist_code,
        arithmetic_code,
    ))
    .map(|value| PredictionF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_scaled_f32")]
pub fn binding_prediction_from_magnitude_scaled_f32(
    values: napi::bindgen_prelude::Float32Array,
    order: f64,
    code: u32,
) -> napi::Result<ScaledPredictionF32> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_scaled_f32(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| ScaledPredictionF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_gain_f32")]
pub fn binding_prediction_gain_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
    correlation: napi::bindgen_prelude::Float32Array,
) -> napi::Result<f64> {
    let storage_coefficients = coefficients.to_vec();
    let storage_correlation = correlation.to_vec();
    (crate::api::prediction::prediction_gain_f32(&storage_coefficients, &storage_correlation))
        .map(|value| value as f64)
}
#[napi(js_name = "prediction_gain_with_f32")]
pub fn binding_prediction_gain_with_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
    correlation: napi::bindgen_prelude::Float32Array,
    code: u32,
) -> napi::Result<f64> {
    let storage_coefficients = coefficients.to_vec();
    let storage_correlation = correlation.to_vec();
    (crate::api::prediction::prediction_gain_with_f32(
        &storage_coefficients,
        &storage_correlation,
        code,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "prediction_resonances_f32")]
pub fn binding_prediction_resonances_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
) -> napi::Result<PredictionResonanceEstimateF32> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::prediction::prediction_resonances_f32(&storage_coefficients)).map(|value| {
        PredictionResonanceEstimateF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
#[napi(js_name = "prediction_levinson_f64")]
pub fn binding_prediction_levinson_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_levinson_f64(&storage_values))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "prediction_levinson_with_f64")]
pub fn binding_prediction_levinson_with_f64(
    values: napi::bindgen_prelude::Float64Array,
    code: u32,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_levinson_with_f64(&storage_values, code))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "prediction_from_samples_f64")]
pub fn binding_prediction_from_samples_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
) -> napi::Result<PredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_f64(&storage_values, checked_usize(order)?))
        .map(|value| PredictionF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
}
#[napi(js_name = "prediction_from_samples_with_f64")]
pub fn binding_prediction_from_samples_with_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
    code: u32,
) -> napi::Result<PredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_with_f64(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| PredictionF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_samples_scaled_f64")]
pub fn binding_prediction_from_samples_scaled_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
) -> napi::Result<ScaledPredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_samples_scaled_f64(
        &storage_values,
        checked_usize(order)?,
    ))
    .map(|value| ScaledPredictionF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_f64")]
pub fn binding_prediction_from_magnitude_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
) -> napi::Result<PredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_f64(&storage_values, checked_usize(order)?))
        .map(|value| PredictionF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
}
#[napi(js_name = "prediction_from_magnitude_with_f64")]
pub fn binding_prediction_from_magnitude_with_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
    code: u32,
) -> napi::Result<PredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_with_f64(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| PredictionF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_with_arithmetic_f64")]
pub fn binding_prediction_from_magnitude_with_arithmetic_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> napi::Result<PredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_with_arithmetic_f64(
        &storage_values,
        checked_usize(order)?,
        nyquist_code,
        arithmetic_code,
    ))
    .map(|value| PredictionF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_from_magnitude_scaled_f64")]
pub fn binding_prediction_from_magnitude_scaled_f64(
    values: napi::bindgen_prelude::Float64Array,
    order: f64,
    code: u32,
) -> napi::Result<ScaledPredictionF64> {
    let storage_values = values.to_vec();
    (crate::api::prediction::prediction_from_magnitude_scaled_f64(
        &storage_values,
        checked_usize(order)?,
        code,
    ))
    .map(|value| ScaledPredictionF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "prediction_gain_f64")]
pub fn binding_prediction_gain_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
    correlation: napi::bindgen_prelude::Float64Array,
) -> napi::Result<f64> {
    let storage_coefficients = coefficients.to_vec();
    let storage_correlation = correlation.to_vec();
    crate::api::prediction::prediction_gain_f64(&storage_coefficients, &storage_correlation)
}
#[napi(js_name = "prediction_gain_with_f64")]
pub fn binding_prediction_gain_with_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
    correlation: napi::bindgen_prelude::Float64Array,
    code: u32,
) -> napi::Result<f64> {
    let storage_coefficients = coefficients.to_vec();
    let storage_correlation = correlation.to_vec();
    crate::api::prediction::prediction_gain_with_f64(
        &storage_coefficients,
        &storage_correlation,
        code,
    )
}
#[napi(js_name = "prediction_resonances_f64")]
pub fn binding_prediction_resonances_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
) -> napi::Result<PredictionResonanceEstimateF64> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::prediction::prediction_resonances_f64(&storage_coefficients)).map(|value| {
        PredictionResonanceEstimateF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
