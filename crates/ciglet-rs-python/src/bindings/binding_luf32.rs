use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct LuF32 {
    pub(crate) inner: Option<crate::api::LuF32>,
}
#[pymethods]
impl LuF32 {
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
impl LuF32 {
    #[staticmethod]
    #[pyo3(name = "factor")]
    #[pyo3(signature = (matrix))]
    pub fn binding_factor(mut matrix: pyo3::PyRefMut<'_, MatrixF32>) -> pyo3::PyResult<Self> {
        (crate::api::LuF32::factor(matrix.inner.take().ok_or_else(consumed)?))
            .map(|value| LuF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "factor_unpivoted")]
    #[pyo3(signature = (matrix))]
    pub fn binding_factor_unpivoted(
        mut matrix: pyo3::PyRefMut<'_, MatrixF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::LuF32::factor_unpivoted(matrix.inner.take().ok_or_else(consumed)?))
            .map(|value| LuF32 { inner: Some(value) })
    }
    #[pyo3(name = "packed")]
    #[pyo3(signature = ())]
    pub fn binding_packed(&self) -> pyo3::PyResult<MatrixF32> {
        Ok(MatrixF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).packed()),
        })
    }
    #[pyo3(name = "row_swaps")]
    #[pyo3(signature = ())]
    pub fn binding_row_swaps(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).row_swaps())
    }
    #[pyo3(name = "solve")]
    #[pyo3(signature = (right))]
    pub fn binding_solve(&self, right: Vec<f32>) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).solve(&right)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(LuF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
