use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionCorrelationParts {
    pub(crate) inner: Option<crate::api::PredictionCorrelationParts>,
}
#[pymethods]
impl PredictionCorrelationParts {
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
impl PredictionCorrelationParts {
    #[getter(fractions)]
    pub fn binding_fractions(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).fractions())
    }
    #[getter(exponents)]
    pub fn binding_exponents(&self) -> pyo3::PyResult<Vec<i32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).exponents())
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
}
