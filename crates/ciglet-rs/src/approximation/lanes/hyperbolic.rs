//! Four-lane bundled hyperbolic and logistic approximations.
//! Each lane retains the source's exponential and outer formula stages.
use super::{Floats, map};
use crate::Error;
use crate::approximation::{Precision, hyperbolic};

/// Source `vfastsinh`/`vfastersinh`, selected by Approximate/Fast.
pub fn sine(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |value| hyperbolic::sine(value, precision))
}

/// Source `vfastcosh`/`vfastercosh`, with checked binary32 sum stages.
pub fn cosine(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |value| hyperbolic::cosine(value, precision))
}

/// Source `vfasttanh`/`vfastertanh`, retaining the -2*value exponential input.
pub fn tangent(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |value| hyperbolic::tangent(value, precision))
}

/// Source `vfastsigmoid`/`vfastersigmoid`, retaining the logistic quotient.
pub fn sigmoid(values: Floats, precision: Precision) -> Result<Floats, Error> {
    map(values, |value| hyperbolic::sigmoid(value, precision))
}
