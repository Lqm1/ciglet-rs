use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct MelOptionsF64 {
    pub(crate) inner: Option<crate::api::MelOptionsF64>,
}
#[pymethods]
impl MelOptionsF64 {
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
impl MelOptionsF64 {
    #[getter(minimum_hz)]
    fn binding_get_minimum_hz(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.minimum_hz)
    }
    #[setter(minimum_hz)]
    fn binding_set_minimum_hz(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.minimum_hz = value;
        Ok(())
    }
    #[getter(maximum_hz)]
    fn binding_get_maximum_hz(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.maximum_hz)
    }
    #[setter(maximum_hz)]
    fn binding_set_maximum_hz(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.maximum_hz = value;
        Ok(())
    }
    #[getter(scale)]
    fn binding_get_scale(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.scale)
    }
    #[setter(scale)]
    fn binding_set_scale(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.scale = value;
        Ok(())
    }
    #[getter(minimum_width_hz)]
    fn binding_get_minimum_width_hz(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.minimum_width_hz)
    }
    #[setter(minimum_width_hz)]
    fn binding_set_minimum_width_hz(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.minimum_width_hz = value;
        Ok(())
    }
}
#[pymethods]
impl MelOptionsF64 {
    #[new]
    #[pyo3(signature = (minimum_hz, maximum_hz, scale, minimum_width_hz))]
    pub fn binding_new(
        minimum_hz: f64,
        maximum_hz: f64,
        scale: f64,
        minimum_width_hz: f64,
    ) -> pyo3::PyResult<Self> {
        Ok(MelOptionsF64 {
            inner: Some(crate::api::MelOptionsF64::new(
                minimum_hz,
                maximum_hz,
                scale,
                minimum_width_hz,
            )),
        })
    }
}
