use crate::sinc::{self, Policy};
use napi::Error as BindingError;
/// Native stable limits or the original separate denominator plateaus.
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum SincPolicy {
    Stable,
    Legacy,
}
impl From<SincPolicy> for Policy {
    fn from(value: SincPolicy) -> Self {
        match value {
            SincPolicy::Stable => Self::Stable,
            SincPolicy::Legacy => Self::Legacy,
        }
    }
}
pub fn aliased_sinc_f32(length: f32, angle: f32) -> Result<f32, BindingError> {
    sinc::aliased(length, angle).map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_with_f32(
    length: f32,
    angle: f32,
    policy: SincPolicy,
) -> Result<f32, BindingError> {
    sinc::aliased_with(length, angle, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_derivative_f32(length: f32, angle: f32) -> Result<f32, BindingError> {
    sinc::derivative(length, angle).map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_derivative_with_f32(
    length: f32,
    angle: f32,
    policy: SincPolicy,
) -> Result<f32, BindingError> {
    sinc::derivative_with(length, angle, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_f64(length: f64, angle: f64) -> Result<f64, BindingError> {
    sinc::aliased(length, angle).map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> Result<f64, BindingError> {
    sinc::aliased_with(length, angle, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_derivative_f64(length: f64, angle: f64) -> Result<f64, BindingError> {
    sinc::derivative(length, angle).map_err(|error| napi::Error::from_reason(error.0))
}
pub fn aliased_sinc_derivative_with_f64(
    length: f64,
    angle: f64,
    policy: SincPolicy,
) -> Result<f64, BindingError> {
    sinc::derivative_with(length, angle, policy.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
