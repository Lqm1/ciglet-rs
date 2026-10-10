use super::*;
#[pyclass(unsendable, module = "ciglet_rs")]
pub struct RandomWordSource {
    pub(crate) inner: Option<crate::api::RandomWordSource>,
}
#[pymethods]
impl RandomWordSource {
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
impl RandomWordSource {
    #[staticmethod]
    #[pyo3(name = "from_seed")]
    #[pyo3(signature = (seed))]
    pub fn binding_from_seed(seed: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::RandomWordSource::from_seed(&seed))
            .map(|value| RandomWordSource { inner: Some(value) })
    }
    #[new]
    #[pyo3(signature = (callback))]
    pub fn binding_new(callback: Py<PyAny>) -> pyo3::PyResult<Self> {
        Ok(RandomWordSource {
            inner: Some(crate::api::RandomWordSource::new(
                crate::callbacks::Function::new(callback)?,
            )),
        })
    }
    #[pyo3(name = "callback")]
    #[pyo3(signature = ())]
    pub fn binding_callback(&mut self) -> pyo3::PyResult<Option<pyo3::Py<pyo3::PyAny>>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).callback())
            .map(|value| (value).map(|value| value.into_callable()))
    }
    #[pyo3(name = "next_u32")]
    #[pyo3(signature = ())]
    pub fn binding_next_u32(&mut self) -> pyo3::PyResult<u32> {
        (self.inner.as_ref().ok_or_else(consumed)?).next_u32()
    }
    #[pyo3(name = "next_u64")]
    #[pyo3(signature = ())]
    pub fn binding_next_u64(&mut self) -> pyo3::PyResult<u64> {
        (self.inner.as_ref().ok_or_else(consumed)?).next_u64()
    }
    #[pyo3(name = "fill_bytes")]
    #[pyo3(signature = (count))]
    pub fn binding_fill_bytes(
        &mut self,
        count: usize,
    ) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).fill_bytes(count)).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "seed")]
    #[pyo3(signature = ())]
    pub fn binding_seed(&mut self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).seed()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "stream")]
    #[pyo3(signature = ())]
    pub fn binding_stream(&mut self) -> pyo3::PyResult<u64> {
        (self.inner.as_ref().ok_or_else(consumed)?).stream()
    }
    #[pyo3(name = "set_stream")]
    #[pyo3(signature = (stream))]
    pub fn binding_set_stream(&mut self, stream: u64) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).set_stream(stream)
    }
    #[pyo3(name = "word_position")]
    #[pyo3(signature = ())]
    pub fn binding_word_position(&mut self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).word_position()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "set_word_position")]
    #[pyo3(signature = (position))]
    pub fn binding_set_word_position(&mut self, position: Vec<u8>) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).set_word_position(&position)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&mut self) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).cloned())
            .map(|value| RandomWordSource { inner: Some(value) })
    }
}
