use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct InstantaneousFrequencyDetectorF32 {
    pub(crate) inner: Option<crate::api::InstantaneousFrequencyDetectorF32>,
}
#[pymethods]
impl InstantaneousFrequencyDetectorF32 {
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
impl InstantaneousFrequencyDetectorF32 {
    #[new]
    #[pyo3(signature = (center_frequency, resolution))]
    pub fn binding_new(center_frequency: f32, resolution: f32) -> pyo3::PyResult<Self> {
        (crate::api::InstantaneousFrequencyDetectorF32::new(center_frequency, resolution))
            .map(|value| InstantaneousFrequencyDetectorF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_kernels")]
    #[pyo3(signature = (center_frequency, kernels))]
    pub fn binding_from_kernels(
        center_frequency: f32,
        kernels: pyo3::PyRef<'_, InstantaneousFrequencyKernelsF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::InstantaneousFrequencyDetectorF32::from_kernels(
            center_frequency,
            kernels.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| InstantaneousFrequencyDetectorF32 { inner: Some(value) })
    }
    #[getter(center_frequency)]
    pub fn binding_center_frequency(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).center_frequency())
    }
    #[pyo3(name = "set_center_frequency")]
    #[pyo3(signature = (value))]
    pub fn binding_set_center_frequency(&mut self, value: f32) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_center_frequency(value)
    }
    #[getter(kernels)]
    pub fn binding_kernels(&self) -> pyo3::PyResult<InstantaneousFrequencyKernelsF32> {
        Ok(InstantaneousFrequencyKernelsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).kernels()),
        })
    }
    #[setter(kernels)]
    pub fn binding_set_kernels(
        &mut self,
        kernels: pyo3::PyRef<'_, InstantaneousFrequencyKernelsF32>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_kernels(kernels.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "estimate")]
    #[pyo3(signature = (samples))]
    pub fn binding_estimate(&self, samples: Vec<f32>) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).estimate(&samples)
    }
    #[pyo3(name = "estimate_with")]
    #[pyo3(signature = (samples, policy_code))]
    pub fn binding_estimate_with(
        &self,
        samples: Vec<f32>,
        policy_code: u32,
    ) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).estimate_with(&samples, policy_code)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(InstantaneousFrequencyDetectorF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
