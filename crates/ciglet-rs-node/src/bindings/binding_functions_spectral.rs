use super::*;
#[napi(js_name = "spectral_envelope_estimate_f32")]
pub fn binding_spectral_envelope_estimate_f32(
    spectrum: napi::bindgen_prelude::Float32Array,
    fft_length: f64,
    fundamental: f64,
    harmonics: Option<f64>,
) -> napi::Result<SpectralEnvelopeF32> {
    let storage_spectrum = spectrum.to_vec();
    (crate::api::spectral_envelope::spectral_envelope_estimate_f32(
        &storage_spectrum,
        checked_usize(fft_length)?,
        fundamental as f32,
        harmonics.map(checked_usize).transpose()?,
    ))
    .map(|value| SpectralEnvelopeF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "spectral_envelope_estimate_f64")]
pub fn binding_spectral_envelope_estimate_f64(
    spectrum: napi::bindgen_prelude::Float64Array,
    fft_length: f64,
    fundamental: f64,
    harmonics: Option<f64>,
) -> napi::Result<SpectralEnvelopeF64> {
    let storage_spectrum = spectrum.to_vec();
    (crate::api::spectral_envelope::spectral_envelope_estimate_f64(
        &storage_spectrum,
        checked_usize(fft_length)?,
        fundamental,
        harmonics.map(checked_usize).transpose()?,
    ))
    .map(|value| SpectralEnvelopeF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "spectral_peak_interpolate_f32")]
pub fn binding_spectral_peak_interpolate_f32(
    magnitudes: napi::bindgen_prelude::Float32Array,
    bin: f64,
) -> napi::Result<SpectralPeakF32> {
    let storage_magnitudes = magnitudes.to_vec();
    (crate::api::spectral_peak::spectral_peak_interpolate_f32(
        &storage_magnitudes,
        checked_usize(bin)?,
    ))
    .map(|value| SpectralPeakF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "spectral_peak_interpolate_f64")]
pub fn binding_spectral_peak_interpolate_f64(
    magnitudes: napi::bindgen_prelude::Float64Array,
    bin: f64,
) -> napi::Result<SpectralPeakF64> {
    let storage_magnitudes = magnitudes.to_vec();
    (crate::api::spectral_peak::spectral_peak_interpolate_f64(
        &storage_magnitudes,
        checked_usize(bin)?,
    ))
    .map(|value| SpectralPeakF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
