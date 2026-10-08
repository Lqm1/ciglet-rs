//! Staged native FIR design, convolution and recursive filtering.
use super::{generated_result, range, windows};
use crate::{
    Error,
    filtering::{self, Passband, Startup},
    statistics::Scalar,
};
use std::panic::RefUnwindSafe;

/// Borrow only after the caller's range was checked. Null empty inputs stay empty.
unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains initialized stable readable checked storage.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
#[derive(Clone, Copy)]
pub(super) enum Design {
    Windowed(u32),
    Fir(u32),
    Bandpass,
}
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn design<T: Scalar + RefUnwindSafe>(
    length: usize,
    lower: T,
    upper: T,
    window: u32,
    output: *mut T,
    count: usize,
    design: Design,
) -> u32 {
    if length != count {
        return 2;
    }
    // SAFETY: Exclusively writable caller buffer; results fully staged.
    unsafe {
        generated_result(output, count, || {
            let window = windows::kind(window)?;
            match design {
                Design::Windowed(kind) => {
                    let band = match kind {
                        0 => Passband::Lowpass(lower),
                        1 => Passband::Highpass(lower),
                        2 => Passband::Bandpass { lower, upper },
                        3 => Passband::Bandstop { lower, upper },
                        _ => return Err(Error("unknown FIR passband")),
                    };
                    filtering::windowed_fir(length, band, window)
                }
                Design::Fir(highpass) => filtering::fir(
                    length,
                    lower,
                    match highpass {
                        0 => false,
                        1 => true,
                        _ => return Err(Error("unknown FIR highpass selector")),
                    },
                    window,
                ),
                Design::Bandpass => filtering::bandpass_fir(length, lower, upper, window),
            }
        })
    }
}
pub(super) unsafe fn convolve<T: Scalar + RefUnwindSafe>(
    signal: *const T,
    signal_count: usize,
    impulse: *const T,
    impulse_count: usize,
    output: *mut T,
    count: usize,
) -> u32 {
    if let Err(status) = range(signal, signal_count).and_then(|()| range(impulse, impulse_count)) {
        return status;
    }
    let expected = if signal_count == 0 || impulse_count == 0 {
        0
    } else {
        match signal_count.checked_add(impulse_count - 1) {
            Some(n) if n <= i32::MAX as usize => n,
            _ => return 2,
        }
    };
    if count != expected {
        return 2;
    }
    // SAFETY: Checked readable inputs; complete output staging ends borrowing
    // before commit, allowing signal/impulse/output overlap.
    unsafe {
        generated_result(output, count, || {
            filtering::convolve(input(signal, signal_count), input(impulse, impulse_count))
        })
    }
}
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn recursive<T: Scalar + RefUnwindSafe>(
    signal: *const T,
    signal_count: usize,
    numerator: *const T,
    numerator_count: usize,
    denominator: *const T,
    denominator_count: usize,
    startup: u32,
    output: *mut T,
    count: usize,
    two_pass: bool,
) -> u32 {
    if count != signal_count {
        return 2;
    }
    let startup = match startup {
        0 => Startup::ZeroState,
        1 => Startup::LegacyZeroPrefix,
        _ => return 3,
    };
    if let Err(status) = range(signal, signal_count)
        .and_then(|()| range(numerator, numerator_count))
        .and_then(|()| range(denominator, denominator_count))
    {
        return status;
    }
    // SAFETY: Checked initialized stable readable inputs; all native operations
    // finish before committing the staged exclusively writable output.
    unsafe {
        generated_result(output, count, || {
            let signal = input(signal, signal_count);
            let numerator = input(numerator, numerator_count);
            let denominator = input(denominator, denominator_count);
            if two_pass {
                filtering::forward_backward_with(signal, numerator, denominator, startup)
            } else {
                filtering::filter_with(signal, numerator, denominator, startup)
            }
        })
    }
}
