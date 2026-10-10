use super::*;
#[napi(js_name = "sinusoids_f32")]
pub fn binding_sinusoids_f32(
    frequencies: napi::bindgen_prelude::Float32Array,
    amplitudes: napi::bindgen_prelude::Float32Array,
    phases: napi::bindgen_prelude::Float32Array,
    sample_rate: u32,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_frequencies = frequencies.to_vec();
    let storage_amplitudes = amplitudes.to_vec();
    let storage_phases = phases.to_vec();
    (crate::api::synthesis::sinusoids_f32(
        &storage_frequencies,
        &storage_amplitudes,
        &storage_phases,
        sample_rate,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "sinusoids_f64")]
pub fn binding_sinusoids_f64(
    frequencies: napi::bindgen_prelude::Float64Array,
    amplitudes: napi::bindgen_prelude::Float64Array,
    phases: napi::bindgen_prelude::Float64Array,
    sample_rate: u32,
    length: f64,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_frequencies = frequencies.to_vec();
    let storage_amplitudes = amplitudes.to_vec();
    let storage_phases = phases.to_vec();
    (crate::api::synthesis::sinusoids_f64(
        &storage_frequencies,
        &storage_amplitudes,
        &storage_phases,
        sample_rate,
        checked_usize(length)?,
    ))
    .map(<napi::bindgen_prelude::Float64Array>::from)
}
