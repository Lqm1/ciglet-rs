use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct BandCepstralOptions {
    pub(crate) inner: Option<crate::api::BandCepstralOptions>,
}
#[pymethods]
impl BandCepstralOptions {
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
impl BandCepstralOptions {
    #[getter(order)]
    fn binding_get_order(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.order)
    }
    #[setter(order)]
    fn binding_set_order(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.order = value;
        Ok(())
    }
    #[getter(include_dc)]
    fn binding_get_include_dc(&self) -> PyResult<bool> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.include_dc)
    }
    #[setter(include_dc)]
    fn binding_set_include_dc(&mut self, value: bool) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.include_dc = value;
        Ok(())
    }
}
#[pymethods]
impl BandCepstralOptions {
    #[new]
    #[pyo3(signature = (order, include_dc))]
    pub fn binding_new(order: usize, include_dc: bool) -> pyo3::PyResult<Self> {
        Ok(BandCepstralOptions {
            inner: Some(crate::api::BandCepstralOptions::new(order, include_dc)),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(BandCepstralOptions {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "coefficient_count")]
    #[pyo3(signature = (bands))]
    pub fn binding_coefficient_count(&self, bands: usize) -> pyo3::PyResult<usize> {
        (self.inner.as_ref().ok_or_else(consumed)?).coefficient_count(bands)
    }
}
