//! Independently owned complete LU factors and staged solves.
use super::{generated_result, matrices, range};
use crate::{matrix::Lu, statistics::Scalar};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque owned binary32 packed LU factors with row swaps.
pub struct CigletRsLuF32(Lu<f32>);
/// Opaque owned binary64 packed LU factors with row swaps.
pub struct CigletRsLuF64(Lu<f64>);
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn owned(factor: Lu<Self::Value>) -> Self;
    fn factor(&self) -> &Lu<Self::Value>;
}
impl Handle for CigletRsLuF32 {
    type Value = f32;
    fn owned(factor: Lu<f32>) -> Self {
        Self(factor)
    }
    fn factor(&self) -> &Lu<f32> {
        &self.0
    }
}
impl Handle for CigletRsLuF64 {
    type Value = f64;
    fn owned(factor: Lu<f64>) -> Self {
        Self(factor)
    }
    fn factor(&self) -> &Lu<f64> {
        &self.0
    }
}
pub(super) unsafe fn factor<M: matrices::Handle>(
    input: *const M,
    policy: u32,
    output: *mut *mut M::Factor,
) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    if policy > 1 {
        return 3;
    }
    // SAFETY: Live immutable retained matrix and independent ownership slot.
    unsafe {
        matrices::create(output, || {
            let matrix = (&*input).matrix().clone();
            let factor = if policy == 0 {
                Lu::factor(matrix)
            } else {
                Lu::factor_unpivoted(matrix)
            }?;
            Ok(M::Factor::owned(factor))
        })
    }
}
pub(super) unsafe fn clone_handle<H: Handle>(input: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Correctly typed immutable retained owner and independent slot.
    unsafe { matrices::create(output, || Ok(H::owned((&*input).factor().clone()))) }
}
pub(super) unsafe fn shape<H: Handle>(input: *const H, output: *mut usize, count: usize) -> u32 {
    if count != 3 {
        return 2;
    }
    if let Err(status) = range(input, 1).and_then(|()| range(output.cast_const(), 3)) {
        return status;
    }
    // SAFETY: Live retained immutable typed owner.
    let packed = unsafe { &*input }.factor().packed();
    let values = [packed.rows(), packed.columns(), packed.as_slice().len()];
    // SAFETY: Three exclusively writable elements independent of owner allocation.
    unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, 3) };
    0
}
pub(super) unsafe fn packed<H: Handle>(
    input: *const H,
    output: *mut H::Value,
    count: usize,
) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Live retained immutable typed owner.
    let values = unsafe { &*input }.factor().packed().as_slice();
    if count != values.len() {
        return 2;
    }
    // SAFETY: Independent output; owned complete values staged before commit.
    unsafe {
        generated_result(output, count, || {
            let mut result = crate::vector::buffer(count)?;
            result.extend_from_slice(values);
            Ok(result)
        })
    }
}
pub(super) unsafe fn swaps<H: Handle>(input: *const H, output: *mut usize, count: usize) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Live retained immutable typed owner.
    let values = unsafe { &*input }.factor().row_swaps();
    if count != values.len() {
        return 2;
    }
    // SAFETY: Independent output; owned complete plan staged before commit.
    unsafe {
        generated_result(output, count, || {
            let mut result = crate::vector::buffer(count)?;
            result.extend_from_slice(values);
            Ok(result)
        })
    }
}
pub(super) unsafe fn solve<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    input_count: usize,
    output: *mut H::Value,
    output_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, input_count)) {
        return status;
    }
    // SAFETY: Live retained immutable typed factorization.
    let factor = unsafe { &*handle }.factor();
    if input_count != factor.packed().rows() || output_count != input_count {
        return 2;
    }
    // SAFETY: Stable initialized input and independent owner; complete staging
    // permits output/input overlap after all borrowing ends.
    unsafe {
        generated_result(output, output_count, || {
            let values = if input_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_count)
            };
            factor.solve(values)
        })
    }
}
