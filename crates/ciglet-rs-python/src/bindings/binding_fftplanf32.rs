use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct FftPlanF32 {
    pub(crate) inner: Option<crate::api::FftPlanF32>,
}
#[pymethods]
impl FftPlanF32 {
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
impl FftPlanF32 {
    #[new]
    #[pyo3(signature = (length))]
    pub fn binding_new(length: usize) -> pyo3::PyResult<Self> {
        (crate::api::FftPlanF32::new(length)).map(|value| FftPlanF32 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "is_empty")]
    #[pyo3(signature = ())]
    pub fn binding_is_empty(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).is_empty())
    }
    #[pyo3(name = "transform")]
    #[pyo3(signature = (real, imaginary, direction))]
    pub fn binding_transform(
        &mut self,
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
        direction: u32,
    ) -> pyo3::PyResult<ComplexValuesF32> {
        ((self.inner.as_mut().ok_or_else(consumed)?).transform(real, imaginary, direction))
            .map(|value| ComplexValuesF32 { inner: Some(value) })
    }
    #[pyo3(name = "process")]
    #[pyo3(signature = (interleaved, direction))]
    pub fn binding_process(
        &mut self,
        interleaved: Vec<f32>,
        direction: u32,
    ) -> pyo3::PyResult<ComplexValuesF32> {
        ((self.inner.as_mut().ok_or_else(consumed)?).process(&interleaved, direction))
            .map(|value| ComplexValuesF32 { inner: Some(value) })
    }
}
