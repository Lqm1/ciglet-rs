use crate::callbacks::Function;
use crate::window::{Precision, Window};
use napi::Error as BindingError;
/// Every native window selection, in the same order as the C interface.
#[derive(Clone, Copy, Debug)]
#[napi_derive::napi]
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
#[derive(Clone, Copy, Debug)]
#[napi_derive::napi]
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
pub fn window_f32(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> Result<Vec<f32>, BindingError> {
    Window::from(kind)
        .generate_with(length, precision.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Generate complete binary64 window weights in independent typed-array storage.
pub fn window_f64(
    kind: WindowKind,
    length: usize,
    precision: WindowPrecision,
) -> Result<Vec<f64>, BindingError> {
    Window::from(kind)
        .generate_with(length, precision.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
fn custom<T: crate::statistics::Scalar, R: crate::statistics::Scalar>(
    kind: WindowKind,
    length: usize,
    callback: &Function,
) -> Result<Vec<T>, BindingError> {
    let mut exception = None;
    let values = Window::from(kind).generate_custom::<T, R>(length, |angle| {
        let result = match callback.call1(&(), &crate::callbacks::Value::number(angle)) {
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
        None => values.map_err(|error| napi::Error::from_reason(error.0)),
    }
}
/// Generate native custom weights with explicit output/callback precision.
/// The callback receives radians and returns a number. Exceptions retain
/// their original JavaScript value. Empty/singleton/boxcar windows do not
/// call it. Successful arrays own independent JavaScript storage.
pub fn window_custom_f32_f32(
    kind: WindowKind,
    length: usize,
    callback: &Function,
) -> Result<Vec<f32>, BindingError> {
    custom::<f32, f32>(kind, length, callback)
}
/// Generate native custom weights with explicit output/callback precision.
/// The callback receives radians and returns a number. Exceptions retain
/// their original JavaScript value. Empty/singleton/boxcar windows do not
/// call it. Successful arrays own independent JavaScript storage.
pub fn window_custom_f32_f64(
    kind: WindowKind,
    length: usize,
    callback: &Function,
) -> Result<Vec<f32>, BindingError> {
    custom::<f32, f64>(kind, length, callback)
}
/// Generate native custom weights with explicit output/callback precision.
/// The callback receives radians and returns a number. Exceptions retain
/// their original JavaScript value. Empty/singleton/boxcar windows do not
/// call it. Successful arrays own independent JavaScript storage.
pub fn window_custom_f64_f32(
    kind: WindowKind,
    length: usize,
    callback: &Function,
) -> Result<Vec<f64>, BindingError> {
    custom::<f64, f32>(kind, length, callback)
}
/// Generate native custom weights with explicit output/callback precision.
/// The callback receives radians and returns a number. Exceptions retain
/// their original JavaScript value. Empty/singleton/boxcar windows do not
/// call it. Successful arrays own independent JavaScript storage.
pub fn window_custom_f64_f64(
    kind: WindowKind,
    length: usize,
    callback: &Function,
) -> Result<Vec<f64>, BindingError> {
    custom::<f64, f64>(kind, length, callback)
}
