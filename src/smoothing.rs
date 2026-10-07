//! Source-compatible moving averages with fractional half-widths.
use crate::{Error, interpolation};
use rustfft::FftNum;

/// Preserve C cumulative-sum and fractional interpolation arithmetic.
/// Integer half-width h averages samples i-h+1 through i+h with endpoint
/// replication. Zero half-width returns an unchanged owned copy.
pub fn moving_average<T: FftNum + Into<f64>>(signal: &[T], half_width: T) -> Result<Vec<T>, Error> {
    if signal.len() > i32::MAX as usize
        || !half_width.into().is_finite()
        || half_width.into() < 0.0
        || half_width.into() > i32::MAX as f64
        || signal.iter().any(|value| !(*value).into().is_finite())
    {
        return Err(Error(
            "moving average requires finite samples and a nonnegative bounded half-width",
        ));
    }
    if signal.is_empty() {
        return Ok(Vec::new());
    }
    let mut output = Vec::new();
    if half_width == T::zero() {
        output
            .try_reserve_exact(signal.len())
            .map_err(|_| Error("moving average allocation failed"))?;
        output.extend_from_slice(signal);
        return Ok(output);
    }
    let padding = half_width.into() as usize;
    let size = padding
        .checked_mul(2)
        .and_then(|extra| signal.len().checked_add(extra))
        .filter(|&size| size <= i32::MAX as usize)
        .ok_or(Error(
            "moving average padded length exceeds the legacy range",
        ))?;
    let mut sums = Vec::new();
    sums.try_reserve_exact(size)
        .map_err(|_| Error("moving average accumulation allocation failed"))?;
    let mut total = signal[0];
    sums.push(total);
    for _ in 0..padding {
        total = total + signal[0];
        sums.push(total);
    }
    for &sample in &signal[1..] {
        total = total + sample;
        sums.push(total);
    }
    for _ in 0..padding {
        total = total + signal[signal.len() - 1];
        sums.push(total);
    }
    if sums.iter().any(|value| !(*value).into().is_finite()) {
        return Err(Error("moving average cumulative sum overflows"));
    }
    let mut positions = Vec::new();
    positions
        .try_reserve_exact(signal.len())
        .map_err(|_| Error("moving average position allocation failed"))?;
    for index in 0..signal.len() {
        positions.push(
            T::from_usize(index).ok_or(Error("moving average index cannot be represented"))?
                + half_width,
        );
    }
    let end = T::from_usize(signal.len() + padding)
        .ok_or(Error("moving average endpoint cannot be represented"))?;
    let mut upper = interpolation::linear_uniform(-half_width, end, &sums, &positions)?;
    for (index, position) in positions.iter_mut().enumerate() {
        *position = T::from_usize(index)
            .ok_or(Error("moving average index cannot be represented"))?
            - half_width;
    }
    let lower = interpolation::linear_uniform(-half_width, end, &sums, &positions)?;
    for (upper, lower) in upper.iter_mut().zip(lower) {
        let difference = (*upper - lower) / half_width;
        *upper = T::from_f64(difference.into() * 0.5)
            .ok_or(Error("moving average value cannot be represented"))?;
        if !(*upper).into().is_finite() {
            return Err(Error("moving average result overflows"));
        }
    }
    Ok(upper)
}

/// Square, average using the source integer half-width, and take a double
/// precision square root before rounding to the selected scalar type.
pub fn moving_rms<T: FftNum + Into<f64>>(signal: &[T], half_width: usize) -> Result<Vec<T>, Error> {
    if signal.len() > i32::MAX as usize || half_width > i32::MAX as usize {
        return Err(Error("moving RMS half-width exceeds the legacy range"));
    }
    let mut squares = Vec::new();
    squares
        .try_reserve_exact(signal.len())
        .map_err(|_| Error("moving RMS allocation failed"))?;
    for &sample in signal {
        let square = sample * sample;
        if !square.into().is_finite() {
            return Err(Error("moving RMS squared sample overflows"));
        }
        squares.push(square);
    }
    let half_width =
        T::from_usize(half_width).ok_or(Error("moving RMS half-width cannot be represented"))?;
    let mut output = moving_average(&squares, half_width)?;
    for value in &mut output {
        *value = if (*value).into() <= 0.0 {
            T::zero()
        } else {
            T::from_f64((*value).into().sqrt())
                .ok_or(Error("moving RMS value cannot be represented"))?
        };
    }
    Ok(output)
}
