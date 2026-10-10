use crate::callbacks::Function;
use napi::Error as BindingError;
/// Find a native bracketed root using a selected-precision callback.
/// JavaScript exceptions retain their original value. Non-number results
/// and nonfinite values are ordinary errors; captures replace C context.
pub fn root_find_f32(callback: &Function, start: f32, end: f32) -> Result<f32, BindingError> {
    let mut exception = None;
    let result = crate::root::find(
        |value: f32| match callback.call1(&(), &crate::callbacks::Value::number(value.into())) {
            Ok(result) => match result.as_f64() {
                Some(number) => number as f32,
                None => {
                    exception = Some(napi::Error::from_reason(
                        "root callback must return a number",
                    ));
                    <f32>::NAN
                }
            },
            Err(error) => {
                exception = Some(error);
                <f32>::NAN
            }
        },
        start,
        end,
    );
    match exception {
        Some(error) => Err(error),
        None => result.map_err(|error| napi::Error::from_reason(error.0)),
    }
}
/// Find a native bracketed root using a selected-precision callback.
/// JavaScript exceptions retain their original value. Non-number results
/// and nonfinite values are ordinary errors; captures replace C context.
pub fn root_find_f64(callback: &Function, start: f64, end: f64) -> Result<f64, BindingError> {
    let mut exception = None;
    let result = crate::root::find(
        |value: f64| match callback.call1(&(), &crate::callbacks::Value::number(value)) {
            Ok(result) => match result.as_f64() {
                Some(number) => number,
                None => {
                    exception = Some(napi::Error::from_reason(
                        "root callback must return a number",
                    ));
                    <f64>::NAN
                }
            },
            Err(error) => {
                exception = Some(error);
                <f64>::NAN
            }
        },
        start,
        end,
    );
    match exception {
        Some(error) => Err(error),
        None => result.map_err(|error| napi::Error::from_reason(error.0)),
    }
}
