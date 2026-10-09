//! Explicit f32/f64 entry points for all interpolation variants.
use super::interpolation::{self, Uniform};

/// Scalar interpolation and extrapolation with native overflow fallback.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_between_f32(
    start: f32,
    end: f32,
    ratio: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe { interpolation::between(start, end, ratio, output) }
}

/// Interpolate increasing irregular coordinates, unordered queries and clamped endpoints.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_linear_f32(
    coordinates: *const f32,
    values: *const f32,
    count: usize,
    positions: *const f32,
    query_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::linear(
            coordinates,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
        )
    }
}

/// Uniform coordinates occupy the half-open bounds; retain near-final-sample clamp.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_uniform_f32(
    start: f32,
    end: f32,
    values: *const f32,
    count: usize,
    positions: *const f32,
    query_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Linear,
        )
    }
}

/// Sinc interpolation with corrected IncludeFirst boundary and Stable kernel.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_f32(
    start: f32,
    end: f32,
    values: *const f32,
    count: usize,
    positions: *const f32,
    query_count: usize,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc {
                boundary: 0,
                kernel: 0,
            },
        )
    }
}

/// Boundary zero IncludeFirst, one LegacySkipFirst; original Legacy kernel.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_with_f32(
    start: f32,
    end: f32,
    values: *const f32,
    count: usize,
    positions: *const f32,
    query_count: usize,
    boundary: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc {
                boundary,
                kernel: 1,
            },
        )
    }
}

/// Independent boundary and kernel selectors. Kernel zero Stable, one Legacy.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_with_kernel_f32(
    start: f32,
    end: f32,
    values: *const f32,
    count: usize,
    positions: *const f32,
    query_count: usize,
    boundary: u32,
    kernel: u32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc { boundary, kernel },
        )
    }
}

/// Replace sentinel gaps, including NaN sentinel; all missing yields zero.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_fill_missing_f32(
    values: *const f32,
    count: usize,
    blank: f32,
    output: *mut f32,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe { interpolation::fill(values, count, blank, output, output_count) }
}

/// Scalar interpolation and extrapolation with native overflow fallback.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_between_f64(
    start: f64,
    end: f64,
    ratio: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe { interpolation::between(start, end, ratio, output) }
}

/// Interpolate increasing irregular coordinates, unordered queries and clamped endpoints.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_linear_f64(
    coordinates: *const f64,
    values: *const f64,
    count: usize,
    positions: *const f64,
    query_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::linear(
            coordinates,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
        )
    }
}

/// Uniform coordinates occupy the half-open bounds; retain near-final-sample clamp.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_uniform_f64(
    start: f64,
    end: f64,
    values: *const f64,
    count: usize,
    positions: *const f64,
    query_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Linear,
        )
    }
}

/// Sinc interpolation with corrected IncludeFirst boundary and Stable kernel.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_f64(
    start: f64,
    end: f64,
    values: *const f64,
    count: usize,
    positions: *const f64,
    query_count: usize,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc {
                boundary: 0,
                kernel: 0,
            },
        )
    }
}

/// Boundary zero IncludeFirst, one LegacySkipFirst; original Legacy kernel.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_with_f64(
    start: f64,
    end: f64,
    values: *const f64,
    count: usize,
    positions: *const f64,
    query_count: usize,
    boundary: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc {
                boundary,
                kernel: 1,
            },
        )
    }
}

/// Independent boundary and kernel selectors. Kernel zero Stable, one Legacy.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_sinc_with_kernel_f64(
    start: f64,
    end: f64,
    values: *const f64,
    count: usize,
    positions: *const f64,
    query_count: usize,
    boundary: u32,
    kernel: u32,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe {
        interpolation::uniform(
            start,
            end,
            values,
            count,
            positions,
            query_count,
            output,
            output_count,
            Uniform::Sinc { boundary, kernel },
        )
    }
}

/// Replace sentinel gaps, including NaN sentinel; all missing yields zero.
/// Native operations and complete output staging retain source precision and
/// documented corrections. Failures leave outputs unchanged. Null empty buffers
/// are never borrowed. Unknown compatibility selectors return three.
///
/// # Safety
/// Nonempty inputs are aligned initialized stable readable single-allocation
/// storage. Outputs are aligned exclusively writable single-allocation storage.
/// Complete staging allows input/output overlap after every input borrow ends.
/// Scalar outputs provide one element. Arrays require their complete stated counts.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_interpolate_fill_missing_f64(
    values: *const f64,
    count: usize,
    blank: f64,
    output: *mut f64,
    output_count: usize,
) -> u32 {
    // SAFETY: Forward checked caller storage contracts and complete native staging.
    unsafe { interpolation::fill(values, count, blank, output, output_count) }
}
