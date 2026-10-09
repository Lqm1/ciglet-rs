//! Optional browser and Node.js bindings for complete DSP operations.
//!
//! Returned numeric arrays own their JavaScript storage. Native validation is
//! retained, and ordinary errors become JavaScript exceptions. A WebAssembly
//! panic abort is not recoverable through these bindings.
mod approximation;
mod cepstrum;
mod complex;
mod filterbank;
mod filtering;
mod fourier;
mod frame_extrema;
mod frequency_distance;
mod hyperbolic;
mod interpolation;
mod kalman;
mod lanes;
mod ordering;
mod phase;
mod resampling;
mod root;
mod sinc;
mod smoothing;
mod special;
mod spectral_peak;
mod statistics;
mod synthesis;
mod trigonometric;
mod vector;
mod window;

pub use approximation::*;
pub use cepstrum::*;
pub use complex::*;
pub use filterbank::*;
pub use filtering::*;
pub use fourier::*;
pub use frame_extrema::*;
pub use frequency_distance::*;
pub use hyperbolic::*;
pub use interpolation::*;
pub use kalman::*;
pub use lanes::*;
pub use ordering::*;
pub use phase::*;
pub use resampling::*;
pub use root::*;
pub use sinc::*;
pub use smoothing::*;
pub use special::*;
pub use spectral_peak::*;
pub use statistics::*;
pub use synthesis::*;
pub use trigonometric::*;
pub use vector::*;
pub use window::*;
