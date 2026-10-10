use super::*;
#[napi(js_name = "real_cepstrum_from_log_spectrum_f32")]
pub fn binding_real_cepstrum_from_log_spectrum_f32(
    logs: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_logs = logs.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_from_log_spectrum_f32(&storage_logs))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "real_cepstrum_to_log_spectrum_f32")]
pub fn binding_real_cepstrum_to_log_spectrum_f32(
    coefficients: napi::bindgen_prelude::Float32Array,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_to_log_spectrum_f32(
        &storage_coefficients,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "real_cepstrum_minimum_phase_f32")]
pub fn binding_real_cepstrum_minimum_phase_f32(
    logs: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_logs = logs.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_minimum_phase_f32(&storage_logs))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "real_cepstrum_from_log_spectrum_f64")]
pub fn binding_real_cepstrum_from_log_spectrum_f64(
    logs: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_logs = logs.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_from_log_spectrum_f64(&storage_logs))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "real_cepstrum_to_log_spectrum_f64")]
pub fn binding_real_cepstrum_to_log_spectrum_f64(
    coefficients: napi::bindgen_prelude::Float64Array,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_coefficients = coefficients.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_to_log_spectrum_f64(
        &storage_coefficients,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
#[napi(js_name = "real_cepstrum_minimum_phase_f64")]
pub fn binding_real_cepstrum_minimum_phase_f64(
    logs: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_logs = logs.to_vec();
    (crate::api::real_cepstrum::real_cepstrum_minimum_phase_f64(&storage_logs))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
