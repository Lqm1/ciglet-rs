use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct ComplexValuesF64 {
    pub(crate) inner: Option<crate::api::ComplexValuesF64>,
}
#[pymethods]
impl ComplexValuesF64 {
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
impl ComplexValuesF64 {
    #[new]
    #[pyo3(signature = (real, imaginary))]
    pub fn binding_new(real: Vec<f64>, imaginary: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::ComplexValuesF64::new(real, imaginary))
            .map(|value| ComplexValuesF64 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[getter(real)]
    pub fn binding_real(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).real())
    }
    #[getter(imaginary)]
    pub fn binding_imaginary(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).imaginary())
    }
    #[getter(interleaved)]
    pub fn binding_interleaved(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).interleaved())
    }
}
