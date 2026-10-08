//! Caller-owned windows with checked modes and custom trigonometric callbacks.
use super::windows;
use std::ffi::c_void;

/// Generate built-in windows. Precision: zero accurate, one approximate.
/// Blackman retains accurate cosine in both modes for legacy compatibility.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_f32(
    window: u32,
    precision: u32,
    output: *mut f32,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output contract.
    unsafe { windows::builtin(window, precision, output, length) }
}

/// Generate custom f32 weights using a f32 callback result.
/// MLT sine requests sine; other tapered kinds request cosine.
/// Callback returns zero after writing its result; any other code gives status
/// three. Nonfinite results are rejected. Calls are synchronous and serial on
/// the invoking thread. Callback/context are never retained. Null callback is
/// accepted for Boxcar, empty and singleton windows, which make no calls.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
/// Callback must use the exact C signature, return normally without unwinding
/// or nonlocal jumps, and write the provided result on success. It must not
/// retain that result pointer or access the final output during generation.
/// Context is opaque and may be null if the callback permits it; any referenced
/// context must remain valid with exclusive access required by the callback.
/// Output/context must have no conflicting concurrent accesses. Context changes
/// made by callbacks are not rolled back after a later failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_custom_f32_f32(
    window: u32,
    callback: Option<unsafe extern "C" fn(f64, *mut c_void, *mut f32) -> u32>,
    context: *mut c_void,
    output: *mut f32,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output, callback and context contracts.
    unsafe { windows::custom(window, callback, context, output, length) }
}

/// Generate custom f32 weights using a f64 callback result.
/// MLT sine requests sine; other tapered kinds request cosine.
/// Callback returns zero after writing its result; any other code gives status
/// three. Nonfinite results are rejected. Calls are synchronous and serial on
/// the invoking thread. Callback/context are never retained. Null callback is
/// accepted for Boxcar, empty and singleton windows, which make no calls.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
/// Callback must use the exact C signature, return normally without unwinding
/// or nonlocal jumps, and write the provided result on success. It must not
/// retain that result pointer or access the final output during generation.
/// Context is opaque and may be null if the callback permits it; any referenced
/// context must remain valid with exclusive access required by the callback.
/// Output/context must have no conflicting concurrent accesses. Context changes
/// made by callbacks are not rolled back after a later failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_custom_f32_f64(
    window: u32,
    callback: Option<unsafe extern "C" fn(f64, *mut c_void, *mut f64) -> u32>,
    context: *mut c_void,
    output: *mut f32,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output, callback and context contracts.
    unsafe { windows::custom(window, callback, context, output, length) }
}

/// Generate built-in windows. Precision: zero accurate, one approximate.
/// Blackman retains accurate cosine in both modes for legacy compatibility.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_f64(
    window: u32,
    precision: u32,
    output: *mut f64,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output contract.
    unsafe { windows::builtin(window, precision, output, length) }
}

/// Generate custom f64 weights using a f32 callback result.
/// MLT sine requests sine; other tapered kinds request cosine.
/// Callback returns zero after writing its result; any other code gives status
/// three. Nonfinite results are rejected. Calls are synchronous and serial on
/// the invoking thread. Callback/context are never retained. Null callback is
/// accepted for Boxcar, empty and singleton windows, which make no calls.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
/// Callback must use the exact C signature, return normally without unwinding
/// or nonlocal jumps, and write the provided result on success. It must not
/// retain that result pointer or access the final output during generation.
/// Context is opaque and may be null if the callback permits it; any referenced
/// context must remain valid with exclusive access required by the callback.
/// Output/context must have no conflicting concurrent accesses. Context changes
/// made by callbacks are not rolled back after a later failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_custom_f64_f32(
    window: u32,
    callback: Option<unsafe extern "C" fn(f64, *mut c_void, *mut f32) -> u32>,
    context: *mut c_void,
    output: *mut f64,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output, callback and context contracts.
    unsafe { windows::custom(window, callback, context, output, length) }
}

/// Generate custom f64 weights using a f64 callback result.
/// MLT sine requests sine; other tapered kinds request cosine.
/// Callback returns zero after writing its result; any other code gives status
/// three. Nonfinite results are rejected. Calls are synchronous and serial on
/// the invoking thread. Callback/context are never retained. Null callback is
/// accepted for Boxcar, empty and singleton windows, which make no calls.
/// Window kinds: 0 Boxcar, 1 Hann, 2 Hamming, 3 Blackman,
/// 4 Blackman-Harris, 5 Nuttall98, 6 MLT sine.
/// Empty output accepts null; singleton output is unity. Unknown codes return
/// status three. Errors leave output unchanged.
///
/// # Safety
/// Nonempty output must be aligned and exclusively writable for length values.
/// Callback must use the exact C signature, return normally without unwinding
/// or nonlocal jumps, and write the provided result on success. It must not
/// retain that result pointer or access the final output during generation.
/// Context is opaque and may be null if the callback permits it; any referenced
/// context must remain valid with exclusive access required by the callback.
/// Output/context must have no conflicting concurrent accesses. Context changes
/// made by callbacks are not rolled back after a later failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_window_custom_f64_f64(
    window: u32,
    callback: Option<unsafe extern "C" fn(f64, *mut c_void, *mut f64) -> u32>,
    context: *mut c_void,
    output: *mut f64,
    length: usize,
) -> u32 {
    // SAFETY: Forward the documented output, callback and context contracts.
    unsafe { windows::custom(window, callback, context, output, length) }
}
