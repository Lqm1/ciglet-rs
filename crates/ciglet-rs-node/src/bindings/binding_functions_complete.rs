use super::*;
#[napi(js_name = "complete_spectrum_f32")]
pub fn binding_complete_spectrum_f32(
    values: napi::bindgen_prelude::Float32Array,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    (crate::api::phase::complete_spectrum_f32(values.to_vec(), symmetry, policy))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "complete_spectrum_f64")]
pub fn binding_complete_spectrum_f64(
    values: napi::bindgen_prelude::Float64Array,
    symmetry: SpectrumSymmetry,
    policy: Option<SpectrumNyquist>,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    (crate::api::phase::complete_spectrum_f64(values.to_vec(), symmetry, policy))
        .map(<napi::bindgen_prelude::Float64Array>::from)
}
