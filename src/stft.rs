//! Centered short-time Fourier analysis with reusable transform storage.
use crate::{
    Error,
    fourier::{Complex, Direction, FftPlan},
    window::{Precision, Window},
};
use rustfft::FftNum;

#[derive(Debug, Clone, Copy)]
pub struct AnalysisOptions {
    pub subtract_mean: bool,
    pub magnitude: bool,
    pub phase: bool,
    /// Positive spacing used to sum the first window's overlap weights.
    /// Explicit spacing also supports a single frame safely.
    pub normalization_hop: Option<usize>,
}
impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            subtract_mean: false,
            magnitude: true,
            phase: true,
            normalization_hop: None,
        }
    }
}

/// Spectral arrays use frame-major ordering, with bins 0 through FFT length/2.
#[derive(Debug, Clone)]
pub struct Analysis<T> {
    pub frames: usize,
    pub bins: usize,
    pub magnitude: Option<Vec<T>>,
    pub phase: Option<Vec<T>>,
    pub window_weight: Option<T>,
    pub normalization: Option<T>,
}

/// Caches the FFT plan, frame storage, and the most recently used window.
#[derive(Debug)]
pub struct StftPlan<T: FftNum> {
    fft: FftPlan<T>,
    window: Window,
    precision: Precision,
    weights: Vec<T>,
    frame: Vec<T>,
    spectrum: Vec<Complex<T>>,
}

impl<T: FftNum + Into<f64>> StftPlan<T> {
    pub fn new(fft_length: usize, window: Window, precision: Precision) -> Result<Self, Error> {
        Ok(Self {
            fft: FftPlan::new(fft_length)?,
            window,
            precision,
            weights: Vec::new(),
            frame: filled(fft_length, T::zero())?,
            spectrum: filled(fft_length, Complex::new(T::zero(), T::zero()))?,
        })
    }

    /// Analyze zero-padded centered frames. Every window must be nonempty and
    /// fit the FFT. Unlike C, variable windows always use their own weights,
    /// and odd windows retain their final sample.
    pub fn analyze(
        &mut self,
        signal: &[T],
        centers: &[i32],
        window_lengths: &[usize],
        options: AnalysisOptions,
    ) -> Result<Analysis<T>, Error> {
        if centers.len() != window_lengths.len()
            || centers.len() > i32::MAX as usize
            || signal.len() > i32::MAX as usize
        {
            return Err(Error("STFT frame or signal shape exceeds the legacy range"));
        }
        if window_lengths.iter().any(|&n| n == 0 || n > self.fft.len()) {
            return Err(Error("STFT windows must be nonempty and fit the transform"));
        }
        if options.normalization_hop == Some(0) {
            return Err(Error("STFT normalization hop must be positive"));
        }
        if signal.iter().any(|&sample| !sample.into().is_finite()) {
            return Err(Error("STFT signal must be finite"));
        }
        let bins = self.fft.len() / 2 + 1;
        let count = centers
            .len()
            .checked_mul(bins)
            .ok_or(Error("STFT output size overflow"))?;
        let mut result = Analysis {
            frames: centers.len(),
            bins,
            magnitude: if options.magnitude {
                Some(filled(count, T::zero())?)
            } else {
                None
            },
            phase: if options.phase {
                Some(filled(count, T::zero())?)
            } else {
                None
            },
            window_weight: None,
            normalization: None,
        };
        for (frame_index, (&center, &length)) in centers.iter().zip(window_lengths).enumerate() {
            if self.weights.len() != length {
                self.weights = self.window.generate_with(length, self.precision)?;
            }
            if frame_index == 0 {
                result.window_weight = Some(self.weights.iter().fold(T::zero(), |sum, &w| sum + w));
                result.normalization = options.normalization_hop.map(|hop| {
                    self.weights
                        .iter()
                        .step_by(hop)
                        .fold(T::zero(), |sum, &w| sum + w)
                });
            }
            let origin = i64::from(center) - (length / 2) as i64;
            let mut sum = T::zero();
            for (index, value) in self.frame[..length].iter_mut().enumerate() {
                let position = origin + index as i64;
                *value = usize::try_from(position)
                    .ok()
                    .and_then(|i| signal.get(i))
                    .copied()
                    .unwrap_or(T::zero());
                sum = sum + *value;
            }
            let mean = if options.subtract_mean {
                sum / T::from_usize(length)
                    .ok_or(Error("STFT window length cannot be represented"))?
            } else {
                T::zero()
            };
            self.spectrum.fill(Complex::new(T::zero(), T::zero()));
            for index in 0..length {
                let destination = if index < length / 2 {
                    self.fft.len() - length / 2 + index
                } else {
                    index - length / 2
                };
                self.spectrum[destination].re = (self.frame[index] - mean) * self.weights[index];
            }
            self.fft.process(&mut self.spectrum, Direction::Forward)?;
            for (bin, value) in self.spectrum[..bins].iter().enumerate() {
                let output_index = frame_index * bins + bin;
                if let Some(magnitudes) = &mut result.magnitude {
                    // C squares/adds in FP_TYPE before evaluating double sqrt.
                    let square: f64 = (value.re * value.re + value.im * value.im).into();
                    magnitudes[output_index] = T::from_f64(square.sqrt())
                        .ok_or(Error("STFT magnitude cannot be represented"))?;
                }
                if let Some(phases) = &mut result.phase {
                    phases[output_index] = T::from_f64(value.im.into().atan2(value.re.into()))
                        .ok_or(Error("STFT phase cannot be represented"))?;
                }
            }
        }
        Ok(result)
    }
}

fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("STFT allocation failed"))?;
    values.resize(length, value);
    Ok(values)
}
