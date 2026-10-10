use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionResonanceF32 {
    pub(crate) inner: Option<crate::api::PredictionResonanceF32>,
}
#[pymethods]
impl PredictionResonanceF32 {
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
impl PredictionResonanceF32 {
    #[new]
    #[pyo3(signature = (frequency_nyquist, real, imaginary))]
    pub fn binding_new(frequency_nyquist: f32, real: f32, imaginary: f32) -> pyo3::PyResult<Self> {
        Ok(PredictionResonanceF32 {
            inner: Some(crate::api::PredictionResonanceF32::new(
                frequency_nyquist,
                real,
                imaginary,
            )),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_components")]
    #[pyo3(signature = (values))]
    pub fn binding_from_components(values: Vec<f32>) -> pyo3::PyResult<Self> {
        (crate::api::PredictionResonanceF32::from_components(&values))
            .map(|value| PredictionResonanceF32 { inner: Some(value) })
    }
    #[pyo3(name = "components")]
    #[pyo3(signature = ())]
    pub fn binding_components(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).components())
    }
    #[pyo3(name = "set_components")]
    #[pyo3(signature = (values))]
    pub fn binding_set_components(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_components(&values)
    }
    #[getter(frequency_nyquist)]
    pub fn binding_frequency_nyquist(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frequency_nyquist())
    }
    #[setter(frequency_nyquist)]
    pub fn binding_set_frequency_nyquist(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_frequency_nyquist(value);
        Ok(())
    }
    #[pyo3(name = "pole")]
    #[pyo3(signature = ())]
    pub fn binding_pole(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).pole())
    }
    #[pyo3(name = "set_pole")]
    #[pyo3(signature = (values))]
    pub fn binding_set_pole(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_pole(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PredictionResonanceF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
