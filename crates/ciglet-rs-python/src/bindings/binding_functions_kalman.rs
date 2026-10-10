use super::*;
#[pyfunction(name = "kalman_filter_f32")]
#[pyo3(signature = (measurements, process_variances, measurement_variances))]
pub fn binding_kalman_filter_f32(
    measurements: Vec<f32>,
    process_variances: Vec<f32>,
    measurement_variances: Vec<f32>,
) -> pyo3::PyResult<KalmanEstimateF32> {
    (crate::api::kalman::kalman_filter_f32(
        &measurements,
        &process_variances,
        &measurement_variances,
    ))
    .map(|value| KalmanEstimateF32 { inner: Some(value) })
}
#[pyfunction(name = "kalman_filter_with_initial_f32")]
#[pyo3(
    signature = (measurements, process_variances, measurement_variances, initial_mean)
)]
pub fn binding_kalman_filter_with_initial_f32(
    measurements: Vec<f32>,
    process_variances: Vec<f32>,
    measurement_variances: Vec<f32>,
    initial_mean: f32,
) -> pyo3::PyResult<KalmanEstimateF32> {
    (crate::api::kalman::kalman_filter_with_initial_f32(
        &measurements,
        &process_variances,
        &measurement_variances,
        initial_mean,
    ))
    .map(|value| KalmanEstimateF32 { inner: Some(value) })
}
#[pyfunction(name = "kalman_smooth_f32")]
#[pyo3(signature = (means, posterior_variances, process_variances))]
pub fn binding_kalman_smooth_f32(
    means: Vec<f32>,
    posterior_variances: Vec<f32>,
    process_variances: Vec<f32>,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::kalman::kalman_smooth_f32(&means, &posterior_variances, &process_variances)
}
#[pyfunction(name = "kalman_filter_f64")]
#[pyo3(signature = (measurements, process_variances, measurement_variances))]
pub fn binding_kalman_filter_f64(
    measurements: Vec<f64>,
    process_variances: Vec<f64>,
    measurement_variances: Vec<f64>,
) -> pyo3::PyResult<KalmanEstimateF64> {
    (crate::api::kalman::kalman_filter_f64(
        &measurements,
        &process_variances,
        &measurement_variances,
    ))
    .map(|value| KalmanEstimateF64 { inner: Some(value) })
}
#[pyfunction(name = "kalman_filter_with_initial_f64")]
#[pyo3(
    signature = (measurements, process_variances, measurement_variances, initial_mean)
)]
pub fn binding_kalman_filter_with_initial_f64(
    measurements: Vec<f64>,
    process_variances: Vec<f64>,
    measurement_variances: Vec<f64>,
    initial_mean: f64,
) -> pyo3::PyResult<KalmanEstimateF64> {
    (crate::api::kalman::kalman_filter_with_initial_f64(
        &measurements,
        &process_variances,
        &measurement_variances,
        initial_mean,
    ))
    .map(|value| KalmanEstimateF64 { inner: Some(value) })
}
#[pyfunction(name = "kalman_smooth_f64")]
#[pyo3(signature = (means, posterior_variances, process_variances))]
pub fn binding_kalman_smooth_f64(
    means: Vec<f64>,
    posterior_variances: Vec<f64>,
    process_variances: Vec<f64>,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::kalman::kalman_smooth_f64(&means, &posterior_variances, &process_variances)
}
