use super::*;
#[pyfunction(name = "spectral_envelope_estimate_f32")]
#[pyo3(signature = (spectrum, fft_length, fundamental, harmonics))]
pub fn binding_spectral_envelope_estimate_f32(
    spectrum: Vec<f32>,
    fft_length: usize,
    fundamental: f32,
    harmonics: Option<usize>,
) -> pyo3::PyResult<SpectralEnvelopeF32> {
    (crate::api::spectral_envelope::spectral_envelope_estimate_f32(
        &spectrum,
        fft_length,
        fundamental,
        harmonics,
    ))
    .map(|value| SpectralEnvelopeF32 { inner: Some(value) })
}
#[pyfunction(name = "spectral_envelope_estimate_f64")]
#[pyo3(signature = (spectrum, fft_length, fundamental, harmonics))]
pub fn binding_spectral_envelope_estimate_f64(
    spectrum: Vec<f64>,
    fft_length: usize,
    fundamental: f64,
    harmonics: Option<usize>,
) -> pyo3::PyResult<SpectralEnvelopeF64> {
    (crate::api::spectral_envelope::spectral_envelope_estimate_f64(
        &spectrum,
        fft_length,
        fundamental,
        harmonics,
    ))
    .map(|value| SpectralEnvelopeF64 { inner: Some(value) })
}
#[pyfunction(name = "spectral_peak_interpolate_f32")]
#[pyo3(signature = (magnitudes, bin))]
pub fn binding_spectral_peak_interpolate_f32(
    magnitudes: Vec<f32>,
    bin: usize,
) -> pyo3::PyResult<SpectralPeakF32> {
    (crate::api::spectral_peak::spectral_peak_interpolate_f32(&magnitudes, bin))
        .map(|value| SpectralPeakF32 { inner: Some(value) })
}
#[pyfunction(name = "spectral_peak_interpolate_f64")]
#[pyo3(signature = (magnitudes, bin))]
pub fn binding_spectral_peak_interpolate_f64(
    magnitudes: Vec<f64>,
    bin: usize,
) -> pyo3::PyResult<SpectralPeakF64> {
    (crate::api::spectral_peak::spectral_peak_interpolate_f64(&magnitudes, bin))
        .map(|value| SpectralPeakF64 { inner: Some(value) })
}
