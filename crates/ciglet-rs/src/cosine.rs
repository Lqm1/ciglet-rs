//! DCT-II with the original approximate cosine and orthonormal scaling.
use rustfft::FftNum;

use crate::{Error, approximation};

/// Compute ciglet's DCT-II for a nonempty real signal.
///
/// Both f32 and f64 inputs retain the original f32 cosine approximation.
/// The DC coefficient is divided by sqrt(n), and others by sqrt(n / 2).
pub fn dct<T: FftNum + Into<f64>>(signal: &[T]) -> Result<Vec<T>, Error> {
    let length = signal.len();
    if length == 0 || length > i32::MAX as usize {
        return Err(Error("DCT input must be nonempty within the legacy range"));
    }
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| Error("DCT buffer allocation failed"))?;
    output.resize(length, T::zero());
    let phase_step = std::f64::consts::PI / length as f64;
    for (frequency, coefficient) in output.iter_mut().enumerate() {
        for (position, &sample) in signal.iter().enumerate() {
            let angle = phase_step * (position as f64 + 0.5) * frequency as f64;
            let cosine = T::from_f32(approximation::cosine(angle as f32)?)
                .ok_or(Error("DCT cosine cannot be represented"))?;
            *coefficient = *coefficient + sample * cosine;
        }
        let divisor = if frequency == 0 {
            (length as f64).sqrt()
        } else {
            (length as f64 * 0.5).sqrt()
        };
        *coefficient = T::from_f64((*coefficient).into() / divisor)
            .ok_or(Error("DCT coefficient cannot be represented"))?;
    }
    Ok(output)
}
