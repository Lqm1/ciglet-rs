//! Framewise periodicity measures and period-to-frequency interpolation.
use crate::{Error, array::Array2, interpolation, statistics::Scalar, vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Autocorrelation,
    AbsoluteDifference,
    SquaredDifference,
    Yin,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Center and scale the autocorrelation pairs before reduction.
    Stable,
    /// Retain the original prefix-sum autocorrelation arithmetic.
    Legacy,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub center: i32,
    pub length: usize,
}

/// Each output row contains lags `0..periods`. Negative centers clamp to zero.
pub fn compute<T: Scalar>(
    samples: &[T],
    frames: &[Frame],
    periods: usize,
    method: Method,
) -> Result<Array2<T>, Error> {
    compute_with(samples, frames, periods, method, Policy::Stable)
}

/// Preserve the original strict trailing boundary: start + window + lag < length.
/// Unavailable lags yield zero, or one for YIN. Constant ACF windows yield zero.
pub fn compute_with<T: Scalar>(
    samples: &[T],
    frames: &[Frame],
    periods: usize,
    method: Method,
    policy: Policy,
) -> Result<Array2<T>, Error> {
    vector::validate(samples)?;
    if periods == 0 || periods > i32::MAX as usize || frames.len() > i32::MAX as usize {
        return Err(Error("correlogram dimensions are out of range"));
    }
    if (!frames.is_empty() && samples.is_empty())
        || frames
            .iter()
            .any(|f| f.length == 0 || f.length > i32::MAX as usize)
    {
        return Err(Error(
            "correlogram requires nonempty samples and positive windows",
        ));
    }
    let size = frames
        .len()
        .checked_mul(periods)
        .ok_or(Error("correlogram dimensions overflow"))?;
    let mut output = vector::buffer(size)?;
    let mut sums = vector::buffer::<f64>(0)?;
    let mut squares = vector::buffer::<f64>(0)?;
    if method == Method::Autocorrelation && policy == Policy::Legacy && !frames.is_empty() {
        let prefix_length = samples
            .len()
            .checked_add(1)
            .ok_or(Error("prefix length overflows"))?;
        sums = vector::buffer(prefix_length)?;
        squares = vector::buffer(prefix_length)?;
        sums.push(0.0);
        squares.push(0.0);
        for &sample in samples {
            let sample = sample.into();
            sums.push(sums.last().unwrap() + sample);
            squares.push(squares.last().unwrap() + sample * sample);
        }
    }
    for frame in frames {
        let start = (frame.center as i64 - (frame.length / 2) as i64)
            .clamp(0, samples.len() as i64 - 1) as usize;
        let count = T::from_usize(frame.length).unwrap();
        let mut cumulative = T::zero();
        for lag in 0..periods {
            let value = if method == Method::Yin && lag == 0 {
                T::one()
            } else if start + frame.length + lag >= samples.len() {
                if method == Method::Yin {
                    T::one()
                } else {
                    T::zero()
                }
            } else {
                let first = &samples[start..start + frame.length];
                let second = &samples[start + lag..start + lag + frame.length];
                match method {
                    Method::Autocorrelation if policy == Policy::Stable => {
                        stable_correlation(first, second)?
                    }
                    Method::Autocorrelation => {
                        let sum: T = T::from_f64(sums[start + frame.length] - sums[start]).unwrap();
                        let shifted_sum: T =
                            T::from_f64(sums[start + lag + frame.length] - sums[start + lag])
                                .unwrap();
                        let square: T =
                            T::from_f64(squares[start + frame.length] - squares[start]).unwrap();
                        let shifted_square: T =
                            T::from_f64(squares[start + lag + frame.length] - squares[start + lag])
                                .unwrap();
                        let product = first
                            .iter()
                            .zip(second)
                            .fold(T::zero(), |a, (&x, &y)| a + x * y);
                        let numerator = product * count - sum * shifted_sum;
                        let denominator = (count * square - sum * sum)
                            * (count * shifted_square - shifted_sum * shifted_sum);
                        let ratio =
                            T::from_f64(numerator.into() / denominator.into().sqrt()).unwrap();
                        // The C min/max macros map unordered/NaN correlation to zero.
                        if ratio > T::zero() {
                            if ratio < T::one() { ratio } else { T::one() }
                        } else {
                            T::zero()
                        }
                    }
                    Method::AbsoluteDifference => {
                        let mut sum = T::zero();
                        for (&x, &y) in first.iter().zip(second) {
                            sum = vector::scalar(sum.into() + (x - y).into().abs())?;
                        }
                        sum
                    }
                    Method::SquaredDifference | Method::Yin => {
                        let difference = first
                            .iter()
                            .zip(second)
                            .fold(T::zero(), |a, (&x, &y)| a + (x - y) * (x - y));
                        vector::validate(&[difference])?;
                        if method == Method::Yin {
                            let floor = T::from_f64(1e-10).unwrap();
                            // C compares against a double literal then assigns the scalar.
                            let difference = if difference.into() > 1e-10 {
                                difference
                            } else {
                                floor
                            };
                            cumulative = cumulative + difference;
                            difference * T::from_usize(lag).unwrap() / cumulative
                        } else {
                            difference
                        }
                    }
                }
            };
            vector::validate(&[value])?;
            output.push(value);
        }
    }
    Array2::from_row_major(frames.len(), periods, output)
}

fn stable_correlation<T: Scalar>(first: &[T], second: &[T]) -> Result<T, Error> {
    let scale = first
        .iter()
        .chain(second)
        .map(|&x| x.into().abs())
        .fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return Ok(T::zero());
    }
    // Subtract a local origin before averaging to retain small DC fluctuations.
    let origin_x = first[0].into() / scale;
    let origin_y = second[0].into() / scale;
    let n = first.len() as f64;
    let mean_x = first
        .iter()
        .map(|&x| x.into() / scale - origin_x)
        .sum::<f64>()
        / n;
    let mean_y = second
        .iter()
        .map(|&y| y.into() / scale - origin_y)
        .sum::<f64>()
        / n;
    let (mut product, mut xx, mut yy) = (0.0, 0.0, 0.0);
    for (&x, &y) in first.iter().zip(second) {
        let x = x.into() / scale - origin_x - mean_x;
        let y = y.into() / scale - origin_y - mean_y;
        product += x * y;
        xx += x * x;
        yy += y * y;
    }
    if xx == 0.0 || yy == 0.0 {
        return Ok(T::zero());
    }
    vector::scalar((product / xx.sqrt() / yy.sqrt()).clamp(0.0, 1.0))
}

/// Map lags to arbitrary finite frequencies in hertz, retaining input order.
/// Positive zero maps to the last lag; negative values/zero clamp to lag zero.
pub fn to_frequency<T: Scalar>(
    periods: &Array2<T>,
    sampling_rate: i32,
    frequencies: &[T],
) -> Result<Array2<T>, Error> {
    if sampling_rate <= 0 || periods.columns() == 0 {
        return Err(Error(
            "frequency mapping requires positive sample rate and period count",
        ));
    }
    vector::validate(periods.as_slice())?;
    vector::validate(frequencies)?;
    let size = periods
        .rows()
        .checked_mul(frequencies.len())
        .ok_or(Error("frequency mapping dimensions overflow"))?;
    let mut output = vector::buffer(size)?;
    let coordinates = vector::linear_grid(
        T::zero(),
        T::from_usize(periods.columns() - 1).unwrap(),
        periods.columns(),
    )?;
    let mut positions = vector::buffer(frequencies.len())?;
    for &frequency in frequencies {
        let position = T::from_i32(sampling_rate).unwrap() / frequency;
        let position = if position.into().is_infinite() {
            if position > T::zero() {
                *coordinates.last().unwrap() * T::from_f64(2.0).unwrap() + T::one()
            } else {
                T::zero()
            }
        } else {
            position
        };
        positions.push(position);
    }
    for row in 0..periods.rows() {
        output.extend(interpolation::linear(
            &coordinates,
            periods.row(row).unwrap(),
            &positions,
        )?);
    }
    Array2::from_row_major(periods.rows(), frequencies.len(), output)
}
