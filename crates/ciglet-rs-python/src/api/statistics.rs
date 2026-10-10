use pyo3::PyErr as BindingError;
/// Native compensated/scaled moments or original selected-precision arithmetic.
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum StatisticsPolicy {
    Stable,
    Legacy,
}
impl From<StatisticsPolicy> for crate::statistics::Policy {
    fn from(value: StatisticsPolicy) -> Self {
        match value {
            StatisticsPolicy::Stable => Self::Stable,
            StatisticsPolicy::Legacy => Self::Legacy,
        }
    }
}
/// Native source-precision reduction, including IEEE nonfinite behavior.
pub fn sum_f32(values: &[f32]) -> f32 {
    crate::statistics::sum(values)
}
/// Native source-precision reduction, including IEEE nonfinite behavior.
pub fn sum_f64(values: &[f64]) -> f64 {
    crate::statistics::sum(values)
}
/// Native source-precision reduction, including IEEE nonfinite behavior.
pub fn sum_squares_f32(values: &[f32]) -> f32 {
    crate::statistics::sum_squares(values)
}
/// Native source-precision reduction, including IEEE nonfinite behavior.
pub fn sum_squares_f64(values: &[f64]) -> f64 {
    crate::statistics::sum_squares(values)
}
/// Native stable moment with checked finite inputs and output range.
pub fn mean_f32(values: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::mean(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native moment with explicit stable or source-compatible arithmetic.
pub fn mean_with_f32(values: &[f32], policy: StatisticsPolicy) -> Result<f32, BindingError> {
    crate::statistics::mean_with(values, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable moment with checked finite inputs and output range.
pub fn mean_f64(values: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::mean(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native moment with explicit stable or source-compatible arithmetic.
pub fn mean_with_f64(values: &[f64], policy: StatisticsPolicy) -> Result<f64, BindingError> {
    crate::statistics::mean_with(values, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable moment with checked finite inputs and output range.
pub fn variance_f32(values: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::variance(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native moment with explicit stable or source-compatible arithmetic.
pub fn variance_with_f32(values: &[f32], policy: StatisticsPolicy) -> Result<f32, BindingError> {
    crate::statistics::variance_with(values, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable moment with checked finite inputs and output range.
pub fn variance_f64(values: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::variance(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native moment with explicit stable or source-compatible arithmetic.
pub fn variance_with_f64(values: &[f64], policy: StatisticsPolicy) -> Result<f64, BindingError> {
    crate::statistics::variance_with(values, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable paired moment with complete shape and range validation.
pub fn covariance_f32(left: &[f32], right: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::covariance(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native paired moment with explicit stable or source-compatible arithmetic.
pub fn covariance_with_f32(
    left: &[f32],
    right: &[f32],
    policy: StatisticsPolicy,
) -> Result<f32, BindingError> {
    crate::statistics::covariance_with(left, right, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable paired moment with complete shape and range validation.
pub fn covariance_f64(left: &[f64], right: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::covariance(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native paired moment with explicit stable or source-compatible arithmetic.
pub fn covariance_with_f64(
    left: &[f64],
    right: &[f64],
    policy: StatisticsPolicy,
) -> Result<f64, BindingError> {
    crate::statistics::covariance_with(left, right, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable paired moment with complete shape and range validation.
pub fn correlation_f32(left: &[f32], right: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::correlation(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native paired moment with explicit stable or source-compatible arithmetic.
pub fn correlation_with_f32(
    left: &[f32],
    right: &[f32],
    policy: StatisticsPolicy,
) -> Result<f32, BindingError> {
    crate::statistics::correlation_with(left, right, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native stable paired moment with complete shape and range validation.
pub fn correlation_f64(left: &[f64], right: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::correlation(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native paired moment with explicit stable or source-compatible arithmetic.
pub fn correlation_with_f64(
    left: &[f64],
    right: &[f64],
    policy: StatisticsPolicy,
) -> Result<f64, BindingError> {
    crate::statistics::correlation_with(left, right, policy.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native sequential dot product, retaining selected-precision IEEE arithmetic.
pub fn dot_f32(left: &[f32], right: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::dot(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Nonnegative-lag unnormalized cross correlation with zero-filled excess lags.
pub fn cross_correlation_f32(
    left: &[f32],
    right: &[f32],
    lags: usize,
) -> Result<Vec<f32>, BindingError> {
    crate::statistics::cross_correlation(left, right, lags)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native sequential dot product, retaining selected-precision IEEE arithmetic.
pub fn dot_f64(left: &[f64], right: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::dot(left, right)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Nonnegative-lag unnormalized cross correlation with zero-filled excess lags.
pub fn cross_correlation_f64(
    left: &[f64],
    right: &[f64],
    lags: usize,
) -> Result<Vec<f64>, BindingError> {
    crate::statistics::cross_correlation(left, right, lags)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
