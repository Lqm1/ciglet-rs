use napi::Error as BindingError;
/// Scaled stable divergence or original approximate-log arithmetic.
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum DistancePolicy {
    Stable,
    Legacy,
}
impl From<DistancePolicy> for crate::distance::Policy {
    fn from(value: DistancePolicy) -> Self {
        match value {
            DistancePolicy::Stable => Self::Stable,
            DistancePolicy::Legacy => Self::Legacy,
        }
    }
}
/// Native frequency conversion with original staging and domain checks.
pub fn frequency_to_mel_f32(value: f32) -> Result<f32, BindingError> {
    crate::frequency::frequency_to_mel(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn frequency_to_mel_f64(value: f64) -> Result<f64, BindingError> {
    crate::frequency::frequency_to_mel(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn mel_to_frequency_f32(value: f32) -> Result<f32, BindingError> {
    crate::frequency::mel_to_frequency(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn mel_to_frequency_f64(value: f64) -> Result<f64, BindingError> {
    crate::frequency::mel_to_frequency(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn frequency_to_bark_f32(value: f32) -> Result<f32, BindingError> {
    crate::frequency::frequency_to_bark(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn frequency_to_bark_f64(value: f64) -> Result<f64, BindingError> {
    crate::frequency::frequency_to_bark(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn bark_to_frequency_f32(value: f32) -> Result<f32, BindingError> {
    crate::frequency::bark_to_frequency(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn bark_to_frequency_f64(value: f64) -> Result<f64, BindingError> {
    crate::frequency::bark_to_frequency(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn equal_loudness_f32(value: f32) -> Result<f32, BindingError> {
    crate::frequency::equal_loudness(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native frequency conversion with original staging and domain checks.
pub fn equal_loudness_f64(value: f64) -> Result<f64, BindingError> {
    crate::frequency::equal_loudness(value).map_err(|error| napi::Error::from_reason(error.0))
}
/// Native approximate Mel grid with intervals plus one independently owned points.
pub fn mel_space_f32(
    minimum: f32,
    maximum: f32,
    intervals: usize,
) -> Result<Vec<f32>, BindingError> {
    crate::frequency::mel_space(minimum, maximum, intervals)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable mean divergence, retaining native zero/infinity semantics.
pub fn itakura_saito_f32(source: &[f32], reference: &[f32]) -> Result<f32, BindingError> {
    crate::distance::itakura_saito(source, reference)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable log mean divergence with finite extreme-ratio output.
pub fn log_itakura_saito_f32(source: &[f32], reference: &[f32]) -> Result<f32, BindingError> {
    crate::distance::log_itakura_saito(source, reference)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete stable or original selected-precision approximate-log distance.
pub fn log_itakura_saito_with_f32(
    source: &[f32],
    reference: &[f32],
    policy: DistancePolicy,
) -> Result<f32, BindingError> {
    crate::distance::log_itakura_saito_with(source, reference, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native approximate Mel grid with intervals plus one independently owned points.
pub fn mel_space_f64(
    minimum: f64,
    maximum: f64,
    intervals: usize,
) -> Result<Vec<f64>, BindingError> {
    crate::frequency::mel_space(minimum, maximum, intervals)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable mean divergence, retaining native zero/infinity semantics.
pub fn itakura_saito_f64(source: &[f64], reference: &[f64]) -> Result<f64, BindingError> {
    crate::distance::itakura_saito(source, reference)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Stable log mean divergence with finite extreme-ratio output.
pub fn log_itakura_saito_f64(source: &[f64], reference: &[f64]) -> Result<f64, BindingError> {
    crate::distance::log_itakura_saito(source, reference)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete stable or original selected-precision approximate-log distance.
pub fn log_itakura_saito_with_f64(
    source: &[f64],
    reference: &[f64],
    policy: DistancePolicy,
) -> Result<f64, BindingError> {
    crate::distance::log_itakura_saito_with(source, reference, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
