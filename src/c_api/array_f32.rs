//! Typed binary32 entry points for the shared opaque-array implementation.
use super::{CigletRsArrayF32, arrays};

/// Create an owned filled array; use value zero to create zeros.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_filled_f32(
    rows: usize,
    columns: usize,
    value: f32,
    output: *mut *mut CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::filled(rows, columns, value, output) }
}

/// Copy a row-major buffer into an independent owned array.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_from_slice_f32(
    rows: usize,
    columns: usize,
    input: *const f32,
    count: usize,
    output: *mut *mut CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::from_slice(rows, columns, input, count, output) }
}

/// Copy pointer/count row tables; ragged rows return status three.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_from_rows_f32(
    rows: *const *const f32,
    counts: *const usize,
    row_count: usize,
    output: *mut *mut CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::from_rows(rows, counts, row_count, output) }
}

/// Create an independent copy, leaving the original unchanged.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_clone_f32(
    handle: *const CigletRsArrayF32,
    output: *mut *mut CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::cloned(handle, output, false) }
}

/// Create an independent transposed copy.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_transposed_f32(
    handle: *const CigletRsArrayF32,
    output: *mut *mut CigletRsArrayF32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::cloned(handle, output, true) }
}

/// Release a handle and clear its slot. A null handle in a valid slot succeeds.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_release_f32(slot: *mut *mut CigletRsArrayF32) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::release(slot) }
}

/// Write rows, columns and element count to exactly three size_t entries.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_shape_f32(
    handle: *const CigletRsArrayF32,
    output: *mut usize,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::shape(handle, output, count) }
}

/// Copy all row-major values to an equally sized caller-owned buffer.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_copy_flat_f32(
    handle: *const CigletRsArrayF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::copy_values(handle, output, count, None) }
}

/// Copy one row to a caller-owned buffer with matching column count.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_copy_row_f32(
    handle: *const CigletRsArrayF32,
    row: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::copy_values(handle, output, count, Some(row)) }
}

/// Write one array element; out-of-range coordinates return status three.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_get_f32(
    handle: *const CigletRsArrayF32,
    row: usize,
    column: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::get(handle, row, column, output) }
}

/// Replace one array element; out-of-range coordinates return status three.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_set_f32(
    handle: *mut CigletRsArrayF32,
    row: usize,
    column: usize,
    value: f32,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::set(handle, row, column, value) }
}

/// Replace all row-major values from an equally sized caller buffer.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_write_flat_f32(
    handle: *mut CigletRsArrayF32,
    input: *const f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::write_values(handle, input, count, None) }
}

/// Replace one row from a buffer with matching column count.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_write_row_f32(
    handle: *mut CigletRsArrayF32,
    row: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::write_values(handle, input, count, Some(row)) }
}

/// Change shape while retaining element count and row-major sequence.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_reshape_f32(
    handle: *mut CigletRsArrayF32,
    rows: usize,
    columns: usize,
) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::reshape(handle, rows, columns) }
}

/// Transpose the owned array, committing only after a complete copy succeeds.
/// Errors leave outputs and the existing array unchanged.
///
/// # Safety
/// Null/misaligned handles are rejected except for documented null cases.
/// Other handles must be live objects from this precision's constructors. Mutations
/// and release require exclusive access; reads forbid concurrent mutations.
/// Nonempty caller inputs/tables must be initialized single-allocation ranges,
/// stable throughout the call. Outputs must be writable for declared types and
/// lengths. Every caller buffer/slot must be independent of private object storage.
/// Release takes an initialized pointer slot and invalidates all handle aliases.
/// Constructor outputs transfer ownership and must later be released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_array_transpose_f32(handle: *mut CigletRsArrayF32) -> u32 {
    // SAFETY: Forward the documented handle and caller-buffer contracts.
    unsafe { arrays::transpose(handle) }
}
