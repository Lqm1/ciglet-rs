use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct GlottalPreparedF64 {
    pub(crate) inner: Option<crate::api::GlottalPreparedF64>,
}
#[pymethods]
impl GlottalPreparedF64 {
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
impl GlottalPreparedF64 {
    #[pyo3(name = "model")]
    #[pyo3(signature = ())]
    pub fn binding_model(&self) -> pyo3::PyResult<GlottalModelF64> {
        Ok(GlottalModelF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).model()),
        })
    }
    #[pyo3(name = "spectrum")]
    #[pyo3(signature = (frequencies))]
    pub fn binding_spectrum(&self, frequencies: Vec<f64>) -> pyo3::PyResult<GlottalSpectrumF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).spectrum(&frequencies))
            .map(|value| GlottalSpectrumF64 { inner: Some(value) })
    }
    #[pyo3(name = "spectrum_with")]
    #[pyo3(signature = (frequencies, code))]
    pub fn binding_spectrum_with(
        &self,
        frequencies: Vec<f64>,
        code: u32,
    ) -> pyo3::PyResult<GlottalSpectrumF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).spectrum_with(&frequencies, code))
            .map(|value| GlottalSpectrumF64 { inner: Some(value) })
    }
    #[pyo3(name = "sampled_period")]
    #[pyo3(signature = (sampling_rate, count))]
    pub fn binding_sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_ref().ok_or_else(consumed)?).sampled_period(sampling_rate, count)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GlottalPreparedF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
