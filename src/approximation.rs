//! Compatible single-precision trigonometric approximations.
// Adapted from Paul Mineiro's fastapprox. See LICENSES/fastapprox-BSD.txt.
use crate::Error;

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
    let mantissa = f32::from_bits((value.to_bits() & 0x007f_ffff) | 0x3f00_0000);
    let exponent = value.to_bits() as f32 * 1.192_092_9e-7;
    Ok(exponent - 124.225_52 - 1.498_030_3 * mantissa - 1.725_88 / (0.352_088_72 + mantissa))
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
