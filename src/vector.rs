//! Source-compatible first differences, cumulative sums and sample grids.
use crate::{Error, statistics::Scalar};

pub(crate) fn validate<T: Scalar>(values: &[T]) -> Result<(), Error> {
    if values.len() > i32::MAX as usize || values.iter().any(|&value| !value.into().is_finite()) {
        return Err(Error("vector requires finite samples and a bounded length"));
    }
    Ok(())
}
pub(crate) fn buffer<T>(length: usize) -> Result<Vec<T>, Error> {
    if length > i32::MAX as usize {
        return Err(Error("vector length exceeds the supported range"));
    }
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("vector allocation failed"))?;
    Ok(values)
}
pub(crate) fn scalar<T: Scalar>(value: f64) -> Result<T, Error> {
    let value = T::from_f64(value).unwrap();
    if !value.into().is_finite() {
        return Err(Error("vector arithmetic overflowed"));
    }
    Ok(value)
}
/// First sample is retained; later samples are adjacent differences.
pub fn difference<T: Scalar>(values: &[T]) -> Result<Vec<T>, Error> {
    validate(values)?;
    let mut output = buffer(values.len())?;
    if let Some(&first) = values.first() {
        output.push(first);
    }
    output.extend(values.windows(2).map(|pair| pair[1] - pair[0]));
    validate(&output)?;
    Ok(output)
}
/// Cumulative sum starts with the original first sample, preserving its zero sign.
pub fn cumulative_sum<T: Scalar>(values: &[T]) -> Result<Vec<T>, Error> {
    validate(values)?;
    let mut output = buffer(values.len())?;
    if let Some((&first, remaining)) = values.split_first() {
        let mut total = first;
        output.push(total);
        for &value in remaining {
            total = total + value;
            output.push(total);
        }
    }
    validate(&output)?;
    Ok(output)
}
pub fn reverse<T: Clone>(values: &[T]) -> Vec<T> {
    values.iter().rev().cloned().collect()
}

/// Inclusive linear grid with source multiply-before-divide rounding.
/// Zero points is empty; one point is the starting value.
pub fn linear_grid<T: Scalar>(start: T, end: T, count: usize) -> Result<Vec<T>, Error> {
    validate(&[start, end])?;
    let mut output = buffer(count)?;
    if count == 1 {
        output.push(start);
        return Ok(output);
    }
    if count == 0 {
        return Ok(output);
    }
    let denominator = T::from_usize(count - 1).unwrap();
    for i in 0..count {
        let index = T::from_usize(i).unwrap();
        let value = start + (end - start) * index / denominator;
        output.push(if value.into().is_finite() {
            value
        } else {
            crate::interpolation::between(start, end, index / denominator)?
        });
    }
    Ok(output)
}
/// Checked i32 arithmetic progression. Intermediate arithmetic uses i64.
pub fn integer_sequence(start: i32, step: i32, count: usize) -> Result<Vec<i32>, Error> {
    let mut output = buffer(count)?;
    for i in 0..count {
        let value = i64::from(start) + i64::from(step) * i as i64;
        output.push(
            i32::try_from(value).map_err(|_| Error("integer progression exceeds i32 range"))?,
        );
    }
    Ok(output)
}
