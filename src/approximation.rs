//! Source-compatible scalar approximations and explicit numerical precision.
// Adapted from Paul Mineiro's fastapprox. See LICENSES/fastapprox-BSD.txt.
use crate::Error;

/// Upstream numerical levels, from the fastest approximation to host libm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    Fast,
    Approximate,
    Exact,
}

/// Selected sine precision. Exact uses double libm before scalar conversion.
pub fn sine_with<T: crate::statistics::Scalar>(value: T, precision: Precision) -> Result<T, Error> {
    crate::vector::validate(&[value])?;
    crate::vector::scalar(match precision {
        Precision::Fast => sine_fast(value.into() as f32)? as f64,
        Precision::Approximate => sine(value.into() as f32)? as f64,
        Precision::Exact => value.into().sin(),
    })
}
/// Selected cosine precision, retaining binary32 approximation stages.
pub fn cosine_with<T: crate::statistics::Scalar>(
    value: T,
    precision: Precision,
) -> Result<T, Error> {
    crate::vector::validate(&[value])?;
    crate::vector::scalar(match precision {
        Precision::Fast => cosine_fast(value.into() as f32)? as f64,
        Precision::Approximate => cosine(value.into() as f32)? as f64,
        Precision::Exact => value.into().cos(),
    })
}
/// Levels two and three both use double atan2 in the source.
pub fn atan2_with<T: crate::statistics::Scalar>(
    y: T,
    x: T,
    precision: Precision,
) -> Result<T, Error> {
    if precision == Precision::Fast {
        return atan2_fast(y, x);
    }
    crate::vector::validate(&[y, x])?;
    crate::vector::scalar(y.into().atan2(x.into()))
}

/// Level-one base-two logarithm, preserving its linear bit approximation.
pub fn logarithm2_fast(value: f32) -> Result<f32, Error> {
    if !value.is_finite() || value <= 0.0 {
        return Err(Error(
            "approximate logarithm requires a positive finite value",
        ));
    }
    Ok(value.to_bits() as f32 * 1.192_092_9e-7 - 126.942_696)
}

/// The natural logarithm has its own source coefficients, not ln(2) times log2.
pub fn logarithm_fast(value: f32) -> Result<f32, Error> {
    if !value.is_finite() || value <= 0.0 {
        return Err(Error(
            "approximate logarithm requires a positive finite value",
        ));
    }
    Ok(value.to_bits() as f32 * 8.262_958e-8 - 87.989_97)
}

pub fn exponential2_fast(exponent: f32) -> Result<f32, Error> {
    if !exponent.is_finite() || exponent >= 128.0 {
        return Err(Error("approximate exponential exceeds the legacy range"));
    }
    let bits = 8_388_608.0 * (exponent.max(-126.0) + 126.942_696);
    if !(0.0..4_294_967_296.0).contains(&bits) {
        return Err(Error(
            "approximate exponential bit conversion exceeds the legacy range",
        ));
    }
    let result = f32::from_bits(bits as u32);
    if !result.is_finite() {
        return Err(Error("approximate exponential overflowed"));
    }
    Ok(result)
}

pub fn exponential_fast(exponent: f32) -> Result<f32, Error> {
    exponential2_fast(std::f32::consts::LOG2_E * exponent)
}

/// Source rational atan2 approximation with selected-precision arithmetic.
pub fn atan2_fast<T: crate::statistics::Scalar>(y: T, x: T) -> Result<T, Error> {
    crate::vector::validate(&[y, x])?;
    let half_pi = std::f64::consts::FRAC_PI_2;
    if x == T::zero() {
        return crate::vector::scalar(if y > T::zero() {
            half_pi
        } else if y == T::zero() {
            0.0
        } else {
            -half_pi
        });
    }
    let ratio = y / x;
    if !ratio.into().is_finite() {
        return crate::vector::scalar(if y > T::zero() { half_pi } else { -half_pi });
    }
    let coefficient = T::from_f32(0.28).unwrap();
    let angle = if ratio.into().abs() < 1.0 {
        let angle = ratio / (T::one() + coefficient * ratio * ratio);
        if x < T::zero() {
            crate::vector::scalar(
                angle.into()
                    + if y < T::zero() {
                        -std::f64::consts::PI
                    } else {
                        std::f64::consts::PI
                    },
            )?
        } else {
            angle
        }
    } else {
        let square = ratio * ratio;
        let fraction = if square.into().is_finite() {
            ratio / (square + coefficient)
        } else {
            // Avoid a spurious zero from an overflowing square while retaining
            // source arithmetic throughout its ordinary range.
            let inverse = T::one() / ratio;
            inverse / (T::one() + coefficient * inverse * inverse)
        };
        let angle: T = crate::vector::scalar(half_pi - fraction.into())?;
        if y < T::zero() {
            crate::vector::scalar(angle.into() - std::f64::consts::PI)?
        } else {
            angle
        }
    };
    Ok(angle)
}

pub fn exponential_with<T: crate::statistics::Scalar>(
    value: T,
    precision: Precision,
) -> Result<T, Error> {
    crate::vector::validate(&[value])?;
    crate::vector::scalar(match precision {
        Precision::Fast => exponential_fast(value.into() as f32)? as f64,
        Precision::Approximate => exponential(value.into() as f32)? as f64,
        Precision::Exact => value.into().exp(),
    })
}

pub fn logarithm_with<T: crate::statistics::Scalar>(
    value: T,
    precision: Precision,
) -> Result<T, Error> {
    crate::vector::validate(&[value])?;
    if value <= T::zero() {
        return Err(Error("logarithm requires a positive value"));
    }
    crate::vector::scalar(match precision {
        Precision::Fast => logarithm_fast(value.into() as f32)? as f64,
        Precision::Approximate => logarithm(value.into() as f32)? as f64,
        Precision::Exact => value.into().ln(),
    })
}

/// Upstream level-one sine, retaining fastapprox's binary32 arithmetic.
pub fn sine_fast(angle: f32) -> Result<f32, Error> {
    let turns = angle * 0.159_154_94;
    if !turns.is_finite() || turns < i32::MIN as f32 || turns >= 2_147_483_648.0 {
        return Err(Error("approximate sine angle exceeds the legacy range"));
    }
    let half = if angle < 0.0 { -0.5 } else { 0.5 };
    let reduced = (half + turns as i32 as f32) * std::f32::consts::TAU - angle;
    let magnitude = f32::from_bits(reduced.to_bits() & 0x7fff_ffff);
    let signed = f32::from_bits(0.223_085_1f32.to_bits() | (reduced.to_bits() & 0x8000_0000));
    let base = 1.273_239_5 * reduced - 0.405_284_73 * reduced * magnitude;
    Ok(base * (0.776_330_23 + signed * base))
}

/// Upstream level-one cosine, including the binary32 half-pi addition.
pub fn cosine_fast(angle: f32) -> Result<f32, Error> {
    sine_fast(angle + std::f32::consts::FRAC_PI_2)
}

/// The upstream `sin_2` approximation, including its scalar range reduction.
/// Accuracy degrades for large arguments, as in the original implementation.
pub fn sine(angle: f32) -> Result<f32, Error> {
    let turns = angle * 0.159_154_94;
    if !turns.is_finite() || turns < i32::MIN as f32 || turns >= 2_147_483_648.0 {
        return Err(Error("approximate sine angle exceeds the legacy range"));
    }
    let whole_turns = turns as i32;
    let half_turn = if angle < 0.0 { -0.5 } else { 0.5 };
    let reduced = (half_turn + whole_turns as f32) * std::f32::consts::TAU - angle;
    let sign = reduced.to_bits() & 0x8000_0000;
    let magnitude = f32::from_bits(reduced.to_bits() & 0x7fff_ffff);
    let base = 1.273_239_5 * reduced - 0.405_284_73 * reduced * magnitude;
    let square = base * base;
    let linear = f32::from_bits(0.203_639_37f32.to_bits() | sign);
    let cubic = f32::from_bits(0.015_124_941f32.to_bits() | sign);
    let quintic = f32::from_bits((-0.003_222_59f32).to_bits() ^ sign);
    Ok(0.784_444_87 * base + square * (linear + square * (cubic + square * quintic)))
}

/// The upstream `cos_2` approximation. Its internal arithmetic remains f32
/// even when callers perform the surrounding computation in f64.
pub fn cosine(angle: f32) -> Result<f32, Error> {
    sine(angle + std::f32::consts::FRAC_PI_2)
}

/// Upstream level-two base-two logarithm. Internal arithmetic remains f32.
pub fn logarithm2(value: f32) -> Result<f32, Error> {
    if !value.is_finite() || value <= 0.0 {
        return Err(Error(
            "approximate logarithm requires a positive finite value",
        ));
    }
    Ok(logarithm2_bits(value))
}

fn logarithm2_bits(value: f32) -> f32 {
    let mantissa = f32::from_bits((value.to_bits() & 0x007f_ffff) | 0x3f00_0000);
    let exponent = value.to_bits() as f32 * 1.192_092_9e-7;
    exponent - 124.225_52 - 1.498_030_3 * mantissa - 1.725_88 / (0.352_088_72 + mantissa)
}

// Explicit source compatibility for distance intermediates and random draws
// outside the mathematical log domain. Public log APIs validate.
pub(crate) fn logarithm_legacy_bits(value: f32) -> f32 {
    std::f32::consts::LN_2 * logarithm2_bits(value)
}

pub fn logarithm(value: f32) -> Result<f32, Error> {
    Ok(std::f32::consts::LN_2 * logarithm2(value)?)
}

/// Upstream level-two base-two exponential with its -126 underflow clamp.
/// Out-of-range exponents return an error instead of casting invalid integers.
pub fn exponential2(exponent: f32) -> Result<f32, Error> {
    if !exponent.is_finite() || exponent >= 128.0 {
        return Err(Error("approximate exponential exceeds the legacy range"));
    }
    let offset = if exponent < 0.0 { 1.0 } else { 0.0 };
    let clipped = exponent.max(-126.0);
    let fraction = clipped - (clipped as i32) as f32 + offset;
    let bits = 8_388_608.0
        * (clipped + 121.274_055 + 27.728_024 / (4.842_525_5 - fraction) - 1.490_129_1 * fraction);
    if !(0.0..4_294_967_296.0).contains(&bits) {
        return Err(Error(
            "approximate exponential bit conversion exceeds the legacy range",
        ));
    }
    Ok(f32::from_bits(bits as u32))
}

pub fn exponential(exponent: f32) -> Result<f32, Error> {
    exponential2(std::f32::consts::LOG2_E * exponent)
}

pub fn power(value: f32, exponent: f32) -> Result<f32, Error> {
    exponential2(exponent * logarithm2(value)?)
}
