//! CheapTrick-style log spectral envelope with the original approximation stages.
pub use crate::phase::NyquistPolicy;
use crate::{Error, approximation, real_cepstrum, smoothing, statistics::Scalar, vector};

#[derive(Debug, Clone, PartialEq)]
pub struct Envelope<T> {
    /// Full symmetric FFT grid, in natural-log units; no exponential is applied.
    pub log_spectrum: Vec<T>,
    /// The original optional coefficient output, including DC and Nyquist.
    pub cepstrum: Vec<T>,
}

/// Reuses forward/inverse FFT plans across spectra of the same length.
#[derive(Debug)]
pub struct Plan<T: Scalar> {
    cepstral: real_cepstrum::Plan<T>,
}
impl<T: Scalar> Plan<T> {
    pub fn new(length: usize) -> Result<Self, Error> {
        if length < 2 {
            return Err(Error("spectral envelope requires at least two FFT points"));
        }
        Ok(Self {
            cepstral: real_cepstrum::Plan::new(length)?,
        })
    }
    pub fn len(&self) -> usize {
        self.cepstral.len()
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn bins(&self) -> usize {
        self.cepstral.bins()
    }

    /// Preserve Nyquist. None uses the source convenience wrapper's harmonic count.
    pub fn estimate(
        &mut self,
        spectrum: &[T],
        fundamental: T,
        harmonics: Option<usize>,
    ) -> Result<Envelope<T>, Error> {
        self.estimate_with(spectrum, fundamental, harmonics, NyquistPolicy::Preserve)
    }
    /// Legacy replaces Nyquist with its preceding bin in both cepstral transforms.
    /// Fundamental is a ratio of sampling rate, in (0, 0.5]. Explicit counts >= 2.
    pub fn estimate_with(
        &mut self,
        spectrum: &[T],
        fundamental: T,
        harmonics: Option<usize>,
        policy: NyquistPolicy,
    ) -> Result<Envelope<T>, Error> {
        vector::validate(&[fundamental])?;
        vector::validate(spectrum)?;
        if spectrum.len() != self.bins()
            || spectrum.iter().any(|&x| x < T::zero())
            || fundamental <= T::zero()
            || fundamental.into() > 0.5
        {
            return Err(Error(
                "envelope requires nonnegative one-sided spectrum and valid fundamental",
            ));
        }
        if harmonics.is_some_and(|count| !(2..=i32::MAX as usize).contains(&count)) {
            return Err(Error("envelope harmonic count is out of range"));
        }
        let n = T::from_usize(self.len()).unwrap();
        let smoothing_width: T = vector::scalar(n.into() / (3.0 / fundamental.into()))?;
        let reflection = (fundamental * n).into().ceil() as usize;
        let mut reflected = vector::buffer(self.bins())?;
        reflected.extend_from_slice(spectrum);
        for index in 0..reflection / 2 {
            reflected[index] = reflected[reflection - index];
        }
        let mut smoothed = smoothing::moving_average(&reflected, smoothing_width)?;
        let count = match harmonics {
            Some(count) => Some(T::from_usize(count).unwrap()),
            None => {
                let count = (T::from_usize(self.len() / 2).unwrap() / fundamental)
                    .into()
                    .floor();
                if count > i32::MAX as f64 {
                    None
                } else {
                    Some(T::from_f64(count).unwrap())
                }
            }
        };
        let cutoff = match count {
            // For an unbounded implicit count the mathematical cutoff is
            // floor(n * (n/2 - 2*f0 - fractional_count*f0)). It exceeds the
            // one-sided spectrum for n >= 4, but is one for the two-point case.
            None if self.len() == 2 => 1,
            None => self.bins(),
            Some(count) => (fundamental * (count - T::from_f64(2.0).unwrap()) * n)
                .into()
                .floor()
                .clamp(0.0, self.bins() as f64) as usize,
        };
        // The source reads smoothed[-1] when cutoff is zero. Retain the first bin.
        if cutoff < self.bins() {
            let value = smoothed[cutoff.saturating_sub(1)];
            smoothed[cutoff..].fill(value);
        }
        for value in &mut smoothed {
            *value =
                T::from_f64(approximation::logarithm(((*value).into() + 1e-15) as f32)? as f64)
                    .unwrap();
        }
        let mut coefficients = self.cepstral.analyze(&smoothed, policy)?;
        for (index, value) in coefficients
            .iter_mut()
            .enumerate()
            .take(self.len() / 2 + 1)
            .skip(1)
        {
            let phase = (T::from_usize(index).unwrap() * fundamental).into() * std::f64::consts::PI;
            let sine = approximation::sine(phase as f32)? as f64;
            let cosine = approximation::cosine(
                (2.0 * std::f64::consts::PI * index as f64 * fundamental.into()) as f32,
            )? as f64;
            let sinc = if phase == 0.0 { 1.0 } else { sine / phase };
            *value = vector::scalar((*value).into() * (sinc * (1.18 - 2.0 * 0.09 * cosine)))?;
        }
        if policy == NyquistPolicy::Legacy {
            coefficients[self.len() / 2] = coefficients[self.len() / 2 - 1];
        }
        coefficients.truncate(self.bins());
        let log_spectrum = self.cepstral.reconstruct(&coefficients, policy)?;
        Ok(Envelope {
            log_spectrum,
            cepstrum: coefficients,
        })
    }
}

/// Estimate with fresh FFT plans; use Plan for repeated processing.
pub fn estimate<T: Scalar>(
    spectrum: &[T],
    fft_length: usize,
    fundamental: T,
    harmonics: Option<usize>,
) -> Result<Envelope<T>, Error> {
    Plan::new(fft_length)?.estimate(spectrum, fundamental, harmonics)
}
