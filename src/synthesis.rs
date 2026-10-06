//! Centered sinusoid synthesis with constant-storage scalar recurrence.
use rustfft::FftNum;

use crate::Error;

/// One sinusoid. Phase is specified at sample index floor(length / 2).
#[derive(Debug, Clone, Copy)]
pub struct Sinusoid<T> {
    pub frequency_hz: T,
    pub amplitude: T,
    pub phase_radians: T,
}

/// Synthesize one sinusoid, applying its amplitude to the initial samples.
/// This corrects the ignored amplitude argument in upstream `gensin`.
pub fn sinusoid<T: FftNum + Into<f64>>(
    tone: Sinusoid<T>,
    sample_rate: u32,
    length: usize,
) -> Result<Vec<T>, Error> {
    sinusoids(&[tone], sample_rate, length)
}

/// Sum sinusoids in input order, preserving ciglet's scalar recurrence.
/// Empty tone lists produce silence; zero and one sample lengths are safe.
/// Frequencies and amplitudes may be negative, as in the original API.
pub fn sinusoids<T: FftNum + Into<f64>>(
    tones: &[Sinusoid<T>],
    sample_rate: u32,
    length: usize,
) -> Result<Vec<T>, Error> {
    if sample_rate == 0 || sample_rate > i32::MAX as u32 {
        return Err(Error(
            "sinusoid sample rate must be positive within the legacy range",
        ));
    }
    if length > i32::MAX as usize || tones.len() > i32::MAX as usize {
        return Err(Error("sinusoid dimensions exceed the legacy range"));
    }
    for tone in tones {
        if !tone.frequency_hz.into().is_finite()
            || !tone.amplitude.into().is_finite()
            || !tone.phase_radians.into().is_finite()
        {
            return Err(Error("sinusoid parameters must be finite"));
        }
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| Error("sinusoid buffer allocation failed"))?;
    output.resize(length, T::zero());
    if length == 0 {
        return Ok(output);
    }
    let origin = -(length as i32 / 2);
    for tone in tones {
        let step =
            T::from_f64(std::f64::consts::TAU / sample_rate as f64 * tone.frequency_hz.into())
                .ok_or(Error("sinusoid step cannot be represented"))?;
        let step_double: f64 = step.into();
        if !step_double.is_finite() {
            return Err(Error("sinusoid step must remain finite"));
        }
        let coefficient = T::from_f64(2.0 * step_double.cos())
            .ok_or(Error("sinusoid coefficient cannot be represented"))?;
        let initial = |position: i32| -> Result<T, Error> {
            let position =
                T::from_i32(position).ok_or(Error("sinusoid position cannot be represented"))?;
            let angle: f64 = (step * position + tone.phase_radians).into();
            if !angle.is_finite() {
                return Err(Error("sinusoid phase must remain finite"));
            }
            T::from_f64(angle.cos() * tone.amplitude.into())
                .ok_or(Error("sinusoid sample cannot be represented"))
        };
        let mut previous = initial(origin)?;
        output[0] = output[0] + previous;
        if length == 1 {
            continue;
        }
        let mut current = initial(origin + 1)?;
        output[1] = output[1] + current;
        for sample in output.iter_mut().skip(2) {
            let next = coefficient * current - previous;
            *sample = *sample + next;
            previous = current;
            current = next;
        }
    }
    Ok(output)
}
