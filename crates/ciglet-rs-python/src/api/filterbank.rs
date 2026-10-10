use crate::filterbank::{Compression, FilterBank, MelOptions};
use pyo3::PyErr as BindingError;
/// Native filterbank output compression.
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum FilterBankCompression {
    Logarithmic,
    Plp,
}
impl From<FilterBankCompression> for Compression {
    fn from(value: FilterBankCompression) -> Self {
        match value {
            FilterBankCompression::Logarithmic => Self::Logarithmic,
            FilterBankCompression::Plp => Self::Plp,
        }
    }
}
fn compression(value: u32) -> Result<Compression, BindingError> {
    match value {
        0 => Ok(FilterBankCompression::Logarithmic.into()),
        1 => Ok(FilterBankCompression::Plp.into()),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid filterbank compression",
        )),
    }
}
/// Complete mutable Mel configuration in the selected scalar precision.
#[derive(Clone, Copy)]
pub struct MelOptionsF32 {
    pub minimum_hz: f32,
    pub maximum_hz: f32,
    pub scale: f32,
    pub minimum_width_hz: f32,
}
impl MelOptionsF32 {
    pub fn new(minimum_hz: f32, maximum_hz: f32, scale: f32, minimum_width_hz: f32) -> Self {
        Self {
            minimum_hz,
            maximum_hz,
            scale,
            minimum_width_hz,
        }
    }
}
/// Independently owned responses, half-open supports and Nyquist metadata.
pub struct FilterBankF32 {
    bank: FilterBank<f32>,
}
impl FilterBankF32 {
    /// Create a zero-filled custom bank, including empty dimensions.
    pub fn new(bins: usize, nyquist_hz: f32, channels: usize) -> Result<Self, BindingError> {
        FilterBank::empty(bins, nyquist_hz, channels)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn mel(
        bins: usize,
        nyquist_hz: f32,
        channels: usize,
        options: &MelOptionsF32,
    ) -> Result<Self, BindingError> {
        FilterBank::mel(
            bins,
            nyquist_hz,
            channels,
            MelOptions {
                minimum_hz: options.minimum_hz,
                maximum_hz: options.maximum_hz,
                scale: options.scale,
                minimum_width_hz: options.minimum_width_hz,
            },
        )
        .map(|bank| Self { bank })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn mel_default(
        bins: usize,
        nyquist_hz: f32,
        channels: usize,
        minimum_hz: f32,
        maximum_hz: f32,
    ) -> Result<Self, BindingError> {
        FilterBank::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn plp(bins: usize, nyquist_hz: f32, channels: usize) -> Result<Self, BindingError> {
        FilterBank::plp(bins, nyquist_hz, channels)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            bank: self.bank.clone(),
        }
    }
    pub fn bins(&self) -> usize {
        self.bank.bins()
    }
    pub fn channels(&self) -> usize {
        self.bank.channels()
    }
    pub fn nyquist_hz(&self) -> f32 {
        self.bank.nyquist_hz()
    }
    /// Metadata changes do not change responses or application results.
    pub fn set_nyquist_hz(&mut self, value: f32) {
        self.bank.set_nyquist_hz(value);
    }
    /// Copy a complete response, including inactive scalar payloads.
    pub fn response(&self, channel: usize) -> Option<Vec<f32>> {
        self.bank.response(channel).map(<[f32]>::to_vec)
    }
    /// Return the half-open start and end, or undefined for a missing channel.
    pub fn support(&self, channel: usize) -> Option<Vec<u32>> {
        self.bank
            .support(channel)
            .map(|range| vec![range.start as u32, range.end as u32])
    }
    /// Atomically replace a complete response and its half-open support.
    pub fn set_channel(
        &mut self,
        channel: usize,
        response: &[f32],
        start: usize,
        end: usize,
    ) -> Result<(), BindingError> {
        self.bank
            .set_channel(channel, response, start..end)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn apply(&self, spectrum: &[f32], compression_kind: u32) -> Result<Vec<f32>, BindingError> {
        self.bank
            .apply(spectrum, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn apply_spectrogram(
        &self,
        spectra: &[f32],
        compression_kind: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.bank
            .apply_spectrogram(spectra, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    /// Explicit frame counts retain zero-bin and zero-channel behavior.
    pub fn apply_frames(
        &self,
        spectra: &[f32],
        frames: usize,
        compression_kind: u32,
    ) -> Result<Vec<f32>, BindingError> {
        self.bank
            .apply_frames(spectra, frames, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
/// Complete mutable Mel configuration in the selected scalar precision.
#[derive(Clone, Copy)]
pub struct MelOptionsF64 {
    pub minimum_hz: f64,
    pub maximum_hz: f64,
    pub scale: f64,
    pub minimum_width_hz: f64,
}
impl MelOptionsF64 {
    pub fn new(minimum_hz: f64, maximum_hz: f64, scale: f64, minimum_width_hz: f64) -> Self {
        Self {
            minimum_hz,
            maximum_hz,
            scale,
            minimum_width_hz,
        }
    }
}
/// Independently owned responses, half-open supports and Nyquist metadata.
pub struct FilterBankF64 {
    bank: FilterBank<f64>,
}
impl FilterBankF64 {
    /// Create a zero-filled custom bank, including empty dimensions.
    pub fn new(bins: usize, nyquist_hz: f64, channels: usize) -> Result<Self, BindingError> {
        FilterBank::empty(bins, nyquist_hz, channels)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn mel(
        bins: usize,
        nyquist_hz: f64,
        channels: usize,
        options: &MelOptionsF64,
    ) -> Result<Self, BindingError> {
        FilterBank::mel(
            bins,
            nyquist_hz,
            channels,
            MelOptions {
                minimum_hz: options.minimum_hz,
                maximum_hz: options.maximum_hz,
                scale: options.scale,
                minimum_width_hz: options.minimum_width_hz,
            },
        )
        .map(|bank| Self { bank })
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn mel_default(
        bins: usize,
        nyquist_hz: f64,
        channels: usize,
        minimum_hz: f64,
        maximum_hz: f64,
    ) -> Result<Self, BindingError> {
        FilterBank::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn plp(bins: usize, nyquist_hz: f64, channels: usize) -> Result<Self, BindingError> {
        FilterBank::plp(bins, nyquist_hz, channels)
            .map(|bank| Self { bank })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            bank: self.bank.clone(),
        }
    }
    pub fn bins(&self) -> usize {
        self.bank.bins()
    }
    pub fn channels(&self) -> usize {
        self.bank.channels()
    }
    pub fn nyquist_hz(&self) -> f64 {
        self.bank.nyquist_hz()
    }
    /// Metadata changes do not change responses or application results.
    pub fn set_nyquist_hz(&mut self, value: f64) {
        self.bank.set_nyquist_hz(value);
    }
    /// Copy a complete response, including inactive scalar payloads.
    pub fn response(&self, channel: usize) -> Option<Vec<f64>> {
        self.bank.response(channel).map(<[f64]>::to_vec)
    }
    /// Return the half-open start and end, or undefined for a missing channel.
    pub fn support(&self, channel: usize) -> Option<Vec<u32>> {
        self.bank
            .support(channel)
            .map(|range| vec![range.start as u32, range.end as u32])
    }
    /// Atomically replace a complete response and its half-open support.
    pub fn set_channel(
        &mut self,
        channel: usize,
        response: &[f64],
        start: usize,
        end: usize,
    ) -> Result<(), BindingError> {
        self.bank
            .set_channel(channel, response, start..end)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn apply(&self, spectrum: &[f64], compression_kind: u32) -> Result<Vec<f64>, BindingError> {
        self.bank
            .apply(spectrum, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn apply_spectrogram(
        &self,
        spectra: &[f64],
        compression_kind: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.bank
            .apply_spectrogram(spectra, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    /// Explicit frame counts retain zero-bin and zero-channel behavior.
    pub fn apply_frames(
        &self,
        spectra: &[f64],
        frames: usize,
        compression_kind: u32,
    ) -> Result<Vec<f64>, BindingError> {
        self.bank
            .apply_frames(spectra, frames, compression(compression_kind)?)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
}
