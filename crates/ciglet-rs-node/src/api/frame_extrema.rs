use napi::Error as BindingError;
/// Native maximum or minimum search, retaining strict comparison tie handling.
#[derive(Clone, Copy)]
#[napi_derive::napi]
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
/// Native centered frame with positive-zero padding and complete sample bits.
pub fn fetch_frame_f32(
    input: &[f32],
    center: isize,
    length: usize,
) -> Result<Vec<f32>, BindingError> {
    crate::frame::fetch_frame(input, center, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// First global extremum within the complete inclusive range.
pub fn extremum_f32(
    signal: &[f32],
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> Result<usize, BindingError> {
    crate::extrema::extremum(signal, start..=end, direction.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native strict two-sided peak search, including original start fallback.
pub fn peak_f32(
    signal: &[f32],
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> Result<usize, BindingError> {
    crate::extrema::peak(signal, start..=end, direction.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native centered frame with positive-zero padding and complete sample bits.
pub fn fetch_frame_f64(
    input: &[f64],
    center: isize,
    length: usize,
) -> Result<Vec<f64>, BindingError> {
    crate::frame::fetch_frame(input, center, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// First global extremum within the complete inclusive range.
pub fn extremum_f64(
    signal: &[f64],
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> Result<usize, BindingError> {
    crate::extrema::extremum(signal, start..=end, direction.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Native strict two-sided peak search, including original start fallback.
pub fn peak_f64(
    signal: &[f64],
    start: usize,
    end: usize,
    direction: ExtremumDirection,
) -> Result<usize, BindingError> {
    crate::extrema::peak(signal, start..=end, direction.into())
        .map_err(|error| napi::Error::from_reason(error.0))
}
