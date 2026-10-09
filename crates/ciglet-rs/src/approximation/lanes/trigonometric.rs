//! Four-lane trigonometry retaining the bundled SSE2 expression order.
use super::{Floats, map};
use crate::Error;
use crate::approximation::Precision;

fn finite(value: f32) -> Result<f32, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error(
            "four-lane trigonometry requires finite inputs and results",
        ))
    }
}

fn sine_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    let result = if precision == Precision::Approximate {
        let magnitude = f32::from_bits(angle.to_bits() & 0x7fff_ffff);
        let base = 1.273_239_5 * angle - 0.405_284_73 * angle * magnitude;
        let square = base * base;
        let correction =
            square * (0.203_639_37 + square * (0.015_124_941 + square * -0.003_222_59));
        let signed = f32::from_bits(correction.to_bits() ^ (angle.to_bits() & 0x8000_0000));
        0.784_444_87 * base + signed
    } else {
        crate::approximation::trigonometric::sine_kernel(angle, precision)
    };
    finite(result)
}

fn cosine_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    if precision == Precision::Approximate {
        // SSE2 cmpnlt includes equality; scalar fastcos uses strictly greater.
        let offset = if angle >= std::f32::consts::FRAC_PI_2 {
            -4.712_389
        } else {
            std::f32::consts::FRAC_PI_2
        };
        sine_lane(angle + offset, precision)
    } else {
        crate::approximation::trigonometric::cosine(angle, precision)
    }
}

fn tangent_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    finite(angle)?;
    if precision == Precision::Exact {
        return finite(f64::from(angle).tan() as f32);
    }
    let denominator = if precision == Precision::Fast {
        cosine_lane(angle, precision)?
    } else {
        sine_lane(angle + std::f32::consts::FRAC_PI_2, precision)?
    };
    finite(sine_lane(angle, precision)? / denominator)
}

fn turn_center(angle: f32) -> Result<f32, Error> {
    finite(angle)?;
    let turns = angle * 0.159_154_94;
    if !turns.is_finite() || turns < i32::MIN as f32 || turns >= 2_147_483_648.0 {
        return Err(Error(
            "four-lane trigonometric angle exceeds the legacy range",
        ));
    }
    let half = if angle < 0.0 { -0.5 } else { 0.5 };
    Ok((half + turns as i32 as f32) * std::f32::consts::TAU)
}

fn sine_full_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    if precision == Precision::Exact {
        return sine_lane(angle, precision);
    }
    sine_lane(turn_center(angle)? - angle, precision)
}

fn cosine_full_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    if precision == Precision::Exact {
        return cosine_lane(angle, precision);
    }
    sine_full_lane(angle + std::f32::consts::FRAC_PI_2, precision)
}

fn tangent_full_lane(angle: f32, precision: Precision) -> Result<f32, Error> {
    if precision == Precision::Exact {
        return tangent_lane(angle, precision);
    }
    let reduced = angle - turn_center(angle)?;
    finite(sine_lane(reduced, precision)? / cosine_lane(reduced, precision)?)
}

/// Source vfastsin/vfastersin restricted polynomial, with no periodic reduction.
pub fn sine(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| sine_lane(angle, precision))
}
/// Source vfastcos/vfastercos, including the vector-specific equality branch.
pub fn cosine(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| cosine_lane(angle, precision))
}
/// Source vfasttan/vfastertan, with the original denominator selection.
pub fn tangent(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| tangent_lane(angle, precision))
}
/// Source vfastsinfull/vfastersinfull, with checked signed turn conversion.
pub fn sine_full(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| sine_full_lane(angle, precision))
}
/// Source vfastcosfull/vfastercosfull, including binary32 half-pi addition.
pub fn cosine_full(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| cosine_full_lane(angle, precision))
}
/// Source vfasttanfull/vfastertanfull, retaining its distinct reduction direction.
pub fn tangent_full(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |angle| tangent_full_lane(angle, precision))
}
