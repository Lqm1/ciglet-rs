use super::*;
#[napi(js_name = "kalman_filter_f32")]
pub fn binding_kalman_filter_f32(
    measurements: napi::bindgen_prelude::Float32Array,
    process_variances: napi::bindgen_prelude::Float32Array,
    measurement_variances: napi::bindgen_prelude::Float32Array,
) -> napi::Result<KalmanEstimateF32> {
    let storage_measurements = measurements.to_vec();
    let storage_process_variances = process_variances.to_vec();
    let storage_measurement_variances = measurement_variances.to_vec();
    (crate::api::kalman::kalman_filter_f32(
        &storage_measurements,
        &storage_process_variances,
        &storage_measurement_variances,
    ))
    .map(|value| KalmanEstimateF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "kalman_filter_with_initial_f32")]
pub fn binding_kalman_filter_with_initial_f32(
    measurements: napi::bindgen_prelude::Float32Array,
    process_variances: napi::bindgen_prelude::Float32Array,
    measurement_variances: napi::bindgen_prelude::Float32Array,
    initial_mean: f64,
) -> napi::Result<KalmanEstimateF32> {
    let storage_measurements = measurements.to_vec();
    let storage_process_variances = process_variances.to_vec();
    let storage_measurement_variances = measurement_variances.to_vec();
    (crate::api::kalman::kalman_filter_with_initial_f32(
        &storage_measurements,
        &storage_process_variances,
        &storage_measurement_variances,
        initial_mean as f32,
    ))
    .map(|value| KalmanEstimateF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "kalman_smooth_f32")]
pub fn binding_kalman_smooth_f32(
    means: napi::bindgen_prelude::Float32Array,
    posterior_variances: napi::bindgen_prelude::Float32Array,
    process_variances: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_means = means.to_vec();
    let storage_posterior_variances = posterior_variances.to_vec();
    let storage_process_variances = process_variances.to_vec();
    (crate::api::kalman::kalman_smooth_f32(
        &storage_means,
        &storage_posterior_variances,
        &storage_process_variances,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "kalman_filter_f64")]
pub fn binding_kalman_filter_f64(
    measurements: napi::bindgen_prelude::Float64Array,
    process_variances: napi::bindgen_prelude::Float64Array,
    measurement_variances: napi::bindgen_prelude::Float64Array,
) -> napi::Result<KalmanEstimateF64> {
    let storage_measurements = measurements.to_vec();
    let storage_process_variances = process_variances.to_vec();
    let storage_measurement_variances = measurement_variances.to_vec();
    (crate::api::kalman::kalman_filter_f64(
        &storage_measurements,
        &storage_process_variances,
        &storage_measurement_variances,
    ))
    .map(|value| KalmanEstimateF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "kalman_filter_with_initial_f64")]
pub fn binding_kalman_filter_with_initial_f64(
    measurements: napi::bindgen_prelude::Float64Array,
    process_variances: napi::bindgen_prelude::Float64Array,
    measurement_variances: napi::bindgen_prelude::Float64Array,
    initial_mean: f64,
) -> napi::Result<KalmanEstimateF64> {
    let storage_measurements = measurements.to_vec();
    let storage_process_variances = process_variances.to_vec();
    let storage_measurement_variances = measurement_variances.to_vec();
    (crate::api::kalman::kalman_filter_with_initial_f64(
        &storage_measurements,
        &storage_process_variances,
        &storage_measurement_variances,
        initial_mean,
    ))
    .map(|value| KalmanEstimateF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "kalman_smooth_f64")]
pub fn binding_kalman_smooth_f64(
    means: napi::bindgen_prelude::Float64Array,
    posterior_variances: napi::bindgen_prelude::Float64Array,
    process_variances: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_means = means.to_vec();
    let storage_posterior_variances = posterior_variances.to_vec();
    let storage_process_variances = process_variances.to_vec();
    (crate::api::kalman::kalman_smooth_f64(
        &storage_means,
        &storage_posterior_variances,
        &storage_process_variances,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
