//! Owned mel/PLP responses with validated half-open channel supports.
use crate::{Error, approximation, frequency};
use rustfft::FftNum;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Logarithmic,
    Plp,
}

#[derive(Debug, Clone, Copy)]
pub struct MelOptions<T> {
    pub minimum_hz: T,
    pub maximum_hz: T,
    pub scale: T,
    pub minimum_width_hz: T,
}

#[derive(Debug, Clone)]
pub struct FilterBank<T> {
    bins: usize,
    nyquist_hz: T,
    responses: Vec<Vec<T>>,
    supports: Vec<Range<usize>>,
}

impl<T: FftNum + Into<f64>> FilterBank<T> {
    pub fn empty(bins: usize, nyquist_hz: T, channels: usize) -> Result<Self, Error> {
        if bins == 0
            || bins > i32::MAX as usize
            || channels > i32::MAX as usize
            || !nyquist_hz.into().is_finite()
            || nyquist_hz.into() <= 0.0
        {
            return Err(Error("invalid filterbank dimensions or Nyquist frequency"));
        }
        let mut responses = Vec::new();
        let mut supports = Vec::new();
        responses
            .try_reserve_exact(channels)
            .map_err(|_| Error("filterbank allocation failed"))?;
        supports
            .try_reserve_exact(channels)
            .map_err(|_| Error("filterbank allocation failed"))?;
        for _ in 0..channels {
            responses.push(zeros(bins)?);
            supports.push(0..bins);
        }
        Ok(Self {
            bins,
            nyquist_hz,
            responses,
            supports,
        })
    }
    pub fn bins(&self) -> usize {
        self.bins
    }
    pub fn channels(&self) -> usize {
        self.responses.len()
    }
    pub fn nyquist_hz(&self) -> T {
        self.nyquist_hz
    }
    pub fn response(&self, channel: usize) -> Option<&[T]> {
        self.responses.get(channel).map(Vec::as_slice)
    }
    pub fn support(&self, channel: usize) -> Option<Range<usize>> {
        self.supports.get(channel).cloned()
    }

    /// Replace an owned channel after validating its shape and support.
    /// Signed custom weights remain allowed, as in the original public struct.
    pub fn set_channel(
        &mut self,
        channel: usize,
        response: &[T],
        support: Range<usize>,
    ) -> Result<(), Error> {
        if channel >= self.channels()
            || response.len() != self.bins
            || support.start > support.end
            || support.end > self.bins
            || response.iter().any(|&x| !x.into().is_finite())
        {
            return Err(Error("invalid filterbank channel response or support"));
        }
        self.responses[channel].copy_from_slice(response);
        self.supports[channel] = support;
        Ok(())
    }

    pub fn mel(
        bins: usize,
        nyquist_hz: T,
        channels: usize,
        options: MelOptions<T>,
    ) -> Result<Self, Error> {
        if channels == 0
            || [
                options.minimum_hz,
                options.maximum_hz,
                options.scale,
                options.minimum_width_hz,
            ]
            .iter()
            .any(|&x| !x.into().is_finite())
            || options.minimum_hz.into() < 0.0
            || options.minimum_hz.into() > options.maximum_hz.into()
            || options.scale.into() <= 0.0
            || options.minimum_width_hz.into() < 0.0
        {
            return Err(Error(
                "invalid mel filterbank frequency bounds, scale, or width",
            ));
        }
        let mut bank = Self::empty(bins, nyquist_hz, channels)?;
        let frequencies = frequency::mel_space(options.minimum_hz, options.maximum_hz, channels)?;
        let count = scalar::<T>(bins as f64)?;
        for channel in 0..channels {
            let center_hz = frequencies[channel];
            let mut lower_hz = if channel == 0 {
                T::zero()
            } else {
                frequencies[channel - 1]
            };
            let mut upper_hz = frequencies[channel + 1];
            let lower_limit = center_hz - options.minimum_width_hz;
            if lower_hz.into() > lower_limit.into() {
                lower_hz = if lower_limit.into() > 0.0 {
                    lower_limit
                } else {
                    T::zero()
                };
            }
            let upper_limit = center_hz + options.minimum_width_hz;
            if upper_hz.into() < upper_limit.into() {
                upper_hz = upper_limit;
            }
            let coordinate = |frequency: T| -> Result<f64, Error> {
                let value: f64 = (frequency * count / nyquist_hz * options.scale).into();
                if !value.is_finite() {
                    return Err(Error("mel filterbank bin coordinate overflow"));
                }
                Ok(value)
            };
            // Clamp supports only where original C would leave the allocation.
            // The zero-centered first band safely has no ascending segment.
            let upper = coordinate(upper_hz)?.ceil().clamp(1.0, bins as f64) as usize;
            let center = coordinate(center_hz)?
                .round()
                .clamp(0.0, (upper - 1) as f64) as usize;
            let lower = if channel == 0 {
                0
            } else {
                coordinate(lower_hz)?
                    .floor()
                    .clamp(0.0, center.saturating_sub(1) as f64) as usize
            };
            bank.supports[channel] = lower..upper;
            for bin in lower..center {
                bank.responses[channel][bin] =
                    scalar::<T>((bin - lower + 1) as f64)? / scalar::<T>((center - lower) as f64)?;
            }
            for bin in center..upper {
                let ratio: f64 = (scalar::<T>((bin - center) as f64)?
                    / scalar::<T>((upper - center) as f64)?)
                .into();
                bank.responses[channel][bin] = scalar(1.0 - ratio)?;
            }
        }
        Ok(bank)
    }

    /// Original create_melfilterbank defaults: unit scale and 400-Hz width.
    pub fn mel_default(
        bins: usize,
        nyquist_hz: T,
        channels: usize,
        minimum_hz: T,
        maximum_hz: T,
    ) -> Result<Self, Error> {
        Self::mel(
            bins,
            nyquist_hz,
            channels,
            MelOptions {
                minimum_hz,
                maximum_hz,
                scale: T::one(),
                minimum_width_hz: scalar(400.0)?,
            },
        )
    }

    pub fn plp(bins: usize, nyquist_hz: T, channels: usize) -> Result<Self, Error> {
        let mut bank = Self::empty(bins, nyquist_hz, channels)?;
        let count = scalar::<T>(bins as f64)?;
        for channel in 0..channels {
            let center = scalar::<T>(channel as f64)?;
            let weight = frequency::equal_loudness(frequency::bark_to_frequency(center)?)?;
            for bin in 0..bins {
                let hz = scalar::<T>(bin as f64)? / count * nyquist_hz;
                let distance: f64 = (center - frequency::frequency_to_bark(hz)?).into();
                let mask = if !(-1.3..=2.5).contains(&distance) {
                    T::zero()
                } else if distance < -0.5 {
                    scalar(approximation::power(10.0, (2.5 * (distance + 0.5)) as f32)? as f64)?
                } else if distance <= 0.5 {
                    T::one()
                } else {
                    scalar(approximation::power(10.0, (0.5 - distance) as f32)? as f64)?
                };
                bank.responses[channel][bin] = mask * weight;
            }
        }
        Ok(bank)
    }

    pub fn apply(&self, spectrum: &[T], compression: Compression) -> Result<Vec<T>, Error> {
        if spectrum.len() != self.bins || spectrum.iter().any(|&x| !x.into().is_finite()) {
            return Err(Error(
                "filterbank spectrum shape differs or contains nonfinite values",
            ));
        }
        let mut output = zeros(self.channels())?;
        self.apply_validated_into(spectrum, compression, &mut output)?;
        Ok(output)
    }

    fn apply_validated_into(
        &self,
        spectrum: &[T],
        compression: Compression,
        output: &mut [T],
    ) -> Result<(), Error> {
        for (channel, value) in output.iter_mut().enumerate() {
            let sum = self.supports[channel].clone().fold(T::zero(), |sum, bin| {
                sum + self.responses[channel][bin] * spectrum[bin]
            });
            *value = compress(sum, compression)?;
        }
        Ok(())
    }

    /// Apply to a flattened frame-major spectrogram with the bank's bin count.
    pub fn apply_spectrogram(
        &self,
        spectrum: &[T],
        compression: Compression,
    ) -> Result<Vec<T>, Error> {
        if !spectrum.len().is_multiple_of(self.bins)
            || spectrum.iter().any(|&x| !x.into().is_finite())
        {
            return Err(Error("filterbank spectrogram shape differs"));
        }
        let size = (spectrum.len() / self.bins)
            .checked_mul(self.channels())
            .ok_or(Error("filterbank output size overflow"))?;
        let mut output = zeros(size)?;
        if self.channels() == 0 {
            return Ok(output);
        }
        for (frame, destination) in spectrum
            .chunks_exact(self.bins)
            .zip(output.chunks_exact_mut(self.channels()))
        {
            self.apply_validated_into(frame, compression, destination)?;
        }
        Ok(output)
    }
}
fn scalar<T: FftNum + Into<f64>>(value: f64) -> Result<T, Error> {
    let result = T::from_f64(value).ok_or(Error("filterbank scalar cannot be represented"))?;
    if !result.into().is_finite() {
        return Err(Error("filterbank scalar is not finite"));
    }
    Ok(result)
}
fn zeros<T: FftNum>(length: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("filterbank allocation failed"))?;
    values.resize(length, T::zero());
    Ok(values)
}
fn compress<T: FftNum + Into<f64>>(sum: T, compression: Compression) -> Result<T, Error> {
    match compression {
        Compression::Logarithmic => {
            scalar(approximation::logarithm((sum.into() + 1e-15) as f32)? as f64)
        }
        Compression::Plp => scalar(sum.into().powf(0.33)),
    }
}
