use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct InstantaneousFrequencyKernelsF32 {
    pub(crate) inner: Option<crate::api::InstantaneousFrequencyKernelsF32>,
}
#[pymethods]
impl InstantaneousFrequencyKernelsF32 {
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
impl InstantaneousFrequencyKernelsF32 {
    #[new]
    #[pyo3(signature = (real, imaginary, derivative_real, derivative_imaginary))]
    pub fn binding_new(
        real: Vec<f32>,
        imaginary: Vec<f32>,
        derivative_real: Vec<f32>,
        derivative_imaginary: Vec<f32>,
    ) -> pyo3::PyResult<Self> {
        Ok(InstantaneousFrequencyKernelsF32 {
            inner: Some(crate::api::InstantaneousFrequencyKernelsF32::new(
                real,
                imaginary,
                derivative_real,
                derivative_imaginary,
            )),
        })
    }
    #[getter(real)]
    pub fn binding_real(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).real())
    }
    #[setter(real)]
    pub fn binding_set_real(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_real(values);
        Ok(())
    }
    #[getter(imaginary)]
    pub fn binding_imaginary(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).imaginary())
    }
    #[setter(imaginary)]
    pub fn binding_set_imaginary(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_imaginary(values);
        Ok(())
    }
    #[getter(derivative_real)]
    pub fn binding_derivative_real(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).derivative_real())
    }
    #[setter(derivative_real)]
    pub fn binding_set_derivative_real(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_derivative_real(values);
        Ok(())
    }
    #[getter(derivative_imaginary)]
    pub fn binding_derivative_imaginary(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).derivative_imaginary())
    }
    #[setter(derivative_imaginary)]
    pub fn binding_set_derivative_imaginary(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_derivative_imaginary(values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(InstantaneousFrequencyKernelsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
