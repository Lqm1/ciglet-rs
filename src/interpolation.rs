//! Finite linear interpolation with the upstream coordinate conventions.
use crate::Error;

/// Scalar interpolation/extrapolation with source operation order on finite
/// arithmetic and a weighted-sum fallback when the endpoint difference overflows.
pub fn between<T: rustfft::FftNum + Into<f64>>(start: T, end: T, ratio: T) -> Result<T, Error> {
    if !start.into().is_finite() || !end.into().is_finite() || !ratio.into().is_finite() {
        return Err(Error("scalar interpolation requires finite inputs"));
    }
    let value = start + (end - start) * ratio;
    if value.into().is_finite() {
        return Ok(value);
    }
    let weight = ratio.into();
    let fallback = if weight == 0.0 {
        start.into()
    } else if weight == 1.0 {
        end.into()
    } else {
        (1.0 - weight) * start.into() + weight * end.into()
    };
    let value = T::from_f64(fallback).unwrap();
    if !value.into().is_finite() {
        return Err(Error("scalar interpolation overflowed"));
    }
    Ok(value)
}
use rustfft::FftNum;

fn validate<T: FftNum + Into<f64>>(values: &[T], positions: &[T]) -> Result<(), Error> {
    if values.is_empty() || values.len() > i32::MAX as usize || positions.len() > i32::MAX as usize
    {
        return Err(Error("interpolation requires a nonempty bounded source"));
    }
    if values
        .iter()
        .chain(positions)
        .any(|value| !(*value).into().is_finite())
    {
        return Err(Error("interpolation input must be finite"));
    }
    Ok(())
}

fn allocate<T>(length: usize) -> Result<Vec<T>, Error> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| Error("interpolation allocation failed"))?;
    Ok(output)
}

fn checked<T: FftNum + Into<f64>>(value: T) -> Result<T, Error> {
    if value.into().is_finite() {
        Ok(value)
    } else {
        Err(Error("interpolation arithmetic overflows"))
    }
}

/// Interpolate strictly increasing coordinates, clamping beyond both endpoints.
/// Queries may arrive in any order. Arithmetic retains the upstream scalar type.
pub fn linear<T: FftNum + Into<f64>>(
    coordinates: &[T],
    values: &[T],
    positions: &[T],
) -> Result<Vec<T>, Error> {
    validate(values, positions)?;
    if coordinates.len() != values.len()
        || coordinates.iter().any(|value| !(*value).into().is_finite())
        || coordinates
            .windows(2)
            .any(|pair| pair[0].into() >= pair[1].into())
    {
        return Err(Error(
            "interpolation coordinates must be finite and strictly increasing",
        ));
    }
    let mut output = allocate(positions.len())?;
    for &position in positions {
        let value = if position.into() <= coordinates[0].into() {
            values[0]
        } else {
            // Strict comparison retains C's preceding segment at exact knots.
            let upper = coordinates.partition_point(|value| (*value).into() < position.into());
            if upper == coordinates.len() {
                *values.last().unwrap()
            } else {
                let lower = upper - 1;
                checked(
                    (values[upper] - values[lower]) * (position - coordinates[lower])
                        / (coordinates[upper] - coordinates[lower])
                        + values[lower],
                )?
            }
        };
        output.push(value);
    }
    Ok(output)
}

/// Uniform source coordinates occupy [start, end), with count samples.
/// Preserve C's near-final-sample clamp at fractional index count - 1.01.
/// Unlike upstream, empty and unordered queries and singleton sources are safe.
pub fn linear_uniform<T: FftNum + Into<f64>>(
    start: T,
    end: T,
    values: &[T],
    positions: &[T],
) -> Result<Vec<T>, Error> {
    validate(values, positions)?;
    if !start.into().is_finite() || !end.into().is_finite() || start.into() >= end.into() {
        return Err(Error(
            "uniform interpolation requires finite increasing bounds",
        ));
    }
    let width = checked(end - start)?;
    let count =
        T::from_usize(values.len()).ok_or(Error("interpolation count cannot be represented"))?;
    let mut output = allocate(positions.len())?;
    for &position in positions {
        let value = if position.into() < start.into() || values.len() == 1 {
            values[0]
        } else {
            let index = checked((position - start) / width * count)?;
            if index.into() + 1.01 >= values.len() as f64 {
                *values.last().unwrap()
            } else {
                let lower = index.into() as usize;
                let remainder = index
                    - T::from_usize(lower)
                        .ok_or(Error("interpolation index cannot be represented"))?;
                checked(values[lower] + (values[lower + 1] - values[lower]) * remainder)?
            }
        };
        output.push(value);
    }
    Ok(output)
}

pub use crate::resampling::BoundaryPolicy;

/// Four-lobe Lanczos interpolation on source coordinates occupying [start, end).
/// Include sample zero by default, correcting upstream's strict-positive bound.
pub fn sinc_uniform<T: FftNum + Into<f64>>(
    start: T,
    end: T,
    values: &[T],
    positions: &[T],
) -> Result<Vec<T>, Error> {
    sinc_uniform_with(start, end, values, positions, BoundaryPolicy::IncludeFirst)
}

/// LegacySkipFirst retains the source's omission of sample zero. Both policies
/// initialize the output accumulator, correcting an uninitialized C allocation.
pub fn sinc_uniform_with<T: FftNum + Into<f64>>(
    start: T,
    end: T,
    values: &[T],
    positions: &[T],
    policy: BoundaryPolicy,
) -> Result<Vec<T>, Error> {
    validate(values, positions)?;
    if !start.into().is_finite() || !end.into().is_finite() || start.into() >= end.into() {
        return Err(Error(
            "sinc interpolation requires finite increasing bounds",
        ));
    }
    let width = checked(end - start)?;
    let count =
        T::from_usize(values.len()).ok_or(Error("interpolation count cannot be represented"))?;
    let mut output = allocate(positions.len())?;
    for &position in positions {
        let index = checked((position - start) / width * count)?;
        let base = index.into().floor();
        let mut value = T::zero();
        // Distant queries have no sample within the four-lobe kernel. Avoid
        // narrowing those coordinates to a signed integer unnecessarily.
        if base >= -4.0 && base <= values.len() as f64 + 3.0 {
            let fraction = index
                - T::from_f64(base).ok_or(Error("interpolation index cannot be represented"))?;
            for offset in -3..=4 {
                let source = base as i64 + i64::from(offset);
                if source < 0
                    || source >= values.len() as i64
                    || (source == 0 && policy == BoundaryPolicy::LegacySkipFirst)
                {
                    continue;
                }
                let distance =
                    fraction - T::from_i32(offset).expect("small offset is representable");
                value = checked(
                    value + crate::resampling::kernel(distance)? * values[source as usize],
                )?;
            }
        }
        output.push(value);
    }
    Ok(output)
}

/// Replace exact sentinel values with linear interpolation between known samples.
/// NaN sentinels match NaNs. Leading/trailing gaps repeat the nearest known value;
/// an all-missing series becomes zero, retaining C's finite-sentinel fallback.
pub fn fill_missing<T: FftNum + Into<f64>>(signal: &[T], blank: T) -> Result<Vec<T>, Error> {
    if signal.len() > i32::MAX as usize {
        return Err(Error(
            "missing-value interpolation exceeds the legacy range",
        ));
    }
    let missing = |value: T| {
        if blank.into().is_nan() {
            value.into().is_nan()
        } else {
            value == blank
        }
    };
    let mut output = allocate(signal.len())?;
    output.extend_from_slice(signal);
    let mut previous = None;
    for (index, &value) in signal.iter().enumerate() {
        if missing(value) {
            continue;
        }
        if !value.into().is_finite() {
            return Err(Error("known interpolation samples must be finite"));
        }
        match previous {
            None => output[..index].fill(value),
            Some((begin, first)) => {
                let span = T::from_usize(index - begin)
                    .ok_or(Error("interpolation span cannot be represented"))?;
                for (offset, target) in output[begin + 1..index].iter_mut().enumerate() {
                    let step = T::from_usize(offset + 1)
                        .ok_or(Error("interpolation offset cannot be represented"))?;
                    *target = checked((value - first) * step / span + first)?;
                }
            }
        }
        previous = Some((index, value));
    }
    match previous {
        Some((index, value)) => output[index + 1..].fill(value),
        None => output.fill(T::zero()),
    }
    Ok(output)
}
