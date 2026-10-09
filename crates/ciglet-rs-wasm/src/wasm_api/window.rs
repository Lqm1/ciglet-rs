use crate::window::{Precision, Window};
use wasm_bindgen::prelude::*;

/// Every native window selection, in the same order as the C interface.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum WindowKind {
    Boxcar,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    Nuttall98,
    MltSine,
}

impl From<WindowKind> for Window {
    fn from(value: WindowKind) -> Self {
        match value {
            WindowKind::Boxcar => Self::Boxcar,
            WindowKind::Hann => Self::Hann,
            WindowKind::Hamming => Self::Hamming,
            WindowKind::Blackman => Self::Blackman,
            WindowKind::BlackmanHarris => Self::BlackmanHarris,
            WindowKind::Nuttall98 => Self::Nuttall98,
            WindowKind::MltSine => Self::MltSine,
        }
    }
}

/// Native accurate or upstream approximate trigonometric arithmetic.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum WindowPrecision {
    Accurate,
    Approximate,
}

impl From<WindowPrecision> for Precision {
    fn from(value: WindowPrecision) -> Self {
        match value {
            WindowPrecision::Accurate => Self::Accurate,
            WindowPrecision::Approximate => Self::Approximate,
        }
    }
}

/// Generate complete binary32 window weights in independent typed-array storage.
#[wasm_bindgen]
pub fn window_f32(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> Result<Vec<f32>, JsValue> {
    Window::from(kind)
        .generate_with(length, precision.into())
        .map_err(|error| JsValue::from_str(error.0))
}

/// Generate complete binary64 window weights in independent typed-array storage.
#[wasm_bindgen]
pub fn window_f64(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> Result<Vec<f64>, JsValue> {
    Window::from(kind)
        .generate_with(length, precision.into())
        .map_err(|error| JsValue::from_str(error.0))
}

fn custom<T: crate::statistics::Scalar, R: crate::statistics::Scalar>(
    kind: WindowKind,
    length: usize,
    callback: &js_sys::Function,
) -> Result<Vec<T>, JsValue> {
    let mut exception = None;
    let values = Window::from(kind).generate_custom::<T, R>(length, |angle| {
        let result = match callback.call1(&JsValue::UNDEFINED, &JsValue::from_f64(angle)) {
            Ok(value) => value,
            Err(error) => {
                exception = Some(error);
                return Err(crate::Error("custom window callback threw an exception"));
            }
        };
        let number = result
            .as_f64()
            .ok_or(crate::Error("custom window callback must return a number"))?;
        R::from_f64(number).ok_or(crate::Error("custom window callback value is out of range"))
    });
    match exception {
        Some(error) => Err(error),
        None => values.map_err(|error| JsValue::from_str(error.0)),
    }
}

macro_rules! custom_window {
    ($name:ident, $output:ty, $callback:ty) => {
        /// Generate native custom weights with explicit output/callback precision.
        /// The callback receives radians and returns a number. Exceptions retain
        /// their original JavaScript value. Empty/singleton/boxcar windows do not
        /// call it. Successful arrays own independent JavaScript storage.
        #[wasm_bindgen]
        pub fn $name(
            kind: WindowKind,
            length: usize,
            callback: &js_sys::Function,
        ) -> Result<Vec<$output>, JsValue> {
            custom::<$output, $callback>(kind, length, callback)
        }
    };
}

custom_window!(window_custom_f32_f32, f32, f32);
custom_window!(window_custom_f32_f64, f32, f64);
custom_window!(window_custom_f64_f32, f64, f32);
custom_window!(window_custom_f64_f64, f64, f64);
