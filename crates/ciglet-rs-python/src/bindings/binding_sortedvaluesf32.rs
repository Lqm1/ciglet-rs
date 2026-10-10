use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct SortedValuesF32 {
    pub(crate) inner: Option<crate::api::SortedValuesF32>,
}
#[pymethods]
impl SortedValuesF32 {
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
impl SortedValuesF32 {
    #[new]
    #[pyo3(signature = (values, indices))]
    pub fn binding_new(values: Vec<f32>, indices: Vec<u32>) -> pyo3::PyResult<Self> {
        Ok(SortedValuesF32 {
            inner: Some(crate::api::SortedValuesF32::new(values, indices)),
        })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[getter(values)]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[getter(indices)]
    pub fn binding_indices(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).indices())
    }
}
