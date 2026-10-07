//! Range-based extreme values and strict local peaks with source tie handling.
use crate::Error;
use rustfft::FftNum;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Maximum,
    Minimum,
}

fn better<T: FftNum + Into<f64>>(left: T, right: T, direction: Direction) -> bool {
    match direction {
        Direction::Maximum => left.into() > right.into(),
        Direction::Minimum => left.into() < right.into(),
    }
}

fn bounds<T>(signal: &[T], range: RangeInclusive<usize>) -> Result<(usize, usize), Error> {
    let (start, end) = range.into_inner();
    if signal.len() > i32::MAX as usize || start > end || end >= signal.len() {
        return Err(Error(
            "extremum search requires a nonempty in-bounds inclusive range",
        ));
    }
    Ok((start, end))
}

fn finite<T: FftNum + Into<f64>>(values: &[T]) -> Result<(), Error> {
    if values.iter().any(|value| !(*value).into().is_finite()) {
        return Err(Error("extremum search requires finite inspected samples"));
    }
    Ok(())
}

/// Return the first global extremum in an inclusive range, preserving strict
/// source comparisons and returning the starting index when all values tie.
pub fn extremum<T: FftNum + Into<f64>>(
    signal: &[T],
    range: RangeInclusive<usize>,
    direction: Direction,
) -> Result<usize, Error> {
    let (start, end) = bounds(signal, range)?;
    finite(&signal[start..=end])?;
    let mut best = start;
    for (offset, &sample) in signal[start..=end].iter().enumerate().skip(1) {
        if better(sample, signal[best], direction) {
            best = start + offset;
        }
    }
    Ok(best)
}

/// Search strict local peaks in a range, comparing both immediate neighbors.
/// Preserve C's start-index fallback, including when no peak beats its value.
/// Endpoints have no two-sided peak; they are skipped without out-of-bounds reads.
pub fn peak<T: FftNum + Into<f64>>(
    signal: &[T],
    range: RangeInclusive<usize>,
    direction: Direction,
) -> Result<usize, Error> {
    let (start, end) = bounds(signal, range)?;
    let mut best = start;
    if signal.len() < 3 {
        finite(&signal[start..=start])?;
        return Ok(best);
    }
    let first = start.max(1);
    let last = end.min(signal.len() - 2);
    if first > last {
        finite(&signal[start..=start])?;
        return Ok(best);
    }
    finite(&signal[first - 1..=last + 1])?;
    for (offset, window) in signal[first - 1..=last + 1].windows(3).enumerate() {
        if better(window[1], window[0], direction)
            && better(window[1], window[2], direction)
            && better(window[1], signal[best], direction)
        {
            best = first + offset;
        }
    }
    Ok(best)
}
