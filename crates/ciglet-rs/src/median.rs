//! Median filtering with the original progressive endpoint windows.
use crate::Error;
use rustfft::FftNum;

fn window_median<T: FftNum + Into<f64>>(values: &mut [T]) -> Result<T, Error> {
    let even = values.len().is_multiple_of(2);
    let middle = values.len() / 2;
    let (lower, selected, _) =
        values.select_nth_unstable_by(middle, |a, b| (*a).into().total_cmp(&(*b).into()));
    let upper = *selected;
    if !even {
        return Ok(upper);
    }
    let lower = lower
        .iter()
        .copied()
        .max_by(|a, b| (*a).into().total_cmp(&(*b).into()))
        .expect("an even nonempty window has a lower partition");
    let sum = lower + upper;
    let average = if sum.into().is_finite() {
        sum.into() * 0.5
    } else {
        lower.into() * 0.5 + upper.into() * 0.5
    };
    let value = T::from_f64(average).ok_or(Error("median cannot be represented"))?;
    if !value.into().is_finite() {
        return Err(Error("median arithmetic overflows"));
    }
    Ok(value)
}

/// Filter finite f32/f64 samples. Interior windows contain `width` samples;
/// the first width/2 outputs use growing prefixes and the last width/2 use
/// shrinking suffixes, preserving C's asymmetric edge rule.
/// Nonempty input requires 1 <= width <= length. Empty input returns empty.
pub fn filter<T: FftNum + Into<f64>>(signal: &[T], width: usize) -> Result<Vec<T>, Error> {
    if width == 0 || width > i32::MAX as usize || signal.len() > i32::MAX as usize {
        return Err(Error(
            "median filtering requires a positive bounded width and length",
        ));
    }
    if signal.is_empty() {
        return Ok(Vec::new());
    }
    if width > signal.len() {
        return Err(Error("median filter width exceeds signal length"));
    }
    if signal.iter().any(|value| !(*value).into().is_finite()) {
        return Err(Error("median filtering requires finite samples"));
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(signal.len())
        .map_err(|_| Error("median output allocation failed"))?;
    let mut scratch = Vec::new();
    scratch
        .try_reserve_exact(width)
        .map_err(|_| Error("median scratch allocation failed"))?;
    let half = width / 2;
    for index in 0..signal.len() {
        let window = if index < half {
            &signal[..index + 1]
        } else if index >= signal.len() - half {
            &signal[index..]
        } else {
            &signal[index - half..index - half + width]
        };
        scratch.clear();
        scratch.extend_from_slice(window);
        output.push(window_median(&mut scratch)?);
    }
    Ok(output)
}
