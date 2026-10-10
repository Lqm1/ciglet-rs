use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct StftAnalysisOptions {
    pub(crate) inner: Option<crate::api::StftAnalysisOptions>,
}
#[pymethods]
impl StftAnalysisOptions {
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
impl StftAnalysisOptions {
    #[getter(subtract_mean)]
    fn binding_get_subtract_mean(&self) -> PyResult<bool> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.subtract_mean)
    }
    #[setter(subtract_mean)]
    fn binding_set_subtract_mean(&mut self, value: bool) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.subtract_mean = value;
        Ok(())
    }
    #[getter(magnitude)]
    fn binding_get_magnitude(&self) -> PyResult<bool> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.magnitude)
    }
    #[setter(magnitude)]
    fn binding_set_magnitude(&mut self, value: bool) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.magnitude = value;
        Ok(())
    }
    #[getter(phase)]
    fn binding_get_phase(&self) -> PyResult<bool> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.phase)
    }
    #[setter(phase)]
    fn binding_set_phase(&mut self, value: bool) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.phase = value;
        Ok(())
    }
    #[getter(normalization_hop)]
    fn binding_get_normalization_hop(&self) -> PyResult<Option<usize>> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.normalization_hop)
    }
    #[setter(normalization_hop)]
    fn binding_set_normalization_hop(&mut self, value: Option<usize>) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.normalization_hop = value;
        Ok(())
    }
}
#[pymethods]
impl StftAnalysisOptions {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(StftAnalysisOptions {
            inner: Some(crate::api::StftAnalysisOptions::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StftAnalysisOptions {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
