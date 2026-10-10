use super::array::{Array2F32, Array2F64};
use crate::correlogram::{self, Frame, Method, Policy};
use napi::Error as BindingError;
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum CorrelogramMethod {
    Autocorrelation = 0,
    AbsoluteDifference = 1,
    SquaredDifference = 2,
    Yin = 3,
}
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum CorrelogramPolicy {
    Stable = 0,
    Legacy = 1,
}
fn method(value: u32) -> Result<Method, BindingError> {
    i32::try_from(value)
        .map_err(|_| napi::Error::from_reason("unknown correlogram method code"))?
        .try_into()
        .map_err(|error: crate::Error| napi::Error::from_reason(error.0))
}
fn policy(value: u32) -> Result<Policy, BindingError> {
    match value {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(napi::Error::from_reason("unknown correlogram policy code")),
    }
}
fn frames(centers: &[i32], lengths: &[u32]) -> Result<Vec<Frame>, BindingError> {
    if centers.len() != lengths.len() {
        return Err(napi::Error::from_reason(
            "correlogram frame arrays must have matching lengths",
        ));
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(centers.len())
        .map_err(|_| napi::Error::from_reason("correlogram frame allocation failed"))?;
    output.extend(centers.iter().zip(lengths).map(|(&center, &length)| Frame {
        center,
        length: length as usize,
    }));
    Ok(output)
}
/// Complete native frame configuration. Validation occurs during computation.
#[derive(Clone, Copy)]
pub struct CorrelogramFrame {
    pub center: i32,
    pub length: usize,
}
impl CorrelogramFrame {
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
    ) -> Result<Array2F32, BindingError> {
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
    ) -> Result<Array2F64, BindingError> {
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
pub fn correlogram_compute_f32(
    samples: &[f32],
    centers: &[i32],
    lengths: &[u32],
    periods: usize,
    method_code: u32,
) -> Result<Array2F32, BindingError> {
    correlogram::compute(
        samples,
        &frames(centers, lengths)?,
        periods,
        method(method_code)?,
    )
    .map(|value| Array2F32 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn correlogram_compute_with_f32(
    samples: &[f32],
    centers: &[i32],
    lengths: &[u32],
    periods: usize,
    method_code: u32,
    policy_code: u32,
) -> Result<Array2F32, BindingError> {
    correlogram::compute_with(
        samples,
        &frames(centers, lengths)?,
        periods,
        method(method_code)?,
        policy(policy_code)?,
    )
    .map(|value| Array2F32 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn correlogram_to_frequency_f32(
    periods: &Array2F32,
    sampling_rate: i32,
    frequencies: &[f32],
) -> Result<Array2F32, BindingError> {
    correlogram::to_frequency(&periods.value, sampling_rate, frequencies)
        .map(|value| Array2F32 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn correlogram_compute_f64(
    samples: &[f64],
    centers: &[i32],
    lengths: &[u32],
    periods: usize,
    method_code: u32,
) -> Result<Array2F64, BindingError> {
    correlogram::compute(
        samples,
        &frames(centers, lengths)?,
        periods,
        method(method_code)?,
    )
    .map(|value| Array2F64 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn correlogram_compute_with_f64(
    samples: &[f64],
    centers: &[i32],
    lengths: &[u32],
    periods: usize,
    method_code: u32,
    policy_code: u32,
) -> Result<Array2F64, BindingError> {
    correlogram::compute_with(
        samples,
        &frames(centers, lengths)?,
        periods,
        method(method_code)?,
        policy(policy_code)?,
    )
    .map(|value| Array2F64 { value })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn correlogram_to_frequency_f64(
    periods: &Array2F64,
    sampling_rate: i32,
    frequencies: &[f64],
) -> Result<Array2F64, BindingError> {
    correlogram::to_frequency(&periods.value, sampling_rate, frequencies)
        .map(|value| Array2F64 { value })
        .map_err(|error| napi::Error::from_reason(error.0))
}
