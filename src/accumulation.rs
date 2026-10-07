//! Compensated binary-exponent sums and products for finite IEEE samples.
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
pub(crate) struct Scaled {
    pub(crate) fraction: f64,
    pub(crate) exponent: i32,
}
impl Scaled {
    pub(crate) fn new(value: f64, exponent: i32) -> Self {
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
    pub(crate) fn average(self, count: usize) -> Self {
        Self::new(self.fraction / count as f64, self.exponent)
    }
    pub(crate) fn value(self) -> f64 {
        scale(self.fraction, self.exponent)
    }
    pub(crate) fn divided_by(self, denominator: Self) -> Self {
        Self::new(
            self.fraction / denominator.fraction,
            self.exponent - denominator.exponent,
        )
    }
    pub(crate) fn sqrt(self) -> f64 {
        if self.fraction <= 0.0 {
            return 0.0;
        }
        scale(
            (self.fraction
                * if self.exponent.rem_euclid(2) == 0 {
                    1.0
                } else {
                    2.0
                })
            .sqrt(),
            self.exponent.div_euclid(2),
        )
    }
    pub(crate) fn hypot(self, other: Self) -> Self {
        if self.fraction == 0.0 {
            return Self::new(other.fraction.abs(), other.exponent);
        }
        if other.fraction == 0.0 {
            return Self::new(self.fraction.abs(), self.exponent);
        }
        let exponent = self.exponent.max(other.exponent);
        Self::new(
            scale(self.fraction, self.exponent - exponent)
                .hypot(scale(other.fraction, other.exponent - exponent)),
            exponent,
        )
    }
}

/// Scale a finite significand without an overflowing/underflowing power factor.
pub(crate) fn scale(value: f64, exponent: i32) -> f64 {
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

pub(crate) struct Sum {
    buckets: [Compensated; BUCKETS],
    first: usize,
    last: usize,
}
impl Sum {
    pub(crate) fn new() -> Self {
        Self {
            buckets: [Compensated::default(); BUCKETS],
            first: BUCKETS,
            last: 0,
        }
    }
    pub(crate) fn coefficient(&mut self, value: f64, exponent: i32) {
        if value == 0.0 {
            return;
        }
        let index = (exponent - LOWEST_EXPONENT) as usize;
        debug_assert!(index < BUCKETS);
        self.buckets[index].add(value);
        self.first = self.first.min(index);
        self.last = self.last.max(index);
    }
    pub(crate) fn scalar(&mut self, value: f64, exponent: i32) {
        let value = Scaled::new(value, exponent);
        self.coefficient(value.fraction, value.exponent);
    }
    pub(crate) fn product(&mut self, left: Scaled, right: Scaled) {
        let product = left.fraction * right.fraction;
        let exponent = left.exponent + right.exponent;
        self.coefficient(product, exponent);
        self.coefficient(left.fraction.mul_add(right.fraction, -product), exponent);
    }
    pub(crate) fn weighted_square(&mut self, value: f64, weight: f64) {
        let value = Scaled::new(value, 0);
        let weight = Scaled::new(weight, 0);
        let square = value.fraction * value.fraction;
        let exponent = value.exponent * 2;
        self.product(Scaled::new(square, exponent), weight);
        self.product(
            Scaled::new(value.fraction.mul_add(value.fraction, -square), exponent),
            weight,
        );
    }
    pub(crate) fn finish(&self, divisor: f64) -> Scaled {
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
        let divisor = Scaled::new(divisor, 0);
        let quotient = total.sum / divisor.fraction;
        let remainder = (-quotient).mul_add(divisor.fraction, total.sum) + total.correction;
        Scaled::new(
            quotient + remainder / divisor.fraction,
            exponent - divisor.exponent,
        )
    }
}
