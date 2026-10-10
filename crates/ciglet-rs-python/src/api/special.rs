use crate::approximation::{lanes, special};
use pyo3::PyErr as BindingError;
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_complementary_error(value: f32) -> Result<f32, BindingError> {
    special::complementary_error(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_complementary_error_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::complementary_error(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_complementary_error_fast(value: f32) -> Result<f32, BindingError> {
    special::complementary_error_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_complementary_error_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::complementary_error_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_error(value: f32) -> Result<f32, BindingError> {
    special::error(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_error_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::error(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_error_fast(value: f32) -> Result<f32, BindingError> {
    special::error_fast(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_error_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::error_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_inverse_error(value: f32) -> Result<f32, BindingError> {
    special::inverse_error(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_inverse_error_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::inverse_error(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_inverse_error_fast(value: f32) -> Result<f32, BindingError> {
    special::inverse_error_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_inverse_error_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::inverse_error_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_log_gamma(value: f32) -> Result<f32, BindingError> {
    special::log_gamma(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_log_gamma_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::log_gamma(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_log_gamma_fast(value: f32) -> Result<f32, BindingError> {
    special::log_gamma_fast(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_log_gamma_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::log_gamma_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_digamma(value: f32) -> Result<f32, BindingError> {
    special::digamma(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_digamma_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::digamma(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_digamma_fast(value: f32) -> Result<f32, BindingError> {
    special::digamma_fast(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_digamma_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::digamma_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_lambert_w(value: f32) -> Result<f32, BindingError> {
    special::lambert_w(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_lambert_w_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::lambert_w(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_lambert_w_fast(value: f32) -> Result<f32, BindingError> {
    special::lambert_w_fast(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_lambert_w_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::lambert_w_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_lambert_w_exp(value: f32) -> Result<f32, BindingError> {
    special::lambert_w_exp(value).map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_lambert_w_exp_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::lambert_w_exp(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native binary32 special-function kernel, retaining its validated domain.
pub fn special_lambert_w_exp_fast(value: f32) -> Result<f32, BindingError> {
    special::lambert_w_exp_fast(value)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Complete independently owned four-lane output; no partial result escapes.
pub fn special_lambert_w_exp_fast_lanes(values: &[f32]) -> Result<Vec<f32>, BindingError> {
    let values: [f32; 4] = values.try_into().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(
            "special-function input must contain exactly four lanes",
        )
    })?;
    lanes::special::lambert_w_exp_fast(values)
        .map(|values| values.to_vec())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
