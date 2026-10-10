use crate::stft::{
    self, Analysis, AnalysisOptions, FrameGrid, NyquistPolicy, StftPlan, SynthesisOptions,
};
use crate::window::{Precision, Window};
use napi::Error as BindingError;
fn window(code: u32) -> Result<Window, BindingError> {
    match code {
        0 => Ok(Window::Boxcar),
        1 => Ok(Window::Hann),
        2 => Ok(Window::Hamming),
        3 => Ok(Window::Blackman),
        4 => Ok(Window::BlackmanHarris),
        5 => Ok(Window::Nuttall98),
        6 => Ok(Window::MltSine),
        _ => Err(napi::Error::from_reason("invalid STFT window")),
    }
}
fn precision(code: u32) -> Result<Precision, BindingError> {
    match code {
        0 => Ok(Precision::Accurate),
        1 => Ok(Precision::Approximate),
        _ => Err(napi::Error::from_reason("invalid STFT window precision")),
    }
}
fn nyquist(code: u32) -> Result<NyquistPolicy, BindingError> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(napi::Error::from_reason("invalid STFT Nyquist policy")),
    }
}
#[derive(Clone)]
pub struct StftAnalysisOptions {
    pub subtract_mean: bool,
    pub magnitude: bool,
    pub phase: bool,
    pub normalization_hop: Option<usize>,
}
impl StftAnalysisOptions {
    pub fn new() -> Self {
        Self {
            subtract_mean: false,
            magnitude: true,
            phase: true,
            normalization_hop: None,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl Default for StftAnalysisOptions {
    fn default() -> Self {
        Self::new()
    }
}
impl From<&StftAnalysisOptions> for AnalysisOptions {
    fn from(value: &StftAnalysisOptions) -> Self {
        Self {
            subtract_mean: value.subtract_mean,
            magnitude: value.magnitude,
            phase: value.phase,
            normalization_hop: value.normalization_hop,
        }
    }
}
#[derive(Clone)]
pub struct StftFrameGrid {
    pub hop: usize,
    pub frames: usize,
    pub window_factor: usize,
    pub zero_padding_factor: usize,
}
impl StftFrameGrid {
    pub fn new(
        hop: usize,
        frames: usize,
        window_factor: usize,
        zero_padding_factor: usize,
    ) -> Self {
        Self {
            hop,
            frames,
            window_factor,
            zero_padding_factor,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl From<&StftFrameGrid> for FrameGrid {
    fn from(value: &StftFrameGrid) -> Self {
        Self {
            hop: value.hop,
            frames: value.frames,
            window_factor: value.window_factor,
            zero_padding_factor: value.zero_padding_factor,
        }
    }
}
fn optional_scalar<T: Copy>(values: Option<Vec<T>>) -> Result<Option<T>, BindingError> {
    values
        .map(|v| {
            let [value]: [T; 1] = v.as_slice().try_into().map_err(|_| {
                napi::Error::from_reason("optional scalar requires exactly one value")
            })?;
            Ok(value)
        })
        .transpose()
}
/// Full native analysis fields, including absent arrays and metadata.
pub struct StftAnalysisF32 {
    analysis: Analysis<f32>,
}
impl StftAnalysisF32 {
    pub fn new(
        frames: usize,
        bins: usize,
        magnitude: Option<Vec<f32>>,
        phase: Option<Vec<f32>>,
        window_weight: Option<f32>,
        normalization: Option<f32>,
    ) -> Self {
        Self {
            analysis: Analysis {
                frames,
                bins,
                magnitude,
                phase,
                window_weight,
                normalization,
            },
        }
    }
    pub fn frames(&self) -> usize {
        self.analysis.frames
    }
    pub fn set_frames(&mut self, value: usize) {
        self.analysis.frames = value;
    }
    pub fn bins(&self) -> usize {
        self.analysis.bins
    }
    pub fn set_bins(&mut self, value: usize) {
        self.analysis.bins = value;
    }
    pub fn magnitude(&self) -> Option<Vec<f32>> {
        self.analysis.magnitude.clone()
    }
    pub fn set_magnitude(&mut self, value: Option<Vec<f32>>) {
        self.analysis.magnitude = value;
    }
    pub fn phase(&self) -> Option<Vec<f32>> {
        self.analysis.phase.clone()
    }
    pub fn set_phase(&mut self, value: Option<Vec<f32>>) {
        self.analysis.phase = value;
    }
    pub fn window_weight(&self) -> Option<f32> {
        self.analysis.window_weight
    }
    pub fn set_window_weight(&mut self, value: Option<f32>) {
        self.analysis.window_weight = value;
    }
    pub fn normalization(&self) -> Option<f32> {
        self.analysis.normalization
    }
    pub fn set_normalization(&mut self, value: Option<f32>) {
        self.analysis.normalization = value;
    }
    pub fn window_weight_storage(&self) -> Option<Vec<f32>> {
        self.analysis.window_weight.map(|v| vec![v])
    }
    pub fn set_window_weight_storage(
        &mut self,
        values: Option<Vec<f32>>,
    ) -> Result<(), BindingError> {
        self.analysis.window_weight = optional_scalar(values)?;
        Ok(())
    }
    pub fn normalization_storage(&self) -> Option<Vec<f32>> {
        self.analysis.normalization.map(|v| vec![v])
    }
    pub fn set_normalization_storage(
        &mut self,
        values: Option<Vec<f32>>,
    ) -> Result<(), BindingError> {
        self.analysis.normalization = optional_scalar(values)?;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            analysis: self.analysis.clone(),
        }
    }
}
#[derive(Clone)]
pub struct StftSynthesisOptionsF32 {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: f32,
    pub nyquist: u32,
}
impl StftSynthesisOptionsF32 {
    pub fn new(
        hop: usize,
        offset: i32,
        fade_samples: usize,
        normalization: f32,
        nyquist: u32,
    ) -> Self {
        Self {
            hop,
            offset,
            fade_samples,
            normalization,
            nyquist,
        }
    }
    pub fn normalization_storage(&self) -> Vec<f32> {
        vec![self.normalization]
    }
    pub fn set_normalization_storage(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [value]: [f32; 1] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("normalization requires exactly one value"))?;
        self.normalization = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
pub struct StftPlanF32 {
    plan: StftPlan<f32>,
}
impl StftPlanF32 {
    pub fn new(length: usize, window_code: u32, precision_code: u32) -> Result<Self, BindingError> {
        StftPlan::new(length, window(window_code)?, precision(precision_code)?)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn analyze(
        &mut self,
        signal: &[f32],
        centers: &[i32],
        window_lengths: &[u32],
        options: &StftAnalysisOptions,
    ) -> Result<StftAnalysisF32, BindingError> {
        let lengths: Vec<usize> = window_lengths.iter().map(|&n| n as usize).collect();
        self.plan
            .analyze(signal, centers, &lengths, options.into())
            .map(|analysis| StftAnalysisF32 { analysis })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn synthesize(
        &mut self,
        magnitude: &[f32],
        phase: &[f32],
        options: &StftSynthesisOptionsF32,
    ) -> Result<Vec<f32>, BindingError> {
        let native = SynthesisOptions {
            hop: options.hop,
            offset: options.offset,
            fade_samples: options.fade_samples,
            normalization: options.normalization,
            nyquist: nyquist(options.nyquist)?,
        };
        self.plan
            .synthesize(magnitude, phase, native)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
pub fn stft_analyze_regular_f32(
    signal: &[f32],
    grid: &StftFrameGrid,
    precision_code: u32,
    options: &StftAnalysisOptions,
) -> Result<StftAnalysisF32, BindingError> {
    stft::analyze_regular(
        signal,
        grid.into(),
        precision(precision_code)?,
        options.into(),
    )
    .map(|analysis| StftAnalysisF32 { analysis })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn stft_analyze_regular_legacy_f32(
    signal: &[f32],
    grid: &StftFrameGrid,
    level: i32,
    options: &StftAnalysisOptions,
) -> Result<StftAnalysisF32, BindingError> {
    stft::analyze_regular(
        signal,
        grid.into(),
        Precision::from_legacy_level(level),
        options.into(),
    )
    .map(|analysis| StftAnalysisF32 { analysis })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn stft_synthesize_regular_f32(
    magnitude: &[f32],
    phase: &[f32],
    grid: &StftFrameGrid,
    normalization: f32,
    nyquist_code: u32,
) -> Result<Vec<f32>, BindingError> {
    stft::synthesize_regular(
        magnitude,
        phase,
        grid.into(),
        normalization,
        nyquist(nyquist_code)?,
    )
    .map_err(|error| napi::Error::from_reason(error.0))
}
/// Full native analysis fields, including absent arrays and metadata.
pub struct StftAnalysisF64 {
    analysis: Analysis<f64>,
}
impl StftAnalysisF64 {
    pub fn new(
        frames: usize,
        bins: usize,
        magnitude: Option<Vec<f64>>,
        phase: Option<Vec<f64>>,
        window_weight: Option<f64>,
        normalization: Option<f64>,
    ) -> Self {
        Self {
            analysis: Analysis {
                frames,
                bins,
                magnitude,
                phase,
                window_weight,
                normalization,
            },
        }
    }
    pub fn frames(&self) -> usize {
        self.analysis.frames
    }
    pub fn set_frames(&mut self, value: usize) {
        self.analysis.frames = value;
    }
    pub fn bins(&self) -> usize {
        self.analysis.bins
    }
    pub fn set_bins(&mut self, value: usize) {
        self.analysis.bins = value;
    }
    pub fn magnitude(&self) -> Option<Vec<f64>> {
        self.analysis.magnitude.clone()
    }
    pub fn set_magnitude(&mut self, value: Option<Vec<f64>>) {
        self.analysis.magnitude = value;
    }
    pub fn phase(&self) -> Option<Vec<f64>> {
        self.analysis.phase.clone()
    }
    pub fn set_phase(&mut self, value: Option<Vec<f64>>) {
        self.analysis.phase = value;
    }
    pub fn window_weight(&self) -> Option<f64> {
        self.analysis.window_weight
    }
    pub fn set_window_weight(&mut self, value: Option<f64>) {
        self.analysis.window_weight = value;
    }
    pub fn normalization(&self) -> Option<f64> {
        self.analysis.normalization
    }
    pub fn set_normalization(&mut self, value: Option<f64>) {
        self.analysis.normalization = value;
    }
    pub fn window_weight_storage(&self) -> Option<Vec<f64>> {
        self.analysis.window_weight.map(|v| vec![v])
    }
    pub fn set_window_weight_storage(
        &mut self,
        values: Option<Vec<f64>>,
    ) -> Result<(), BindingError> {
        self.analysis.window_weight = optional_scalar(values)?;
        Ok(())
    }
    pub fn normalization_storage(&self) -> Option<Vec<f64>> {
        self.analysis.normalization.map(|v| vec![v])
    }
    pub fn set_normalization_storage(
        &mut self,
        values: Option<Vec<f64>>,
    ) -> Result<(), BindingError> {
        self.analysis.normalization = optional_scalar(values)?;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self {
            analysis: self.analysis.clone(),
        }
    }
}
#[derive(Clone)]
pub struct StftSynthesisOptionsF64 {
    pub hop: usize,
    pub offset: i32,
    pub fade_samples: usize,
    pub normalization: f64,
    pub nyquist: u32,
}
impl StftSynthesisOptionsF64 {
    pub fn new(
        hop: usize,
        offset: i32,
        fade_samples: usize,
        normalization: f64,
        nyquist: u32,
    ) -> Self {
        Self {
            hop,
            offset,
            fade_samples,
            normalization,
            nyquist,
        }
    }
    pub fn normalization_storage(&self) -> Vec<f64> {
        vec![self.normalization]
    }
    pub fn set_normalization_storage(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value]: [f64; 1] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("normalization requires exactly one value"))?;
        self.normalization = value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
pub struct StftPlanF64 {
    plan: StftPlan<f64>,
}
impl StftPlanF64 {
    pub fn new(length: usize, window_code: u32, precision_code: u32) -> Result<Self, BindingError> {
        StftPlan::new(length, window(window_code)?, precision(precision_code)?)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn analyze(
        &mut self,
        signal: &[f64],
        centers: &[i32],
        window_lengths: &[u32],
        options: &StftAnalysisOptions,
    ) -> Result<StftAnalysisF64, BindingError> {
        let lengths: Vec<usize> = window_lengths.iter().map(|&n| n as usize).collect();
        self.plan
            .analyze(signal, centers, &lengths, options.into())
            .map(|analysis| StftAnalysisF64 { analysis })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn synthesize(
        &mut self,
        magnitude: &[f64],
        phase: &[f64],
        options: &StftSynthesisOptionsF64,
    ) -> Result<Vec<f64>, BindingError> {
        let native = SynthesisOptions {
            hop: options.hop,
            offset: options.offset,
            fade_samples: options.fade_samples,
            normalization: options.normalization,
            nyquist: nyquist(options.nyquist)?,
        };
        self.plan
            .synthesize(magnitude, phase, native)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
}
pub fn stft_analyze_regular_f64(
    signal: &[f64],
    grid: &StftFrameGrid,
    precision_code: u32,
    options: &StftAnalysisOptions,
) -> Result<StftAnalysisF64, BindingError> {
    stft::analyze_regular(
        signal,
        grid.into(),
        precision(precision_code)?,
        options.into(),
    )
    .map(|analysis| StftAnalysisF64 { analysis })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn stft_analyze_regular_legacy_f64(
    signal: &[f64],
    grid: &StftFrameGrid,
    level: i32,
    options: &StftAnalysisOptions,
) -> Result<StftAnalysisF64, BindingError> {
    stft::analyze_regular(
        signal,
        grid.into(),
        Precision::from_legacy_level(level),
        options.into(),
    )
    .map(|analysis| StftAnalysisF64 { analysis })
    .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn stft_synthesize_regular_f64(
    magnitude: &[f64],
    phase: &[f64],
    grid: &StftFrameGrid,
    normalization: f64,
    nyquist_code: u32,
) -> Result<Vec<f64>, BindingError> {
    stft::synthesize_regular(
        magnitude,
        phase,
        grid.into(),
        normalization,
        nyquist(nyquist_code)?,
    )
    .map_err(|error| napi::Error::from_reason(error.0))
}
