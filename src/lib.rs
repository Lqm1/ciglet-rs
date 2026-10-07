//! Signal-processing operations reimplemented from ciglet.
//! The complete public upstream API is being ported in stages.
#![forbid(unsafe_code)]
pub mod approximation;
pub mod cepstrum;
pub mod chirp_z;
pub mod cosine;
pub mod extrema;
pub mod filterbank;
pub mod filtering;
pub mod fourier;
pub mod frame;
pub mod frequency;
pub mod interpolation;
pub mod kalman;
pub mod median;
pub mod resampling;
pub mod root;
pub mod smoothing;
pub mod statistics;
pub mod stft;
pub mod synthesis;
pub mod wave;
pub mod wave_write;
pub mod window;

/// Invalid DSP input shape or parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(pub &'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for Error {}
