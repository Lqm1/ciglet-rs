use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionResonanceF64 {
    pub(crate) inner: Option<crate::api::PredictionResonanceF64>,
}
#[pymethods]
impl PredictionResonanceF64 {
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
impl PredictionResonanceF64 {
    #[new]
    #[pyo3(signature = (frequency_nyquist, real, imaginary))]
    pub fn binding_new(frequency_nyquist: f64, real: f64, imaginary: f64) -> pyo3::PyResult<Self> {
        Ok(PredictionResonanceF64 {
            inner: Some(crate::api::PredictionResonanceF64::new(
                frequency_nyquist,
                real,
                imaginary,
            )),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_components")]
    #[pyo3(signature = (values))]
    pub fn binding_from_components(values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::PredictionResonanceF64::from_components(&values))
            .map(|value| PredictionResonanceF64 { inner: Some(value) })
    }
    #[pyo3(name = "components")]
    #[pyo3(signature = ())]
    pub fn binding_components(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).components())
    }
    #[pyo3(name = "set_components")]
    #[pyo3(signature = (values))]
    pub fn binding_set_components(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_components(&values)
    }
    #[getter(frequency_nyquist)]
    pub fn binding_frequency_nyquist(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frequency_nyquist())
    }
    #[setter(frequency_nyquist)]
    pub fn binding_set_frequency_nyquist(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_frequency_nyquist(value);
        Ok(())
    }
    #[pyo3(name = "pole")]
    #[pyo3(signature = ())]
    pub fn binding_pole(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).pole())
    }
    #[pyo3(name = "set_pole")]
    #[pyo3(signature = (values))]
    pub fn binding_set_pole(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_pole(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PredictionResonanceF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
