use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct StftSynthesisOptionsF32 {
    pub(crate) inner: Option<crate::api::StftSynthesisOptionsF32>,
}
#[pymethods]
impl StftSynthesisOptionsF32 {
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
impl StftSynthesisOptionsF32 {
    #[getter(hop)]
    fn binding_get_hop(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.hop)
    }
    #[setter(hop)]
    fn binding_set_hop(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.hop = value;
        Ok(())
    }
    #[getter(offset)]
    fn binding_get_offset(&self) -> PyResult<i32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.offset)
    }
    #[setter(offset)]
    fn binding_set_offset(&mut self, value: i32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.offset = value;
        Ok(())
    }
    #[getter(fade_samples)]
    fn binding_get_fade_samples(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.fade_samples)
    }
    #[setter(fade_samples)]
    fn binding_set_fade_samples(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.fade_samples = value;
        Ok(())
    }
    #[getter(normalization)]
    fn binding_get_normalization(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.normalization)
    }
    #[setter(normalization)]
    fn binding_set_normalization(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.normalization = value;
        Ok(())
    }
    #[getter(nyquist)]
    fn binding_get_nyquist(&self) -> PyResult<u32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.nyquist)
    }
    #[setter(nyquist)]
    fn binding_set_nyquist(&mut self, value: u32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.nyquist = value;
        Ok(())
    }
}
#[pymethods]
impl StftSynthesisOptionsF32 {
    #[new]
    #[pyo3(signature = (hop, offset, fade_samples, normalization, nyquist))]
    pub fn binding_new(
        hop: usize,
        offset: i32,
        fade_samples: usize,
        normalization: f32,
        nyquist: u32,
    ) -> pyo3::PyResult<Self> {
        Ok(StftSynthesisOptionsF32 {
            inner: Some(crate::api::StftSynthesisOptionsF32::new(
                hop,
                offset,
                fade_samples,
                normalization,
                nyquist,
            )),
        })
    }
    #[pyo3(name = "normalization_storage")]
    #[pyo3(signature = ())]
    pub fn binding_normalization_storage(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).normalization_storage())
    }
    #[pyo3(name = "set_normalization_storage")]
    #[pyo3(signature = (values))]
    pub fn binding_set_normalization_storage(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_normalization_storage(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StftSynthesisOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
