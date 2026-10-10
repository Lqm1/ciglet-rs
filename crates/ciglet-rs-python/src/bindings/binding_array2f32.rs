use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct Array2F32 {
    pub(crate) inner: Option<crate::api::Array2F32>,
}
#[pymethods]
impl Array2F32 {
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
impl Array2F32 {
    #[new]
    #[pyo3(signature = (rows, columns, values))]
    pub fn binding_new(rows: usize, columns: usize, values: Vec<f32>) -> pyo3::PyResult<Self> {
        (crate::api::Array2F32::new(rows, columns, &values))
            .map(|value| Array2F32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "zeros")]
    #[pyo3(signature = (rows, columns))]
    pub fn binding_zeros(rows: usize, columns: usize) -> pyo3::PyResult<Self> {
        (crate::api::Array2F32::zeros(rows, columns)).map(|value| Array2F32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "filled")]
    #[pyo3(signature = (rows, columns, element))]
    pub fn binding_filled(rows: usize, columns: usize, element: f32) -> pyo3::PyResult<Self> {
        (crate::api::Array2F32::filled(rows, columns, element))
            .map(|value| Array2F32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_rows")]
    #[pyo3(signature = (values, row_lengths))]
    pub fn binding_from_rows(values: Vec<f32>, row_lengths: Vec<u32>) -> pyo3::PyResult<Self> {
        (crate::api::Array2F32::from_rows(&values, &row_lengths))
            .map(|value| Array2F32 { inner: Some(value) })
    }
    #[getter(rows)]
    pub fn binding_rows(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).rows())
    }
    #[getter(columns)]
    pub fn binding_columns(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).columns())
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
    #[pyo3(name = "get")]
    #[pyo3(signature = (row, column))]
    pub fn binding_get(&self, row: usize, column: usize) -> pyo3::PyResult<Option<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(row, column))
    }
    #[pyo3(name = "set")]
    #[pyo3(signature = (row, column, value))]
    pub fn binding_set(&mut self, row: usize, column: usize, value: f32) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_mut().ok_or_else(consumed)?).set(row, column, value))
    }
    #[pyo3(name = "row")]
    #[pyo3(signature = (row))]
    pub fn binding_row(&self, row: usize) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).row(row))
    }
    #[pyo3(name = "set_row")]
    #[pyo3(signature = (row, values))]
    pub fn binding_set_row(&mut self, row: usize, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_row(row, &values)
    }
    #[pyo3(name = "reshape")]
    #[pyo3(signature = (rows, columns))]
    pub fn binding_reshape(&mut self, rows: usize, columns: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).reshape(rows, columns)
    }
    #[pyo3(name = "transpose")]
    #[pyo3(signature = ())]
    pub fn binding_transpose(&mut self) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).transpose()
    }
    #[pyo3(name = "transposed")]
    #[pyo3(signature = ())]
    pub fn binding_transposed(&self) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).transposed())
            .map(|value| Array2F32 { inner: Some(value) })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).cloned())
            .map(|value| Array2F32 { inner: Some(value) })
    }
    #[pyo3(name = "flatten")]
    #[pyo3(signature = ())]
    pub fn binding_flatten(&self) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).flatten()
    }
    #[pyo3(name = "into_row_major")]
    #[pyo3(signature = ())]
    pub fn binding_into_row_major(&mut self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.take().ok_or_else(consumed)?).into_row_major())
    }
}
