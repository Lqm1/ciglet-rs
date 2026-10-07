//! Portable four-lane approximations corresponding to the installed SSE2 API.
//! Arrays own their lanes and impose no native SIMD layout or alignment.
//! Checked calculations return an error if any lane is invalid; no partial
//! output escapes. They assume ordinary nearest-even floating-point rounding.
use crate::Error;

pub mod hyperbolic;
pub mod trigonometric;

/// Four binary32 lanes in source index order.
pub type Floats = [f32; 4];
/// Four signed 32-bit lanes in source index order.
pub type Integers = [i32; 4];

/// Source float-to-integer conversion: truncate, or the SSE2 indefinite value.
/// NaN, infinity and values outside the signed conversion range yield i32::MIN.
/// Floating-point exception flags are not part of this portable API.
pub fn to_integers(values: Floats) -> Integers {
    values.map(|value| {
        if !value.is_finite() || value < i32::MIN as f32 || value >= 2_147_483_648.0 {
            i32::MIN
        } else {
            value as i32
        }
    })
}

/// Convert signed integer lanes to binary32 with nearest-even rounding.
pub fn from_integers(values: Integers) -> Floats {
    values.map(|value| value as f32)
}

/// Safe replacement for the float/integer bit-punning union.
pub fn to_bits(values: Floats) -> [u32; 4] {
    values.map(f32::to_bits)
}

/// Restore lane bits without numeric conversion, preserving NaN payloads.
pub fn from_bits(values: [u32; 4]) -> Floats {
    values.map(f32::from_bits)
}

/// Source absolute-value helper, clearing only each lane's sign bit.
pub fn absolute(values: Floats) -> Floats {
    from_bits(to_bits(values).map(|bits| bits & 0x7fff_ffff))
}

/// Source two-64-bit-lane construction represented in four-lane index order.
/// Each 64-bit value contributes its low word followed by its high word.
pub fn from_integer64_lanes(values: [i64; 2]) -> Integers {
    [
        values[0] as i32,
        (values[0] >> 32) as i32,
        values[1] as i32,
        (values[1] >> 32) as i32,
    ]
}

fn map(values: Floats, mut kernel: impl FnMut(f32) -> Result<f32, Error>) -> Result<Floats, Error> {
    let mut result = [0.0; 4];
    for (output, value) in result.iter_mut().zip(values) {
        *output = kernel(value)?;
        if !output.is_finite() {
            return Err(Error("four-lane approximation produced a nonfinite result"));
        }
    }
    Ok(result)
}

/// Source `vfastpow2`, including its underflow clamp.
pub fn exponential2(values: Floats) -> Result<Floats, Error> {
    map(values, super::exponential2)
}
/// Source `vfasterpow2`.
pub fn exponential2_fast(values: Floats) -> Result<Floats, Error> {
    map(values, super::exponential2_fast)
}
/// Source `vfastexp`.
pub fn exponential(values: Floats) -> Result<Floats, Error> {
    map(values, super::exponential)
}
/// Source `vfasterexp`.
pub fn exponential_fast(values: Floats) -> Result<Floats, Error> {
    map(values, super::exponential_fast)
}
/// Source `vfastlog2` on positive finite lanes.
pub fn logarithm2(values: Floats) -> Result<Floats, Error> {
    map(values, super::logarithm2)
}
/// Source `vfasterlog2`.
pub fn logarithm2_fast(values: Floats) -> Result<Floats, Error> {
    map(values, super::logarithm2_fast)
}
/// Source `vfastlog`.
pub fn logarithm(values: Floats) -> Result<Floats, Error> {
    map(values, super::logarithm)
}
/// Source `vfasterlog` with its own natural-log coefficients.
pub fn logarithm_fast(values: Floats) -> Result<Floats, Error> {
    map(values, super::logarithm_fast)
}

fn powers(
    bases: Floats,
    exponents: Floats,
    kernel: fn(f32, f32) -> Result<f32, Error>,
) -> Result<Floats, Error> {
    let mut result = [0.0; 4];
    for ((output, base), exponent) in result.iter_mut().zip(bases).zip(exponents) {
        *output = kernel(base, exponent)?;
        if !output.is_finite() {
            return Err(Error("four-lane power produced a nonfinite result"));
        }
    }
    Ok(result)
}

/// Source `vfastpow` with an independent exponent for each lane.
pub fn power(bases: Floats, exponents: Floats) -> Result<Floats, Error> {
    powers(bases, exponents, super::power)
}
/// Source `vfasterpow`.
pub fn power_fast(bases: Floats, exponents: Floats) -> Result<Floats, Error> {
    powers(bases, exponents, super::power_fast)
}
