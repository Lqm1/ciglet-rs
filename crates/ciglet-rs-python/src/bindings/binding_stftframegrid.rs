use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct StftFrameGrid {
    pub(crate) inner: Option<crate::api::StftFrameGrid>,
}
#[pymethods]
impl StftFrameGrid {
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
impl StftFrameGrid {
    #[getter(hop)]
    fn binding_get_hop(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.hop)
    }
    #[setter(hop)]
    fn binding_set_hop(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.hop = value;
        Ok(())
    }
    #[getter(frames)]
    fn binding_get_frames(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.frames)
    }
    #[setter(frames)]
    fn binding_set_frames(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.frames = value;
        Ok(())
    }
    #[getter(window_factor)]
    fn binding_get_window_factor(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.window_factor)
    }
    #[setter(window_factor)]
    fn binding_set_window_factor(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.window_factor = value;
        Ok(())
    }
    #[getter(zero_padding_factor)]
    fn binding_get_zero_padding_factor(&self) -> PyResult<usize> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(consumed)?
            .zero_padding_factor)
    }
    #[setter(zero_padding_factor)]
    fn binding_set_zero_padding_factor(&mut self, value: usize) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(consumed)?
            .zero_padding_factor = value;
        Ok(())
    }
}
#[pymethods]
impl StftFrameGrid {
    #[new]
    #[pyo3(signature = (hop, frames, window_factor, zero_padding_factor))]
    pub fn binding_new(
        hop: usize,
        frames: usize,
        window_factor: usize,
        zero_padding_factor: usize,
    ) -> pyo3::PyResult<Self> {
        Ok(StftFrameGrid {
            inner: Some(crate::api::StftFrameGrid::new(
                hop,
                frames,
                window_factor,
                zero_padding_factor,
            )),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StftFrameGrid {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
