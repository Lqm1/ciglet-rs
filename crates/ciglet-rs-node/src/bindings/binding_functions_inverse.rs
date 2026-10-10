use super::*;
#[napi(js_name = "inverse_dft_f32")]
pub fn binding_inverse_dft_f32(
    length: f64,
    real: Option<napi::bindgen_prelude::Float32Array>,
    imaginary: Option<napi::bindgen_prelude::Float32Array>,
) -> napi::Result<ComplexValuesF32> {
    (crate::api::fourier::inverse_dft_f32(
        checked_usize(length)?,
        real.map(|value| value.to_vec()),
        imaginary.map(|value| value.to_vec()),
    ))
    .map(|value| ComplexValuesF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "inverse_dft_f64")]
pub fn binding_inverse_dft_f64(
    length: f64,
    real: Option<napi::bindgen_prelude::Float64Array>,
    imaginary: Option<napi::bindgen_prelude::Float64Array>,
) -> napi::Result<ComplexValuesF64> {
    (crate::api::fourier::inverse_dft_f64(
        checked_usize(length)?,
        real.map(|value| value.to_vec()),
        imaginary.map(|value| value.to_vec()),
    ))
    .map(|value| ComplexValuesF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
