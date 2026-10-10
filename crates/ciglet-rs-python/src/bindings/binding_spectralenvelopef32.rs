use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct SpectralEnvelopeF32 {
    pub(crate) inner: Option<crate::api::SpectralEnvelopeF32>,
}
#[pymethods]
impl SpectralEnvelopeF32 {
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
impl SpectralEnvelopeF32 {
    #[new]
    #[pyo3(signature = (log_spectrum, cepstrum))]
    pub fn binding_new(log_spectrum: Vec<f32>, cepstrum: Vec<f32>) -> pyo3::PyResult<Self> {
        Ok(SpectralEnvelopeF32 {
            inner: Some(crate::api::SpectralEnvelopeF32::new(log_spectrum, cepstrum)),
        })
    }
    #[getter(log_spectrum)]
    pub fn binding_log_spectrum(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).log_spectrum())
    }
    #[setter(log_spectrum)]
    pub fn binding_set_log_spectrum(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_log_spectrum(values);
        Ok(())
    }
    #[getter(cepstrum)]
    pub fn binding_cepstrum(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).cepstrum())
    }
    #[setter(cepstrum)]
    pub fn binding_set_cepstrum(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_cepstrum(values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SpectralEnvelopeF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
