use super::*;
#[pyclass(unsendable, module = "ciglet_rs")]
pub struct Figure {
    pub(crate) inner: Option<crate::api::Figure>,
}
#[pymethods]
impl Figure {
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
impl Figure {
    #[staticmethod]
    #[pyo3(name = "open")]
    #[pyo3(signature = ())]
    pub fn binding_open() -> pyo3::PyResult<Self> {
        (crate::api::Figure::open()).map(|value| Figure { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "open_with")]
    #[pyo3(signature = (program))]
    pub fn binding_open_with(program: &str) -> pyo3::PyResult<Self> {
        (crate::api::Figure::open_with(program)).map(|value| Figure { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "spawn")]
    #[pyo3(signature = (program, arguments))]
    pub fn binding_spawn(program: &str, arguments: Vec<String>) -> pyo3::PyResult<Self> {
        (crate::api::Figure::spawn(program, arguments)).map(|value| Figure { inner: Some(value) })
    }
    #[pyo3(name = "id")]
    #[pyo3(signature = ())]
    pub fn binding_id(&self) -> pyo3::PyResult<u32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).id())
    }
    #[pyo3(name = "append")]
    #[pyo3(signature = (bytes))]
    pub fn binding_append(&mut self, bytes: Vec<u8>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).append(&bytes)
    }
    #[pyo3(name = "flush")]
    #[pyo3(signature = ())]
    pub fn binding_flush(&mut self) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).flush()
    }
    #[pyo3(name = "close_with_output")]
    #[pyo3(signature = ())]
    pub fn binding_close_with_output(&mut self) -> pyo3::PyResult<ProcessOutput> {
        ((self.inner.take().ok_or_else(consumed)?).close_with_output())
            .map(|value| ProcessOutput { inner: Some(value) })
    }
}
