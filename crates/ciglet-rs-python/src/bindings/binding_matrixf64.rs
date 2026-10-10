use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct MatrixF64 {
    pub(crate) inner: Option<crate::api::MatrixF64>,
}
#[pymethods]
impl MatrixF64 {
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
impl MatrixF64 {
    #[new]
    #[pyo3(signature = (rows, columns, values))]
    pub fn binding_new(rows: usize, columns: usize, values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::MatrixF64::new(rows, columns, values))
            .map(|value| MatrixF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "zeros")]
    #[pyo3(signature = (rows, columns))]
    pub fn binding_zeros(rows: usize, columns: usize) -> pyo3::PyResult<Self> {
        (crate::api::MatrixF64::zeros(rows, columns)).map(|value| MatrixF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "identity")]
    #[pyo3(signature = (size))]
    pub fn binding_identity(size: usize) -> pyo3::PyResult<Self> {
        (crate::api::MatrixF64::identity(size)).map(|value| MatrixF64 { inner: Some(value) })
    }
    #[getter(rows)]
    pub fn binding_rows(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).rows())
    }
    #[getter(columns)]
    pub fn binding_columns(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).columns())
    }
    #[pyo3(name = "as_slice")]
    #[pyo3(signature = ())]
    pub fn binding_as_slice(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).as_slice())
    }
    #[pyo3(name = "into_column_major")]
    #[pyo3(signature = ())]
    pub fn binding_into_column_major(&mut self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.take().ok_or_else(consumed)?).into_column_major())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (row, column))]
    pub fn binding_get(&self, row: usize, column: usize) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(row, column))
    }
    #[pyo3(name = "apply_row_swaps")]
    #[pyo3(signature = (partners))]
    pub fn binding_apply_row_swaps(&mut self, partners: Vec<u32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).apply_row_swaps(&partners)
    }
    #[pyo3(name = "pivot_rows")]
    #[pyo3(signature = ())]
    pub fn binding_pivot_rows(&mut self) -> pyo3::PyResult<Vec<u32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).pivot_rows()
    }
    #[pyo3(name = "multiply")]
    #[pyo3(signature = (right))]
    pub fn binding_multiply(&self, right: pyo3::PyRef<'_, MatrixF64>) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .multiply(right.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| MatrixF64 { inner: Some(value) })
    }
    #[pyo3(name = "multiply_vector")]
    #[pyo3(signature = (values))]
    pub fn binding_multiply_vector(&self, values: Vec<f64>) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_ref().ok_or_else(consumed)?).multiply_vector(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(MatrixF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
