//! Explicit C value layouts; private Rust complex layouts are not exported.
use crate::{Error, approximation::Precision, complex::Complex};
/// Fast approximation selector.
pub const CIGLET_RS_COMPLEX_FAST: u32 = 0;
/// Source-compatible approximation selector; the original c_exp default.
pub const CIGLET_RS_COMPLEX_APPROXIMATE: u32 = 1;
/// Host libm selector.
pub const CIGLET_RS_COMPLEX_EXACT: u32 = 2;
pub(super) fn precision(code: u32) -> Result<Precision, Error> {
    match code {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(Error("unknown complex precision selector")),
    }
}

/// Binary32 complex value with natural C field layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsComplexF32 {
    pub real: f32,
    pub imaginary: f32,
}
impl From<CigletRsComplexF32> for Complex<f32> {
    fn from(value: CigletRsComplexF32) -> Self {
        Self::new(value.real, value.imaginary)
    }
}
impl From<Complex<f32>> for CigletRsComplexF32 {
    fn from(value: Complex<f32>) -> Self {
        Self {
            real: value.re,
            imaginary: value.im,
        }
    }
}

/// Binary64 complex value with natural C field layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsComplexF64 {
    pub real: f64,
    pub imaginary: f64,
}
impl From<CigletRsComplexF64> for Complex<f64> {
    fn from(value: CigletRsComplexF64) -> Self {
        Self::new(value.real, value.imaginary)
    }
}
impl From<Complex<f64>> for CigletRsComplexF64 {
    fn from(value: Complex<f64>) -> Self {
        Self {
            real: value.re,
            imaginary: value.im,
        }
    }
}
