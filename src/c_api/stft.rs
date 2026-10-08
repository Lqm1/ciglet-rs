//! Owned STFT scratch with checked descriptors and joint output commits.
use super::stft_types::{CigletRsStftAnalysisOptions, CigletRsStftGrid};
use super::{boundary, disjoint_outputs, generated_result, range, scalar_value_result, windows};
use crate::{
    Error,
    statistics::Scalar,
    stft::{self, Analysis, AnalysisOptions, NyquistPolicy, StftPlan, SynthesisOptions},
    window::{Precision, Window},
};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Magnitude is present in STFT analysis metadata.
pub const CIGLET_RS_STFT_MAGNITUDE: u32 = 1;
/// Phase is present in STFT analysis metadata.
pub const CIGLET_RS_STFT_PHASE: u32 = 2;
/// Window weight is present in STFT analysis metadata.
pub const CIGLET_RS_STFT_WINDOW_WEIGHT: u32 = 4;
/// Normalization is present in STFT analysis metadata.
pub const CIGLET_RS_STFT_NORMALIZATION: u32 = 8;

/// Opaque binary32 STFT plan with serialized FFT/window scratch.
pub struct CigletRsStftF32 {
    length: usize,
    plan: Mutex<StftPlan<f32>>,
}
/// Opaque binary64 STFT plan with serialized FFT/window scratch.
pub struct CigletRsStftF64 {
    length: usize,
    plan: Mutex<StftPlan<f64>>,
}
pub(super) trait Information<T>: Copy {
    fn from_analysis(value: &Analysis<T>) -> Self;
}
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    type Info: Information<Self::Value>;
    fn new(length: usize, window: Window, precision: Precision) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<StftPlan<Self::Value>>;
}
impl Handle for CigletRsStftF32 {
    type Value = f32;
    type Info = super::stft_types::CigletRsStftInfoF32;
    fn new(length: usize, window: Window, precision: Precision) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(StftPlan::new(length, window, precision)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<StftPlan<f32>> {
        &self.plan
    }
}
impl Handle for CigletRsStftF64 {
    type Value = f64;
    type Info = super::stft_types::CigletRsStftInfoF64;
    fn new(length: usize, window: Window, precision: Precision) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(StftPlan::new(length, window, precision)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<StftPlan<f64>> {
        &self.plan
    }
}
pub(super) struct RawAnalysisInput<T> {
    pub signal: *const T,
    pub signal_count: usize,
    pub centers: *const i32,
    pub window_lengths: *const usize,
    pub frames: usize,
}
pub(super) struct RawAnalysisOutput<T, I> {
    pub magnitude: *mut T,
    pub magnitude_count: usize,
    pub phase: *mut T,
    pub phase_count: usize,
    pub info: *mut I,
}
pub(super) struct RawSpectrum<T> {
    pub magnitude: *const T,
    pub phase: *const T,
    pub count: usize,
}
pub(super) struct RawSynthesisOptions<T> {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: T,
    pub nyquist: u32,
}
pub(super) struct RawSamples<T> {
    pub samples: *mut T,
    pub count: usize,
}
fn precision(code: u32) -> Result<Precision, Error> {
    match code {
        0 => Ok(Precision::Accurate),
        1 => Ok(Precision::Approximate),
        _ => Err(Error("unknown STFT window precision")),
    }
}
fn flag(code: u32) -> Result<bool, Error> {
    match code {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error("STFT boolean must be zero or one")),
    }
}
fn options(value: CigletRsStftAnalysisOptions) -> Result<AnalysisOptions, Error> {
    Ok(AnalysisOptions {
        subtract_mean: flag(value.subtract_mean)?,
        magnitude: flag(value.magnitude)?,
        phase: flag(value.phase)?,
        normalization_hop: if value.normalization_hop == 0 {
            None
        } else {
            Some(value.normalization_hop)
        },
    })
}
fn nyquist(code: u32) -> Result<NyquistPolicy, Error> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(Error("unknown STFT Nyquist policy")),
    }
}
unsafe fn input<'a, T>(pointer: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Caller retains checked initialized stable single-allocation input.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}
pub(super) unsafe fn create<H: Handle>(
    length: usize,
    window: u32,
    mode: u32,
    output: *mut *mut H,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| H::new(length, windows::kind(window)?, precision(mode)?).map(Box::new)) {
        Ok(value) => {
            // SAFETY: Independent initialized exclusive ownership output slot.
            unsafe {
                output.write(Box::into_raw(value));
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn default_options(output: *mut CigletRsStftAnalysisOptions) -> u32 {
    // SAFETY: Checked independent exclusive output slot; native default is owned.
    unsafe {
        scalar_value_result(output, || {
            let value = AnalysisOptions::default();
            Ok(CigletRsStftAnalysisOptions {
                subtract_mean: u32::from(value.subtract_mean),
                magnitude: u32::from(value.magnitude),
                phase: u32::from(value.phase),
                normalization_hop: value.normalization_hop.unwrap_or(0),
            })
        })
    }
}
unsafe fn analysis_result<T: Scalar, I: Information<T>>(
    frames: usize,
    bins: usize,
    opts: AnalysisOptions,
    output: RawAnalysisOutput<T, I>,
    compute: impl FnOnce() -> Result<Analysis<T>, Error> + UnwindSafe,
) -> u32 {
    let Some(count) = frames.checked_mul(bins) else {
        return 2;
    };
    let magnitude_count = if opts.magnitude { count } else { 0 };
    let phase_count = if opts.phase { count } else { 0 };
    if output.magnitude_count != magnitude_count || output.phase_count != phase_count {
        return 2;
    }
    let checked = disjoint_outputs(
        output.magnitude.cast_const(),
        magnitude_count,
        output.phase.cast_const(),
        phase_count,
    )
    .and_then(|()| {
        disjoint_outputs(
            output.magnitude.cast_const(),
            magnitude_count,
            output.info.cast_const(),
            1,
        )
    })
    .and_then(|()| {
        disjoint_outputs(
            output.phase.cast_const(),
            phase_count,
            output.info.cast_const(),
            1,
        )
    });
    if let Err(status) = checked {
        return status;
    }
    match boundary(compute) {
        Ok(value) => {
            if value.frames != frames
                || value.bins != bins
                || value.magnitude.as_ref().map(Vec::len) != opts.magnitude.then_some(count)
                || value.phase.as_ref().map(Vec::len) != opts.phase.then_some(count)
            {
                return 3;
            }
            let info = I::from_analysis(&value);
            // SAFETY: All disjoint exclusive output ranges and result shapes checked;
            // every input borrow ended. Primitive copies and metadata write cannot fail.
            unsafe {
                if magnitude_count != 0 {
                    std::ptr::copy_nonoverlapping(
                        value.magnitude.as_ref().unwrap().as_ptr(),
                        output.magnitude,
                        magnitude_count,
                    );
                }
                if phase_count != 0 {
                    std::ptr::copy_nonoverlapping(
                        value.phase.as_ref().unwrap().as_ptr(),
                        output.phase,
                        phase_count,
                    );
                }
                output.info.write(info);
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn analyze<H: Handle>(
    handle: *const H,
    arrays: RawAnalysisInput<H::Value>,
    configuration: CigletRsStftAnalysisOptions,
    output: RawAnalysisOutput<H::Value, H::Info>,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| range(arrays.signal, arrays.signal_count))
        .and_then(|()| range(arrays.centers, arrays.frames))
        .and_then(|()| range(arrays.window_lengths, arrays.frames))
    {
        return status;
    }
    let opts = match options(configuration) {
        Ok(value) => value,
        Err(_) => return 3,
    };
    // SAFETY: Retained live matching owner; release excluded, scratch synchronized.
    let handle = unsafe { &*handle };
    // SAFETY: Forward checked initialized inputs and independent exclusive outputs.
    unsafe {
        analysis_result(
            arrays.frames,
            handle.length() / 2 + 1,
            opts,
            output,
            move || {
                let signal = input(arrays.signal, arrays.signal_count);
                let centers = input(arrays.centers, arrays.frames);
                let lengths = input(arrays.window_lengths, arrays.frames);
                let mut plan = handle
                    .plan()
                    .lock()
                    .map_err(|_| Error("STFT plan is poisoned"))?;
                plan.analyze(signal, centers, lengths, opts)
            },
        )
    }
}
fn grid(value: CigletRsStftGrid) -> Result<(stft::FrameGrid, usize), u32> {
    if value.hop == 0
        || value.window_factor == 0
        || value.zero_padding_factor == 0
        || value.frames > i32::MAX as usize
    {
        return Err(3);
    }
    let length = value
        .hop
        .checked_mul(value.window_factor)
        .and_then(|n| n.checked_mul(value.zero_padding_factor))
        .ok_or(2u32)?;
    Ok((
        stft::FrameGrid {
            hop: value.hop,
            frames: value.frames,
            window_factor: value.window_factor,
            zero_padding_factor: value.zero_padding_factor,
        },
        length,
    ))
}
pub(super) unsafe fn analyze_regular<T: Scalar + RefUnwindSafe, I: Information<T>>(
    signal: *const T,
    count: usize,
    configuration: CigletRsStftGrid,
    mode: u32,
    configuration_options: CigletRsStftAnalysisOptions,
    output: RawAnalysisOutput<T, I>,
) -> u32 {
    if let Err(status) = range(signal, count) {
        return status;
    }
    let (grid, length) = match grid(configuration) {
        Ok(value) => value,
        Err(status) => return status,
    };
    let mut opts = match options(configuration_options) {
        Ok(value) => value,
        Err(_) => return 3,
    };
    opts.normalization_hop = Some(grid.hop);
    // SAFETY: Checked retained readable signal; all output ranges checked before compute.
    unsafe {
        analysis_result(grid.frames, length / 2 + 1, opts, output, move || {
            stft::analyze_regular(input(signal, count), grid, precision(mode)?, opts)
        })
    }
}
fn synthesis_length(length: usize, count: usize, hop: usize, offset: i32) -> Result<usize, u32> {
    let bins = length / 2 + 1;
    if !count.is_multiple_of(bins) {
        return Err(3);
    }
    let span = hop.checked_mul(count / bins).ok_or(2u32)?;
    i64::try_from(span)
        .ok()
        .and_then(|n| n.checked_add(i64::from(offset)))
        .filter(|&n| (0..=i64::from(i32::MAX)).contains(&n))
        .map(|n| n as usize)
        .ok_or(2)
}
pub(super) unsafe fn synthesize<H: Handle>(
    handle: *const H,
    arrays: RawSpectrum<H::Value>,
    opts: RawSynthesisOptions<H::Value>,
    output: RawSamples<H::Value>,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| range(arrays.magnitude, arrays.count))
        .and_then(|()| range(arrays.phase, arrays.count))
    {
        return status;
    }
    // SAFETY: Live retained matching owner; every scratch access uses the mutex.
    let handle = unsafe { &*handle };
    let length = match synthesis_length(handle.length(), arrays.count, opts.hop, opts.offset) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if output.count != length {
        return 2;
    }
    // SAFETY: Checked stable inputs; owned staging ends borrowing before output commit.
    unsafe {
        generated_result(output.samples, length, move || {
            let magnitude = input(arrays.magnitude, arrays.count);
            let phase = input(arrays.phase, arrays.count);
            let mut plan = handle
                .plan()
                .lock()
                .map_err(|_| Error("STFT plan is poisoned"))?;
            plan.synthesize(
                magnitude,
                phase,
                SynthesisOptions {
                    hop: opts.hop,
                    offset: opts.offset,
                    fade_samples: opts.fade_samples,
                    normalization: opts.normalization,
                    nyquist: nyquist(opts.nyquist)?,
                },
            )
        })
    }
}
pub(super) unsafe fn synthesize_regular<T: Scalar + RefUnwindSafe + UnwindSafe>(
    arrays: RawSpectrum<T>,
    configuration: CigletRsStftGrid,
    normalization: T,
    code: u32,
    output: RawSamples<T>,
) -> u32 {
    if let Err(status) =
        range(arrays.magnitude, arrays.count).and_then(|()| range(arrays.phase, arrays.count))
    {
        return status;
    }
    let (grid, length) = match grid(configuration) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if grid.frames.checked_mul(length / 2 + 1) != Some(arrays.count) {
        return 3;
    }
    let count = match synthesis_length(length, arrays.count, grid.hop, 0) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if output.count != count {
        return 2;
    }
    // SAFETY: Checked initialized inputs and exclusive output; native result is owned.
    unsafe {
        generated_result(output.samples, count, move || {
            stft::synthesize_regular(
                input(arrays.magnitude, arrays.count),
                input(arrays.phase, arrays.count),
                grid,
                normalization,
                nyquist(code)?,
            )
        })
    }
}
