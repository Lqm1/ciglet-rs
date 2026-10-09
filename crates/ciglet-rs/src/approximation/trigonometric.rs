//! Restricted-polynomial kernels and the bundled full-range tangent.
//! Restricted kernels retain their polynomial outside its calibration interval;
//! they perform no modulo reduction. Nonfinite inputs/results return errors.
use super::Precision;
use crate::Error;

fn finite(value: f32) -> Result<f32, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error(
            "trigonometric approximation requires finite inputs and results",
        ))
    }
}

pub(super) fn sine_kernel(angle: f32, precision: Precision) -> f32 {
    if precision == Precision::Exact {
        return f64::from(angle).sin() as f32;
    }
    let sign = angle.to_bits() & 0x8000_0000;
    let magnitude = f32::from_bits(angle.to_bits() & 0x7fff_ffff);
    let base = 1.273_239_5 * angle - 0.405_284_73 * angle * magnitude;
    if precision == Precision::Fast {
        let signed = f32::from_bits(0.223_085_1_f32.to_bits() | sign);
        base * (0.776_330_23 + signed * base)
    } else {
        let square = base * base;
        let linear = f32::from_bits(0.203_639_37_f32.to_bits() | sign);
        let cubic = f32::from_bits(0.015_124_941_f32.to_bits() | sign);
        let quintic = f32::from_bits((-0.003_222_59_f32).to_bits() ^ sign);
        0.784_444_87 * base + square * (linear + square * (cubic + square * quintic))
    }
}

/// Source `fastsin`/`fastersin` polynomial, calibrated on [-pi, pi].
pub fn sine(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    finite(sine_kernel(angle, precision))
}

/// Source `fastcos` shifted polynomial or `fastercos` even polynomial.
pub fn cosine(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    let result = match precision {
        Precision::Approximate => {
            let offset = if angle > std::f32::consts::FRAC_PI_2 {
                -4.712_389
            } else {
                std::f32::consts::FRAC_PI_2
            };
            sine_kernel(angle + offset, precision)
        }
        Precision::Fast => {
            let magnitude = f32::from_bits(angle.to_bits() & 0x7fff_ffff);
            let base = 1.0 - std::f32::consts::FRAC_2_PI * magnitude;
            base + 0.546_413_36 * base * (1.0 - base * base)
        }
        Precision::Exact => f64::from(angle).cos() as f32,
    };
    finite(result)
}

/// Source `fasttan` or `fastertan`, with no periodic reduction.
pub fn tangent(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    if precision == Precision::Exact {
        return finite(f64::from(angle).tan() as f32);
    }
    let denominator = if precision == Precision::Fast {
        cosine(angle, precision)?
    } else {
        sine(angle + std::f32::consts::FRAC_PI_2, precision)?
    };
    finite(sine(angle, precision)? / denominator)
}

/// Source `fasttanfull`/`fastertanfull`, including its own reduction direction.
/// Turn counts must fit the original signed 32-bit conversion domain.
pub fn tangent_full(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    if precision == Precision::Exact {
        return tangent(angle, precision);
    }
    let turns = angle * 0.159_154_94;
    if !turns.is_finite() || turns < i32::MIN as f32 || turns >= 2_147_483_648.0 {
        return Err(Error("approximate tangent angle exceeds the legacy range"));
    }
    let half = if angle < 0.0 { -0.5 } else { 0.5 };
    let reduced = angle - (half + turns as i32 as f32) * std::f32::consts::TAU;
    finite(sine(reduced, precision)? / cosine(reduced, precision)?)
}
