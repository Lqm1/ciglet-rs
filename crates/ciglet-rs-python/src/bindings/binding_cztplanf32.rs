use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct CztPlanF32 {
    pub(crate) inner: Option<crate::api::CztPlanF32>,
}
#[pymethods]
impl CztPlanF32 {
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
impl CztPlanF32 {
    #[new]
    #[pyo3(signature = (length, angular_step, direction))]
    pub fn binding_new(length: usize, angular_step: f32, direction: u32) -> pyo3::PyResult<Self> {
        (crate::api::CztPlanF32::new(length, angular_step, direction))
            .map(|value| CztPlanF32 { inner: Some(value) })
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
    #[pyo3(signature = (real, imaginary))]
    pub fn binding_transform(
        &mut self,
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
    ) -> pyo3::PyResult<ComplexValuesF32> {
        ((self.inner.as_mut().ok_or_else(consumed)?).transform(real, imaginary))
            .map(|value| ComplexValuesF32 { inner: Some(value) })
    }
    #[pyo3(name = "transform_into")]
    #[pyo3(signature = (real, imaginary, output))]
    pub fn binding_transform_into(
        &mut self,
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
        mut output: Vec<f32>,
    ) -> pyo3::PyResult<((), Vec<f32>)> {
        {
            ((self.inner.as_mut().ok_or_else(consumed)?).transform_into(
                real,
                imaginary,
                &mut output,
            ))?;
            Ok(((), output))
        }
    }
}
