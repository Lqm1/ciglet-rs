use super::WindowKind;
use wasm_bindgen::prelude::*;

/// FIR response shape, with cutoffs measured in cycles per sample.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum FilterPassband {
    Lowpass,
    Highpass,
    Bandpass,
    Bandstop,
}

/// Corrected zero-state processing or the original skipped leading samples.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum FilterStartup {
    ZeroState,
    LegacyZeroPrefix,
}

impl From<FilterStartup> for crate::filtering::Startup {
    fn from(value: FilterStartup) -> Self {
        match value {
            FilterStartup::ZeroState => Self::ZeroState,
            FilterStartup::LegacyZeroPrefix => Self::LegacyZeroPrefix,
        }
    }
}

macro_rules! filtering_api {
    ($scalar:ty, $windowed:ident, $fir:ident, $band:ident, $convolve:ident, $filter:ident, $with:ident, $backward:ident, $backward_with:ident) => {
        /// Original fractional-bin FIR design with every native response and window.
        /// Upper cutoff is unused for lowpass and highpass responses.
        #[wasm_bindgen]
        pub fn $windowed(
            length: usize,
            kind: FilterPassband,
            lower: $scalar,
            upper: $scalar,
            window: WindowKind,
        ) -> Result<Vec<$scalar>, JsValue> {
            use crate::filtering::Passband;
            let passband = match kind {
                FilterPassband::Lowpass => Passband::Lowpass(lower),
                FilterPassband::Highpass => Passband::Highpass(lower),
                FilterPassband::Bandpass => Passband::Bandpass { lower, upper },
                FilterPassband::Bandstop => Passband::Bandstop { lower, upper },
            };
            crate::filtering::windowed_fir(length, passband, window.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Lowpass/highpass FIR with cutoff normalized to Nyquist.
        #[wasm_bindgen]
        pub fn $fir(
            length: usize,
            cutoff: $scalar,
            highpass: bool,
            window: WindowKind,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::fir(length, cutoff, highpass, window.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Bandpass FIR with both cutoffs normalized to Nyquist.
        #[wasm_bindgen]
        pub fn $band(
            length: usize,
            lower: $scalar,
            upper: $scalar,
            window: WindowKind,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::bandpass_fir(length, lower, upper, window.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Full linear convolution in native coefficient accumulation order.
        #[wasm_bindgen]
        pub fn $convolve(signal: &[$scalar], impulse: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::convolve(signal, impulse).map_err(|error| JsValue::from_str(error.0))
        }

        /// Causal filtering with native zero-state startup and monic denominator.
        #[wasm_bindgen]
        pub fn $filter(
            signal: &[$scalar],
            numerator: &[$scalar],
            denominator: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::filter(signal, numerator, denominator)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Causal filtering with explicit corrected or legacy startup.
        #[wasm_bindgen]
        pub fn $with(
            signal: &[$scalar],
            numerator: &[$scalar],
            denominator: &[$scalar],
            startup: FilterStartup,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::filter_with(signal, numerator, denominator, startup.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Native two-pass forward/backward filtering with zero-state startup.
        #[wasm_bindgen]
        pub fn $backward(
            signal: &[$scalar],
            numerator: &[$scalar],
            denominator: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::forward_backward(signal, numerator, denominator)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Native two-pass filtering with explicit startup, without endpoint padding.
        #[wasm_bindgen]
        pub fn $backward_with(
            signal: &[$scalar],
            numerator: &[$scalar],
            denominator: &[$scalar],
            startup: FilterStartup,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::filtering::forward_backward_with(signal, numerator, denominator, startup.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

filtering_api!(
    f32,
    windowed_fir_f32,
    fir_f32,
    bandpass_fir_f32,
    convolve_f32,
    filter_f32,
    filter_with_f32,
    forward_backward_filter_f32,
    forward_backward_filter_with_f32
);
filtering_api!(
    f64,
    windowed_fir_f64,
    fir_f64,
    bandpass_fir_f64,
    convolve_f64,
    filter_f64,
    filter_with_f64,
    forward_backward_filter_f64,
    forward_backward_filter_with_f64
);
