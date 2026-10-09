//! Immutable owned byte payloads with checked partial copies.
use super::{range, scalar_value_result};
/// Opaque byte payload owned by this library; callers copy bytes without allocator transfer.
pub struct CigletRsBytes(pub(super) Vec<u8>);
/// Read byte payload length.
///
/// # Safety
/// Owner is live and retained; output is independent aligned exclusively writable usize storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bytes_length(
    handle: *const CigletRsBytes,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained immutable owner and independent output.
    unsafe { scalar_value_result(output, || Ok((&*handle).0.len())) }
}
/// Copy a checked byte interval. Count zero permits null output; offset may equal length.
/// Partial copies allow retrieving payloads larger than the single-copy range limit.
///
/// # Safety
/// Owner remains live and immutable. Nonempty output is independent exclusively
/// writable single-allocation storage for count bytes, without overlap with the owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bytes_copy(
    handle: *const CigletRsBytes,
    offset: usize,
    output: *mut u8,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(output.cast_const(), count)) {
        return status;
    }
    // SAFETY: Retained immutable owner; checked byte interval and independent output.
    unsafe {
        let bytes = &(&*handle).0;
        let Some(end) = offset.checked_add(count).filter(|&end| end <= bytes.len()) else {
            return 2;
        };
        if count != 0 {
            std::ptr::copy_nonoverlapping(bytes[offset..end].as_ptr(), output, count);
        }
    }
    0
}
/// Clone a byte payload into an independent owner.
///
/// # Safety
/// Owner remains live; output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bytes_clone(
    handle: *const CigletRsBytes,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Input borrow ends before new owner publication to independent output.
    unsafe {
        scalar_value_result(output, || {
            Ok(Box::into_raw(Box::new(CigletRsBytes((&*handle).0.clone()))))
        })
    }
}
/// Release and clear a byte ownership slot; null owner is already released.
///
/// # Safety
/// Slot is aligned initialized exclusively writable pointer storage. Its nonnull
/// owner was created here and is exclusively transferred with no outstanding users.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_bytes_release(slot: *mut *mut CigletRsBytes) -> u32 {
    if let Err(status) = range(slot.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller transfers the live Box allocation through its exclusive slot.
    unsafe {
        let handle = slot.read();
        if handle.is_null() {
            return 0;
        }
        if let Err(status) = range(handle.cast_const(), 1) {
            return status;
        }
        slot.write(std::ptr::null_mut());
        drop(Box::from_raw(handle));
    }
    0
}
