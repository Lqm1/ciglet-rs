//! Optional browser and Node.js bindings for complete DSP operations.
//!
//! Returned numeric arrays own their JavaScript storage. Native validation is
//! retained, and ordinary errors become JavaScript exceptions. A WebAssembly
//! panic abort is not recoverable through these bindings.
mod approximation;
mod filterbank;
mod filtering;
mod fourier;
mod frame_extrema;
mod frequency_distance;
mod hyperbolic;
mod interpolation;
mod ordering;
mod phase;
mod resampling;
mod smoothing;
mod statistics;
mod trigonometric;
mod vector;
mod window;

pub use approximation::*;
pub use filterbank::*;
pub use filtering::*;
pub use fourier::*;
pub use frame_extrema::*;
pub use frequency_distance::*;
pub use hyperbolic::*;
pub use interpolation::*;
pub use ordering::*;
pub use phase::*;
pub use resampling::*;
pub use smoothing::*;
pub use statistics::*;
pub use trigonometric::*;
pub use vector::*;
pub use window::*;
