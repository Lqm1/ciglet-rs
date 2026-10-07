//! Bundled special-function approximations with checked finite results.
//! Functions ending in `_fast` select the source's faster approximation.
use super::{exponential2, exponential2_fast, logarithm2, logarithm2_fast};
use crate::Error;

fn finite(value: f32) -> Result<f32, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error(
            "special-function approximation requires finite inputs and results",
        ))
    }
}

/// Source `fasterfc`, the more accurate complementary-error approximation.
/// Its name means fast erfc, not the faster variant `fastererfc`.
pub fn complementary_error(value: f32) -> Result<f32, Error> {
    finite(value)?;
    let negative_magnitude = f32::from_bits((5.609_846 * value).to_bits() | 0x8000_0000);
    let square = value * value;
    let fourth = square * square;
    finite(
        2.0 / (1.0 + exponential2(3.350_963_4 * value)?)
            - 0.072_190_546
                * value
                * (15.418_192 * fourth - 1.0)
                * exponential2_fast(negative_magnitude)?,
    )
}

/// Source `fastererfc`, retaining the faster exponential bit kernel.
pub fn complementary_error_fast(value: f32) -> Result<f32, Error> {
    finite(value)?;
    finite(2.0 / (1.0 + exponential2_fast(3.350_963_4 * value)?))
}

/// Source `fasterf`, formed by subtracting complementary error from one.
pub fn error(value: f32) -> Result<f32, Error> {
    finite(1.0 - complementary_error(value)?)
}

/// Source `fastererf`, formed using the faster complementary approximation.
pub fn error_fast(value: f32) -> Result<f32, Error> {
    finite(1.0 - complementary_error_fast(value)?)
}

fn inverse_ratio(value: f32) -> Result<f32, Error> {
    finite(value)?;
    if value <= -1.0 || value >= 1.0 {
        return Err(Error(
            "inverse error requires an argument strictly between -1 and 1",
        ));
    }
    Ok((1.0 + value) / (1.0 - value))
}

/// Source `fastinverseerf`, including its rational correction.
pub fn inverse_error(value: f32) -> Result<f32, Error> {
    let log = logarithm2(inverse_ratio(value)?)?;
    let square = value * value;
    finite(
        0.300_045_8 * log
            + value * (0.020_287_853 - 0.072_368_93 * square) / (0.991_303 - 0.805_977_6 * square),
    )
}

/// Source `fasterinverseerf`, with the linear logarithm bit approximation.
pub fn inverse_error_fast(value: f32) -> Result<f32, Error> {
    finite(0.300_045_8 * logarithm2_fast(inverse_ratio(value)?)?)
}

/// Source `fastlgamma`, with checked logarithm arguments and finite results.
/// The source formula also supports some negative arguments; no reflection
/// formula is substituted for those source-defined values.
pub fn log_gamma(value: f32) -> Result<f32, Error> {
    finite(value)?;
    let log = super::logarithm(value * (1.0 + value) * (2.0 + value))?;
    let shifted = 3.0 + value;
    finite(
        -2.081_061_4 - value + 0.083_333_3 / shifted - log
            + (2.5 + value) * super::logarithm(shifted)?,
    )
}

/// Source `fasterlgamma`, using the faster natural logarithm coefficients.
pub fn log_gamma_fast(value: f32) -> Result<f32, Error> {
    finite(value)?;
    finite(
        -0.081_061_47 - value - super::logarithm_fast(value)?
            + (0.5 + value) * super::logarithm_fast(1.0 + value)?,
    )
}

/// Source `fastdigamma`, including its rational correction.
pub fn digamma(value: f32) -> Result<f32, Error> {
    finite(value)?;
    let shifted = 2.0 + value;
    let log = super::logarithm(shifted)?;
    finite(
        (-48.0 + value * (-157.0 + value * (-127.0 - 30.0 * value)))
            / (12.0 * value * (1.0 + value) * shifted * shifted)
            + log,
    )
}

/// Source `fasterdigamma`, with checked logarithm arguments and poles.
pub fn digamma_fast(value: f32) -> Result<f32, Error> {
    finite(value)?;
    let shifted = 1.0 + value;
    finite(-1.0 / value - 1.0 / (2.0 * shifted) + super::logarithm_fast(shifted)?)
}

fn log(value: f32, fast: bool) -> Result<f32, Error> {
    if fast {
        super::logarithm_fast(value)
    } else {
        super::logarithm(value)
    }
}

fn exp(value: f32, fast: bool) -> Result<f32, Error> {
    if fast {
        super::exponential_fast(value)
    } else {
        super::exponential(value)
    }
}

fn lambert(value: f32, fast: bool) -> Result<f32, Error> {
    finite(value)?;
    if f64::from(value) < -1.0 / std::f64::consts::E {
        return Err(Error(
            "principal real Lambert W requires an argument at least -1/e",
        ));
    }
    let (scale, offset, correction) = if value < 2.264_45 {
        (1.546_865_6, 2.250_367, -0.737_769_96)
    } else {
        (1.0, 0.0, 0.0)
    };
    let log_value = log(scale * value + offset, fast)?;
    let log_log = log(log_value, fast)?;
    if fast {
        let estimate = correction + log_value - log_log + log_log / log_value;
        let exponential = exp(-estimate, true)?;
        finite((estimate * estimate + exponential * value) / (1.0 + estimate))
    } else {
        let negative = -correction - log_value + log_log - log_log / log_value;
        let scaled = value * exp(negative, false)?;
        let difference = scaled - negative;
        finite(
            (2.0 * scaled - negative * (4.0 * scaled - negative * difference))
                / (2.0 + difference * (2.0 - negative)),
        )
    }
}

/// Source `fastlambertw`, approximating the principal real branch.
pub fn lambert_w(value: f32) -> Result<f32, Error> {
    lambert(value, false)
}

/// Source `fasterlambertw`, with its distinct refinement formula.
pub fn lambert_w_fast(value: f32) -> Result<f32, Error> {
    lambert(value, true)
}

fn lambert_exp(value: f32, fast: bool) -> Result<f32, Error> {
    finite(value)?;
    let log_argument = value.max(1.176_563_1);
    let power_argument = if value < 1.176_563_1 {
        0.945_376_2 * (value - 1.176_563_1)
    } else {
        0.0
    };
    let log_value = log(log_argument, fast)?;
    let estimate =
        exponential2_fast(power_argument)? * (log_argument - log_value + log_value / log_argument);
    let log_estimate = log(estimate, fast)?;
    if fast {
        finite(estimate * (1.0 + value - log_estimate) / (1.0 + estimate))
    } else {
        let difference = value - log_estimate;
        finite(
            estimate * (2.0 + difference + estimate * (3.0 + 2.0 * difference))
                / (2.0 - difference + estimate * (5.0 + 2.0 * estimate)),
        )
    }
}

/// Source `fastlambertwexpx`, evaluating W(exp(value)) without forming exp(value).
pub fn lambert_w_exp(value: f32) -> Result<f32, Error> {
    lambert_exp(value, false)
}

/// Source `fasterlambertwexpx`, retaining the source's faster refinement.
pub fn lambert_w_exp_fast(value: f32) -> Result<f32, Error> {
    lambert_exp(value, true)
}
