use super::*;
#[napi(js_name = "fft_f32")]
pub fn binding_fft_f32(
    length: f64,
    real: Option<napi::bindgen_prelude::Float32Array>,
    imaginary: Option<napi::bindgen_prelude::Float32Array>,
    direction: u32,
) -> napi::Result<ComplexValuesF32> {
    (crate::api::fourier::fft_f32(
        checked_usize(length)?,
        real.map(|value| value.to_vec()),
        imaginary.map(|value| value.to_vec()),
        direction,
    ))
    .map(|value| ComplexValuesF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "fft_shift_f32")]
pub fn binding_fft_shift_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    Ok(<napi::bindgen_prelude::Float32Array>::from(
        crate::api::fourier::fft_shift_f32(&storage_values),
    ))
}
#[napi(js_name = "fft_f64")]
pub fn binding_fft_f64(
    length: f64,
    real: Option<napi::bindgen_prelude::Float64Array>,
    imaginary: Option<napi::bindgen_prelude::Float64Array>,
    direction: u32,
) -> napi::Result<ComplexValuesF64> {
    (crate::api::fourier::fft_f64(
        checked_usize(length)?,
        real.map(|value| value.to_vec()),
        imaginary.map(|value| value.to_vec()),
        direction,
    ))
    .map(|value| ComplexValuesF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "fft_shift_f64")]
pub fn binding_fft_shift_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<napi::bindgen_prelude::Float64Array> {
    let storage_values = values.to_vec();
    Ok(<napi::bindgen_prelude::Float64Array>::from(
        crate::api::fourier::fft_shift_f64(&storage_values),
    ))
}
