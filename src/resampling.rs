//! Original antialiased Lanczos resampling with an explicit boundary policy.
use crate::{Error, approximation, filtering, window::Window};
use rustfft::FftNum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundaryPolicy {
    /// Include the first sample, correcting the original strict-positive index.
    #[default]
    IncludeFirst,
    /// Reproduce the original omission of sample zero for numerical comparison.
    LegacySkipFirst,
}

pub(crate) fn kernel<T: FftNum + Into<f64>>(distance: T) -> Result<T, Error> {
    let x = distance.into();
    if x == 0.0 {
        return Ok(T::one());
    }
    if !(-4.0..=4.0).contains(&x) {
        return Ok(T::zero());
    }
    let first = T::from_f32(approximation::sine((std::f64::consts::PI * x) as f32)?)
        .ok_or(Error("resampling weight cannot be represented"))?;
    let second = T::from_f32(approximation::sine(
        (std::f64::consts::PI * x / 4.0) as f32,
    )?)
    .ok_or(Error("resampling weight cannot be represented"))?;
    let product = T::from_u8(4).expect("four is representable") * first * second;
    T::from_f64(product.into() / std::f64::consts::PI / std::f64::consts::PI / x / x)
        .ok_or(Error("resampling weight cannot be represented"))
}

/// Ratio is output sample rate / input sample rate. Downsampling first applies
/// the original 32-tap Hamming FIR and compensates its 16-sample delay.
pub fn resample<T: FftNum + Into<f64>>(signal: &[T], ratio: T) -> Result<Vec<T>, Error> {
    resample_with(signal, ratio, BoundaryPolicy::IncludeFirst)
}

pub fn resample_with<T: FftNum + Into<f64>>(
    signal: &[T],
    ratio: T,
    policy: BoundaryPolicy,
) -> Result<Vec<T>, Error> {
    if signal.len() > i32::MAX as usize
        || !ratio.into().is_finite()
        || ratio.into() <= 0.0
        || signal.iter().any(|value| !(*value).into().is_finite())
    {
        return Err(Error(
            "resampling requires finite samples and a positive finite ratio within the legacy range",
        ));
    }
    if ratio == T::one() {
        let mut output = filtering::zeros(signal.len())?;
        output.copy_from_slice(signal);
        return Ok(output);
    }
    let size =
        T::from_usize(signal.len()).ok_or(Error("resampling length cannot be represented"))?;
    let output_length = (size * ratio).into().round();
    if !output_length.is_finite() || output_length > i32::MAX as f64 {
        return Err(Error("resampling output exceeds the legacy range"));
    }
    let mut output = filtering::zeros(output_length as usize)?;
    if output.is_empty() {
        return Ok(output);
    }
    let filtered;
    let input = if ratio.into() < 1.0 {
        let impulse = filtering::fir(32, ratio, false, Window::Hamming)?;
        filtered = filtering::convolve(signal, &impulse)?;
        &filtered[16..16 + signal.len()]
    } else {
        signal
    };
    for (index, value) in output.iter_mut().enumerate() {
        let position =
            T::from_usize(index).ok_or(Error("resampling index cannot be represented"))? / ratio;
        let base = position.into().floor();
        if base > i32::MAX as f64 {
            return Err(Error("resampling index exceeds the legacy range"));
        }
        let fraction =
            position - T::from_f64(base).ok_or(Error("resampling index cannot be represented"))?;
        for offset in -3..=4 {
            let source = base as i64 + i64::from(offset);
            if source < 0
                || source >= signal.len() as i64
                || (source == 0 && policy == BoundaryPolicy::LegacySkipFirst)
            {
                continue;
            }
            let distance = fraction - T::from_i32(offset).expect("small offset is representable");
            *value = *value + kernel(distance)? * input[source as usize];
        }
    }
    Ok(output)
}
