use crate::stft::{
    self, Analysis, AnalysisOptions, FrameGrid, NyquistPolicy, StftPlan, SynthesisOptions,
};
use crate::window::{Precision, Window};
use wasm_bindgen::prelude::*;

fn window(code: u32) -> Result<Window, JsValue> {
    match code {
        0 => Ok(Window::Boxcar),
        1 => Ok(Window::Hann),
        2 => Ok(Window::Hamming),
        3 => Ok(Window::Blackman),
        4 => Ok(Window::BlackmanHarris),
        5 => Ok(Window::Nuttall98),
        6 => Ok(Window::MltSine),
        _ => Err(JsValue::from_str("invalid STFT window")),
    }
}
fn precision(code: u32) -> Result<Precision, JsValue> {
    match code {
        0 => Ok(Precision::Accurate),
        1 => Ok(Precision::Approximate),
        _ => Err(JsValue::from_str("invalid STFT window precision")),
    }
}
fn nyquist(code: u32) -> Result<NyquistPolicy, JsValue> {
    match code {
        0 => Ok(NyquistPolicy::Preserve),
        1 => Ok(NyquistPolicy::Legacy),
        _ => Err(JsValue::from_str("invalid STFT Nyquist policy")),
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct StftAnalysisOptions {
    pub subtract_mean: bool,
    pub magnitude: bool,
    pub phase: bool,
    pub normalization_hop: Option<usize>,
}
#[wasm_bindgen]
impl StftAnalysisOptions {
    #[wasm_bindgen(constructor)]
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

#[wasm_bindgen]
#[derive(Clone)]
pub struct StftFrameGrid {
    pub hop: usize,
    pub frames: usize,
    pub window_factor: usize,
    pub zero_padding_factor: usize,
}
#[wasm_bindgen]
impl StftFrameGrid {
    #[wasm_bindgen(constructor)]
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

macro_rules! stft_api {
    ($scalar:ty, $result:ident, $synthesis:ident, $plan:ident, $analyze:ident, $legacy:ident, $synthesize:ident) => {
        /// Full native analysis fields, including absent arrays and metadata.
        #[wasm_bindgen]
        pub struct $result {
            analysis: Analysis<$scalar>,
        }
        #[wasm_bindgen]
        impl $result {
            #[wasm_bindgen(constructor)]
            pub fn new(
                frames: usize,
                bins: usize,
                magnitude: Option<Vec<$scalar>>,
                phase: Option<Vec<$scalar>>,
                window_weight: Option<$scalar>,
                normalization: Option<$scalar>,
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
            #[wasm_bindgen(getter)]
            pub fn frames(&self) -> usize {
                self.analysis.frames
            }
            #[wasm_bindgen(setter)]
            pub fn set_frames(&mut self, value: usize) {
                self.analysis.frames = value;
            }
            #[wasm_bindgen(getter)]
            pub fn bins(&self) -> usize {
                self.analysis.bins
            }
            #[wasm_bindgen(setter)]
            pub fn set_bins(&mut self, value: usize) {
                self.analysis.bins = value;
            }
            #[wasm_bindgen(getter)]
            pub fn magnitude(&self) -> Option<Vec<$scalar>> {
                self.analysis.magnitude.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_magnitude(&mut self, value: Option<Vec<$scalar>>) {
                self.analysis.magnitude = value;
            }
            #[wasm_bindgen(getter)]
            pub fn phase(&self) -> Option<Vec<$scalar>> {
                self.analysis.phase.clone()
            }
            #[wasm_bindgen(setter)]
            pub fn set_phase(&mut self, value: Option<Vec<$scalar>>) {
                self.analysis.phase = value;
            }
            #[wasm_bindgen(getter)]
            pub fn window_weight(&self) -> Option<$scalar> {
                self.analysis.window_weight
            }
            #[wasm_bindgen(setter)]
            pub fn set_window_weight(&mut self, value: Option<$scalar>) {
                self.analysis.window_weight = value;
            }
            #[wasm_bindgen(getter)]
            pub fn normalization(&self) -> Option<$scalar> {
                self.analysis.normalization
            }
            #[wasm_bindgen(setter)]
            pub fn set_normalization(&mut self, value: Option<$scalar>) {
                self.analysis.normalization = value;
            }
            pub fn window_weight_storage(&self) -> Option<Vec<$scalar>> {
                self.analysis.window_weight.map(|v| vec![v])
            }
            pub fn set_window_weight_storage(
                &mut self,
                values: Option<Vec<$scalar>>,
            ) -> Result<(), JsValue> {
                self.analysis.window_weight = optional_scalar(values)?;
                Ok(())
            }
            pub fn normalization_storage(&self) -> Option<Vec<$scalar>> {
                self.analysis.normalization.map(|v| vec![v])
            }
            pub fn set_normalization_storage(
                &mut self,
                values: Option<Vec<$scalar>>,
            ) -> Result<(), JsValue> {
                self.analysis.normalization = optional_scalar(values)?;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                Self {
                    analysis: self.analysis.clone(),
                }
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $synthesis {
            pub hop: usize,
            pub offset: i32,
            pub fade_samples: usize,
            pub normalization: $scalar,
            pub nyquist: u32,
        }
        #[wasm_bindgen]
        impl $synthesis {
            #[wasm_bindgen(constructor)]
            pub fn new(
                hop: usize,
                offset: i32,
                fade_samples: usize,
                normalization: $scalar,
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
            pub fn normalization_storage(&self) -> Vec<$scalar> {
                vec![self.normalization]
            }
            pub fn set_normalization_storage(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [value]: [$scalar; 1] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("normalization requires exactly one value"))?;
                self.normalization = value;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        pub struct $plan {
            plan: StftPlan<$scalar>,
        }
        #[wasm_bindgen]
        impl $plan {
            #[wasm_bindgen(constructor)]
            pub fn new(
                length: usize,
                window_code: u32,
                precision_code: u32,
            ) -> Result<Self, JsValue> {
                StftPlan::new(length, window(window_code)?, precision(precision_code)?)
                    .map(|plan| Self { plan })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn analyze(
                &mut self,
                signal: &[$scalar],
                centers: &[i32],
                window_lengths: &[u32],
                options: &StftAnalysisOptions,
            ) -> Result<$result, JsValue> {
                let lengths: Vec<usize> = window_lengths.iter().map(|&n| n as usize).collect();
                self.plan
                    .analyze(signal, centers, &lengths, options.into())
                    .map(|analysis| $result { analysis })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn synthesize(
                &mut self,
                magnitude: &[$scalar],
                phase: &[$scalar],
                options: &$synthesis,
            ) -> Result<Vec<$scalar>, JsValue> {
                let native = SynthesisOptions {
                    hop: options.hop,
                    offset: options.offset,
                    fade_samples: options.fade_samples,
                    normalization: options.normalization,
                    nyquist: nyquist(options.nyquist)?,
                };
                self.plan
                    .synthesize(magnitude, phase, native)
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
        #[wasm_bindgen]
        pub fn $analyze(
            signal: &[$scalar],
            grid: &StftFrameGrid,
            precision_code: u32,
            options: &StftAnalysisOptions,
        ) -> Result<$result, JsValue> {
            stft::analyze_regular(
                signal,
                grid.into(),
                precision(precision_code)?,
                options.into(),
            )
            .map(|analysis| $result { analysis })
            .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $legacy(
            signal: &[$scalar],
            grid: &StftFrameGrid,
            level: i32,
            options: &StftAnalysisOptions,
        ) -> Result<$result, JsValue> {
            stft::analyze_regular(
                signal,
                grid.into(),
                Precision::from_legacy_level(level),
                options.into(),
            )
            .map(|analysis| $result { analysis })
            .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $synthesize(
            magnitude: &[$scalar],
            phase: &[$scalar],
            grid: &StftFrameGrid,
            normalization: $scalar,
            nyquist_code: u32,
        ) -> Result<Vec<$scalar>, JsValue> {
            stft::synthesize_regular(
                magnitude,
                phase,
                grid.into(),
                normalization,
                nyquist(nyquist_code)?,
            )
            .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
fn optional_scalar<T: Copy>(values: Option<Vec<T>>) -> Result<Option<T>, JsValue> {
    values
        .map(|v| {
            let [value]: [T; 1] = v
                .as_slice()
                .try_into()
                .map_err(|_| JsValue::from_str("optional scalar requires exactly one value"))?;
            Ok(value)
        })
        .transpose()
}
stft_api!(
    f32,
    StftAnalysisF32,
    StftSynthesisOptionsF32,
    StftPlanF32,
    stft_analyze_regular_f32,
    stft_analyze_regular_legacy_f32,
    stft_synthesize_regular_f32
);
stft_api!(
    f64,
    StftAnalysisF64,
    StftSynthesisOptionsF64,
    StftPlanF64,
    stft_analyze_regular_f64,
    stft_analyze_regular_legacy_f64,
    stft_synthesize_regular_f64
);
