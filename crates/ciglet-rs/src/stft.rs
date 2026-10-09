//! Centered short-time Fourier analysis with reusable transform storage.
use crate::{
    Error, approximation,
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

/// Nyquist reconstruction policy. Preserve fixes the upstream overwrite;
/// Legacy reproduces the preceding-bin substitution for existing workflows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NyquistPolicy {
    #[default]
    Preserve,
    Legacy,
}

#[derive(Debug, Clone, Copy)]
pub struct SynthesisOptions<T> {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: T,
    pub nyquist: NyquistPolicy,
}

impl<T: FftNum + Into<f64>> StftPlan<T> {
    /// Reconstruct half-spectrum frames using upstream level-two polar
    /// conversion, normalized inverse FFT, Hann endpoint fades and overlap-add.
    /// Output length is hop * frame count + offset. Input arrays are frame-major.
    pub fn synthesize(
        &mut self,
        magnitude: &[T],
        phase: &[T],
        options: SynthesisOptions<T>,
    ) -> Result<Vec<T>, Error> {
        let length = self.fft.len();
        let bins = length / 2 + 1;
        if magnitude.len() != phase.len() || !magnitude.len().is_multiple_of(bins) {
            return Err(Error("inverse STFT spectral shapes differ"));
        }
        if options.hop == 0
            || options.hop > i32::MAX as usize
            || options.fade_samples > length
            || (options.nyquist == NyquistPolicy::Legacy && length < 2)
        {
            return Err(Error(
                "invalid inverse STFT spacing, fade, or legacy transform length",
            ));
        }
        let normalization: f64 = options.normalization.into();
        if !normalization.is_finite() || normalization == 0.0 {
            return Err(Error(
                "inverse STFT normalization must be finite and nonzero",
            ));
        }
        if magnitude
            .iter()
            .chain(phase)
            .any(|&value| !value.into().is_finite())
        {
            return Err(Error("inverse STFT spectra must be finite"));
        }
        let frames = magnitude.len() / bins;
        let span = options
            .hop
            .checked_mul(frames)
            .ok_or(Error("inverse STFT output size overflow"))?;
        let output_length = i64::try_from(span)
            .ok()
            .and_then(|n| n.checked_add(i64::from(options.offset)))
            .filter(|&n| (0..=i64::from(i32::MAX)).contains(&n))
            .ok_or(Error("inverse STFT output length exceeds the legacy range"))?
            as usize;
        let mut output = filled(output_length, T::zero())?;
        let fade = Window::Hann.generate_with::<T>(
            options
                .fade_samples
                .checked_mul(2)
                .ok_or(Error("inverse STFT fade length overflow"))?,
            Precision::Accurate,
        )?;
        for frame in 0..frames {
            for bin in 0..bins {
                let index = frame * bins + bin;
                let angle = phase[index].into() as f32;
                let cosine = T::from_f32(approximation::cosine(angle)?)
                    .ok_or(Error("inverse STFT cosine cannot be represented"))?;
                let sine = T::from_f32(approximation::sine(angle)?)
                    .ok_or(Error("inverse STFT sine cannot be represented"))?;
                self.spectrum[bin] =
                    Complex::new(magnitude[index] * cosine, magnitude[index] * sine);
            }
            match options.nyquist {
                NyquistPolicy::Legacy => self.spectrum[length / 2] = self.spectrum[length / 2 - 1],
                NyquistPolicy::Preserve => {
                    self.spectrum[0].im = T::zero();
                    if length > 1 {
                        self.spectrum[length / 2].im = T::zero();
                    }
                }
            }
            for bin in bins..length {
                self.spectrum[bin] = self.spectrum[length - bin].conj();
            }
            self.fft.process(&mut self.spectrum, Direction::Inverse)?;
            self.spectrum.rotate_right(length / 2);
            for (index, value) in self.frame.iter_mut().enumerate() {
                *value = self.spectrum[index].re;
            }
            for (index, &weight) in fade[..options.fade_samples].iter().enumerate() {
                self.frame[index] = self.frame[index] * weight;
                self.frame[length - index - 1] = self.frame[length - index - 1] * weight;
            }
            let center = (frame * options.hop) as i64 + i64::from(options.offset);
            for (index, &sample) in self.frame.iter().enumerate() {
                let position = center + index as i64 - (length / 2) as i64;
                if let Ok(position) = usize::try_from(position)
                    && let Some(value) = output.get_mut(position)
                {
                    *value = *value + sample / options.normalization;
                }
            }
        }
        Ok(output)
    }
}

/// Parameters of the original Blackman-window STFT convenience functions.
#[derive(Debug, Clone, Copy)]
pub struct FrameGrid {
    pub hop: usize,
    pub frames: usize,
    pub window_factor: usize,
    pub zero_padding_factor: usize,
}
impl FrameGrid {
    fn lengths(self) -> Result<(usize, usize), Error> {
        if self.hop == 0
            || self.window_factor == 0
            || self.zero_padding_factor == 0
            || self.frames > i32::MAX as usize
        {
            return Err(Error(
                "STFT grid factors must be positive and frame count bounded",
            ));
        }
        let window = self
            .hop
            .checked_mul(self.window_factor)
            .ok_or(Error("STFT window size overflow"))?;
        let fft = window
            .checked_mul(self.zero_padding_factor)
            .ok_or(Error("STFT transform size overflow"))?;
        Ok((window, fft))
    }
}

/// Equivalent to stft/stft_2, selecting their level with precision.
pub fn analyze_regular<T: FftNum + Into<f64>>(
    signal: &[T],
    grid: FrameGrid,
    precision: Precision,
    mut options: AnalysisOptions,
) -> Result<Analysis<T>, Error> {
    let (window, fft) = grid.lengths()?;
    let mut plan = StftPlan::new(fft, Window::Blackman, precision)?;
    let mut centers = filled(grid.frames, 0i32)?;
    for (frame, center) in centers.iter_mut().enumerate() {
        *center = frame
            .checked_mul(grid.hop)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or(Error("STFT center exceeds the legacy range"))?;
    }
    let lengths = filled(grid.frames, window)?;
    options.normalization_hop = Some(grid.hop);
    plan.analyze(signal, &centers, &lengths, options)
}

/// Equivalent to istft: zero offset and a 32-sample endpoint fade.
/// Choose Legacy explicitly to reproduce the original Nyquist overwrite.
pub fn synthesize_regular<T: FftNum + Into<f64>>(
    magnitude: &[T],
    phase: &[T],
    grid: FrameGrid,
    normalization: T,
    nyquist: NyquistPolicy,
) -> Result<Vec<T>, Error> {
    let (_, fft) = grid.lengths()?;
    let bins = fft / 2 + 1;
    if grid.frames.checked_mul(bins) != Some(magnitude.len()) {
        return Err(Error("inverse STFT grid differs from spectrum"));
    }
    StftPlan::new(fft, Window::Blackman, Precision::Accurate)?.synthesize(
        magnitude,
        phase,
        SynthesisOptions {
            hop: grid.hop,
            offset: 0,
            fade_samples: 32,
            normalization,
            nyquist,
        },
    )
}
