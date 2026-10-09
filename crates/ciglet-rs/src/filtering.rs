//! Windowed frequency-sampling FIR design and full linear convolution.
use crate::{
    Error, fourier,
    window::{Precision, Window},
};
use rustfft::FftNum;

/// Cutoffs in cycles per sample, between zero and one half.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Passband<T> {
    Lowpass(T),
    Highpass(T),
    Bandpass { lower: T, upper: T },
    Bandstop { lower: T, upper: T },
}

pub(crate) fn zeros<T: FftNum>(length: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("filter allocation failed"))?;
    values.resize(length, T::zero());
    Ok(values)
}

/// Design the original cig_winfir response, retaining fractional-bin weights
/// and its even-length Nyquist replacement. Length is the number of taps.
pub fn windowed_fir<T: FftNum + Into<f64>>(
    length: usize,
    passband: Passband<T>,
    window: Window,
) -> Result<Vec<T>, Error> {
    if !(2..=i32::MAX as usize).contains(&length) {
        return Err(Error(
            "FIR length must be at least two within the legacy range",
        ));
    }
    let (lower, upper) = match passband {
        Passband::Lowpass(cutoff) | Passband::Highpass(cutoff) => (cutoff, cutoff),
        Passband::Bandpass { lower, upper } | Passband::Bandstop { lower, upper } => (lower, upper),
    };
    if !lower.into().is_finite()
        || !upper.into().is_finite()
        || lower.into() < 0.0
        || upper.into() > 0.5
        || lower.into() > upper.into()
    {
        return Err(Error(
            "FIR cutoffs must be finite and ordered in zero to one half",
        ));
    }
    let size = T::from_usize(length).ok_or(Error("FIR length cannot be represented"))?;
    let lower_bin = lower * size;
    let upper_bin = upper * size;
    let lo = lower_bin.into().floor() as usize;
    let hi = upper_bin.into().floor() as usize;
    let lo_ceil = lower_bin.into().ceil() as usize;
    let hi_ceil = upper_bin.into().ceil() as usize;
    let lo_fraction =
        T::from_f64(lower_bin.into() % 1.0).ok_or(Error("FIR cutoff cannot be represented"))?;
    let hi_fraction =
        T::from_f64(upper_bin.into() % 1.0).ok_or(Error("FIR cutoff cannot be represented"))?;
    let mut response = zeros(length)?;
    match passband {
        Passband::Lowpass(_) => {
            response[..=lo].fill(T::one());
            response[lo_ceil] = lo_fraction;
        }
        Passband::Highpass(_) => {
            if lo < length / 2 {
                response[lo + 1..=length / 2].fill(T::one());
            }
            response[lo] = T::from_f64(1.0 - lo_fraction.into())
                .ok_or(Error("FIR weight cannot be represented"))?;
        }
        Passband::Bandpass { .. } => {
            response[lo + 1..=hi].fill(T::one());
            response[lo] = T::from_f64(1.0 - lo_fraction.into())
                .ok_or(Error("FIR weight cannot be represented"))?;
            response[hi_ceil] = hi_fraction;
        }
        Passband::Bandstop { .. } => {
            response.fill(T::one());
            response[lo + 1..=hi].fill(T::zero());
            response[lo] = lo_fraction;
            response[hi_ceil] = T::from_f64(1.0 - hi_fraction.into())
                .ok_or(Error("FIR weight cannot be represented"))?;
        }
    }
    if length.is_multiple_of(2) {
        response[length / 2] = response[length / 2 - 1];
    }
    for index in length / 2 + 1..length {
        response[index] = response[length - index];
    }
    let mut impulse = fourier::inverse_dft(length, Some(&response), None)?;
    fourier::fft_shift_in_place(&mut impulse);
    let weights = window.generate_with(length, Precision::Accurate)?;
    for ((value, weight), output) in impulse.into_iter().zip(weights).zip(&mut response) {
        *output = value.re * weight;
    }
    Ok(response)
}

/// fir1 equivalent, with cutoff normalized to Nyquist rather than sample rate.
pub fn fir<T: FftNum + Into<f64>>(
    length: usize,
    cutoff: T,
    highpass: bool,
    window: Window,
) -> Result<Vec<T>, Error> {
    let cutoff =
        T::from_f64(cutoff.into() / 2.0).ok_or(Error("FIR cutoff cannot be represented"))?;
    windowed_fir(
        length,
        if highpass {
            Passband::Highpass(cutoff)
        } else {
            Passband::Lowpass(cutoff)
        },
        window,
    )
}

/// fir1bp equivalent, with both cutoffs normalized to Nyquist.
pub fn bandpass_fir<T: FftNum + Into<f64>>(
    length: usize,
    lower: T,
    upper: T,
    window: Window,
) -> Result<Vec<T>, Error> {
    windowed_fir(
        length,
        Passband::Bandpass {
            lower: T::from_f64(lower.into() / 2.0)
                .ok_or(Error("FIR cutoff cannot be represented"))?,
            upper: T::from_f64(upper.into() / 2.0)
                .ok_or(Error("FIR cutoff cannot be represented"))?,
        },
        window,
    )
}

/// Full linear convolution in original coefficient accumulation order.
/// Empty operands produce an empty output. No padded input copy is required.
pub fn convolve<T: FftNum + Into<f64>>(signal: &[T], impulse: &[T]) -> Result<Vec<T>, Error> {
    if signal
        .iter()
        .chain(impulse)
        .any(|value| !(*value).into().is_finite())
    {
        return Err(Error("convolution inputs must be finite"));
    }
    if signal.is_empty() || impulse.is_empty() {
        return Ok(Vec::new());
    }
    let length = signal
        .len()
        .checked_add(impulse.len() - 1)
        .filter(|length| *length <= i32::MAX as usize)
        .ok_or(Error("convolution length exceeds the legacy range"))?;
    let mut output = zeros(length)?;
    for (index, value) in output.iter_mut().enumerate() {
        let begin = (index + 1).saturating_sub(signal.len());
        let end = impulse.len().min(index + 1);
        for coefficient in begin..end {
            *value = *value + impulse[coefficient] * signal[index - coefficient];
        }
    }
    Ok(output)
}

/// Initial-state handling for recursive filters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Startup {
    /// Begin processing at sample zero with zero input/output history.
    #[default]
    ZeroState,
    /// Retain skipped leading samples in the original equal-length 2..=6
    /// coefficient fast paths. Other coefficient shapes use zero state.
    LegacyZeroPrefix,
}

/// Causal filtering with the original monic denominator convention.
/// Denominator element zero is ignored, as in cig_filter; coefficients are
/// not normalized. Empty numerator coefficients produce zeros. The result
/// contains exactly one sample per input, without an unused allocation tail.
pub fn filter<T: FftNum + Into<f64>>(
    signal: &[T],
    numerator: &[T],
    denominator: &[T],
) -> Result<Vec<T>, Error> {
    filter_with(signal, numerator, denominator, Startup::ZeroState)
}

/// Filter with an explicit startup compatibility policy.
pub fn filter_with<T: FftNum + Into<f64>>(
    signal: &[T],
    numerator: &[T],
    denominator: &[T],
    startup: Startup,
) -> Result<Vec<T>, Error> {
    if [signal.len(), numerator.len(), denominator.len()]
        .into_iter()
        .any(|n| n > i32::MAX as usize)
        || signal
            .iter()
            .chain(numerator)
            .chain(denominator)
            .any(|value| !(*value).into().is_finite())
    {
        return Err(Error(
            "filter inputs must be finite and within the legacy range",
        ));
    }
    let mut output = zeros(signal.len())?;
    let grouped = numerator.len() == denominator.len() && (2..=6).contains(&denominator.len());
    let begin = if grouped && startup == Startup::LegacyZeroPrefix {
        denominator.len() - 1
    } else {
        0
    };
    for index in begin..signal.len() {
        let mut value = T::zero();
        if grouped {
            // Retain the source fast path's grouped arithmetic. This affects
            // rounding even after correcting which sample processing starts at.
            let mut feedback = T::zero();
            for (lag, coefficient) in denominator.iter().enumerate().skip(1).take(index) {
                feedback = feedback + *coefficient * output[index - lag];
            }
            let mut forward = T::zero();
            for (lag, coefficient) in numerator.iter().enumerate().take(index + 1) {
                forward = forward + *coefficient * signal[index - lag];
            }
            value = value - feedback;
            value = value + forward;
        } else {
            for (lag, coefficient) in denominator.iter().enumerate().skip(1).take(index) {
                value = value - *coefficient * output[index - lag];
            }
            for (lag, coefficient) in numerator.iter().enumerate().take(index + 1) {
                value = value + *coefficient * signal[index - lag];
            }
        }
        if !value.into().is_finite() {
            return Err(Error("recursive filter output overflows"));
        }
        output[index] = value;
    }
    Ok(output)
}

/// Apply the same filter forward and backward, reversing between passes.
/// This preserves ciglet's simple two-pass workflow without endpoint padding
/// or a steady-state initialization step.
pub fn forward_backward<T: FftNum + Into<f64>>(
    signal: &[T],
    numerator: &[T],
    denominator: &[T],
) -> Result<Vec<T>, Error> {
    forward_backward_with(signal, numerator, denominator, Startup::ZeroState)
}

/// Forward/backward filtering with explicit original-startup compatibility.
pub fn forward_backward_with<T: FftNum + Into<f64>>(
    signal: &[T],
    numerator: &[T],
    denominator: &[T],
    startup: Startup,
) -> Result<Vec<T>, Error> {
    let mut forward = filter_with(signal, numerator, denominator, startup)?;
    forward.reverse();
    let mut backward = filter_with(&forward, numerator, denominator, startup)?;
    backward.reverse();
    Ok(backward)
}
