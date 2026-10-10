use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct GlottalSpectrumF64 {
    pub(crate) inner: Option<crate::api::GlottalSpectrumF64>,
}
#[pymethods]
impl GlottalSpectrumF64 {
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
impl GlottalSpectrumF64 {
    #[new]
    #[pyo3(signature = (magnitude, phase))]
    pub fn binding_new(magnitude: Vec<f64>, phase: Vec<f64>) -> pyo3::PyResult<Self> {
        Ok(GlottalSpectrumF64 {
            inner: Some(crate::api::GlottalSpectrumF64::new(magnitude, phase)),
        })
    }
    #[getter(magnitude)]
    pub fn binding_magnitude(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).magnitude())
    }
    #[setter(magnitude)]
    pub fn binding_set_magnitude(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_magnitude(values);
        Ok(())
    }
    #[getter(phase)]
    pub fn binding_phase(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).phase())
    }
    #[setter(phase)]
    pub fn binding_set_phase(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_phase(values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GlottalSpectrumF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
