use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct ScaledPredictionF64 {
    pub(crate) inner: Option<crate::api::ScaledPredictionF64>,
}
#[pymethods]
impl ScaledPredictionF64 {
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
impl ScaledPredictionF64 {
    #[getter(coefficients)]
    pub fn binding_coefficients(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).coefficients())
    }
    #[pyo3(name = "autocorrelation_parts")]
    #[pyo3(signature = ())]
    pub fn binding_autocorrelation_parts(&self) -> pyo3::PyResult<PredictionCorrelationParts> {
        Ok(PredictionCorrelationParts {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).autocorrelation_parts()),
        })
    }
    #[pyo3(name = "into_prediction")]
    #[pyo3(signature = ())]
    pub fn binding_into_prediction(&mut self) -> pyo3::PyResult<PredictionF64> {
        ((self.inner.take().ok_or_else(consumed)?).into_prediction())
            .map(|value| PredictionF64 { inner: Some(value) })
    }
    #[pyo3(name = "gain_parts")]
    #[pyo3(signature = ())]
    pub fn binding_gain_parts(&self) -> pyo3::PyResult<PredictionGainParts> {
        ((self.inner.as_ref().ok_or_else(consumed)?).gain_parts())
            .map(|value| PredictionGainParts { inner: Some(value) })
    }
    #[pyo3(name = "gain")]
    #[pyo3(signature = ())]
    pub fn binding_gain(&self) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).gain()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ScaledPredictionF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
