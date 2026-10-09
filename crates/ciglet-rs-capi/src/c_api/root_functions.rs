//! Synchronous C callbacks for native bracketed scalar root finding.
use super::scalar_value_result;
use crate::root;
use std::ffi::c_void;

/// Find a root using native tolerance, interpolation and midpoint fallback.
/// Callback and context are used synchronously and never retained. Null callback
/// returns one; native invalid bounds/nonfinite callback values return three.
/// Output is committed only on success. The callback may mutate its own context
/// but must leave output storage untouched. Native panic/abort limits apply;
/// catch_unwind cannot protect against callback unwinding across this C boundary.
///
/// # Safety
/// A present callback must have the declared C ABI and scalar precision and be
/// callable for every argument in the bracket for this entire call. Context must
/// satisfy its callback's validity/lifetime/access requirements; null is allowed
/// if that callback supports it. Callback must not unwind, throw or panic across
/// its C boundary. Output is aligned exclusively writable single-allocation
/// storage for one scalar, independent of any storage the callback mutates.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_root_find_f32(
    callback: Option<unsafe extern "C" fn(f32, *mut c_void) -> f32>,
    context: *mut c_void,
    start: f32,
    end: f32,
    output: *mut f32,
) -> u32 {
    let Some(callback) = callback else {
        return 1;
    };
    // SAFETY: Forward exclusive output and synchronous valid typed callback. The
    // caller guarantees the opaque context and non-unwinding callback contract.
    unsafe {
        scalar_value_result(output, || {
            root::find(|value| callback(value, context), start, end)
        })
    }
}

/// Find a root using native tolerance, interpolation and midpoint fallback.
/// Callback and context are used synchronously and never retained. Null callback
/// returns one; native invalid bounds/nonfinite callback values return three.
/// Output is committed only on success. The callback may mutate its own context
/// but must leave output storage untouched. Native panic/abort limits apply;
/// catch_unwind cannot protect against callback unwinding across this C boundary.
///
/// # Safety
/// A present callback must have the declared C ABI and scalar precision and be
/// callable for every argument in the bracket for this entire call. Context must
/// satisfy its callback's validity/lifetime/access requirements; null is allowed
/// if that callback supports it. Callback must not unwind, throw or panic across
/// its C boundary. Output is aligned exclusively writable single-allocation
/// storage for one scalar, independent of any storage the callback mutates.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_root_find_f64(
    callback: Option<unsafe extern "C" fn(f64, *mut c_void) -> f64>,
    context: *mut c_void,
    start: f64,
    end: f64,
    output: *mut f64,
) -> u32 {
    let Some(callback) = callback else {
        return 1;
    };
    // SAFETY: Forward exclusive output and synchronous valid typed callback. The
    // caller guarantees the opaque context and non-unwinding callback contract.
    unsafe {
        scalar_value_result(output, || {
            root::find(|value| callback(value, context), start, end)
        })
    }
}
