use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct SinusoidF32 {
    pub(crate) inner: Option<crate::api::SinusoidF32>,
}
#[pymethods]
impl SinusoidF32 {
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
impl SinusoidF32 {
    #[new]
    #[pyo3(signature = (frequency_hz, amplitude, phase_radians))]
    pub fn binding_new(
        frequency_hz: f32,
        amplitude: f32,
        phase_radians: f32,
    ) -> pyo3::PyResult<Self> {
        Ok(SinusoidF32 {
            inner: Some(crate::api::SinusoidF32::new(
                frequency_hz,
                amplitude,
                phase_radians,
            )),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_values")]
    #[pyo3(signature = (values))]
    pub fn binding_from_values(values: Vec<f32>) -> pyo3::PyResult<Self> {
        (crate::api::SinusoidF32::from_values(&values))
            .map(|value| SinusoidF32 { inner: Some(value) })
    }
    #[getter(frequency_hz)]
    pub fn binding_frequency_hz(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frequency_hz())
    }
    #[setter(frequency_hz)]
    pub fn binding_set_frequency_hz(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_frequency_hz(value);
        Ok(())
    }
    #[getter(amplitude)]
    pub fn binding_amplitude(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).amplitude())
    }
    #[setter(amplitude)]
    pub fn binding_set_amplitude(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_amplitude(value);
        Ok(())
    }
    #[getter(phase_radians)]
    pub fn binding_phase_radians(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).phase_radians())
    }
    #[setter(phase_radians)]
    pub fn binding_set_phase_radians(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_phase_radians(value);
        Ok(())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "set_values")]
    #[pyo3(signature = (values))]
    pub fn binding_set_values(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_values(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SinusoidF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
