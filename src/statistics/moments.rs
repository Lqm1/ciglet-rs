//! Centered moments with compensated binary-exponent buckets.
//! Bucket products are formed from bounded significands, including their FMA
//! residuals, so cancellation can recover small products between extreme inputs.
use super::Scalar;
use crate::{Error, vector};

const LOWEST_EXPONENT: i32 = -2240;
const HIGHEST_EXPONENT: i32 = 2112;
const BUCKETS: usize = (HIGHEST_EXPONENT - LOWEST_EXPONENT + 1) as usize;

#[derive(Clone, Copy, Default)]
struct Compensated {
    sum: f64,
    correction: f64,
}
impl Compensated {
    fn add(&mut self, value: f64) {
        let next = self.sum + value;
        self.correction += if self.sum.abs() >= value.abs() {
            (self.sum - next) + value
        } else {
            (value - next) + self.sum
        };
        self.sum = next;
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct Scaled {
    fraction: f64,
    exponent: i32,
}
impl Scaled {
    fn new(value: f64, exponent: i32) -> Self {
        if value == 0.0 {
            return Self::default();
        }
        let bits = value.to_bits();
        let field = ((bits >> 52) & 0x7ff) as i32;
        if field == 0 {
            return Self::new(value * f64::from_bits(2045_u64 << 52), exponent - 1022);
        }
        Self {
            fraction: f64::from_bits((bits & !(0x7ff_u64 << 52)) | (1023_u64 << 52)),
            exponent: exponent + field - 1023,
        }
    }
    fn average(self, count: usize) -> Self {
        Self::new(self.fraction / count as f64, self.exponent)
    }
    pub(super) fn value(self) -> f64 {
        scale(self.fraction, self.exponent)
    }
}

/// Scale a finite significand without an overflowing/underflowing power factor.
fn scale(value: f64, exponent: i32) -> f64 {
    let number = Scaled::new(value, exponent);
    if number.fraction == 0.0 {
        return 0.0;
    }
    if number.exponent > 1023 {
        return f64::INFINITY.copysign(number.fraction);
    }
    if number.exponent < -1075 {
        return 0.0_f64.copysign(number.fraction);
    }
    if number.exponent < -1022 {
        return number.fraction
            * f64::MIN_POSITIVE
            * f64::from_bits(((number.exponent + 1022 + 1023) as u64) << 52);
    }
    number.fraction * f64::from_bits(((number.exponent + 1023) as u64) << 52)
}

struct Sum {
    buckets: [Compensated; BUCKETS],
    first: usize,
    last: usize,
}
impl Sum {
    fn new() -> Self {
        Self {
            buckets: [Compensated::default(); BUCKETS],
            first: BUCKETS,
            last: 0,
        }
    }
    fn coefficient(&mut self, value: f64, exponent: i32) {
        if value == 0.0 {
            return;
        }
        let index = (exponent - LOWEST_EXPONENT) as usize;
        debug_assert!(index < BUCKETS);
        self.buckets[index].add(value);
        self.first = self.first.min(index);
        self.last = self.last.max(index);
    }
    fn scalar(&mut self, value: f64, exponent: i32) {
        let value = Scaled::new(value, exponent);
        self.coefficient(value.fraction, value.exponent);
    }
    fn product(&mut self, left: Scaled, right: Scaled) {
        let product = left.fraction * right.fraction;
        let exponent = left.exponent + right.exponent;
        self.coefficient(product, exponent);
        self.coefficient(left.fraction.mul_add(right.fraction, -product), exponent);
    }
    fn finish(self, divisor: f64) -> Scaled {
        let mut total = Compensated::default();
        let mut exponent = 0;
        if self.first == BUCKETS {
            return Scaled::default();
        }
        for index in (self.first..=self.last).rev() {
            for value in [self.buckets[index].sum, self.buckets[index].correction] {
                let value = Scaled::new(value, index as i32 + LOWEST_EXPONENT);
                if value.fraction == 0.0 {
                    continue;
                }
                let target = if total.sum == 0.0 && total.correction == 0.0 {
                    value.exponent
                } else {
                    exponent.max(value.exponent)
                };
                total.sum = scale(total.sum, exponent - target);
                total.correction = scale(total.correction, exponent - target);
                total.add(scale(value.fraction, value.exponent - target));
                let peak = total.sum.abs().max(total.correction.abs());
                let shift = Scaled::new(peak, 0).exponent;
                total.sum = scale(total.sum, -shift);
                total.correction = scale(total.correction, -shift);
                exponent = target + shift;
            }
        }
        let quotient = total.sum / divisor;
        let remainder = (-quotient).mul_add(divisor, total.sum) + total.correction;
        Scaled::new(quotient + remainder / divisor, exponent)
    }
}

/// Two-part subtraction retains a small mean subtracted from a huge sample.
fn difference(left: f64, right: f64) -> [Scaled; 2] {
    let mut left = left;
    let mut right = right;
    let mut exponent = 0;
    if !(left - right).is_finite() {
        left *= 0.5;
        right *= 0.5;
        exponent = 1;
    }
    let high = left - right;
    let virtual_right = left - high;
    let virtual_left = high + virtual_right;
    let low = (left - virtual_left) + (virtual_right - right);
    [Scaled::new(high, exponent), Scaled::new(low, exponent)]
}

pub(super) fn validate<T: Scalar>(values: &[T]) -> Result<(), Error> {
    if values.is_empty() {
        return Err(Error("statistics require at least one value"));
    }
    vector::validate(values)
}

pub(super) fn mean<T: Scalar>(values: &[T]) -> f64 {
    let mut sum = Sum::new();
    for &value in values {
        sum.scalar(value.into(), 0);
    }
    sum.finish(values.len() as f64).value()
}

fn residual<T: Scalar>(values: &[T], mean: f64) -> Scaled {
    let mut sum = Sum::new();
    for &value in values {
        for part in difference(value.into(), mean) {
            sum.coefficient(part.fraction, part.exponent);
        }
    }
    sum.finish(1.0)
}

pub(super) fn product<T: Scalar>(left: &[T], right: &[T]) -> Scaled {
    let left_mean = mean(left);
    let right_mean = mean(right);
    let mut sum = Sum::new();
    for (&left, &right) in left.iter().zip(right) {
        for left in difference(left.into(), left_mean) {
            for right in difference(right.into(), right_mean) {
                sum.product(left, right);
            }
        }
    }
    // Correct the small residual means of the rounded centering values.
    let left_residual = residual(left, left_mean).average(left.len());
    let mut right_residual = residual(right, right_mean);
    right_residual.fraction = -right_residual.fraction;
    sum.product(left_residual, right_residual);
    sum.finish(left.len() as f64)
}

pub(super) fn correlation(covariance: Scaled, left: Scaled, right: Scaled) -> Result<f64, Error> {
    if left.fraction <= 0.0 || right.fraction <= 0.0 {
        return Err(Error("correlation requires nonconstant vectors"));
    }
    let exponent = left.exponent + right.exponent;
    let divisor = left.fraction.sqrt()
        * right.fraction.sqrt()
        * if exponent.rem_euclid(2) == 0 {
            1.0
        } else {
            std::f64::consts::SQRT_2
        };
    Ok(scale(
        covariance.fraction / divisor,
        covariance.exponent - exponent.div_euclid(2),
    )
    .clamp(-1.0, 1.0))
}
