use super::*;
#[pyfunction(name = "wave_read_f32")]
#[pyo3(signature = (bytes, max_frames))]
pub fn binding_wave_read_f32(bytes: Vec<u8>, max_frames: usize) -> pyo3::PyResult<WaveF32> {
    (crate::api::wave::wave_read_f32(&bytes, max_frames))
        .map(|value| WaveF32 { inner: Some(value) })
}
#[pyfunction(name = "wave_read_at_f32")]
#[pyo3(signature = (bytes, position, max_frames))]
pub fn binding_wave_read_at_f32(
    bytes: Vec<u8>,
    position: u64,
    max_frames: usize,
) -> pyo3::PyResult<WaveF32> {
    (crate::api::wave::wave_read_at_f32(&bytes, position, max_frames))
        .map(|value| WaveF32 { inner: Some(value) })
}
#[pyfunction(name = "wave_write_pcm_f32")]
#[pyo3(signature = (samples, sample_rate, options))]
pub fn binding_wave_write_pcm_f32(
    samples: Vec<f32>,
    sample_rate: u32,
    options: pyo3::PyRef<'_, WaveWriteOptions>,
) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
    (crate::api::wave::wave_write_pcm_f32(
        &samples,
        sample_rate,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| {
        let bytes = value;
        pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
    })
}
#[pyfunction(name = "wave_write_pcm_at_f32")]
#[pyo3(signature = (bytes, position, samples, sample_rate, options))]
pub fn binding_wave_write_pcm_at_f32(
    bytes: Vec<u8>,
    position: u64,
    samples: Vec<f32>,
    sample_rate: u32,
    options: pyo3::PyRef<'_, WaveWriteOptions>,
) -> pyo3::PyResult<WaveBytes> {
    (crate::api::wave::wave_write_pcm_at_f32(
        &bytes,
        position,
        &samples,
        sample_rate,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| WaveBytes { inner: Some(value) })
}
#[pyfunction(name = "wave_read_f64")]
#[pyo3(signature = (bytes, max_frames))]
pub fn binding_wave_read_f64(bytes: Vec<u8>, max_frames: usize) -> pyo3::PyResult<WaveF64> {
    (crate::api::wave::wave_read_f64(&bytes, max_frames))
        .map(|value| WaveF64 { inner: Some(value) })
}
#[pyfunction(name = "wave_read_at_f64")]
#[pyo3(signature = (bytes, position, max_frames))]
pub fn binding_wave_read_at_f64(
    bytes: Vec<u8>,
    position: u64,
    max_frames: usize,
) -> pyo3::PyResult<WaveF64> {
    (crate::api::wave::wave_read_at_f64(&bytes, position, max_frames))
        .map(|value| WaveF64 { inner: Some(value) })
}
#[pyfunction(name = "wave_write_pcm_f64")]
#[pyo3(signature = (samples, sample_rate, options))]
pub fn binding_wave_write_pcm_f64(
    samples: Vec<f64>,
    sample_rate: u32,
    options: pyo3::PyRef<'_, WaveWriteOptions>,
) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
    (crate::api::wave::wave_write_pcm_f64(
        &samples,
        sample_rate,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| {
        let bytes = value;
        pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
    })
}
#[pyfunction(name = "wave_write_pcm_at_f64")]
#[pyo3(signature = (bytes, position, samples, sample_rate, options))]
pub fn binding_wave_write_pcm_at_f64(
    bytes: Vec<u8>,
    position: u64,
    samples: Vec<f64>,
    sample_rate: u32,
    options: pyo3::PyRef<'_, WaveWriteOptions>,
) -> pyo3::PyResult<WaveBytes> {
    (crate::api::wave::wave_write_pcm_at_f64(
        &bytes,
        position,
        &samples,
        sample_rate,
        options.inner.as_ref().ok_or_else(consumed)?,
    ))
    .map(|value| WaveBytes { inner: Some(value) })
}
