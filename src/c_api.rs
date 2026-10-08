//! Opt-in C ABI. Unsafe operations are confined to this boundary.
//!
//! A nonnull, aligned output must point to writable storage for its type.
//! Pointer checks cannot establish allocation validity or exclusive access.
//! Statuses: zero success, one invalid pointer, two invalid buffer length,
//! three calculation error, four caught unwinding panic.
//! Scalar results use output pointers to avoid i686 x87 float-return changes.
#![deny(unsafe_op_in_unsafe_fn)]

mod statistics;
pub use statistics::*;
mod generation;
pub use generation::*;
mod pair_statistics;
pub use pair_statistics::*;
mod ordering;
pub use ordering::*;
mod arrays;
pub use arrays::{CigletRsArrayF32, CigletRsArrayF64};
mod array_f32;
pub use array_f32::*;
mod array_f64;
pub use array_f64::*;
mod fourier;
pub use fourier::{CigletRsFftF32, CigletRsFftF64};
mod fourier_f32;
pub use fourier_f32::*;
mod fourier_f64;
pub use fourier_f64::*;
mod phase;
mod phase_f32;
pub use phase_f32::*;
mod phase_f64;
pub use phase_f64::*;
mod window_functions;
mod windows;
pub use window_functions::*;
mod chirp_z;
pub use chirp_z::{CigletRsCztF32, CigletRsCztF64};
mod chirp_z_f32;
pub use chirp_z_f32::*;
mod chirp_z_f64;
pub use chirp_z_f64::*;
mod frequency_distance;
pub use frequency_distance::*;
mod frame_extrema;
mod frame_extrema_functions;
pub use frame_extrema_functions::*;
mod band_cepstra;
mod band_cepstra_functions;
pub use band_cepstra_functions::*;

fn disjoint_outputs<T, U>(
    first: *const T,
    first_length: usize,
    second: *const U,
    second_length: usize,
) -> Result<(), u32> {
    range(first, first_length)?;
    range(second, second_length)?;
    if first_length != 0 && second_length != 0 {
        let first_start = first as usize;
        let second_start = second as usize;
        // Range validation established bounded multiplication and no address wrap.
        let first_end = first_start + first_length * std::mem::size_of::<T>();
        let second_end = second_start + second_length * std::mem::size_of::<U>();
        if first_start < second_end && second_start < first_end {
            return Err(2);
        }
    }
    Ok(())
}

unsafe fn pair_result<T: std::panic::RefUnwindSafe, R>(
    left: *const T,
    left_length: usize,
    right: *const T,
    right_length: usize,
    compute: impl FnOnce(&[T], &[T]) -> Result<R, crate::Error> + std::panic::UnwindSafe,
) -> Result<R, u32> {
    if left_length != right_length {
        return Err(2);
    }
    range(left, left_length)?;
    range(right, right_length)?;
    boundary(|| {
        let (left, right) = if left_length == 0 {
            (&[][..], &[][..])
        } else {
            // SAFETY: Caller inputs are initialized, stable, single-allocation
            // readable ranges. Alignment, size and matching lengths are checked.
            // Immutable inputs may overlap one another.
            unsafe {
                (
                    std::slice::from_raw_parts(left, left_length),
                    std::slice::from_raw_parts(right, right_length),
                )
            }
        };
        compute(left, right)
    })
}

unsafe fn pair_scalar_result<T: Copy + std::panic::RefUnwindSafe>(
    left: *const T,
    left_length: usize,
    right: *const T,
    right_length: usize,
    output: *mut T,
    compute: impl FnOnce(&[T], &[T]) -> Result<T, crate::Error> + std::panic::UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    // SAFETY: This adapter forwards the caller's input contracts.
    match unsafe { pair_result(left, left_length, right, right_length, compute) } {
        Ok(value) => {
            // SAFETY: Input borrows ended; caller output is exclusively writable.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}

unsafe fn generated_result<T: Copy>(
    output: *mut T,
    length: usize,
    compute: impl FnOnce() -> Result<Vec<T>, crate::Error> + std::panic::UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), length) {
        return status;
    }
    match boundary(compute) {
        Ok(values) => {
            if values.len() != length {
                return 3;
            }
            if length != 0 {
                // SAFETY: Caller output is valid, aligned and writable. The
                // separately allocated result cannot overlap caller storage.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, length) };
            }
            0
        }
        Err(status) => status,
    }
}

unsafe fn scalar_slice_result<T: Copy + std::panic::RefUnwindSafe>(
    input: *const T,
    length: usize,
    output: *mut T,
    compute: impl FnOnce(&[T]) -> Result<T, crate::Error> + std::panic::UnwindSafe,
) -> u32 {
    if let Err(status) = range(input, length).and_then(|()| range(output.cast_const(), 1)) {
        return status;
    }
    let result = boundary(|| {
        let values = if length == 0 {
            &[]
        } else {
            // SAFETY: The caller provides initialized, stable single-allocation
            // input; checked alignment and length are required before borrowing.
            unsafe { std::slice::from_raw_parts(input, length) }
        };
        compute(values)
    });
    match result {
        Ok(value) => {
            // SAFETY: The input borrow has ended and the caller guarantees one
            // exclusively writable output value, which may overlap the input.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}

fn range<T>(pointer: *const T, length: usize) -> Result<(), u32> {
    if length == 0 {
        return Ok(());
    }
    if length > i32::MAX as usize || length > isize::MAX as usize / std::mem::size_of::<T>() {
        return Err(2);
    }
    if pointer.is_null() || !pointer.is_aligned() {
        return Err(1);
    }
    if (pointer as usize)
        .checked_add(length * std::mem::size_of::<T>())
        .is_none()
    {
        return Err(2);
    }
    Ok(())
}

fn boundary<T>(
    operation: impl FnOnce() -> Result<T, crate::Error> + std::panic::UnwindSafe,
) -> Result<T, u32> {
    match std::panic::catch_unwind(operation) {
        Ok(result) => result.map_err(|_| 3),
        Err(payload) => {
            // An arbitrary panic payload's destructor may panic again. Retain it
            // on this exceptional path rather than unwinding across the C ABI.
            std::mem::forget(payload);
            Err(4)
        }
    }
}

unsafe fn scalar_value_result<T: Copy>(
    output: *mut T,
    compute: impl FnOnce() -> Result<T, crate::Error> + std::panic::UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(compute) {
        Ok(value) => {
            // SAFETY: Caller supplies an aligned, exclusively writable scalar.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}

unsafe fn vector_result<T: Copy + std::panic::RefUnwindSafe>(
    input: *const T,
    length: usize,
    output: *mut T,
    output_length: usize,
    compute: fn(&[T]) -> Result<Vec<T>, crate::Error>,
) -> u32 {
    if output_length != length {
        return 2;
    }
    if let Err(status) = range(input, length).and_then(|()| range(output.cast_const(), length)) {
        return status;
    }
    let result = boundary(|| {
        let values = if length == 0 {
            &[]
        } else {
            // SAFETY: Checks establish alignment and representable length; the
            // caller supplies one initialized allocation, stable during compute.
            unsafe { std::slice::from_raw_parts(input, length) }
        };
        compute(values)
    });
    match result {
        Ok(values) => {
            if values.len() != length {
                return 3;
            }
            if length != 0 {
                // SAFETY: Computation ended the input borrow. The owned result
                // cannot overlap caller storage. Output may overlap input and
                // is exclusively writable now under the caller's contract.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, length) };
            }
            0
        }
        Err(status) => status,
    }
}

/// Compute difference into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_difference_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(
            input,
            length,
            output,
            output_length,
            crate::vector::difference,
        )
    }
}

/// Compute cumulative sum into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_cumulative_sum_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(
            input,
            length,
            output,
            output_length,
            crate::vector::cumulative_sum,
        )
    }
}

/// Compute reverse into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_reverse_f32(
    input: *const f32,
    length: usize,
    output: *mut f32,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(input, length, output, output_length, |values| {
            Ok(crate::vector::reverse(values))
        })
    }
}

/// Compute difference into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_difference_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(
            input,
            length,
            output,
            output_length,
            crate::vector::difference,
        )
    }
}

/// Compute cumulative sum into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_cumulative_sum_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(
            input,
            length,
            output,
            output_length,
            crate::vector::cumulative_sum,
        )
    }
}

/// Compute reverse into equally sized caller storage.
/// Empty buffers accept null pointers; errors leave output unchanged.
///
/// # Safety
/// Nonempty input must be initialized in one allocation and readable until
/// computation ends. Nonempty output must be writable for output_length values.
/// No concurrent access may conflict; input/output overlap is supported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_reverse_f64(
    input: *const f64,
    length: usize,
    output: *mut f64,
    output_length: usize,
) -> u32 {
    // SAFETY: The documented buffer contract is forwarded to the adapter.
    unsafe {
        vector_result(input, length, output, output_length, |values| {
            Ok(crate::vector::reverse(values))
        })
    }
}

/// First revision of the Rust port's C interface.
#[unsafe(no_mangle)]
pub extern "C" fn ciglet_rs_abi_version() -> u32 {
    1
}

unsafe fn write_result<T>(output: *mut T, value: T) -> u32 {
    if output.is_null() || !output.is_aligned() {
        return 1;
    }
    // SAFETY: The caller guarantees writable storage and exclusive access.
    // Null and alignment checks above also permit uninitialized output storage.
    unsafe { output.write(value) };
    0
}

/// Write the smaller operand, retaining the second operand on ties or NaNs.
///
/// # Safety
/// Unless null or misaligned, `output` must be valid for one exclusive f32 write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_minimum_f32(first: f32, second: f32, output: *mut f32) -> u32 {
    // SAFETY: This function forwards its documented output contract.
    unsafe { write_result(output, crate::scalar::minimum(first, second)) }
}

/// Write the larger operand, retaining the second operand on ties or NaNs.
///
/// # Safety
/// Unless null or misaligned, `output` must be valid for one exclusive f32 write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_maximum_f32(first: f32, second: f32, output: *mut f32) -> u32 {
    // SAFETY: This function forwards its documented output contract.
    unsafe { write_result(output, crate::scalar::maximum(first, second)) }
}

/// Write the smaller operand, retaining the second operand on ties or NaNs.
///
/// # Safety
/// Unless null or misaligned, `output` must be valid for one exclusive f64 write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_minimum_f64(first: f64, second: f64, output: *mut f64) -> u32 {
    // SAFETY: This function forwards its documented output contract.
    unsafe { write_result(output, crate::scalar::minimum(first, second)) }
}

/// Write the larger operand, retaining the second operand on ties or NaNs.
///
/// # Safety
/// Unless null or misaligned, `output` must be valid for one exclusive f64 write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_maximum_f64(first: f64, second: f64, output: *mut f64) -> u32 {
    // SAFETY: This function forwards its documented output contract.
    unsafe { write_result(output, crate::scalar::maximum(first, second)) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unwind_boundary_retains_a_payload_with_a_panicking_destructor() {
        struct Payload;
        impl Drop for Payload {
            fn drop(&mut self) {
                panic!("the caught payload must not be dropped at the ABI boundary");
            }
        }
        let result = super::boundary::<()>(|| std::panic::panic_any(Payload));
        assert_eq!(result, Err(4));
        assert_eq!(
            super::boundary::<()>(|| Err(crate::Error("expected failure"))),
            Err(3)
        );
    }
}
