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
