//! Window selectors and synchronous C callback adapters.
use super::generated_result;
use crate::{
    Error,
    statistics::Scalar,
    window::{Precision, Window},
};
use std::ffi::c_void;

fn kind(code: u32) -> Result<Window, Error> {
    match code {
        0 => Ok(Window::Boxcar),
        1 => Ok(Window::Hann),
        2 => Ok(Window::Hamming),
        3 => Ok(Window::Blackman),
        4 => Ok(Window::BlackmanHarris),
        5 => Ok(Window::Nuttall98),
        6 => Ok(Window::MltSine),
        _ => Err(Error("unknown window kind")),
    }
}
fn precision(code: u32) -> Result<Precision, Error> {
    match code {
        0 => Ok(Precision::Accurate),
        1 => Ok(Precision::Approximate),
        _ => Err(Error("unknown window precision")),
    }
}

pub(super) unsafe fn builtin<T: Scalar>(
    window: u32,
    mode: u32,
    output: *mut T,
    length: usize,
) -> u32 {
    // SAFETY: Forward the caller's exclusively writable output contract.
    unsafe {
        generated_result(output, length, || {
            kind(window)?.generate_with(length, precision(mode)?)
        })
    }
}

type Trig<R> = unsafe extern "C" fn(f64, *mut c_void, *mut R) -> u32;

pub(super) unsafe fn custom<T: Scalar, R: Scalar>(
    window: u32,
    callback: Option<Trig<R>>,
    context: *mut c_void,
    output: *mut T,
    length: usize,
) -> u32 {
    // SAFETY: Forward output and synchronous callback/context contracts.
    unsafe {
        generated_result(output, length, || {
            let window = kind(window)?;
            if length > 1 && window != Window::Boxcar && callback.is_none() {
                return Err(Error("custom tapered window requires a callback"));
            }
            window.generate_custom(length, |angle| {
                let callback = callback.ok_or(Error("missing window callback"))?;
                let mut value = R::zero();
                // SAFETY: Caller supplies a callable matching C function that
                // returns normally, uses its live context, and writes the
                // provided scalar output. This local value is live for the call;
                // the callback cannot retain its address or access final output.
                if callback(angle, context, &mut value) != 0 {
                    return Err(Error("window callback failed"));
                }
                Ok(value)
            })
        })
    }
}
