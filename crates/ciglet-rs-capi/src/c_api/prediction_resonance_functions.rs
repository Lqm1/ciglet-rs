//! Typed resonance arrays and complete root diagnostics.
use super::{
    polynomial::Diagnostics,
    prediction_resonance::{self, Output},
};

/// Return sorted normalized frequencies, complex poles and root diagnostics.
/// All planes have caller capacity; count reports the populated prefix. Unused
/// capacity is retained. Nonconvergence succeeds with its complete native report.
/// Insufficient capacity returns two, retaining every output; constants can have
/// zero capacity and null planes. Frequencies retain native strict 0.001..0.999 filtering.
///
/// # Safety
/// Nonempty input is initialized stable readable single-allocation storage.
/// Each output plane is aligned and exclusively writable for its capacity, and
/// scalar outputs are aligned and exclusively writable for their types. All seven
/// output ranges must be pairwise disjoint. Input/output overlap is supported after
/// all input borrowing ends. Pointer checks cannot prove caller allocation validity.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit planar C results and complete mixed-type diagnostics.
pub unsafe extern "C" fn ciglet_rs_prediction_resonances_f32(
    input: *const f32,
    input_count: usize,
    frequencies: *mut f32,
    real: *mut f32,
    imaginary: *mut f32,
    capacity: usize,
    count: *mut usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward initialized input and pairwise-disjoint writable result contracts.
    unsafe {
        prediction_resonance::estimate(
            input,
            input_count,
            Output {
                frequencies,
                real,
                imaginary,
                capacity,
                count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}

/// Return sorted normalized frequencies, complex poles and root diagnostics.
/// All planes have caller capacity; count reports the populated prefix. Unused
/// capacity is retained. Nonconvergence succeeds with its complete native report.
/// Insufficient capacity returns two, retaining every output; constants can have
/// zero capacity and null planes. Frequencies retain native strict 0.001..0.999 filtering.
///
/// # Safety
/// Nonempty input is initialized stable readable single-allocation storage.
/// Each output plane is aligned and exclusively writable for its capacity, and
/// scalar outputs are aligned and exclusively writable for their types. All seven
/// output ranges must be pairwise disjoint. Input/output overlap is supported after
/// all input borrowing ends. Pointer checks cannot prove caller allocation validity.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)] // Explicit planar C results and complete mixed-type diagnostics.
pub unsafe extern "C" fn ciglet_rs_prediction_resonances_f64(
    input: *const f64,
    input_count: usize,
    frequencies: *mut f64,
    real: *mut f64,
    imaginary: *mut f64,
    capacity: usize,
    count: *mut usize,
    iterations: *mut usize,
    converged: *mut u32,
    maximum_residual: *mut f64,
) -> u32 {
    // SAFETY: Forward initialized input and pairwise-disjoint writable result contracts.
    unsafe {
        prediction_resonance::estimate(
            input,
            input_count,
            Output {
                frequencies,
                real,
                imaginary,
                capacity,
                count,
            },
            Diagnostics {
                iterations,
                converged,
                maximum_residual,
            },
        )
    }
}
