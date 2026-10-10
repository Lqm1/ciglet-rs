use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionF64 {
    pub(crate) inner: Option<crate::api::PredictionF64>,
}
#[pymethods]
impl PredictionF64 {
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
impl PredictionF64 {
    #[new]
    #[pyo3(signature = (coefficients, autocorrelation))]
    pub fn binding_new(coefficients: Vec<f64>, autocorrelation: Vec<f64>) -> pyo3::PyResult<Self> {
        Ok(PredictionF64 {
            inner: Some(crate::api::PredictionF64::new(
                coefficients,
                autocorrelation,
            )),
        })
    }
    #[getter(coefficients)]
    pub fn binding_coefficients(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).coefficients())
    }
    #[setter(coefficients)]
    pub fn binding_set_coefficients(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_coefficients(values);
        Ok(())
    }
    #[getter(autocorrelation)]
    pub fn binding_autocorrelation(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).autocorrelation())
    }
    #[setter(autocorrelation)]
    pub fn binding_set_autocorrelation(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_autocorrelation(values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PredictionF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
