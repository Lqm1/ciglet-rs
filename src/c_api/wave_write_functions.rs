//! Complete native PCM encoding entry points.
use super::{
    byte_payload::CigletRsBytes, wave_encoding, wave_path::CigletRsPath,
    wave_write_options::CigletRsWaveWriteOptions, wave_writer_stream::CigletRsWriteStream,
};

/// Encode one mono PCM WAVE to an immutable owned byte payload. Errors retain output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Nonempty samples are aligned initialized stable single-allocation storage for count scalars. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_encode_f32(
    samples: *const f32,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::memory(samples, count, sample_rate, options, output) }
}

/// Write mono PCM through synchronous caller-owned callbacks without closing or flushing. Invalid parameters cause no writes; I/O failure may leave partial external output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Samples are initialized stable aligned storage for count scalars. Callback/context remain valid without unwinding throughout the call; write consumes and reports at most the supplied byte count. Callback may mutate independent context, but must not modify or create conflicting references to borrowed samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_stream_f32(
    stream: CigletRsWriteStream,
    samples: *const f32,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::stream(stream, samples, count, sample_rate, options) }
}

/// Write and flush a mono PCM file, validating before creation or truncation. I/O failure may leave partial file output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Sample and path bytes are initialized stable single-allocation storage for their counts; nonempty samples are aligned. Path uses the declared counted encoding without terminator.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_file_f32(
    path: CigletRsPath,
    samples: *const f32,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::file(path, samples, count, sample_rate, options) }
}

/// Encode one mono PCM WAVE to an immutable owned byte payload. Errors retain output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Nonempty samples are aligned initialized stable single-allocation storage for count scalars. Output is an independent aligned exclusively writable ownership slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_encode_f64(
    samples: *const f64,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
    output: *mut *mut CigletRsBytes,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::memory(samples, count, sample_rate, options, output) }
}

/// Write mono PCM through synchronous caller-owned callbacks without closing or flushing. Invalid parameters cause no writes; I/O failure may leave partial external output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Samples are initialized stable aligned storage for count scalars. Callback/context remain valid without unwinding throughout the call; write consumes and reports at most the supplied byte count. Callback may mutate independent context, but must not modify or create conflicting references to borrowed samples.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_stream_f64(
    stream: CigletRsWriteStream,
    samples: *const f64,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::stream(stream, samples, count, sample_rate, options) }
}

/// Write and flush a mono PCM file, validating before creation or truncation. I/O failure may leave partial file output.
/// Empty samples may use null input. Native quantization/clipping and padding are retained.
///
/// # Safety
/// Sample and path bytes are initialized stable single-allocation storage for their counts; nonempty samples are aligned. Path uses the declared counted encoding without terminator.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_wave_write_file_f64(
    path: CigletRsPath,
    samples: *const f64,
    count: usize,
    sample_rate: u32,
    options: CigletRsWaveWriteOptions,
) -> u32 {
    // SAFETY: Forward documented stable input, callback and exclusive output contracts.
    unsafe { wave_encoding::file(path, samples, count, sample_rate, options) }
}
