use napi::Error as BindingError;
/// Whether the Lanczos kernel includes sample zero.
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum BoundaryPolicy {
    IncludeFirst,
    LegacySkipFirst,
}
impl From<BoundaryPolicy> for crate::resampling::BoundaryPolicy {
    fn from(value: BoundaryPolicy) -> Self {
        match value {
            BoundaryPolicy::IncludeFirst => Self::IncludeFirst,
            BoundaryPolicy::LegacySkipFirst => Self::LegacySkipFirst,
        }
    }
}
/// Corrected near-zero evaluation or original approximate kernel arithmetic.
#[derive(Clone, Copy)]
#[napi_derive::napi]
pub enum KernelPolicy {
    Stable,
    Legacy,
}
impl From<KernelPolicy> for crate::resampling::KernelPolicy {
    fn from(value: KernelPolicy) -> Self {
        match value {
            KernelPolicy::Stable => Self::Stable,
            KernelPolicy::Legacy => Self::Legacy,
        }
    }
}
/// Linear interpolation on strictly increasing coordinates with endpoint clamping.
pub fn interpolate_linear_f32(
    coordinates: &[f32],
    values: &[f32],
    positions: &[f32],
) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::linear(coordinates, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Original uniform-source coordinate convention and near-final-sample clamp.
pub fn interpolate_uniform_f32(
    start: f32,
    end: f32,
    values: &[f32],
    positions: &[f32],
) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::linear_uniform(start, end, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Four-lobe Lanczos interpolation with corrected boundary and kernel defaults.
pub fn interpolate_sinc_f32(
    start: f32,
    end: f32,
    values: &[f32],
    positions: &[f32],
) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::sinc_uniform(start, end, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit sample-zero policy with original kernel arithmetic.
pub fn interpolate_sinc_with_f32(
    start: f32,
    end: f32,
    values: &[f32],
    positions: &[f32],
    boundary: BoundaryPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::sinc_uniform_with(start, end, values, positions, boundary.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Independent sample-zero and near-zero kernel policies.
pub fn interpolate_sinc_with_kernel_f32(
    start: f32,
    end: f32,
    values: &[f32],
    positions: &[f32],
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::sinc_uniform_with_kernel(
        start,
        end,
        values,
        positions,
        boundary.into(),
        kernel.into(),
    )
    .map_err(|error| napi::Error::from_reason(error.0))
}
/// Interpolate sentinel gaps, including NaN sentinels, without modifying the input.
pub fn interpolate_fill_missing_f32(signal: &[f32], blank: f32) -> Result<Vec<f32>, BindingError> {
    crate::interpolation::fill_missing(signal, blank)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Linear interpolation on strictly increasing coordinates with endpoint clamping.
pub fn interpolate_linear_f64(
    coordinates: &[f64],
    values: &[f64],
    positions: &[f64],
) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::linear(coordinates, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Original uniform-source coordinate convention and near-final-sample clamp.
pub fn interpolate_uniform_f64(
    start: f64,
    end: f64,
    values: &[f64],
    positions: &[f64],
) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::linear_uniform(start, end, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Four-lobe Lanczos interpolation with corrected boundary and kernel defaults.
pub fn interpolate_sinc_f64(
    start: f64,
    end: f64,
    values: &[f64],
    positions: &[f64],
) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::sinc_uniform(start, end, values, positions)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Explicit sample-zero policy with original kernel arithmetic.
pub fn interpolate_sinc_with_f64(
    start: f64,
    end: f64,
    values: &[f64],
    positions: &[f64],
    boundary: BoundaryPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::sinc_uniform_with(start, end, values, positions, boundary.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Independent sample-zero and near-zero kernel policies.
pub fn interpolate_sinc_with_kernel_f64(
    start: f64,
    end: f64,
    values: &[f64],
    positions: &[f64],
    boundary: BoundaryPolicy,
    kernel: KernelPolicy,
) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::sinc_uniform_with_kernel(
        start,
        end,
        values,
        positions,
        boundary.into(),
        kernel.into(),
    )
    .map_err(|error| napi::Error::from_reason(error.0))
}
/// Interpolate sentinel gaps, including NaN sentinels, without modifying the input.
pub fn interpolate_fill_missing_f64(signal: &[f64], blank: f64) -> Result<Vec<f64>, BindingError> {
    crate::interpolation::fill_missing(signal, blank)
        .map_err(|error| napi::Error::from_reason(error.0))
}
