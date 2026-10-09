//! Typed column-major owners and staged matrix arithmetic.
use super::{boundary, generated_result, range, scalar_value_result};
use crate::{
    Error,
    matrix::{self, Matrix},
    statistics::Scalar,
};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque owned binary32 column-major matrix.
pub struct CigletRsMatrixF32(Matrix<f32>);
/// Opaque owned binary64 column-major matrix.
pub struct CigletRsMatrixF64(Matrix<f64>);
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    type Factor: super::matrix_lu::Handle<Value = Self::Value>;
    fn owned(matrix: Matrix<Self::Value>) -> Self;
    fn matrix(&self) -> &Matrix<Self::Value>;
    fn matrix_mut(&mut self) -> &mut Matrix<Self::Value>;
}
impl Handle for CigletRsMatrixF32 {
    type Value = f32;
    type Factor = super::matrix_lu::CigletRsLuF32;
    fn owned(matrix: Matrix<f32>) -> Self {
        Self(matrix)
    }
    fn matrix(&self) -> &Matrix<f32> {
        &self.0
    }
    fn matrix_mut(&mut self) -> &mut Matrix<f32> {
        &mut self.0
    }
}
impl Handle for CigletRsMatrixF64 {
    type Value = f64;
    type Factor = super::matrix_lu::CigletRsLuF64;
    fn owned(matrix: Matrix<f64>) -> Self {
        Self(matrix)
    }
    fn matrix(&self) -> &Matrix<f64> {
        &self.0
    }
    fn matrix_mut(&mut self) -> &mut Matrix<f64> {
        &mut self.0
    }
}
pub(super) unsafe fn create<H>(
    output: *mut *mut H,
    compute: impl FnOnce() -> Result<H, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| compute().map(Box::new)) {
        Ok(value) => {
            // SAFETY: Independent initialized aligned exclusively writable slot.
            unsafe { output.write(Box::into_raw(value)) };
            0
        }
        Err(status) => status,
    }
}
fn dimensions<T>(rows: usize, columns: usize) -> Result<usize, u32> {
    let count = rows.checked_mul(columns).ok_or(2_u32)?;
    if rows > i32::MAX as usize
        || columns > i32::MAX as usize
        || count > i32::MAX as usize
        || count > isize::MAX as usize / size_of::<T>()
    {
        return Err(2);
    }
    Ok(count)
}
pub(super) unsafe fn from_column_major<H: Handle>(
    rows: usize,
    columns: usize,
    input: *const H::Value,
    count: usize,
    output: *mut *mut H,
) -> u32 {
    match dimensions::<H::Value>(rows, columns) {
        Ok(expected) if expected == count => (),
        _ => return 2,
    }
    if let Err(status) = range(input, count) {
        return status;
    }
    // SAFETY: Initialized stable readable input and independent ownership slot.
    unsafe {
        create(output, || {
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            let mut owned = crate::vector::buffer(count)?;
            owned.extend_from_slice(values);
            Matrix::from_column_major(rows, columns, owned).map(H::owned)
        })
    }
}
pub(super) unsafe fn zeros<H: Handle>(rows: usize, columns: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = dimensions::<H::Value>(rows, columns) {
        return status;
    }
    // SAFETY: Independent initialized writable ownership slot.
    unsafe { create(output, || Matrix::zeros(rows, columns).map(H::owned)) }
}
pub(super) unsafe fn identity<H: Handle>(size: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = dimensions::<H::Value>(size, size) {
        return status;
    }
    // SAFETY: Independent initialized writable ownership slot.
    unsafe { create(output, || Matrix::identity(size).map(H::owned)) }
}
pub(super) unsafe fn clone_handle<H: Handle>(input: *const H, output: *mut *mut H) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Typed live immutable owner and independent writable slot.
    unsafe { create(output, || Ok(H::owned((&*input).matrix().clone()))) }
}
pub(super) unsafe fn shape<H: Handle>(input: *const H, output: *mut usize, count: usize) -> u32 {
    if count != 3 {
        return 2;
    }
    if let Err(status) = range(input, 1).and_then(|()| range(output.cast_const(), 3)) {
        return status;
    }
    // SAFETY: Typed live owner retained unmodified, excluding release.
    let matrix = unsafe { &*input }.matrix();
    let values = [matrix.rows(), matrix.columns(), matrix.as_slice().len()];
    // SAFETY: Independent exclusively writable three-element output.
    unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, 3) };
    0
}
pub(super) unsafe fn copy<H: Handle>(input: *const H, output: *mut H::Value, count: usize) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Correctly typed retained immutable owner.
    let matrix = unsafe { &*input }.matrix();
    if matrix.as_slice().len() != count {
        return 2;
    }
    // SAFETY: Independent writable output; no null zero-length pointer is borrowed.
    unsafe {
        generated_result(output, count, || {
            let mut values = crate::vector::buffer(count)?;
            values.extend_from_slice(matrix.as_slice());
            Ok(values)
        })
    }
}
pub(super) unsafe fn get<H: Handle>(
    input: *const H,
    row: usize,
    column: usize,
    output: *mut H::Value,
) -> u32 {
    if let Err(status) = range(input, 1) {
        return status;
    }
    // SAFETY: Correctly typed retained immutable owner.
    let matrix = unsafe { &*input }.matrix();
    if row >= matrix.rows() || column >= matrix.columns() {
        return 2;
    }
    // SAFETY: Independent writable scalar and valid retained read-only matrix.
    unsafe {
        scalar_value_result(output, || {
            matrix
                .get(row, column)
                .copied()
                .ok_or(Error("matrix index is invalid"))
        })
    }
}
pub(super) unsafe fn multiply<H: Handle>(
    left: *const H,
    right: *const H,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(left, 1).and_then(|()| range(right, 1)) {
        return status;
    }
    // SAFETY: Both live owners retained unmodified; immutable aliasing is valid.
    unsafe {
        create(output, || {
            (&*left).matrix().multiply((&*right).matrix()).map(H::owned)
        })
    }
}
pub(super) unsafe fn multiply_vector<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    input_count: usize,
    output: *mut H::Value,
    output_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, input_count)) {
        return status;
    }
    // SAFETY: Correctly typed immutable retained owner.
    let matrix = unsafe { &*handle }.matrix();
    if input_count != matrix.columns() || output_count != matrix.rows() {
        return 2;
    }
    // SAFETY: Initialized stable input, independent owner and writable output;
    // complete staging permits input/output overlap after input borrowing ends.
    unsafe {
        generated_result(output, output_count, || {
            let values = if input_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, input_count)
            };
            matrix.multiply_vector(values)
        })
    }
}
pub(super) unsafe fn apply_rows<H: Handle>(
    handle: *mut H,
    swaps: *const usize,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle.cast_const(), 1).and_then(|()| range(swaps, count)) {
        return status;
    }
    // SAFETY: Caller grants exclusive access to a live correctly typed owner.
    let owner = unsafe { &mut *handle };
    if count != owner.matrix().rows() {
        return 2;
    }
    let result = boundary(|| {
        let plan = if count == 0 {
            &[]
        } else {
            // SAFETY: Independent initialized stable single-allocation swap plan.
            unsafe { std::slice::from_raw_parts(swaps, count) }
        };
        let mut matrix = owner.matrix().clone();
        matrix.apply_row_swaps(plan)?;
        Ok(matrix)
    });
    match result {
        Ok(matrix) => {
            *owner.matrix_mut() = matrix;
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn pivot<H: Handle>(handle: *mut H, output: *mut usize, count: usize) -> u32 {
    if let Err(status) =
        range(handle.cast_const(), 1).and_then(|()| range(output.cast_const(), count))
    {
        return status;
    }
    // SAFETY: Caller grants exclusive access; output independent of owner allocation.
    let owner = unsafe { &mut *handle };
    if count != owner.matrix().rows() {
        return 2;
    }
    let result = boundary(|| {
        let mut matrix = owner.matrix().clone();
        let swaps = matrix.pivot_rows()?;
        Ok((matrix, swaps))
    });
    match result {
        Ok((matrix, swaps)) => {
            *owner.matrix_mut() = matrix;
            if count != 0 {
                // SAFETY: Owned staged plan and independent exclusively writable output.
                unsafe { std::ptr::copy_nonoverlapping(swaps.as_ptr(), output, count) };
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn apply_vector<T: Scalar + RefUnwindSafe>(
    input: *const T,
    count: usize,
    swaps: *const usize,
    swap_count: usize,
    output: *mut T,
    output_count: usize,
) -> u32 {
    if count != swap_count || count != output_count {
        return 2;
    }
    if let Err(status) = range(input, count).and_then(|()| range(swaps, swap_count)) {
        return status;
    }
    // SAFETY: Initialized stable input and swap plan; complete owned staging
    // allows output/input overlap. Native swaps preserve all scalar bits.
    unsafe {
        generated_result(output, output_count, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            let swaps = if swap_count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(swaps, swap_count)
            };
            let mut values = crate::vector::buffer(count)?;
            values.extend_from_slice(input);
            matrix::apply_swaps(&mut values, swaps)?;
            Ok(values)
        })
    }
}
