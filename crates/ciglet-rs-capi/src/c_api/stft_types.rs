//! C-layout STFT descriptors; booleans/policies use validated integer codes.
use super::stft;
/// Independent analysis requests; zero normalization_hop selects None.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftAnalysisOptions {
    pub subtract_mean: u32,
    pub magnitude: u32,
    pub phase: u32,
    pub normalization_hop: usize,
}
/// Original regular frame grid with checked multiplication and signed centers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftGrid {
    pub hop: usize,
    pub frames: usize,
    pub window_factor: usize,
    pub zero_padding_factor: usize,
}
/// F32 STFT AnalysisInput descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftAnalysisInputF32 {
    pub signal: *const f32,
    pub signal_count: usize,
    pub centers: *const i32,
    pub window_lengths: *const usize,
    pub frames: usize,
}
impl From<CigletRsStftAnalysisInputF32> for stft::RawAnalysisInput<f32> {
    fn from(value: CigletRsStftAnalysisInputF32) -> Self {
        Self {
            signal: value.signal,
            signal_count: value.signal_count,
            centers: value.centers,
            window_lengths: value.window_lengths,
            frames: value.frames,
        }
    }
}
/// F32 STFT AnalysisOutput descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftAnalysisOutputF32 {
    pub magnitude: *mut f32,
    /// frames*bins when requested, otherwise zero.
    pub magnitude_count: usize,
    pub phase: *mut f32,
    /// frames*bins when requested, otherwise zero.
    pub phase_count: usize,
    pub info: *mut CigletRsStftInfoF32,
}
impl From<CigletRsStftAnalysisOutputF32> for stft::RawAnalysisOutput<f32, CigletRsStftInfoF32> {
    fn from(value: CigletRsStftAnalysisOutputF32) -> Self {
        Self {
            magnitude: value.magnitude,
            magnitude_count: value.magnitude_count,
            phase: value.phase,
            phase_count: value.phase_count,
            info: value.info,
        }
    }
}
/// F32 STFT Info descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftInfoF32 {
    pub frames: usize,
    pub bins: usize,
    /// Presence bits use CIGLET_RS_STFT_*; absent scalar values are zero.
    pub available: u32,
    pub window_weight: f32,
    pub normalization: f32,
}
/// F32 STFT Spectrum descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSpectrumF32 {
    pub magnitude: *const f32,
    pub phase: *const f32,
    pub count: usize,
}
impl From<CigletRsStftSpectrumF32> for stft::RawSpectrum<f32> {
    fn from(value: CigletRsStftSpectrumF32) -> Self {
        Self {
            magnitude: value.magnitude,
            phase: value.phase,
            count: value.count,
        }
    }
}
/// F32 STFT SynthesisOptions descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSynthesisOptionsF32 {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: f32,
    pub nyquist: u32,
}
impl From<CigletRsStftSynthesisOptionsF32> for stft::RawSynthesisOptions<f32> {
    fn from(value: CigletRsStftSynthesisOptionsF32) -> Self {
        Self {
            hop: value.hop,
            offset: value.offset,
            fade_samples: value.fade_samples,
            normalization: value.normalization,
            nyquist: value.nyquist,
        }
    }
}
/// F32 STFT Samples descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSamplesF32 {
    pub samples: *mut f32,
    pub count: usize,
}
impl From<CigletRsStftSamplesF32> for stft::RawSamples<f32> {
    fn from(value: CigletRsStftSamplesF32) -> Self {
        Self {
            samples: value.samples,
            count: value.count,
        }
    }
}
impl stft::Information<f32> for CigletRsStftInfoF32 {
    fn from_analysis(value: &crate::stft::Analysis<f32>) -> Self {
        let available = if value.magnitude.is_some() {
            stft::CIGLET_RS_STFT_MAGNITUDE
        } else {
            0
        } | if value.phase.is_some() {
            stft::CIGLET_RS_STFT_PHASE
        } else {
            0
        } | if value.window_weight.is_some() {
            stft::CIGLET_RS_STFT_WINDOW_WEIGHT
        } else {
            0
        } | if value.normalization.is_some() {
            stft::CIGLET_RS_STFT_NORMALIZATION
        } else {
            0
        };
        Self {
            frames: value.frames,
            bins: value.bins,
            available,
            window_weight: value.window_weight.unwrap_or(0.),
            normalization: value.normalization.unwrap_or(0.),
        }
    }
}
/// F64 STFT AnalysisInput descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftAnalysisInputF64 {
    pub signal: *const f64,
    pub signal_count: usize,
    pub centers: *const i32,
    pub window_lengths: *const usize,
    pub frames: usize,
}
impl From<CigletRsStftAnalysisInputF64> for stft::RawAnalysisInput<f64> {
    fn from(value: CigletRsStftAnalysisInputF64) -> Self {
        Self {
            signal: value.signal,
            signal_count: value.signal_count,
            centers: value.centers,
            window_lengths: value.window_lengths,
            frames: value.frames,
        }
    }
}
/// F64 STFT AnalysisOutput descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftAnalysisOutputF64 {
    pub magnitude: *mut f64,
    /// frames*bins when requested, otherwise zero.
    pub magnitude_count: usize,
    pub phase: *mut f64,
    /// frames*bins when requested, otherwise zero.
    pub phase_count: usize,
    pub info: *mut CigletRsStftInfoF64,
}
impl From<CigletRsStftAnalysisOutputF64> for stft::RawAnalysisOutput<f64, CigletRsStftInfoF64> {
    fn from(value: CigletRsStftAnalysisOutputF64) -> Self {
        Self {
            magnitude: value.magnitude,
            magnitude_count: value.magnitude_count,
            phase: value.phase,
            phase_count: value.phase_count,
            info: value.info,
        }
    }
}
/// F64 STFT Info descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftInfoF64 {
    pub frames: usize,
    pub bins: usize,
    /// Presence bits use CIGLET_RS_STFT_*; absent scalar values are zero.
    pub available: u32,
    pub window_weight: f64,
    pub normalization: f64,
}
/// F64 STFT Spectrum descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSpectrumF64 {
    pub magnitude: *const f64,
    pub phase: *const f64,
    pub count: usize,
}
impl From<CigletRsStftSpectrumF64> for stft::RawSpectrum<f64> {
    fn from(value: CigletRsStftSpectrumF64) -> Self {
        Self {
            magnitude: value.magnitude,
            phase: value.phase,
            count: value.count,
        }
    }
}
/// F64 STFT SynthesisOptions descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSynthesisOptionsF64 {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: f64,
    pub nyquist: u32,
}
impl From<CigletRsStftSynthesisOptionsF64> for stft::RawSynthesisOptions<f64> {
    fn from(value: CigletRsStftSynthesisOptionsF64) -> Self {
        Self {
            hop: value.hop,
            offset: value.offset,
            fade_samples: value.fade_samples,
            normalization: value.normalization,
            nyquist: value.nyquist,
        }
    }
}
/// F64 STFT Samples descriptor with platform C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CigletRsStftSamplesF64 {
    pub samples: *mut f64,
    pub count: usize,
}
impl From<CigletRsStftSamplesF64> for stft::RawSamples<f64> {
    fn from(value: CigletRsStftSamplesF64) -> Self {
        Self {
            samples: value.samples,
            count: value.count,
        }
    }
}
impl stft::Information<f64> for CigletRsStftInfoF64 {
    fn from_analysis(value: &crate::stft::Analysis<f64>) -> Self {
        let available = if value.magnitude.is_some() {
            stft::CIGLET_RS_STFT_MAGNITUDE
        } else {
            0
        } | if value.phase.is_some() {
            stft::CIGLET_RS_STFT_PHASE
        } else {
            0
        } | if value.window_weight.is_some() {
            stft::CIGLET_RS_STFT_WINDOW_WEIGHT
        } else {
            0
        } | if value.normalization.is_some() {
            stft::CIGLET_RS_STFT_NORMALIZATION
        } else {
            0
        };
        Self {
            frames: value.frames,
            bins: value.bins,
            available,
            window_weight: value.window_weight.unwrap_or(0.),
            normalization: value.normalization.unwrap_or(0.),
        }
    }
}
