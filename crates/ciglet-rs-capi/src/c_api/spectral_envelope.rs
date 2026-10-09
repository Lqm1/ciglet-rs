//! Serialized spectral-envelope plans and atomic two-plane output.
use super::{boundary, disjoint_outputs, range, scalar_value_result};
use crate::{
    Error,
    spectral_envelope::{self, Envelope, NyquistPolicy, Plan, SmoothingPolicy},
    statistics::Scalar,
};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Mutex,
};

/// Opaque binary32 spectral-envelope plan with serialized reusable scratch.
pub struct CigletRsEnvelopeF32 {
    length: usize,
    plan: Mutex<Plan<f32>>,
}
/// Opaque binary64 spectral-envelope plan with serialized reusable scratch.
pub struct CigletRsEnvelopeF64 {
    length: usize,
    plan: Mutex<Plan<f64>>,
}

/// Binary32 envelope options passed by value with the platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsEnvelopeOptionsF32 {
    /// Fundamental/sampling-rate ratio in (0, 0.5].
    pub fundamental: f32,
    /// Zero selects the implicit native count; explicit counts must be >= 2.
    pub harmonics: usize,
}
/// Binary64 envelope options passed by value with the platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsEnvelopeOptionsF64 {
    /// Fundamental/sampling-rate ratio in (0, 0.5].
    pub fundamental: f64,
    /// Zero selects the implicit native count; explicit counts must be >= 2.
    pub harmonics: usize,
}
/// Binary32 disjoint output planes passed by value with the platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsEnvelopeOutputF32 {
    pub log_spectrum: *mut f32,
    /// Must equal the full FFT length.
    pub log_count: usize,
    pub cepstrum: *mut f32,
    /// Must equal FFT length / 2 + 1.
    pub cepstrum_count: usize,
}
/// Binary64 disjoint output planes passed by value with the platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsEnvelopeOutputF64 {
    pub log_spectrum: *mut f64,
    /// Must equal the full FFT length.
    pub log_count: usize,
    pub cepstrum: *mut f64,
    /// Must equal FFT length / 2 + 1.
    pub cepstrum_count: usize,
}
pub(super) struct Options<T> {
    pub fundamental: T,
    pub harmonics: usize,
}
pub(super) struct Output<T> {
    pub log_spectrum: *mut T,
    pub log_count: usize,
    pub cepstrum: *mut T,
    pub cepstrum_count: usize,
}
impl From<CigletRsEnvelopeOptionsF32> for Options<f32> {
    fn from(value: CigletRsEnvelopeOptionsF32) -> Self {
        Self {
            fundamental: value.fundamental,
            harmonics: value.harmonics,
        }
    }
}
impl From<CigletRsEnvelopeOptionsF64> for Options<f64> {
    fn from(value: CigletRsEnvelopeOptionsF64) -> Self {
        Self {
            fundamental: value.fundamental,
            harmonics: value.harmonics,
        }
    }
}
impl From<CigletRsEnvelopeOutputF32> for Output<f32> {
    fn from(value: CigletRsEnvelopeOutputF32) -> Self {
        Self {
            log_spectrum: value.log_spectrum,
            log_count: value.log_count,
            cepstrum: value.cepstrum,
            cepstrum_count: value.cepstrum_count,
        }
    }
}
impl From<CigletRsEnvelopeOutputF64> for Output<f64> {
    fn from(value: CigletRsEnvelopeOutputF64) -> Self {
        Self {
            log_spectrum: value.log_spectrum,
            log_count: value.log_count,
            cepstrum: value.cepstrum,
            cepstrum_count: value.cepstrum_count,
        }
    }
}
pub(super) trait Handle: RefUnwindSafe {
    type Value: Scalar + RefUnwindSafe + UnwindSafe;
    fn new(length: usize) -> Result<Self, Error>
    where
        Self: Sized;
    fn length(&self) -> usize;
    fn plan(&self) -> &Mutex<Plan<Self::Value>>;
}
impl Handle for CigletRsEnvelopeF32 {
    type Value = f32;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(Plan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<Plan<f32>> {
        &self.plan
    }
}
impl Handle for CigletRsEnvelopeF64 {
    type Value = f64;
    fn new(length: usize) -> Result<Self, Error> {
        Ok(Self {
            length,
            plan: Mutex::new(Plan::new(length)?),
        })
    }
    fn length(&self) -> usize {
        self.length
    }
    fn plan(&self) -> &Mutex<Plan<f64>> {
        &self.plan
    }
}
pub(super) unsafe fn create<H: Handle>(length: usize, output: *mut *mut H) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(|| H::new(length).map(Box::new)) {
        Ok(value) => {
            // SAFETY: Independent initialized exclusively writable ownership slot.
            unsafe {
                output.write(Box::into_raw(value));
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) enum Query {
    Length,
    Bins,
    Empty,
}
pub(super) unsafe fn query<H: Handle>(handle: *const H, output: *mut usize, query: Query) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Retained typed live owner, release excluded, independent scalar slot.
    unsafe {
        scalar_value_result(output, || {
            let plan = (&*handle)
                .plan()
                .lock()
                .map_err(|_| Error("spectral envelope plan is poisoned"))?;
            Ok(match query {
                Query::Length => plan.len(),
                Query::Bins => plan.bins(),
                Query::Empty => usize::from(plan.is_empty()),
            })
        })
    }
}
fn nyquist(code: u32) -> Result<NyquistPolicy, Error> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(Error("unknown envelope Nyquist policy")),
    }
}
fn smoothing(code: u32) -> Result<SmoothingPolicy, Error> {
    match code {
        0 => Ok(SmoothingPolicy::Stable),
        1 => Ok(SmoothingPolicy::Legacy),
        _ => Err(Error("unknown envelope smoothing policy")),
    }
}
pub(super) enum Operation {
    Default,
    Nyquist(u32),
    Both(u32, u32),
}
fn harmonics(count: usize) -> Option<usize> {
    if count == 0 { None } else { Some(count) }
}
unsafe fn result<T: Scalar + RefUnwindSafe>(
    length: usize,
    input: *const T,
    count: usize,
    output: Output<T>,
    compute: impl FnOnce(&[T]) -> Result<Envelope<T>, Error> + UnwindSafe,
) -> u32 {
    let bins = length / 2 + 1;
    if output.log_count != length || output.cepstrum_count != bins {
        return 2;
    }
    if let Err(status) = range(input, count).and_then(|()| {
        disjoint_outputs(
            output.log_spectrum.cast_const(),
            length,
            output.cepstrum.cast_const(),
            bins,
        )
    }) {
        return status;
    }
    let computed = boundary(|| {
        let spectrum = if count == 0 {
            &[]
        } else {
            // SAFETY: Checked initialized single-allocation input retained stable.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        compute(spectrum)
    });
    match computed {
        Ok(value) => {
            if value.log_spectrum.len() != length || value.cepstrum.len() != bins {
                return 3;
            }
            // SAFETY: Checked disjoint exclusive planes and independent owned
            // sources; input borrowing ended and both lengths validated before writes.
            unsafe {
                if length != 0 {
                    std::ptr::copy_nonoverlapping(
                        value.log_spectrum.as_ptr(),
                        output.log_spectrum,
                        length,
                    );
                }
                if bins != 0 {
                    std::ptr::copy_nonoverlapping(value.cepstrum.as_ptr(), output.cepstrum, bins);
                }
            }
            0
        }
        Err(status) => status,
    }
}
pub(super) unsafe fn process<H: Handle>(
    handle: *const H,
    input: *const H::Value,
    count: usize,
    options: Options<H::Value>,
    operation: Operation,
    output: Output<H::Value>,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner; all mutable scratch accesses use its lock.
    let handle = unsafe { &*handle };
    // SAFETY: Caller input/output contracts forwarded to complete range validation.
    unsafe {
        result(handle.length(), input, count, output, move |spectrum| {
            let mut plan = handle
                .plan()
                .lock()
                .map_err(|_| Error("spectral envelope plan is poisoned"))?;
            let harmonic_count = harmonics(options.harmonics);
            match operation {
                Operation::Default => plan.estimate(spectrum, options.fundamental, harmonic_count),
                Operation::Nyquist(code) => plan.estimate_with(
                    spectrum,
                    options.fundamental,
                    harmonic_count,
                    nyquist(code)?,
                ),
                Operation::Both(n, s) => plan.estimate_with_smoothing(
                    spectrum,
                    options.fundamental,
                    harmonic_count,
                    nyquist(n)?,
                    smoothing(s)?,
                ),
            }
        })
    }
}
pub(super) unsafe fn one_shot<T: Scalar + RefUnwindSafe + UnwindSafe>(
    length: usize,
    input: *const T,
    count: usize,
    options: Options<T>,
    output: Output<T>,
) -> u32 {
    // SAFETY: Caller initialized input and exclusive output storage remain valid.
    unsafe {
        result(length, input, count, output, move |spectrum| {
            spectral_envelope::estimate(
                spectrum,
                length,
                options.fundamental,
                harmonics(options.harmonics),
            )
        })
    }
}
