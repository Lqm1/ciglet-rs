use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionF32 {
    pub(crate) inner: Option<crate::api::PredictionF32>,
}
#[pymethods]
impl PredictionF32 {
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
impl PredictionF32 {
    #[new]
    #[pyo3(signature = (coefficients, autocorrelation))]
    pub fn binding_new(coefficients: Vec<f32>, autocorrelation: Vec<f32>) -> pyo3::PyResult<Self> {
        Ok(PredictionF32 {
            inner: Some(crate::api::PredictionF32::new(
                coefficients,
                autocorrelation,
            )),
        })
    }
    #[getter(coefficients)]
    pub fn binding_coefficients(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).coefficients())
    }
    #[setter(coefficients)]
    pub fn binding_set_coefficients(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_coefficients(values);
        Ok(())
    }
    #[getter(autocorrelation)]
    pub fn binding_autocorrelation(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).autocorrelation())
    }
    #[setter(autocorrelation)]
    pub fn binding_set_autocorrelation(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_autocorrelation(values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PredictionF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
