use super::*;
#[napi(js_name = "wave_read_f32")]
pub fn binding_wave_read_f32(
    bytes: napi::bindgen_prelude::Uint8Array,
    max_frames: f64,
) -> napi::Result<WaveF32> {
    let storage_bytes = bytes.to_vec();
    (crate::api::wave::wave_read_f32(&storage_bytes, checked_usize(max_frames)?)).map(|value| {
        WaveF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
#[napi(js_name = "wave_read_at_f32")]
pub fn binding_wave_read_at_f32(
    bytes: napi::bindgen_prelude::Uint8Array,
    position: napi::bindgen_prelude::BigInt,
    max_frames: f64,
) -> napi::Result<WaveF32> {
    let storage_bytes = bytes.to_vec();
    (crate::api::wave::wave_read_at_f32(
        &storage_bytes,
        checked_u64(&position)?,
        checked_usize(max_frames)?,
    ))
    .map(|value| WaveF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "wave_write_pcm_f32")]
pub fn binding_wave_write_pcm_f32(
    samples: napi::bindgen_prelude::Float32Array,
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
    let storage_samples = samples.to_vec();
    (crate::api::wave::wave_write_pcm_f32(
        &storage_samples,
        sample_rate,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Uint8Array>::from)
}
#[napi(js_name = "wave_write_pcm_at_f32")]
pub fn binding_wave_write_pcm_at_f32(
    bytes: napi::bindgen_prelude::Uint8Array,
    position: napi::bindgen_prelude::BigInt,
    samples: napi::bindgen_prelude::Float32Array,
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> napi::Result<WaveBytes> {
    let storage_bytes = bytes.to_vec();
    let storage_samples = samples.to_vec();
    (crate::api::wave::wave_write_pcm_at_f32(
        &storage_bytes,
        checked_u64(&position)?,
        &storage_samples,
        sample_rate,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| WaveBytes {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "wave_read_f64")]
pub fn binding_wave_read_f64(
    bytes: napi::bindgen_prelude::Uint8Array,
    max_frames: f64,
) -> napi::Result<WaveF64> {
    let storage_bytes = bytes.to_vec();
    (crate::api::wave::wave_read_f64(&storage_bytes, checked_usize(max_frames)?)).map(|value| {
        WaveF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }
    })
}
#[napi(js_name = "wave_read_at_f64")]
pub fn binding_wave_read_at_f64(
    bytes: napi::bindgen_prelude::Uint8Array,
    position: napi::bindgen_prelude::BigInt,
    max_frames: f64,
) -> napi::Result<WaveF64> {
    let storage_bytes = bytes.to_vec();
    (crate::api::wave::wave_read_at_f64(
        &storage_bytes,
        checked_u64(&position)?,
        checked_usize(max_frames)?,
    ))
    .map(|value| WaveF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "wave_write_pcm_f64")]
pub fn binding_wave_write_pcm_f64(
    samples: napi::bindgen_prelude::Float64Array,
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
    let storage_samples = samples.to_vec();
    (crate::api::wave::wave_write_pcm_f64(
        &storage_samples,
        sample_rate,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(<napi::bindgen_prelude::Uint8Array>::from)
}
#[napi(js_name = "wave_write_pcm_at_f64")]
pub fn binding_wave_write_pcm_at_f64(
    bytes: napi::bindgen_prelude::Uint8Array,
    position: napi::bindgen_prelude::BigInt,
    samples: napi::bindgen_prelude::Float64Array,
    sample_rate: u32,
    options: &WaveWriteOptions,
) -> napi::Result<WaveBytes> {
    let storage_bytes = bytes.to_vec();
    let storage_samples = samples.to_vec();
    (crate::api::wave::wave_write_pcm_at_f64(
        &storage_bytes,
        checked_u64(&position)?,
        &storage_samples,
        sample_rate,
        options
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
    ))
    .map(|value| WaveBytes {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
