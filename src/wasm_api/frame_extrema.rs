use wasm_bindgen::prelude::*;

/// Native maximum or minimum search, retaining strict comparison tie handling.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ExtremumDirection {
    Maximum,
    Minimum,
}
impl From<ExtremumDirection> for crate::extrema::Direction {
    fn from(value: ExtremumDirection) -> Self {
        match value {
            ExtremumDirection::Maximum => Self::Maximum,
            ExtremumDirection::Minimum => Self::Minimum,
        }
    }
}

macro_rules! frame_extrema_api {
    ($scalar:ty, $frame:ident, $extremum:ident, $peak:ident) => {
        /// Native centered frame with positive-zero padding and complete sample bits.
        #[wasm_bindgen]
        pub fn $frame(
            input: &[$scalar],
            center: isize,
            length: usize,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::frame::fetch_frame(input, center, length)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// First global extremum within the complete inclusive range.
        #[wasm_bindgen]
        pub fn $extremum(
            signal: &[$scalar],
            start: usize,
            end: usize,
            direction: ExtremumDirection,
        ) -> Result<usize, JsValue> {
            crate::extrema::extremum(signal, start..=end, direction.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Native strict two-sided peak search, including original start fallback.
        #[wasm_bindgen]
        pub fn $peak(
            signal: &[$scalar],
            start: usize,
            end: usize,
            direction: ExtremumDirection,
        ) -> Result<usize, JsValue> {
            crate::extrema::peak(signal, start..=end, direction.into())
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
frame_extrema_api!(f32, fetch_frame_f32, extremum_f32, peak_f32);
frame_extrema_api!(f64, fetch_frame_f64, extremum_f64, peak_f64);
