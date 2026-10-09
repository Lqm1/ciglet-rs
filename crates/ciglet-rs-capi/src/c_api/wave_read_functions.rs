//! Complete owned WAVE decoding entry points.
use super::{
    wave_owner::{self, *},
    wave_path::CigletRsPath,
    wave_stream::CigletRsReadStream,
};

/// Decode one bounded WAVE at the selected byte position.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Bytes are initialized stable single-allocation storage for count bytes. Output is an independent aligned exclusively writable pointer slot for a new owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_decode_f32(
    bytes: *const u8,
    count: usize,
    start: u64,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF32,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::memory::<CigletRsWaveF32>(bytes, count, start, max_frames, output) }
}

/// Decode from the current position of a synchronous caller-owned stream. Stream position may change on error; callbacks are not retained or closed.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Both callbacks/context must remain callable and valid without unwinding or conflicting access for this entire call. Read reports initialized bytes <= capacity; seek reports its new absolute position. Output is an independent aligned exclusively writable ownership slot, untouched by callbacks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_read_stream_f32(
    stream: CigletRsReadStream,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF32,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::stream::<CigletRsWaveF32>(stream, max_frames, output) }
}

/// Decode a file using UTF-8 or lossless native path bytes.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Path bytes are initialized stable storage for byte_count bytes with no terminating NUL. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_read_file_f32(
    path: CigletRsPath,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF32,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::file::<CigletRsWaveF32>(path, max_frames, output) }
}

/// Clone complete waveform metadata and samples into an independent owner.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; output is an aligned independent exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_clone_f32(
    handle: *const CigletRsWaveF32,
    output: *mut *mut CigletRsWaveF32,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::clone_owner::<CigletRsWaveF32>(handle, output) }
}

/// Release an owner and clear its pointer slot; null owner is already released.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Slot is initialized aligned exclusively writable pointer storage. Its nonnull owner was created here and transfers exclusive ownership with no outstanding users.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_release_f32(slot: *mut *mut CigletRsWaveF32) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::release::<CigletRsWaveF32>(slot) }
}

/// Read all source metadata and mono sample count without modifying the owner.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; output is independently aligned exclusively writable storage for one metadata descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_info_f32(
    handle: *const CigletRsWaveF32,
    output: *mut CigletRsWaveInfo,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::info::<CigletRsWaveF32>(handle, output) }
}

/// Copy all decoded samples. Count must match exactly; empty output may be null.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; nonempty output is independently aligned exclusively writable single-allocation storage for count scalars.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_copy_samples_f32(
    handle: *const CigletRsWaveF32,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::samples::<CigletRsWaveF32>(handle, output, count) }
}

/// Decode one bounded WAVE at the selected byte position.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Bytes are initialized stable single-allocation storage for count bytes. Output is an independent aligned exclusively writable pointer slot for a new owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_decode_f64(
    bytes: *const u8,
    count: usize,
    start: u64,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF64,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::memory::<CigletRsWaveF64>(bytes, count, start, max_frames, output) }
}

/// Decode from the current position of a synchronous caller-owned stream. Stream position may change on error; callbacks are not retained or closed.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Both callbacks/context must remain callable and valid without unwinding or conflicting access for this entire call. Read reports initialized bytes <= capacity; seek reports its new absolute position. Output is an independent aligned exclusively writable ownership slot, untouched by callbacks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_read_stream_f64(
    stream: CigletRsReadStream,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF64,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::stream::<CigletRsWaveF64>(stream, max_frames, output) }
}

/// Decode a file using UTF-8 or lossless native path bytes.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Path bytes are initialized stable storage for byte_count bytes with no terminating NUL. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_read_file_f64(
    path: CigletRsPath,
    max_frames: usize,
    output: *mut *mut CigletRsWaveF64,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::file::<CigletRsWaveF64>(path, max_frames, output) }
}

/// Clone complete waveform metadata and samples into an independent owner.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; output is an aligned independent exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_clone_f64(
    handle: *const CigletRsWaveF64,
    output: *mut *mut CigletRsWaveF64,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::clone_owner::<CigletRsWaveF64>(handle, output) }
}

/// Release an owner and clear its pointer slot; null owner is already released.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Slot is initialized aligned exclusively writable pointer storage. Its nonnull owner was created here and transfers exclusive ownership with no outstanding users.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_release_f64(slot: *mut *mut CigletRsWaveF64) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::release::<CigletRsWaveF64>(slot) }
}

/// Read all source metadata and mono sample count without modifying the owner.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; output is independently aligned exclusively writable storage for one metadata descriptor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_info_f64(
    handle: *const CigletRsWaveF64,
    output: *mut CigletRsWaveInfo,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::info::<CigletRsWaveF64>(handle, output) }
}

/// Copy all decoded samples. Count must match exactly; empty output may be null.
/// Errors retain caller output; decoded owners retain stereo left-channel semantics.
///
/// # Safety
/// Handle is live and retained; nonempty output is independently aligned exclusively writable single-allocation storage for count scalars.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_copy_samples_f64(
    handle: *const CigletRsWaveF64,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward documented input, callback and exclusive output contracts.
    unsafe { wave_owner::samples::<CigletRsWaveF64>(handle, output, count) }
}
