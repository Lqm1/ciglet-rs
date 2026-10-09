//! Optional browser and Node.js bindings for complete DSP operations.
//!
//! Returned numeric arrays own their JavaScript storage. Native validation is
//! retained, and ordinary errors become JavaScript exceptions. A WebAssembly
//! panic abort is not recoverable through these bindings.
mod fourier;
mod interpolation;
mod phase;
mod vector;
mod window;

pub use fourier::*;
pub use interpolation::*;
pub use phase::*;
pub use vector::*;
pub use window::*;
