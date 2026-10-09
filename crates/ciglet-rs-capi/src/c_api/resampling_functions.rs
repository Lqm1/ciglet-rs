//! Precision-specific resampling and allocation-free output length queries.
use super::resampling;

/// Query full output length without allocation; does not validate sample contents.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_output_length_f32(
    count: usize,
    ratio: f32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::length(count, ratio, output) }
}

/// Resample using corrected IncludeFirst boundary and Stable kernel.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_f32(
    input: *const f32,
    count: usize,
    ratio: f32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, 0, 0, output, output_count) }
}

/// Resample with boundary zero IncludeFirst, one LegacySkipFirst and original Legacy kernel.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_with_f32(
    input: *const f32,
    count: usize,
    ratio: f32,
    boundary: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, boundary, 1, output, output_count) }
}

/// Select boundary and kernel independently. Kernel zero Stable, one Legacy; unknown codes return three.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_with_kernel_f32(
    input: *const f32,
    count: usize,
    ratio: f32,
    boundary: u32,
    kernel: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, boundary, kernel, output, output_count) }
}

/// Query full output length without allocation; does not validate sample contents.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_output_length_f64(
    count: usize,
    ratio: f64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::length(count, ratio, output) }
}

/// Resample using corrected IncludeFirst boundary and Stable kernel.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_f64(
    input: *const f64,
    count: usize,
    ratio: f64,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, 0, 0, output, output_count) }
}

/// Resample with boundary zero IncludeFirst, one LegacySkipFirst and original Legacy kernel.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_with_f64(
    input: *const f64,
    count: usize,
    ratio: f64,
    boundary: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, boundary, 1, output, output_count) }
}

/// Select boundary and kernel independently. Kernel zero Stable, one Legacy; unknown codes return three.
/// Results commit only on success. Counts use the native scalar product and
/// f64 round, except ratio one preserves input length and sample bits.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage; outputs are aligned exclusively writable single-allocation storage.
/// Empty buffers permit null. Owned staging permits input/output overlap after
/// input borrowing ends. Length-query output is valid for one exclusive usize write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_resample_with_kernel_f64(
    input: *const f64,
    count: usize,
    ratio: f64,
    boundary: u32,
    kernel: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward retained caller buffers and checked staged output contracts.
    unsafe { resampling::resample(input, count, ratio, boundary, kernel, output, output_count) }
}
