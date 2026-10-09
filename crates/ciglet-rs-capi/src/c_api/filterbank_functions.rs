//! Typed filterbank ownership, configuration and application entry points.
use super::filterbank::{self, CigletRsFilterBankF32, CigletRsFilterBankF64, Frames};
use crate::filterbank::{FilterBank, MelOptions};

/// Create a zero-response bank. Zero bins/channels and nonfinite metadata are allowed.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_empty_f32(
    bins: usize,
    nyquist_hz: f32,
    channels: usize,
    output: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::create(output, || FilterBank::empty(bins, nyquist_hz, channels)) }
}

/// Create a Mel bank with explicit scale and minimum width. Native bounds are retained.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_mel_f32(
    bins: usize,
    nyquist_hz: f32,
    channels: usize,
    minimum_hz: f32,
    maximum_hz: f32,
    scale: f32,
    minimum_width_hz: f32,
    output: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::create(output, || {
            FilterBank::mel(
                bins,
                nyquist_hz,
                channels,
                MelOptions {
                    minimum_hz,
                    maximum_hz,
                    scale,
                    minimum_width_hz,
                },
            )
        })
    }
}

/// Create a Mel bank with unit scale and 400-Hz minimum width.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_mel_default_f32(
    bins: usize,
    nyquist_hz: f32,
    channels: usize,
    minimum_hz: f32,
    maximum_hz: f32,
    output: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::create(output, || {
            FilterBank::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz)
        })
    }
}

/// Create a PLP bank, preserving original masks and weighting.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_plp_f32(
    bins: usize,
    nyquist_hz: f32,
    channels: usize,
    output: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::create(output, || FilterBank::plp(bins, nyquist_hz, channels)) }
}

/// Copy the complete owned bank into a separate handle.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_clone_f32(
    handle: *const CigletRsFilterBankF32,
    output: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::cloned(handle, output) }
}

/// Release sole ownership and clear the slot. A null stored handle succeeds.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_release_f32(
    slot: *mut *mut CigletRsFilterBankF32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { super::fourier::release(slot) }
}

/// Copy bins/channels to exactly two pointer-sized indices.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_shape_f32(
    handle: *const CigletRsFilterBankF32,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::shape(handle, output, output_count) }
}

/// Copy Nyquist metadata without filtering nonfinite values.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_nyquist_f32(
    handle: *const CigletRsFilterBankF32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::nyquist(handle, output) }
}

/// Set inert Nyquist metadata, retaining all sample bits. Requires exclusive handle access.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_set_nyquist_f32(
    handle: *mut CigletRsFilterBankF32,
    value: f32,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::set_nyquist(handle, value) }
}

/// Copy a complete channel response; output count must equal bins.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_response_f32(
    handle: *const CigletRsFilterBankF32,
    channel: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::response(handle, channel, output, output_count) }
}

/// Copy a channel half-open start/end support into exactly two indices.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_support_f32(
    handle: *const CigletRsFilterBankF32,
    channel: usize,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::support(handle, channel, output, output_count) }
}

/// Replace a complete channel and half-open support with exclusive access.
/// Signed weights and nonfinite values outside the active support are retained.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_set_channel_f32(
    handle: *mut CigletRsFilterBankF32,
    channel: usize,
    response: *const f32,
    response_count: usize,
    start: usize,
    end: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::set_channel(handle, channel, response, response_count, start, end) }
}

/// Apply one spectrum. Compression zero logarithmic, one PLP. Output count equals channels.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_f32(
    handle: *const CigletRsFilterBankF32,
    spectrum: *const f32,
    input_count: usize,
    compression: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Single,
        )
    }
}

/// Apply flattened frames inferred from positive bins. Compression zero logarithmic, one PLP.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_spectrogram_f32(
    handle: *const CigletRsFilterBankF32,
    spectrum: *const f32,
    input_count: usize,
    compression: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Inferred,
        )
    }
}

/// Apply an explicit frame count, including zero-bin frames. Compression zero logarithmic, one PLP.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_frames_f32(
    handle: *const CigletRsFilterBankF32,
    spectrum: *const f32,
    input_count: usize,
    frames: usize,
    compression: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Explicit(frames),
        )
    }
}

/// Create a zero-response bank. Zero bins/channels and nonfinite metadata are allowed.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_empty_f64(
    bins: usize,
    nyquist_hz: f64,
    channels: usize,
    output: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::create(output, || FilterBank::empty(bins, nyquist_hz, channels)) }
}

/// Create a Mel bank with explicit scale and minimum width. Native bounds are retained.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_mel_f64(
    bins: usize,
    nyquist_hz: f64,
    channels: usize,
    minimum_hz: f64,
    maximum_hz: f64,
    scale: f64,
    minimum_width_hz: f64,
    output: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::create(output, || {
            FilterBank::mel(
                bins,
                nyquist_hz,
                channels,
                MelOptions {
                    minimum_hz,
                    maximum_hz,
                    scale,
                    minimum_width_hz,
                },
            )
        })
    }
}

/// Create a Mel bank with unit scale and 400-Hz minimum width.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_mel_default_f64(
    bins: usize,
    nyquist_hz: f64,
    channels: usize,
    minimum_hz: f64,
    maximum_hz: f64,
    output: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::create(output, || {
            FilterBank::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz)
        })
    }
}

/// Create a PLP bank, preserving original masks and weighting.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_plp_f64(
    bins: usize,
    nyquist_hz: f64,
    channels: usize,
    output: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::create(output, || FilterBank::plp(bins, nyquist_hz, channels)) }
}

/// Copy the complete owned bank into a separate handle.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_clone_f64(
    handle: *const CigletRsFilterBankF64,
    output: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::cloned(handle, output) }
}

/// Release sole ownership and clear the slot. A null stored handle succeeds.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_release_f64(
    slot: *mut *mut CigletRsFilterBankF64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { super::fourier::release(slot) }
}

/// Copy bins/channels to exactly two pointer-sized indices.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_shape_f64(
    handle: *const CigletRsFilterBankF64,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::shape(handle, output, output_count) }
}

/// Copy Nyquist metadata without filtering nonfinite values.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_nyquist_f64(
    handle: *const CigletRsFilterBankF64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::nyquist(handle, output) }
}

/// Set inert Nyquist metadata, retaining all sample bits. Requires exclusive handle access.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_set_nyquist_f64(
    handle: *mut CigletRsFilterBankF64,
    value: f64,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::set_nyquist(handle, value) }
}

/// Copy a complete channel response; output count must equal bins.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_response_f64(
    handle: *const CigletRsFilterBankF64,
    channel: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::response(handle, channel, output, output_count) }
}

/// Copy a channel half-open start/end support into exactly two indices.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_support_f64(
    handle: *const CigletRsFilterBankF64,
    channel: usize,
    output: *mut usize,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::support(handle, channel, output, output_count) }
}

/// Replace a complete channel and half-open support with exclusive access.
/// Signed weights and nonfinite values outside the active support are retained.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_set_channel_f64(
    handle: *mut CigletRsFilterBankF64,
    channel: usize,
    response: *const f64,
    response_count: usize,
    start: usize,
    end: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe { filterbank::set_channel(handle, channel, response, response_count, start, end) }
}

/// Apply one spectrum. Compression zero logarithmic, one PLP. Output count equals channels.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_f64(
    handle: *const CigletRsFilterBankF64,
    spectrum: *const f64,
    input_count: usize,
    compression: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Single,
        )
    }
}

/// Apply flattened frames inferred from positive bins. Compression zero logarithmic, one PLP.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_spectrogram_f64(
    handle: *const CigletRsFilterBankF64,
    spectrum: *const f64,
    input_count: usize,
    compression: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Inferred,
        )
    }
}

/// Apply an explicit frame count, including zero-bin frames. Compression zero logarithmic, one PLP.
/// Failures retain caller outputs; creation transfers ownership only on success.
/// Buffer counts are explicit, and nonempty buffers must be properly aligned.
///
/// # Safety
/// Handles must be live, correctly typed library allocations. Reading excludes
/// mutation/release; mutations and release require exclusive access. Owner slots
/// and buffers must be independent of the handle and its Rust-owned storage.
/// Inputs must be initialized and stable; outputs must be exclusively writable.
/// Application input/output buffers may overlap after input borrowing ends.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_filterbank_apply_frames_f64(
    handle: *const CigletRsFilterBankF64,
    spectrum: *const f64,
    input_count: usize,
    frames: usize,
    compression: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward the documented ownership, buffer and access contracts.
    unsafe {
        filterbank::apply(
            handle,
            spectrum,
            input_count,
            compression,
            output,
            output_count,
            Frames::Explicit(frames),
        )
    }
}
