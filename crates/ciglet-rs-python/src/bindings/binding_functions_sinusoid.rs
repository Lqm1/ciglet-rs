use super::*;
#[pyfunction(name = "sinusoid_f32")]
#[pyo3(signature = (tone, sample_rate, length))]
pub fn binding_sinusoid_f32(
    tone: pyo3::PyRef<'_, SinusoidF32>,
    sample_rate: u32,
    length: usize,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::synthesis::sinusoid_f32(
        tone.inner.as_ref().ok_or_else(consumed)?,
        sample_rate,
        length,
    )
}
#[pyfunction(name = "sinusoid_f64")]
#[pyo3(signature = (tone, sample_rate, length))]
pub fn binding_sinusoid_f64(
    tone: pyo3::PyRef<'_, SinusoidF64>,
    sample_rate: u32,
    length: usize,
) -> pyo3::PyResult<Vec<f64>> {
    crate::api::synthesis::sinusoid_f64(
        tone.inner.as_ref().ok_or_else(consumed)?,
        sample_rate,
        length,
    )
}
