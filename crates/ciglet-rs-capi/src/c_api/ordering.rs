//! Transactional selection and sorting with distinct output regions.
use super::{boundary, disjoint_outputs, range};
use crate::statistics::Scalar;

unsafe fn select_result<T: Scalar + std::panic::RefUnwindSafe>(
    values: *mut T,
    length: usize,
    index: usize,
    selected: *mut T,
) -> u32 {
    if let Err(status) = disjoint_outputs(values.cast_const(), length, selected.cast_const(), 1) {
        return status;
    }
    if index >= length {
        return 3;
    }
    let result = boundary(|| {
        // SAFETY: The caller provides initialized single-allocation input, stable
        // until the staged operation completes; checked length is nonzero here.
        let input = unsafe { std::slice::from_raw_parts(values, length) };
        let mut copy = crate::vector::buffer(length)?;
        copy.extend_from_slice(input);
        let selected = crate::statistics::select_nth(&mut copy, index)?;
        Ok((copy, selected))
    });
    match result {
        Ok((partitioned, result)) => {
            if partitioned.len() != length {
                return 3;
            }
            // SAFETY: Input borrow ended. The owned partition cannot overlap
            // caller storage. Both writable outputs were checked to be disjoint.
            unsafe {
                std::ptr::copy_nonoverlapping(partitioned.as_ptr(), values, length);
                selected.write(result);
            }
            0
        }
        Err(status) => status,
    }
}

unsafe fn sort_result<T: Scalar + std::panic::RefUnwindSafe>(
    input: *const T,
    length: usize,
    sorted: *mut T,
    sorted_length: usize,
    indices: *mut usize,
    indices_length: usize,
) -> u32 {
    if length != sorted_length || length != indices_length {
        return 2;
    }
    if let Err(status) = range(input, length).and_then(|()| {
        disjoint_outputs(
            sorted.cast_const(),
            sorted_length,
            indices.cast_const(),
            indices_length,
        )
    }) {
        return status;
    }
    let result = boundary(|| {
        let input = if length == 0 {
            &[]
        } else {
            // SAFETY: The caller provides initialized single-allocation input,
            // stable until computation ends; its range has been checked.
            unsafe { std::slice::from_raw_parts(input, length) }
        };
        Ok(crate::statistics::sort(input))
    });
    match result {
        Ok((values, positions)) => {
            if values.len() != length || positions.len() != length {
                return 3;
            }
            if length != 0 {
                // SAFETY: Input borrow ended; owned result buffers cannot overlap
                // caller outputs, which are valid, writable and mutually disjoint.
                unsafe {
                    std::ptr::copy_nonoverlapping(values.as_ptr(), sorted, length);
                    std::ptr::copy_nonoverlapping(positions.as_ptr(), indices, length);
                }
            }
            0
        }
        Err(status) => status,
    }
}

/// Partition values by IEEE total order and return the value at index.
/// Out-of-range index returns status three. Outputs must be disjoint; overlap
/// returns status two. Any failure leaves both outputs unchanged.
///
/// # Safety
/// Values must contain length initialized elements in a single allocation,
/// readable during computation and exclusively writable on commit. Selected
/// must be exclusively writable for one value. No conflicting concurrent access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_select_nth_f32(
    values: *mut f32,
    length: usize,
    index: usize,
    selected: *mut f32,
) -> u32 {
    // SAFETY: Forward the caller's documented input/output contracts.
    unsafe { select_result(values, length, index, selected) }
}

/// Stable IEEE total-order sort with original indices for ties.
/// Both output lengths must equal the input length; empty ranges accept null.
/// Sorted/indices outputs must be disjoint; either may overlap input. Invalid
/// lengths or overlapping outputs return status two without any writes.
///
/// # Safety
/// Nonempty input must be initialized and readable in one allocation, stable
/// until computation ends. Both nonempty outputs must be exclusively writable
/// for their stated lengths and element types. No conflicting concurrent access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sort_f32(
    input: *const f32,
    length: usize,
    sorted: *mut f32,
    sorted_length: usize,
    indices: *mut usize,
    indices_length: usize,
) -> u32 {
    // SAFETY: Forward the caller's documented input/output contracts.
    unsafe {
        sort_result(
            input,
            length,
            sorted,
            sorted_length,
            indices,
            indices_length,
        )
    }
}

/// Partition values by IEEE total order and return the value at index.
/// Out-of-range index returns status three. Outputs must be disjoint; overlap
/// returns status two. Any failure leaves both outputs unchanged.
///
/// # Safety
/// Values must contain length initialized elements in a single allocation,
/// readable during computation and exclusively writable on commit. Selected
/// must be exclusively writable for one value. No conflicting concurrent access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_select_nth_f64(
    values: *mut f64,
    length: usize,
    index: usize,
    selected: *mut f64,
) -> u32 {
    // SAFETY: Forward the caller's documented input/output contracts.
    unsafe { select_result(values, length, index, selected) }
}

/// Stable IEEE total-order sort with original indices for ties.
/// Both output lengths must equal the input length; empty ranges accept null.
/// Sorted/indices outputs must be disjoint; either may overlap input. Invalid
/// lengths or overlapping outputs return status two without any writes.
///
/// # Safety
/// Nonempty input must be initialized and readable in one allocation, stable
/// until computation ends. Both nonempty outputs must be exclusively writable
/// for their stated lengths and element types. No conflicting concurrent access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_sort_f64(
    input: *const f64,
    length: usize,
    sorted: *mut f64,
    sorted_length: usize,
    indices: *mut usize,
    indices_length: usize,
) -> u32 {
    // SAFETY: Forward the caller's documented input/output contracts.
    unsafe {
        sort_result(
            input,
            length,
            sorted,
            sorted_length,
            indices,
            indices_length,
        )
    }
}
