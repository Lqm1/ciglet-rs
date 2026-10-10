use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct ProcessOutput {
    pub(crate) inner: Option<crate::api::ProcessOutput>,
}
#[pymethods]
impl ProcessOutput {
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
impl ProcessOutput {
    #[getter(success)]
    fn binding_get_success(&self) -> PyResult<bool> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.success)
    }
    #[setter(success)]
    fn binding_set_success(&mut self, value: bool) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.success = value;
        Ok(())
    }
    #[getter(code)]
    fn binding_get_code(&self) -> PyResult<Option<i32>> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.code)
    }
    #[setter(code)]
    fn binding_set_code(&mut self, value: Option<i32>) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.code = value;
        Ok(())
    }
}
#[pymethods]
impl ProcessOutput {
    #[pyo3(name = "stdout")]
    #[pyo3(signature = ())]
    pub fn binding_stdout(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        Ok({
            let bytes = (self.inner.as_ref().ok_or_else(consumed)?).stdout();
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "stderr")]
    #[pyo3(signature = ())]
    pub fn binding_stderr(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        Ok({
            let bytes = (self.inner.as_ref().ok_or_else(consumed)?).stderr();
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
}
