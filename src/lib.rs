//! Signal-processing operations reimplemented from ciglet.
//! The complete public upstream API is being ported in stages.
#![forbid(unsafe_code)]
pub mod approximation;
pub mod cosine;
pub mod fourier;
pub mod frame;
pub mod statistics;
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
