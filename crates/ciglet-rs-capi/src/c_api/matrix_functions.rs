//! Column-major matrix ownership, complete LU factors and checked arithmetic.
use super::{
    fourier,
    matrices::{self, CigletRsMatrixF32, CigletRsMatrixF64},
    matrix_lu::{self, CigletRsLuF32, CigletRsLuF64},
};

/// Own initialized finite column-major values, index row + column * rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_from_column_major_f32(
    rows: usize,
    columns: usize,
    input: *const f32,
    count: usize,
    output: *mut *mut CigletRsMatrixF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::from_column_major(rows, columns, input, count, output) }
}

/// Create an owned zero matrix, retaining native empty/rectangular shapes.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_zeros_f32(
    rows: usize,
    columns: usize,
    output: *mut *mut CigletRsMatrixF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::zeros(rows, columns, output) }
}

/// Create an owned square identity matrix.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_identity_f32(
    size: usize,
    output: *mut *mut CigletRsMatrixF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::identity(size, output) }
}

/// Clone into an independent owned matrix without changing the input.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_clone_f32(
    input: *const CigletRsMatrixF32,
    output: *mut *mut CigletRsMatrixF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::clone_handle(input, output) }
}

/// Release an owned matrix and clear its slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized aligned exclusively writable storage owning exactly one
/// correctly typed handle from this API. Exclude every outstanding call and alias;
/// do not reuse released owners.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_release_f32(slot: *mut *mut CigletRsMatrixF32) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy rows, columns and element count into exactly three usize values.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_shape_f32(
    input: *const CigletRsMatrixF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::shape(input, output, count) }
}

/// Copy all column-major values; count equals rows * columns.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_copy_column_major_f32(
    input: *const CigletRsMatrixF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::copy(input, output, count) }
}

/// Read a zero-based matrix element; out-of-bounds indices return two.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_get_f32(
    input: *const CigletRsMatrixF32,
    row: usize,
    column: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::get(input, row, column, output) }
}

/// Return an independent owned product using native accumulation order.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_multiply_f32(
    left: *const CigletRsMatrixF32,
    right: *const CigletRsMatrixF32,
    output: *mut *mut CigletRsMatrixF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::multiply(left, right, output) }
}

/// Return the full rectangular matrix-vector product; output count equals rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_multiply_vector_f32(
    handle: *const CigletRsMatrixF32,
    input: *const f32,
    input_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::multiply_vector(handle, input, input_count, output, output_count) }
}

/// Apply successive `(i, swaps[i])` row exchanges with the corrected rectangular stride.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_apply_row_swaps_f32(
    handle: *mut CigletRsMatrixF32,
    swaps: *const usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::apply_rows(handle, swaps, count) }
}

/// Commit source preliminary square-matrix pivoting and its complete ordered row-swap plan.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_pivot_rows_f32(
    handle: *mut CigletRsMatrixF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::pivot(handle, output, count) }
}

/// Apply ordered swaps to scalar values, preserving all bits including NaNs.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_apply_swaps_f32(
    input: *const f32,
    count: usize,
    swaps: *const usize,
    swap_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::apply_vector(input, count, swaps, swap_count, output, output_count) }
}

/// Factor an immutable matrix clone: zero partial pivoting, one source-compatible unpivoted.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_factor_f32(
    input: *const CigletRsMatrixF32,
    policy: u32,
    output: *mut *mut CigletRsLuF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::factor(input, policy, output) }
}

/// Clone an independent owner of complete packed factors and ordered row swaps.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_clone_f32(
    input: *const CigletRsLuF32,
    output: *mut *mut CigletRsLuF32,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::clone_handle(input, output) }
}

/// Release owned factors and clear the slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized aligned exclusively writable storage owning exactly one
/// correctly typed handle from this API. Exclude every outstanding call and alias;
/// do not reuse released owners.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_release_f32(slot: *mut *mut CigletRsLuF32) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy packed-factor rows, columns and element count into three usize values.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_shape_f32(
    input: *const CigletRsLuF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::shape(input, output, count) }
}

/// Copy every column-major packed factor; L has an implicit unit diagonal.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_copy_packed_f32(
    input: *const CigletRsLuF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::packed(input, output, count) }
}

/// Copy the complete ordered row-swap plan; count equals packed rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_copy_row_swaps_f32(
    input: *const CigletRsLuF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::swaps(input, output, count) }
}

/// Solve with all native row swaps and substitution order, retaining the right-hand side.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_solve_f32(
    handle: *const CigletRsLuF32,
    input: *const f32,
    input_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::solve(handle, input, input_count, output, output_count) }
}

/// Own initialized finite column-major values, index row + column * rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_from_column_major_f64(
    rows: usize,
    columns: usize,
    input: *const f64,
    count: usize,
    output: *mut *mut CigletRsMatrixF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::from_column_major(rows, columns, input, count, output) }
}

/// Create an owned zero matrix, retaining native empty/rectangular shapes.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_zeros_f64(
    rows: usize,
    columns: usize,
    output: *mut *mut CigletRsMatrixF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::zeros(rows, columns, output) }
}

/// Create an owned square identity matrix.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_identity_f64(
    size: usize,
    output: *mut *mut CigletRsMatrixF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::identity(size, output) }
}

/// Clone into an independent owned matrix without changing the input.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_clone_f64(
    input: *const CigletRsMatrixF64,
    output: *mut *mut CigletRsMatrixF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::clone_handle(input, output) }
}

/// Release an owned matrix and clear its slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized aligned exclusively writable storage owning exactly one
/// correctly typed handle from this API. Exclude every outstanding call and alias;
/// do not reuse released owners.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_release_f64(slot: *mut *mut CigletRsMatrixF64) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy rows, columns and element count into exactly three usize values.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_shape_f64(
    input: *const CigletRsMatrixF64,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::shape(input, output, count) }
}

/// Copy all column-major values; count equals rows * columns.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_copy_column_major_f64(
    input: *const CigletRsMatrixF64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::copy(input, output, count) }
}

/// Read a zero-based matrix element; out-of-bounds indices return two.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_get_f64(
    input: *const CigletRsMatrixF64,
    row: usize,
    column: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::get(input, row, column, output) }
}

/// Return an independent owned product using native accumulation order.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_multiply_f64(
    left: *const CigletRsMatrixF64,
    right: *const CigletRsMatrixF64,
    output: *mut *mut CigletRsMatrixF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::multiply(left, right, output) }
}

/// Return the full rectangular matrix-vector product; output count equals rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_multiply_vector_f64(
    handle: *const CigletRsMatrixF64,
    input: *const f64,
    input_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::multiply_vector(handle, input, input_count, output, output_count) }
}

/// Apply successive `(i, swaps[i])` row exchanges with the corrected rectangular stride.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_apply_row_swaps_f64(
    handle: *mut CigletRsMatrixF64,
    swaps: *const usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::apply_rows(handle, swaps, count) }
}

/// Commit source preliminary square-matrix pivoting and its complete ordered row-swap plan.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_pivot_rows_f64(
    handle: *mut CigletRsMatrixF64,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::pivot(handle, output, count) }
}

/// Apply ordered swaps to scalar values, preserving all bits including NaNs.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_matrix_apply_swaps_f64(
    input: *const f64,
    count: usize,
    swaps: *const usize,
    swap_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrices::apply_vector(input, count, swaps, swap_count, output, output_count) }
}

/// Factor an immutable matrix clone: zero partial pivoting, one source-compatible unpivoted.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_factor_f64(
    input: *const CigletRsMatrixF64,
    policy: u32,
    output: *mut *mut CigletRsLuF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::factor(input, policy, output) }
}

/// Clone an independent owner of complete packed factors and ordered row swaps.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_clone_f64(
    input: *const CigletRsLuF64,
    output: *mut *mut CigletRsLuF64,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::clone_handle(input, output) }
}

/// Release owned factors and clear the slot; initialized null slots succeed.
///
/// # Safety
/// Slot is initialized aligned exclusively writable storage owning exactly one
/// correctly typed handle from this API. Exclude every outstanding call and alias;
/// do not reuse released owners.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_release_f64(slot: *mut *mut CigletRsLuF64) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { fourier::release(slot) }
}

/// Copy packed-factor rows, columns and element count into three usize values.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_shape_f64(
    input: *const CigletRsLuF64,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::shape(input, output, count) }
}

/// Copy every column-major packed factor; L has an implicit unit diagonal.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_copy_packed_f64(
    input: *const CigletRsLuF64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::packed(input, output, count) }
}

/// Copy the complete ordered row-swap plan; count equals packed rows.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_copy_row_swaps_f64(
    input: *const CigletRsLuF64,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::swaps(input, output, count) }
}

/// Solve with all native row swaps and substitution order, retaining the right-hand side.
///
/// # Safety
/// Retain correctly typed live owners from this API through every call, excluding
/// release and concurrent mutation. Mutable owner pointers require exclusive access.
/// Nonempty inputs are initialized stable single-allocation readable storage. Outputs
/// are aligned exclusively writable storage independent of owner allocations.
/// Complete staged vector results permit input/output overlap after borrowing ends.
/// Ownership slots are initialized; retain older ownership before successful
/// replacement, which does not implicitly release it. Failures retain outputs
/// and matrix owners. Native clone allocation/abort limitations still apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_lu_solve_f64(
    handle: *const CigletRsLuF64,
    input: *const f64,
    input_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward caller typed ownership, exclusive mutation and buffer contracts.
    unsafe { matrix_lu::solve(handle, input, input_count, output, output_count) }
}
