use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PlotCommands {
    pub(crate) inner: Option<crate::api::PlotCommands>,
}
#[pymethods]
impl PlotCommands {
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
impl PlotCommands {
    #[new]
    #[pyo3(signature = (bytes))]
    pub fn binding_new(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        Ok(PlotCommands {
            inner: Some(crate::api::PlotCommands::new(bytes)),
        })
    }
    #[getter(bytes)]
    pub fn binding_bytes(&mut self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        Ok({
            let bytes = (self.inner.as_mut().ok_or_else(consumed)?).bytes();
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[setter(bytes)]
    pub fn binding_set_bytes(&mut self, bytes: Vec<u8>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_bytes(bytes);
        Ok(())
    }
    #[pyo3(name = "append")]
    #[pyo3(signature = (bytes))]
    pub fn binding_append(&mut self, bytes: Vec<u8>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).append(&bytes)
    }
    #[pyo3(name = "into_inner")]
    #[pyo3(signature = ())]
    pub fn binding_into_inner(&mut self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        Ok({
            let bytes = (self.inner.take().ok_or_else(consumed)?).into_inner();
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&mut self) -> pyo3::PyResult<Self> {
        Ok(PlotCommands {
            inner: Some((self.inner.as_mut().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "line_f32")]
    #[pyo3(signature = (x, y, code))]
    pub fn binding_line_f32(
        &mut self,
        x: Option<Vec<f32>>,
        y: Vec<f32>,
        code: u32,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).line_f32(x, &y, code)
    }
    #[pyo3(name = "line_f64")]
    #[pyo3(signature = (x, y, code))]
    pub fn binding_line_f64(
        &mut self,
        x: Option<Vec<f64>>,
        y: Vec<f64>,
        code: u32,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).line_f64(x, &y, code)
    }
    #[pyo3(name = "image_f32")]
    #[pyo3(signature = (rows, columns, values))]
    pub fn binding_image_f32(
        &mut self,
        rows: usize,
        columns: usize,
        values: Vec<f32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).image_f32(rows, columns, values)
    }
    #[pyo3(name = "image_f64")]
    #[pyo3(signature = (rows, columns, values))]
    pub fn binding_image_f64(
        &mut self,
        rows: usize,
        columns: usize,
        values: Vec<f64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).image_f64(rows, columns, values)
    }
}
