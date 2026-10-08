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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KernelPolicy {
    /// Evaluate the removable near-zero singularity without binary32 sine reduction.
    #[default]
    Stable,
    /// Preserve the original approximate sine and scalar staging, including its near-zero defect.
    Legacy,
}

pub(crate) fn kernel_with<T: FftNum + Into<f64>>(
    distance: T,
    policy: KernelPolicy,
) -> Result<T, Error> {
    let x = distance.into();
    if policy == KernelPolicy::Stable && x.abs() <= 1e-3 {
        // sinc(pi*x) * sinc(pi*x/4), through the eighth-order term.
        // At this radius the omitted term is below 2.4e-29 for each factor.
        let sinc = |angle: f64| {
            let square = angle * angle;
            1.0 + square
                * (-1.0 / 6.0
                    + square * (1.0 / 120.0 + square * (-1.0 / 5040.0 + square / 362880.0)))
        };
        let angle = std::f64::consts::PI * x;
        return T::from_f64(sinc(angle) * sinc(angle / 4.0))
            .ok_or(Error("resampling weight cannot be represented"));
    }
    kernel(distance)
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
    resample_with_kernel(
        signal,
        ratio,
        BoundaryPolicy::IncludeFirst,
        KernelPolicy::Stable,
    )
}

/// Select the boundary convention while preserving the original kernel arithmetic.
/// Use `resample_with_kernel` to independently select corrected near-zero evaluation.
pub fn resample_with<T: FftNum + Into<f64>>(
    signal: &[T],
    ratio: T,
    policy: BoundaryPolicy,
) -> Result<Vec<T>, Error> {
    resample_with_kernel(signal, ratio, policy, KernelPolicy::Legacy)
}

/// Select sample-zero handling and numerical kernel behavior independently.
pub fn resample_with_kernel<T: FftNum + Into<f64>>(
    signal: &[T],
    ratio: T,
    policy: BoundaryPolicy,
    kernel_policy: KernelPolicy,
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
    let mut output = filtering::zeros(output_length(signal.len(), ratio)?)?;
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
            *value = *value + kernel_with(distance, kernel_policy)? * input[source as usize];
        }
    }
    Ok(output)
}

/// Source-precision length multiplication followed by double-precision rounding.
/// Ratio one preserves the input count without lossy scalar conversion.
pub(crate) fn output_length<T: FftNum + Into<f64>>(count: usize, ratio: T) -> Result<usize, Error> {
    if count > i32::MAX as usize || !ratio.into().is_finite() || ratio.into() <= 0.0 {
        return Err(Error(
            "resampling requires a positive finite ratio and a bounded input length",
        ));
    }
    if ratio == T::one() {
        return Ok(count);
    }
    let size = T::from_usize(count).ok_or(Error("resampling length cannot be represented"))?;
    let length = (size * ratio).into().round();
    if !length.is_finite() || length > i32::MAX as f64 {
        return Err(Error("resampling output exceeds the legacy range"));
    }
    Ok(length as usize)
}
