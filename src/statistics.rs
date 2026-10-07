//! Binary32/binary64 vector statistics with source scalar arithmetic order.
use crate::Error;
use rustfft::FftNum;
use std::cmp::Ordering;

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Supported floating-point statistics scalars with IEEE total ordering.
pub trait Scalar: sealed::Sealed + FftNum + Into<f64> + PartialOrd {
    fn total_cmp(&self, other: &Self) -> Ordering;
}
impl Scalar for f32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        f32::total_cmp(self, other)
    }
}
impl Scalar for f64 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        f64::total_cmp(self, other)
    }
}

/// Accumulate from positive zero, including for empty and negative-zero inputs.
pub fn sum<T: Scalar>(values: &[T]) -> T {
    values.iter().fold(T::zero(), |sum, &value| sum + value)
}
pub fn sum_squares<T: Scalar>(values: &[T]) -> T {
    values
        .iter()
        .fold(T::zero(), |sum, &value| sum + value * value)
}
pub fn mean<T: Scalar>(values: &[T]) -> Result<T, Error> {
    if values.is_empty() {
        return Err(Error("mean requires at least one value"));
    }
    Ok(sum(values) / T::from_usize(values.len()).unwrap())
}
/// Population variance retaining C's sum-of-squares subtraction and cancellation.
pub fn variance<T: Scalar>(values: &[T]) -> Result<T, Error> {
    let average = mean(values)?;
    Ok(sum_squares(values) / T::from_usize(values.len()).unwrap() - average * average)
}
pub fn minimum<T: Scalar>(values: &[T]) -> Option<T> {
    values
        .first()
        .copied()
        .map(|first| values.iter().fold(first, |a, &b| if a < b { a } else { b }))
}
pub fn maximum<T: Scalar>(values: &[T]) -> Option<T> {
    values
        .first()
        .copied()
        .map(|first| values.iter().fold(first, |a, &b| if a > b { a } else { b }))
}
/// Select in place using IEEE total ordering, including NaNs and signed zeros.
pub fn select_nth<T: Scalar>(values: &mut [T], index: usize) -> Result<T, Error> {
    if index >= values.len() {
        return Err(Error("selection index is out of range"));
    }
    let (_, selected, _) = values.select_nth_unstable_by(index, T::total_cmp);
    Ok(*selected)
}
pub fn median<T: Scalar>(values: &[T]) -> Result<T, Error> {
    if values.is_empty() {
        return Err(Error("median requires at least one value"));
    }
    let mut ordered = values.to_vec();
    let middle = values.len() / 2;
    let (lower, upper, _) = ordered.select_nth_unstable_by(middle, T::total_cmp);
    if values.len() % 2 == 1 {
        return Ok(*upper);
    }
    let lower = *lower.iter().max_by(|a, b| T::total_cmp(a, b)).unwrap();
    let sum = lower + *upper;
    let average =
        if !sum.into().is_finite() && lower.into().is_finite() && (*upper).into().is_finite() {
            lower.into() * 0.5 + (*upper).into() * 0.5
        } else {
            sum.into() * 0.5
        };
    Ok(T::from_f64(average).unwrap())
}
/// Stable ascending IEEE total ordering, returning original indices for ties.
pub fn sort<T: Scalar>(values: &[T]) -> (Vec<T>, Vec<usize>) {
    let mut indices: Vec<_> = (0..values.len()).collect();
    indices.sort_by(|&a, &b| T::total_cmp(&values[a], &values[b]));
    (
        indices.iter().map(|&index| values[index]).collect(),
        indices,
    )
}
fn equal_lengths<T>(left: &[T], right: &[T]) -> Result<(), Error> {
    if left.len() != right.len() {
        return Err(Error("vector lengths differ"));
    }
    Ok(())
}
pub fn dot<T: Scalar>(left: &[T], right: &[T]) -> Result<T, Error> {
    equal_lengths(left, right)?;
    Ok(left
        .iter()
        .zip(right)
        .fold(T::zero(), |sum, (&a, &b)| sum + a * b))
}
pub fn covariance<T: Scalar>(left: &[T], right: &[T]) -> Result<T, Error> {
    equal_lengths(left, right)?;
    let left_mean = mean(left)?;
    let right_mean = mean(right)?;
    let sum = left.iter().zip(right).fold(T::zero(), |sum, (&a, &b)| {
        sum + (a - left_mean) * (b - right_mean)
    });
    Ok(sum / T::from_usize(left.len()).unwrap())
}
/// Source correlation on numerically defined positive variances. Constant,
/// cancelled, nonfinite or overflowing/underflowing variances return an error.
pub fn correlation<T: Scalar>(left: &[T], right: &[T]) -> Result<T, Error> {
    let covariance = covariance(left, right)?;
    let left_variance = variance(left)?;
    let right_variance = variance(right)?;
    let divisor = (left_variance * right_variance).into().sqrt();
    if left_variance.into() <= 0.0
        || right_variance.into() <= 0.0
        || !divisor.is_finite()
        || divisor == 0.0
    {
        return Err(Error(
            "correlation requires numerically defined positive variances",
        ));
    }
    let value = covariance.into() / divisor;
    if !value.is_finite() {
        return Err(Error("correlation produced a nonfinite result"));
    }
    let result = T::from_f64(value).unwrap();
    if !result.into().is_finite() {
        return Err(Error("correlation produced a nonfinite result"));
    }
    Ok(result)
}
/// Nonnegative-lag, unnormalized cross correlation with zero-filled excess lags.
pub fn cross_correlation<T: Scalar>(left: &[T], right: &[T], lags: usize) -> Result<Vec<T>, Error> {
    equal_lengths(left, right)?;
    if lags > i32::MAX as usize {
        return Err(Error(
            "cross correlation lag count exceeds the supported range",
        ));
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(lags)
        .map_err(|_| Error("cross correlation allocation failed"))?;
    output.extend((0..lags).map(|lag| {
        if lag >= left.len() {
            return T::zero();
        }
        left[lag..]
            .iter()
            .zip(right)
            .fold(T::zero(), |sum, (&a, &b)| sum + a * b)
    }));
    Ok(output)
}
