//! Bundled fastapprox hyperbolic and logistic formulas with binary32 stages.
//! Fast and Approximate retain the source kernels. Exact selects host exp before
//! binary32 conversion; it does not replace the surrounding source formulas.
use super::{Precision, exponential_with};
use crate::Error;

fn finite(value: f32) -> Result<f32, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error(
            "hyperbolic approximation produced a nonfinite result",
        ))
    }
}

/// Hyperbolic sine using the selected exponential kernel.
/// The two source modes correspond to `fastersinh` and `fastsinh`.
pub fn sine(value: f32, precision: Precision) -> Result<f32, Error> {
    finite(0.5 * (exponential_with(value, precision)? - exponential_with(-value, precision)?))
}

/// Hyperbolic cosine using the selected exponential kernel.
/// Both exponentials and their binary32 sum must remain representable.
pub fn cosine(value: f32, precision: Precision) -> Result<f32, Error> {
    finite(0.5 * (exponential_with(value, precision)? + exponential_with(-value, precision)?))
}

/// Hyperbolic tangent, retaining the source's exponential quotient.
/// Input validity follows the selected kernel at `-2 * value`.
pub fn tangent(value: f32, precision: Precision) -> Result<f32, Error> {
    finite(-1.0 + 2.0 / (1.0 + exponential_with(-2.0 * value, precision)?))
}

/// Logistic sigmoid, retaining binary32 exponential and quotient rounding.
pub fn sigmoid(value: f32, precision: Precision) -> Result<f32, Error> {
    finite(1.0 / (1.0 + exponential_with(-value, precision)?))
}
