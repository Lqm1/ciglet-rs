//! Moving averages and RMS with endpoint replication and fractional half-widths.
use crate::{
    Error,
    accumulation::{Scaled, Sum},
    interpolation,
    statistics::Scalar,
    vector,
};
use rustfft::FftNum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Unit-spaced sample cells, compensated rolling sums and scaled squares.
    Stable,
    /// Original cumulative-sum interpolation grid and selected-scalar arithmetic.
    Legacy,
}

/// Integer half-width h averages samples i-h+1 through i+h. Fractional widths
/// add equal endpoint-cell fractions; positive widths below one average i/i+1.
/// Zero width returns an unchanged copy. Endpoints are replicated.
pub fn moving_average<T: Scalar>(signal: &[T], half_width: T) -> Result<Vec<T>, Error> {
    moving_average_with(signal, half_width, Policy::Stable)
}
pub fn moving_average_with<T: Scalar>(
    signal: &[T],
    half_width: T,
    policy: Policy,
) -> Result<Vec<T>, Error> {
    if policy == Policy::Legacy {
        return legacy_average(signal, half_width);
    }
    vector::validate(signal)?;
    let width = half_width.into();
    if !width.is_finite() || !(0.0..=i32::MAX as f64).contains(&width) {
        return Err(Error(
            "moving average requires a nonnegative bounded half-width",
        ));
    }
    stable(signal, width, false)
}

/// RMS over the same integer-width windows, with squares retained in scaled form.
pub fn moving_rms<T: Scalar>(signal: &[T], half_width: usize) -> Result<Vec<T>, Error> {
    moving_rms_with(signal, half_width, Policy::Stable)
}
pub fn moving_rms_with<T: Scalar>(
    signal: &[T],
    half_width: usize,
    policy: Policy,
) -> Result<Vec<T>, Error> {
    if policy == Policy::Legacy {
        return legacy_rms(signal, half_width);
    }
    vector::validate(signal)?;
    if half_width > i32::MAX as usize {
        return Err(Error("moving RMS half-width exceeds the supported range"));
    }
    stable(signal, half_width as f64, true)
}

fn add(sum: &mut Sum, sample: f64, weight: f64, rms: bool) {
    if rms {
        sum.weighted_square(sample, weight);
    } else {
        sum.product(Scaled::new(sample, 0), Scaled::new(weight, 0));
    }
}

fn stable<T: Scalar>(signal: &[T], width: f64, rms: bool) -> Result<Vec<T>, Error> {
    let mut output = vector::buffer(signal.len())?;
    if signal.is_empty() {
        return Ok(output);
    }
    if width == 0.0 {
        for &sample in signal {
            output.push(if rms {
                vector::scalar(sample.into().abs())?
            } else {
                sample
            });
        }
        return Ok(output);
    }
    let minimum = signal
        .iter()
        .map(|&x| x.into())
        .fold(f64::INFINITY, f64::min);
    let maximum = signal
        .iter()
        .map(|&x| x.into())
        .fold(f64::NEG_INFINITY, f64::max);
    let absolute_maximum = minimum.abs().max(maximum.abs());
    let integer = width.floor() as i64;
    let fraction = width - integer as f64;
    let length = signal.len() as i64;
    let mut sum = Sum::new();
    let mut start = 0_usize;
    let mut end = 0_usize;
    for index in 0..signal.len() {
        let lower = index as i64 - integer + 1;
        let upper = index as i64 + integer + 1;
        let next_start = lower.clamp(0, length) as usize;
        let next_end = upper.clamp(0, length) as usize;
        while start < next_start {
            if start < end {
                add(&mut sum, signal[start].into(), -1.0, rms);
            }
            start += 1;
        }
        end = end.max(start);
        while end < next_end {
            add(&mut sum, signal[end].into(), 1.0, rms);
            end += 1;
        }
        let edges = [
            (signal[0].into(), (-lower).max(0) as f64),
            (
                signal[signal.len() - 1].into(),
                (upper - length).max(0) as f64,
            ),
            (
                signal[(lower - 1).clamp(0, length - 1) as usize].into(),
                fraction,
            ),
            (signal[upper.clamp(0, length - 1) as usize].into(), fraction),
        ];
        for (sample, weight) in edges {
            add(&mut sum, sample, weight, rms);
        }
        let result = sum.finish(2.0 * width);
        let value = if rms {
            result.sqrt().clamp(0.0, absolute_maximum)
        } else {
            result.value().clamp(minimum, maximum)
        };
        output.push(vector::scalar(value)?);
        for (sample, weight) in edges {
            add(&mut sum, sample, -weight, rms);
        }
    }
    Ok(output)
}

/// Preserve C cumulative-sum and fractional interpolation arithmetic.
/// Integer half-width h averages samples i-h+1 through i+h with endpoint
/// replication. Zero half-width returns an unchanged owned copy.
fn legacy_average<T: FftNum + Into<f64>>(signal: &[T], half_width: T) -> Result<Vec<T>, Error> {
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
fn legacy_rms<T: FftNum + Into<f64>>(signal: &[T], half_width: usize) -> Result<Vec<T>, Error> {
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
    let mut output = legacy_average(&squares, half_width)?;
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
