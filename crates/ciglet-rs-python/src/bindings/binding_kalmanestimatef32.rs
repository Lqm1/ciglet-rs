use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct KalmanEstimateF32 {
    pub(crate) inner: Option<crate::api::KalmanEstimateF32>,
}
#[pymethods]
impl KalmanEstimateF32 {
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
impl KalmanEstimateF32 {
    #[new]
    #[pyo3(signature = (means, variances, log_likelihood))]
    pub fn binding_new(
        means: Vec<f32>,
        variances: Vec<f32>,
        log_likelihood: f32,
    ) -> pyo3::PyResult<Self> {
        Ok(KalmanEstimateF32 {
            inner: Some(crate::api::KalmanEstimateF32::new(
                &means,
                &variances,
                log_likelihood,
            )),
        })
    }
    #[getter(means)]
    pub fn binding_means(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).means())
    }
    #[setter(means)]
    pub fn binding_set_means(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_means(&values);
        Ok(())
    }
    #[getter(variances)]
    pub fn binding_variances(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).variances())
    }
    #[setter(variances)]
    pub fn binding_set_variances(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_variances(&values);
        Ok(())
    }
    #[getter(log_likelihood)]
    pub fn binding_log_likelihood(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).log_likelihood())
    }
    #[setter(log_likelihood)]
    pub fn binding_set_log_likelihood(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_log_likelihood(value);
        Ok(())
    }
    #[pyo3(name = "likelihood_value")]
    #[pyo3(signature = ())]
    pub fn binding_likelihood_value(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).likelihood_value())
    }
    #[pyo3(name = "set_likelihood_value")]
    #[pyo3(signature = (values))]
    pub fn binding_set_likelihood_value(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_likelihood_value(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(KalmanEstimateF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
