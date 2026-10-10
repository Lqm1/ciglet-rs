use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionGainParts {
    pub(crate) inner: Option<crate::api::PredictionGainParts>,
}
#[pymethods]
impl PredictionGainParts {
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
impl PredictionGainParts {
    #[getter(fraction)]
    fn binding_get_fraction(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.fraction)
    }
    #[setter(fraction)]
    fn binding_set_fraction(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.fraction = value;
        Ok(())
    }
    #[getter(exponent)]
    fn binding_get_exponent(&self) -> PyResult<i32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.exponent)
    }
    #[setter(exponent)]
    fn binding_set_exponent(&mut self, value: i32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.exponent = value;
        Ok(())
    }
}
#[pymethods]
impl PredictionGainParts {
    #[pyo3(name = "fraction_storage")]
    #[pyo3(signature = ())]
    pub fn binding_fraction_storage(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).fraction_storage())
    }
}
