//! C ABI for ciglet-rs.
#![deny(unsafe_op_in_unsafe_fn)]
use ciglet_rs::*;
pub mod c_api;
pub use c_api::*;
