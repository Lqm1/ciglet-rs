//! Four-lane special functions sharing audited source-order scalar kernels.
//! Domain checks and Lambert exponential corrections apply independently to
//! every lane; an invalid input aborts the complete owned result.
use super::{Floats, map};
use crate::{Error, approximation::special};

/// Source `vfasterfc`, the more accurate complementary-error approximation.
pub fn complementary_error(values: Floats) -> Result<Floats, Error> {
    map(values, special::complementary_error)
}
/// Source `vfastererfc`.
pub fn complementary_error_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::complementary_error_fast)
}
/// Source `vfasterf`.
pub fn error(values: Floats) -> Result<Floats, Error> {
    map(values, special::error)
}
/// Source `vfastererf`.
pub fn error_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::error_fast)
}
/// Source `vfastinverseerf` with its rational correction.
pub fn inverse_error(values: Floats) -> Result<Floats, Error> {
    map(values, special::inverse_error)
}
/// Source `vfasterinverseerf`.
pub fn inverse_error_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::inverse_error_fast)
}
/// Source `vfastlgamma`, including source-defined negative intervals.
pub fn log_gamma(values: Floats) -> Result<Floats, Error> {
    map(values, special::log_gamma)
}
/// Source `vfasterlgamma` with a positive logarithm argument.
pub fn log_gamma_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::log_gamma_fast)
}
/// Source `vfastdigamma`, preserving valid negative inputs and checked poles.
pub fn digamma(values: Floats) -> Result<Floats, Error> {
    map(values, special::digamma)
}
/// Source `vfasterdigamma`.
pub fn digamma_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::digamma_fast)
}
/// Source `vfastlambertw` on its principal real domain.
pub fn lambert_w(values: Floats) -> Result<Floats, Error> {
    map(values, special::lambert_w)
}
/// Source `vfasterlambertw`.
pub fn lambert_w_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::lambert_w_fast)
}
/// Source `vfastlambertwexpx`, correcting invalid refinements per lane.
pub fn lambert_w_exp(values: Floats) -> Result<Floats, Error> {
    map(values, special::lambert_w_exp)
}
/// Source `vfasterlambertwexpx`, preserving valid source bits per lane.
pub fn lambert_w_exp_fast(values: Floats) -> Result<Floats, Error> {
    map(values, special::lambert_w_exp_fast)
}
