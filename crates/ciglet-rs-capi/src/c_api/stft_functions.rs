//! Typed STFT ownership, variable frames, and regular-grid entry points.
use super::stft::{CigletRsStftF32, CigletRsStftF64};
use super::{fourier, stft, stft_types::*};
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analysis_options_default(
    output: *mut CigletRsStftAnalysisOptions,
) -> u32 {
    // SAFETY: Forward the documented exclusive independent output slot.
    unsafe { stft::default_options(output) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_create_f32(
    length: usize,
    window: u32,
    precision: u32,
    output: *mut *mut CigletRsStftF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::create(length, window, precision, output) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_release_f32(handle: *mut *mut CigletRsStftF32) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_f32(
    handle: *const CigletRsStftF32,
    input: CigletRsStftAnalysisInputF32,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::analyze(handle, input.into(), options, output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_synthesize_f32(
    handle: *const CigletRsStftF32,
    input: CigletRsStftSpectrumF32,
    options: CigletRsStftSynthesisOptionsF32,
    output: CigletRsStftSamplesF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::synthesize(handle, input.into(), options.into(), output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_regular_f32(
    signal: *const f32,
    count: usize,
    grid: CigletRsStftGrid,
    precision: u32,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::analyze_regular(signal, count, grid, precision, options, output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_regular_legacy_f32(
    signal: *const f32,
    count: usize,
    grid: CigletRsStftGrid,
    level: i32,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        stft::analyze_regular(
            signal,
            count,
            grid,
            match crate::window::Precision::from_legacy_level(level) {
                crate::window::Precision::Accurate => 0,
                crate::window::Precision::Approximate => 1,
            },
            options,
            output.into(),
        )
    }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_synthesize_regular_f32(
    input: CigletRsStftSpectrumF32,
    grid: CigletRsStftGrid,
    normalization: f32,
    nyquist: u32,
    output: CigletRsStftSamplesF32,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::synthesize_regular(input.into(), grid, normalization, nyquist, output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_create_f64(
    length: usize,
    window: u32,
    precision: u32,
    output: *mut *mut CigletRsStftF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::create(length, window, precision, output) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_release_f64(handle: *mut *mut CigletRsStftF64) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { fourier::release(handle) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_f64(
    handle: *const CigletRsStftF64,
    input: CigletRsStftAnalysisInputF64,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::analyze(handle, input.into(), options, output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_synthesize_f64(
    handle: *const CigletRsStftF64,
    input: CigletRsStftSpectrumF64,
    options: CigletRsStftSynthesisOptionsF64,
    output: CigletRsStftSamplesF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::synthesize(handle, input.into(), options.into(), output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_regular_f64(
    signal: *const f64,
    count: usize,
    grid: CigletRsStftGrid,
    precision: u32,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::analyze_regular(signal, count, grid, precision, options, output.into()) }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_analyze_regular_legacy_f64(
    signal: *const f64,
    count: usize,
    grid: CigletRsStftGrid,
    level: i32,
    options: CigletRsStftAnalysisOptions,
    output: CigletRsStftAnalysisOutputF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe {
        stft::analyze_regular(
            signal,
            count,
            grid,
            match crate::window::Precision::from_legacy_level(level) {
                crate::window::Precision::Accurate => 0,
                crate::window::Precision::Approximate => 1,
            },
            options,
            output.into(),
        )
    }
}
/// Checked STFT operation retaining native window and reconstruction arithmetic.
/// Window selectors match the window API (0..6); precision zero Accurate, one
/// Approximate. Nyquist zero Preserve, one Legacy. Boolean selectors must be 0/1.
/// Disabled magnitude/phase outputs have count zero; enabled counts are frames*bins.
/// Metadata is required and reports availability of all optional results. Missing
/// scalar results are zero with their presence bit cleared. Regular analysis uses
/// Blackman and grid.hop normalization; regular synthesis uses Accurate, zero offset
/// and a 32-sample fade. Legacy analysis levels use the native level mapping.
/// Every output is retained on error; metadata commits with both spectral planes.
///
/// # Safety
/// Handles are live matching-precision constructor owners; release is excluded
/// during calls. Scratch is serialized. Release requires exclusive owner access,
/// clears its initialized slot and invalidates aliases. Constructor destinations
/// must not already own an unreleased object. All descriptors use platform C layout.
/// Nonempty inputs are aligned initialized stable readable single allocations;
/// outputs are aligned exclusively writable for their declared types/counts.
/// All spectral planes and metadata are pairwise disjoint and independent of
/// private owner storage. Input/output overlap is allowed after calculation ends.
/// Null arrays are allowed only for zero counts; metadata slots are always required.
/// Caller retains allocation validity; native panic/allocation/poison limits apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ciglet_rs_stft_synthesize_regular_f64(
    input: CigletRsStftSpectrumF64,
    grid: CigletRsStftGrid,
    normalization: f64,
    nyquist: u32,
    output: CigletRsStftSamplesF64,
) -> u32 {
    // SAFETY: Forward the documented owner and caller storage contracts.
    unsafe { stft::synthesize_regular(input.into(), grid, normalization, nyquist, output.into()) }
}
