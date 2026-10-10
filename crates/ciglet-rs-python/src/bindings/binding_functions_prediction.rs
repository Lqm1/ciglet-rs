use super::*;
#[pyfunction(name = "prediction_levinson_f32")]
#[pyo3(signature = (values))]
pub fn binding_prediction_levinson_f32(values: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
    crate::api::prediction::prediction_levinson_f32(&values)
}
#[pyfunction(name = "prediction_levinson_with_f32")]
#[pyo3(signature = (values, code))]
pub fn binding_prediction_levinson_with_f32(
    values: Vec<f32>,
    code: u32,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::prediction::prediction_levinson_with_f32(&values, code)
}
#[pyfunction(name = "prediction_from_samples_f32")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_samples_f32(
    values: Vec<f32>,
    order: usize,
) -> pyo3::PyResult<PredictionF32> {
    (crate::api::prediction::prediction_from_samples_f32(&values, order))
        .map(|value| PredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_samples_with_f32")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_samples_with_f32(
    values: Vec<f32>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<PredictionF32> {
    (crate::api::prediction::prediction_from_samples_with_f32(&values, order, code))
        .map(|value| PredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_samples_scaled_f32")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_samples_scaled_f32(
    values: Vec<f32>,
    order: usize,
) -> pyo3::PyResult<ScaledPredictionF32> {
    (crate::api::prediction::prediction_from_samples_scaled_f32(&values, order))
        .map(|value| ScaledPredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_f32")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_magnitude_f32(
    values: Vec<f32>,
    order: usize,
) -> pyo3::PyResult<PredictionF32> {
    (crate::api::prediction::prediction_from_magnitude_f32(&values, order))
        .map(|value| PredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_with_f32")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_magnitude_with_f32(
    values: Vec<f32>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<PredictionF32> {
    (crate::api::prediction::prediction_from_magnitude_with_f32(&values, order, code))
        .map(|value| PredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_with_arithmetic_f32")]
#[pyo3(signature = (values, order, nyquist_code, arithmetic_code))]
pub fn binding_prediction_from_magnitude_with_arithmetic_f32(
    values: Vec<f32>,
    order: usize,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> pyo3::PyResult<PredictionF32> {
    (crate::api::prediction::prediction_from_magnitude_with_arithmetic_f32(
        &values,
        order,
        nyquist_code,
        arithmetic_code,
    ))
    .map(|value| PredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_scaled_f32")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_magnitude_scaled_f32(
    values: Vec<f32>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<ScaledPredictionF32> {
    (crate::api::prediction::prediction_from_magnitude_scaled_f32(&values, order, code))
        .map(|value| ScaledPredictionF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_gain_f32")]
#[pyo3(signature = (coefficients, correlation))]
pub fn binding_prediction_gain_f32(
    coefficients: Vec<f32>,
    correlation: Vec<f32>,
) -> pyo3::PyResult<f32> {
    crate::api::prediction::prediction_gain_f32(&coefficients, &correlation)
}
#[pyfunction(name = "prediction_gain_with_f32")]
#[pyo3(signature = (coefficients, correlation, code))]
pub fn binding_prediction_gain_with_f32(
    coefficients: Vec<f32>,
    correlation: Vec<f32>,
    code: u32,
) -> pyo3::PyResult<f32> {
    crate::api::prediction::prediction_gain_with_f32(&coefficients, &correlation, code)
}
#[pyfunction(name = "prediction_resonances_f32")]
#[pyo3(signature = (coefficients))]
pub fn binding_prediction_resonances_f32(
    coefficients: Vec<f32>,
) -> pyo3::PyResult<PredictionResonanceEstimateF32> {
    (crate::api::prediction::prediction_resonances_f32(&coefficients))
        .map(|value| PredictionResonanceEstimateF32 { inner: Some(value) })
}
#[pyfunction(name = "prediction_levinson_f64")]
#[pyo3(signature = (values))]
pub fn binding_prediction_levinson_f64(values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
    crate::api::prediction::prediction_levinson_f64(&values)
}
#[pyfunction(name = "prediction_levinson_with_f64")]
#[pyo3(signature = (values, code))]
pub fn binding_prediction_levinson_with_f64(
    values: Vec<f64>,
    code: u32,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::prediction::prediction_levinson_with_f64(&values, code)
}
#[pyfunction(name = "prediction_from_samples_f64")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_samples_f64(
    values: Vec<f64>,
    order: usize,
) -> pyo3::PyResult<PredictionF64> {
    (crate::api::prediction::prediction_from_samples_f64(&values, order))
        .map(|value| PredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_samples_with_f64")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_samples_with_f64(
    values: Vec<f64>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<PredictionF64> {
    (crate::api::prediction::prediction_from_samples_with_f64(&values, order, code))
        .map(|value| PredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_samples_scaled_f64")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_samples_scaled_f64(
    values: Vec<f64>,
    order: usize,
) -> pyo3::PyResult<ScaledPredictionF64> {
    (crate::api::prediction::prediction_from_samples_scaled_f64(&values, order))
        .map(|value| ScaledPredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_f64")]
#[pyo3(signature = (values, order))]
pub fn binding_prediction_from_magnitude_f64(
    values: Vec<f64>,
    order: usize,
) -> pyo3::PyResult<PredictionF64> {
    (crate::api::prediction::prediction_from_magnitude_f64(&values, order))
        .map(|value| PredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_with_f64")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_magnitude_with_f64(
    values: Vec<f64>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<PredictionF64> {
    (crate::api::prediction::prediction_from_magnitude_with_f64(&values, order, code))
        .map(|value| PredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_with_arithmetic_f64")]
#[pyo3(signature = (values, order, nyquist_code, arithmetic_code))]
pub fn binding_prediction_from_magnitude_with_arithmetic_f64(
    values: Vec<f64>,
    order: usize,
    nyquist_code: u32,
    arithmetic_code: u32,
) -> pyo3::PyResult<PredictionF64> {
    (crate::api::prediction::prediction_from_magnitude_with_arithmetic_f64(
        &values,
        order,
        nyquist_code,
        arithmetic_code,
    ))
    .map(|value| PredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_from_magnitude_scaled_f64")]
#[pyo3(signature = (values, order, code))]
pub fn binding_prediction_from_magnitude_scaled_f64(
    values: Vec<f64>,
    order: usize,
    code: u32,
) -> pyo3::PyResult<ScaledPredictionF64> {
    (crate::api::prediction::prediction_from_magnitude_scaled_f64(&values, order, code))
        .map(|value| ScaledPredictionF64 { inner: Some(value) })
}
#[pyfunction(name = "prediction_gain_f64")]
#[pyo3(signature = (coefficients, correlation))]
pub fn binding_prediction_gain_f64(
    coefficients: Vec<f64>,
    correlation: Vec<f64>,
) -> pyo3::PyResult<f64> {
    crate::api::prediction::prediction_gain_f64(&coefficients, &correlation)
}
#[pyfunction(name = "prediction_gain_with_f64")]
#[pyo3(signature = (coefficients, correlation, code))]
pub fn binding_prediction_gain_with_f64(
    coefficients: Vec<f64>,
    correlation: Vec<f64>,
    code: u32,
) -> pyo3::PyResult<f64> {
    crate::api::prediction::prediction_gain_with_f64(&coefficients, &correlation, code)
}
#[pyfunction(name = "prediction_resonances_f64")]
#[pyo3(signature = (coefficients))]
pub fn binding_prediction_resonances_f64(
    coefficients: Vec<f64>,
) -> pyo3::PyResult<PredictionResonanceEstimateF64> {
    (crate::api::prediction::prediction_resonances_f64(&coefficients))
        .map(|value| PredictionResonanceEstimateF64 { inner: Some(value) })
}
