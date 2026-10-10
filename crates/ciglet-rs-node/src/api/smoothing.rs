use napi::Error as BindingError;
/// Compensated/scaled sample-cell windows or original cumulative-sum arithmetic.
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum SmoothingPolicy {
    Stable,
    Legacy,
}
impl From<SmoothingPolicy> for crate::smoothing::Policy {
    fn from(value: SmoothingPolicy) -> Self {
        match value {
            SmoothingPolicy::Stable => Self::Stable,
            SmoothingPolicy::Legacy => Self::Legacy,
        }
    }
}
/// Finite median filtering with native progressive endpoint windows.
pub fn median_filter_f32(signal: &[f32], width: usize) -> Result<Vec<f32>, BindingError> {
    crate::median::filter(signal, width).map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable moving average with fractional half-width and endpoint replication.
pub fn moving_average_f32(signal: &[f32], half_width: f32) -> Result<Vec<f32>, BindingError> {
    crate::smoothing::moving_average(signal, half_width)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit compensated sample-cell or original cumulative-grid averaging.
pub fn moving_average_with_f32(
    signal: &[f32],
    half_width: f32,
    policy: SmoothingPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::smoothing::moving_average_with(signal, half_width, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable scaled-square RMS over native integer half-width windows.
pub fn moving_rms_f32(signal: &[f32], half_width: usize) -> Result<Vec<f32>, BindingError> {
    crate::smoothing::moving_rms(signal, half_width)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit scaled-square or original square-average-square-root arithmetic.
pub fn moving_rms_with_f32(
    signal: &[f32],
    half_width: usize,
    policy: SmoothingPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::smoothing::moving_rms_with(signal, half_width, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Finite median filtering with native progressive endpoint windows.
pub fn median_filter_f64(signal: &[f64], width: usize) -> Result<Vec<f64>, BindingError> {
    crate::median::filter(signal, width).map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable moving average with fractional half-width and endpoint replication.
pub fn moving_average_f64(signal: &[f64], half_width: f64) -> Result<Vec<f64>, BindingError> {
    crate::smoothing::moving_average(signal, half_width)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit compensated sample-cell or original cumulative-grid averaging.
pub fn moving_average_with_f64(
    signal: &[f64],
    half_width: f64,
    policy: SmoothingPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::smoothing::moving_average_with(signal, half_width, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable scaled-square RMS over native integer half-width windows.
pub fn moving_rms_f64(signal: &[f64], half_width: usize) -> Result<Vec<f64>, BindingError> {
    crate::smoothing::moving_rms(signal, half_width)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit scaled-square or original square-average-square-root arithmetic.
pub fn moving_rms_with_f64(
    signal: &[f64],
    half_width: usize,
    policy: SmoothingPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::smoothing::moving_rms_with(signal, half_width, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
