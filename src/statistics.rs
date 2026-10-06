//! Vector statistics. Reference: upstream `ciglet.h` and `ciglet.c`.
use crate::Error;

pub fn sum(values: &[f32]) -> f32 {
    values.iter().copied().sum()
}
pub fn sum_squares(values: &[f32]) -> f32 {
    values.iter().map(|value| value * value).sum()
}
pub fn mean(values: &[f32]) -> Result<f32, Error> {
    if values.is_empty() {
        return Err(Error("mean requires at least one value"));
    }
    Ok(sum(values) / values.len() as f32)
}
/// Population variance using the original operation order.
pub fn variance(values: &[f32]) -> Result<f32, Error> {
    let average = mean(values)?;
    Ok(sum_squares(values) / values.len() as f32 - average * average)
}
pub fn minimum(values: &[f32]) -> Option<f32> {
    values
        .first()
        .copied()
        .map(|first| values.iter().fold(first, |a, &b| if a < b { a } else { b }))
}
pub fn maximum(values: &[f32]) -> Option<f32> {
    values
        .first()
        .copied()
        .map(|first| values.iter().fold(first, |a, &b| if a > b { a } else { b }))
}
/// Select in place using total ordering, including deterministic NaN ordering.
pub fn select_nth(values: &mut [f32], index: usize) -> Result<f32, Error> {
    if index >= values.len() {
        return Err(Error("selection index is out of range"));
    }
    let (_, selected, _) = values.select_nth_unstable_by(index, f32::total_cmp);
    Ok(*selected)
}
pub fn median(values: &[f32]) -> Result<f32, Error> {
    if values.is_empty() {
        return Err(Error("median requires at least one value"));
    }
    let mut ordered = values.to_vec();
    let middle = values.len() / 2;
    let upper = select_nth(&mut ordered, middle)?;
    if values.len() % 2 == 1 {
        Ok(upper)
    } else {
        Ok((select_nth(&mut ordered, middle - 1)? + upper) * 0.5)
    }
}
/// Stable ascending ordering, returning values and their original indices.
pub fn sort(values: &[f32]) -> (Vec<f32>, Vec<usize>) {
    let mut indices: Vec<_> = (0..values.len()).collect();
    indices.sort_by(|&a, &b| values[a].total_cmp(&values[b]));
    (
        indices.iter().map(|&index| values[index]).collect(),
        indices,
    )
}
fn equal_lengths(left: &[f32], right: &[f32]) -> Result<(), Error> {
    if left.len() != right.len() {
        return Err(Error("vector lengths differ"));
    }
    Ok(())
}
pub fn dot(left: &[f32], right: &[f32]) -> Result<f32, Error> {
    equal_lengths(left, right)?;
    Ok(left.iter().zip(right).map(|(a, b)| a * b).sum())
}
pub fn covariance(left: &[f32], right: &[f32]) -> Result<f32, Error> {
    equal_lengths(left, right)?;
    let left_mean = mean(left)?;
    let right_mean = mean(right)?;
    Ok(left
        .iter()
        .zip(right)
        .map(|(a, b)| (a - left_mean) * (b - right_mean))
        .sum::<f32>()
        / left.len() as f32)
}
pub fn correlation(left: &[f32], right: &[f32]) -> Result<f32, Error> {
    let covariance = covariance(left, right)?;
    // C sqrt promotes its f32 argument to double before division.
    let divisor = f64::from(variance(left)? * variance(right)?).sqrt();
    Ok((f64::from(covariance) / divisor) as f32)
}
/// Nonnegative-lag, unnormalized cross correlation, returning `lags` entries.
pub fn cross_correlation(left: &[f32], right: &[f32], lags: usize) -> Result<Vec<f32>, Error> {
    equal_lengths(left, right)?;
    Ok((0..lags)
        .map(|lag| {
            if lag >= left.len() {
                return 0.0;
            }
            left[lag..].iter().zip(right).map(|(a, b)| a * b).sum()
        })
        .collect())
}
