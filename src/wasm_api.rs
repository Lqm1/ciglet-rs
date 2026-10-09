//! Optional browser and Node.js bindings for complete DSP operations.
//!
//! Returned numeric arrays own their JavaScript storage. Native validation is
//! retained, and ordinary errors become JavaScript exceptions. A WebAssembly
//! panic abort is not recoverable through these bindings.
mod filtering;
mod fourier;
mod interpolation;
mod ordering;
mod phase;
mod resampling;
mod smoothing;
mod statistics;
mod vector;
mod window;

pub use filtering::*;
pub use fourier::*;
pub use interpolation::*;
pub use ordering::*;
pub use phase::*;
pub use resampling::*;
pub use smoothing::*;
pub use statistics::*;
pub use vector::*;
pub use window::*;
