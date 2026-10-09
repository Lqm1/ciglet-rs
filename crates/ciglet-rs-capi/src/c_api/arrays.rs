//! Shared implementation for typed, opaque, Rust-owned array handles.
use super::{boundary, range};
use crate::{Error, array::Array2, statistics::Scalar};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Opaque binary32 array. Obtain and release it through its typed C functions.
pub struct CigletRsArrayF32(Array2<f32>);
/// Opaque binary64 array. Obtain and release it through its typed C functions.
pub struct CigletRsArrayF64(Array2<f64>);

pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + Default + RefUnwindSafe + UnwindSafe;
    fn new(array: Array2<Self::Value>) -> Self;
    fn array(&self) -> &Array2<Self::Value>;
    fn array_mut(&mut self) -> &mut Array2<Self::Value>;
}
impl Handle for CigletRsArrayF32 {
    type Value = f32;
    fn new(array: Array2<f32>) -> Self {
        Self(array)
    }
    fn array(&self) -> &Array2<f32> {
        &self.0
    }
    fn array_mut(&mut self) -> &mut Array2<f32> {
        &mut self.0
    }
}
impl Handle for CigletRsArrayF64 {
    type Value = f64;
    fn new(array: Array2<f64>) -> Self {
        Self(array)
    }
    fn array(&self) -> &Array2<f64> {
        &self.0
    }
    fn array_mut(&mut self) -> &mut Array2<f64> {
        &mut self.0
    }
}

fn length<T>(rows: usize, columns: usize) -> Result<usize, u32> {
    let length = rows.checked_mul(columns).ok_or(2_u32)?;
    if length > i32::MAX as usize || length > isize::MAX as usize / std::mem::size_of::<T>() {
        return Err(2);
    }
    Ok(length)
}

unsafe fn read<H: Handle, R>(
    handle: *const H,
    operation: impl FnOnce(&Array2<H::Value>) -> Result<R, Error> + UnwindSafe,
) -> Result<R, u32> {
    range(handle, 1)?;
    boundary(|| {
        // SAFETY: Caller supplies a live handle of the correct precision and
        // keeps it unmodified and alive until this temporary borrow ends.
        operation(unsafe { &*handle }.array())
    })
}

unsafe fn mutate<H: Handle>(
    handle: *mut H,
    operation: impl FnOnce(&mut Array2<H::Value>) -> Result<(), Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Caller grants exclusive access to a live, correctly typed
        // handle. The temporary mutable reference is created inside the boundary.
        operation(unsafe { &mut *handle }.array_mut())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

unsafe fn publish<H>(output: *mut *mut H, result: Result<Box<H>, u32>) -> u32 {
    match result {
        Ok(object) => {
            // SAFETY: Every caller checked and guarantees writable pointer-slot
            // storage. Ownership transfers only after all computation succeeds.
            unsafe { output.write(Box::into_raw(object)) };
            0
        }
        Err(status) => status,
    }
}

unsafe fn create<H: Handle>(
    output: *mut *mut H,
    operation: impl FnOnce() -> Result<Array2<H::Value>, Error> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    let result = boundary(|| operation().map(H::new).map(Box::new));
    // SAFETY: Output validation precedes construction; caller supplies its slot.
    unsafe { publish(output, result) }
}

pub(super) unsafe fn filled<H: Handle>(
    rows: usize,
    columns: usize,
    value: H::Value,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = length::<H::Value>(rows, columns) {
        return status;
    }
    // SAFETY: Forward the caller's writable pointer-slot contract.
    unsafe { create(output, || Array2::filled(rows, columns, value)) }
}

pub(super) unsafe fn from_slice<H: Handle>(
    rows: usize,
    columns: usize,
    input: *const H::Value,
    count: usize,
    output: *mut *mut H,
) -> u32 {
    match length::<H::Value>(rows, columns) {
        Ok(expected) if expected == count => (),
        Ok(_) => return 2,
        Err(status) => return status,
    }
    if let Err(status) = range(input, count) {
        return status;
    }
    // SAFETY: Caller input is one initialized, stable allocation; output is a
    // valid pointer slot. Empty input does not form a raw null slice.
    unsafe {
        create(output, || {
            let values = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input, count)
            };
            Array2::from_slice(rows, columns, values)
        })
    }
}

pub(super) unsafe fn from_rows<H: Handle>(
    rows: *const *const H::Value,
    counts: *const usize,
    row_count: usize,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(rows, row_count).and_then(|()| range(counts, row_count)) {
        return status;
    }
    let mut columns = 0;
    for index in 0..row_count {
        // SAFETY: Caller supplies initialized pointer/count tables of row_count
        // entries in single allocations and keeps them stable during this call.
        let (row, count) = unsafe { (rows.add(index).read(), counts.add(index).read()) };
        if index == 0 {
            columns = count;
        }
        if count != columns {
            return 3;
        }
        if let Err(status) = range(row, count) {
            return status;
        }
    }
    if let Err(status) = length::<H::Value>(row_count, columns) {
        return status;
    }
    // SAFETY: Tables and every initialized row range were checked above; caller
    // preserves them until construction has copied the values into Rust storage.
    unsafe {
        create(output, || {
            let mut slices = crate::vector::buffer(row_count)?;
            for index in 0..row_count {
                let row = rows.add(index).read();
                slices.push(if columns == 0 {
                    &[][..]
                } else {
                    std::slice::from_raw_parts(row, columns)
                });
            }
            Array2::from_rows(&slices)
        })
    }
}

pub(super) unsafe fn cloned<H: Handle>(
    handle: *const H,
    output: *mut *mut H,
    transpose: bool,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller provides a live immutable handle and independent output slot.
    let result = unsafe {
        read(handle, |array| {
            let copy = if transpose {
                array.transposed()?
            } else {
                array.try_clone()?
            };
            Ok(Box::new(H::new(copy)))
        })
    };
    // SAFETY: Output validation precedes the operation and the caller owns it.
    unsafe { publish(output, result) }
}

pub(super) unsafe fn release<H: Handle>(slot: *mut *mut H) -> u32 {
    if let Err(status) = range(slot.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller provides an initialized, exclusively accessed pointer slot.
    let handle = unsafe { slot.read() };
    if handle.is_null() {
        return 0;
    }
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: The handle was allocated by Box::into_raw in this library and
    // caller transfers sole ownership, without outstanding borrows or aliases.
    unsafe {
        slot.write(std::ptr::null_mut());
        drop(Box::from_raw(handle));
    }
    0
}

pub(super) unsafe fn shape<H: Handle>(handle: *const H, output: *mut usize, count: usize) -> u32 {
    if count != 3 {
        return 2;
    }
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    // SAFETY: Caller grants an immutable live handle for this operation.
    match unsafe {
        read(handle, |array| {
            Ok([array.rows(), array.columns(), array.len()])
        })
    } {
        Ok(dimensions) => {
            // SAFETY: Caller output is writable for three size_t entries and is
            // independent of the object's private allocation.
            unsafe { std::ptr::copy_nonoverlapping(dimensions.as_ptr(), output, 3) };
            0
        }
        Err(status) => status,
    }
}

pub(super) unsafe fn copy_values<H: Handle>(
    handle: *const H,
    output: *mut H::Value,
    count: usize,
    row: Option<usize>,
) -> u32 {
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    // SAFETY: Caller grants a live immutable handle; returned metadata is owned.
    let expected = unsafe {
        read(handle, |array| match row {
            Some(row) => array
                .row(row)
                .map(<[H::Value]>::len)
                .ok_or(Error("array row is out of range")),
            None => Ok(array.len()),
        })
    };
    match expected {
        Ok(expected) if expected == count => (),
        Ok(_) => return 2,
        Err(status) => return status,
    }
    // SAFETY: Caller keeps the immutable handle alive throughout both reads.
    let result = unsafe {
        read(handle, |array| match row {
            None => array.flatten(),
            Some(row) => {
                let values = array.row(row).ok_or(Error("array row is out of range"))?;
                let mut copy = crate::vector::buffer(count)?;
                copy.extend_from_slice(values);
                Ok(copy)
            }
        })
    };
    match result {
        Ok(values) => {
            if values.len() != count {
                return 3;
            }
            if count != 0 {
                // SAFETY: Owned result does not overlap valid caller output.
                unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, count) };
            }
            0
        }
        Err(status) => status,
    }
}

pub(super) unsafe fn get<H: Handle>(
    handle: *const H,
    row: usize,
    column: usize,
    output: *mut H::Value,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller grants an immutable live handle, with independent output.
    match unsafe {
        read(handle, |array| {
            array
                .get(row, column)
                .copied()
                .ok_or(Error("array element is out of range"))
        })
    } {
        Ok(value) => {
            // SAFETY: Caller output is valid and writable for one scalar.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}

pub(super) unsafe fn set<H: Handle>(
    handle: *mut H,
    row: usize,
    column: usize,
    value: H::Value,
) -> u32 {
    // SAFETY: Caller grants exclusive access to a live, correctly typed handle.
    unsafe {
        mutate(handle, |array| {
            *array
                .get_mut(row, column)
                .ok_or(Error("array element is out of range"))? = value;
            Ok(())
        })
    }
}

pub(super) unsafe fn write_values<H: Handle>(
    handle: *mut H,
    input: *const H::Value,
    count: usize,
    row: Option<usize>,
) -> u32 {
    if let Err(status) = range(input, count) {
        return status;
    }
    // SAFETY: Caller keeps the exclusively held handle alive between validation
    // and mutation. Input is independent of its private allocation.
    let expected = unsafe {
        read(handle.cast_const(), |array| match row {
            Some(row) => array
                .row(row)
                .map(<[H::Value]>::len)
                .ok_or(Error("array row is out of range")),
            None => Ok(array.len()),
        })
    };
    match expected {
        Ok(expected) if expected == count => (),
        Ok(_) => return 2,
        Err(status) => return status,
    }
    // SAFETY: Caller grants exclusive object access and stable initialized input.
    unsafe {
        mutate(handle, |array| {
            let destination = match row {
                Some(row) => array
                    .row_mut(row)
                    .ok_or(Error("array row is out of range"))?,
                None => array.as_mut_slice(),
            };
            if destination.len() != count {
                return Err(Error("array input length changed"));
            }
            if count != 0 {
                // Input and private object storage must be disjoint by contract.
                std::ptr::copy_nonoverlapping(input, destination.as_mut_ptr(), count);
            }
            Ok(())
        })
    }
}

pub(super) unsafe fn reshape<H: Handle>(handle: *mut H, rows: usize, columns: usize) -> u32 {
    // SAFETY: Caller grants exclusive access. Native reshape validates before
    // changing metadata and performs no allocation or user callbacks.
    unsafe { mutate(handle, |array| array.reshape(rows, columns)) }
}

pub(super) unsafe fn transpose<H: Handle>(handle: *mut H) -> u32 {
    // SAFETY: Caller grants exclusive access. Build the entire transposed value
    // before committing, retaining the original on errors or caught panics.
    unsafe {
        mutate(handle, |array| {
            let transposed = array.transposed()?;
            *array = transposed;
            Ok(())
        })
    }
}
