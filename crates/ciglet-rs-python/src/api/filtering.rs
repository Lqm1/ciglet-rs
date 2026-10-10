use super::WindowKind;
use pyo3::PyErr as BindingError;
/// FIR response shape, with cutoffs measured in cycles per sample.
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum FilterPassband {
    Lowpass,
    Highpass,
    Bandpass,
    Bandstop,
}
/// Corrected zero-state processing or the original skipped leading samples.
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
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
/// Original fractional-bin FIR design with every native response and window.
/// Upper cutoff is unused for lowpass and highpass responses.
pub fn windowed_fir_f32(
    length: usize,
    kind: FilterPassband,
    lower: f32,
    upper: f32,
    window: WindowKind,
) -> Result<Vec<f32>, BindingError> {
    use crate::filtering::Passband;
    let passband = match kind {
        FilterPassband::Lowpass => Passband::Lowpass(lower),
        FilterPassband::Highpass => Passband::Highpass(lower),
        FilterPassband::Bandpass => Passband::Bandpass { lower, upper },
        FilterPassband::Bandstop => Passband::Bandstop { lower, upper },
    };
    crate::filtering::windowed_fir(length, passband, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Lowpass/highpass FIR with cutoff normalized to Nyquist.
pub fn fir_f32(
    length: usize,
    cutoff: f32,
    highpass: bool,
    window: WindowKind,
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::fir(length, cutoff, highpass, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Bandpass FIR with both cutoffs normalized to Nyquist.
pub fn bandpass_fir_f32(
    length: usize,
    lower: f32,
    upper: f32,
    window: WindowKind,
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::bandpass_fir(length, lower, upper, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Full linear convolution in native coefficient accumulation order.
pub fn convolve_f32(signal: &[f32], impulse: &[f32]) -> Result<Vec<f32>, BindingError> {
    crate::filtering::convolve(signal, impulse)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Causal filtering with native zero-state startup and monic denominator.
pub fn filter_f32(
    signal: &[f32],
    numerator: &[f32],
    denominator: &[f32],
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::filter(signal, numerator, denominator)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Causal filtering with explicit corrected or legacy startup.
pub fn filter_with_f32(
    signal: &[f32],
    numerator: &[f32],
    denominator: &[f32],
    startup: FilterStartup,
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::filter_with(signal, numerator, denominator, startup.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native two-pass forward/backward filtering with zero-state startup.
pub fn forward_backward_filter_f32(
    signal: &[f32],
    numerator: &[f32],
    denominator: &[f32],
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::forward_backward(signal, numerator, denominator)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native two-pass filtering with explicit startup, without endpoint padding.
pub fn forward_backward_filter_with_f32(
    signal: &[f32],
    numerator: &[f32],
    denominator: &[f32],
    startup: FilterStartup,
) -> Result<Vec<f32>, BindingError> {
    crate::filtering::forward_backward_with(signal, numerator, denominator, startup.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Original fractional-bin FIR design with every native response and window.
/// Upper cutoff is unused for lowpass and highpass responses.
pub fn windowed_fir_f64(
    length: usize,
    kind: FilterPassband,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> Result<Vec<f64>, BindingError> {
    use crate::filtering::Passband;
    let passband = match kind {
        FilterPassband::Lowpass => Passband::Lowpass(lower),
        FilterPassband::Highpass => Passband::Highpass(lower),
        FilterPassband::Bandpass => Passband::Bandpass { lower, upper },
        FilterPassband::Bandstop => Passband::Bandstop { lower, upper },
    };
    crate::filtering::windowed_fir(length, passband, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Lowpass/highpass FIR with cutoff normalized to Nyquist.
pub fn fir_f64(
    length: usize,
    cutoff: f64,
    highpass: bool,
    window: WindowKind,
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::fir(length, cutoff, highpass, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Bandpass FIR with both cutoffs normalized to Nyquist.
pub fn bandpass_fir_f64(
    length: usize,
    lower: f64,
    upper: f64,
    window: WindowKind,
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::bandpass_fir(length, lower, upper, window.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Full linear convolution in native coefficient accumulation order.
pub fn convolve_f64(signal: &[f64], impulse: &[f64]) -> Result<Vec<f64>, BindingError> {
    crate::filtering::convolve(signal, impulse)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Causal filtering with native zero-state startup and monic denominator.
pub fn filter_f64(
    signal: &[f64],
    numerator: &[f64],
    denominator: &[f64],
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::filter(signal, numerator, denominator)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Causal filtering with explicit corrected or legacy startup.
pub fn filter_with_f64(
    signal: &[f64],
    numerator: &[f64],
    denominator: &[f64],
    startup: FilterStartup,
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::filter_with(signal, numerator, denominator, startup.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native two-pass forward/backward filtering with zero-state startup.
pub fn forward_backward_filter_f64(
    signal: &[f64],
    numerator: &[f64],
    denominator: &[f64],
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::forward_backward(signal, numerator, denominator)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Native two-pass filtering with explicit startup, without endpoint padding.
pub fn forward_backward_filter_with_f64(
    signal: &[f64],
    numerator: &[f64],
    denominator: &[f64],
    startup: FilterStartup,
) -> Result<Vec<f64>, BindingError> {
    crate::filtering::forward_backward_with(signal, numerator, denominator, startup.into())
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
