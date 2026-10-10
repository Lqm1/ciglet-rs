use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct ScaledPredictionF32 {
    pub(crate) inner: Option<crate::api::ScaledPredictionF32>,
}
#[pymethods]
impl ScaledPredictionF32 {
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
impl ScaledPredictionF32 {
    #[getter(coefficients)]
    pub fn binding_coefficients(&self) -> pyo3::PyResult<Vec<f32>> {
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
    pub fn binding_into_prediction(&mut self) -> pyo3::PyResult<PredictionF32> {
        ((self.inner.take().ok_or_else(consumed)?).into_prediction())
            .map(|value| PredictionF32 { inner: Some(value) })
    }
    #[pyo3(name = "gain_parts")]
    #[pyo3(signature = ())]
    pub fn binding_gain_parts(&self) -> pyo3::PyResult<PredictionGainParts> {
        ((self.inner.as_ref().ok_or_else(consumed)?).gain_parts())
            .map(|value| PredictionGainParts { inner: Some(value) })
    }
    #[pyo3(name = "gain")]
    #[pyo3(signature = ())]
    pub fn binding_gain(&self) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).gain()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ScaledPredictionF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
