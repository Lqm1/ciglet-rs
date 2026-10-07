//! Opt-in C ABI. Unsafe operations are confined to this boundary.
//!
//! A nonnull, aligned output must point to writable storage for its type.
//! Pointer checks cannot establish allocation validity or exclusive access.
//! Status zero means success; status one means a null or misaligned output.
//! Scalar results use output pointers to avoid i686 x87 float-return changes.
#![deny(unsafe_op_in_unsafe_fn)]

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
