use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct GlottalPreparedF32 {
    pub(crate) inner: Option<crate::api::GlottalPreparedF32>,
}
#[pymethods]
impl GlottalPreparedF32 {
    fn close(&mut self) {
        self.inner = None;
    }
    fn free(&mut self) {
        self.inner = None;
    }
    #[getter]
    fn is_closed(&self) -> bool {
        self.inner.is_none()
    }
}
#[pymethods]
impl GlottalPreparedF32 {
    #[pyo3(name = "model")]
    #[pyo3(signature = ())]
    pub fn binding_model(&self) -> pyo3::PyResult<GlottalModelF32> {
        Ok(GlottalModelF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).model()),
        })
    }
    #[pyo3(name = "spectrum")]
    #[pyo3(signature = (frequencies))]
    pub fn binding_spectrum(&self, frequencies: Vec<f32>) -> pyo3::PyResult<GlottalSpectrumF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).spectrum(&frequencies))
            .map(|value| GlottalSpectrumF32 { inner: Some(value) })
    }
    #[pyo3(name = "spectrum_with")]
    #[pyo3(signature = (frequencies, code))]
    pub fn binding_spectrum_with(
        &self,
        frequencies: Vec<f32>,
        code: u32,
    ) -> pyo3::PyResult<GlottalSpectrumF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).spectrum_with(&frequencies, code))
            .map(|value| GlottalSpectrumF32 { inner: Some(value) })
    }
    #[pyo3(name = "sampled_period")]
    #[pyo3(signature = (sampling_rate, count))]
    pub fn binding_sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).sampled_period(sampling_rate, count)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GlottalPreparedF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
