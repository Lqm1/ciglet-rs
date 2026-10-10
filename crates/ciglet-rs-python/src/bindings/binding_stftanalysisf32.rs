use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct StftAnalysisF32 {
    pub(crate) inner: Option<crate::api::StftAnalysisF32>,
}
#[pymethods]
impl StftAnalysisF32 {
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
impl StftAnalysisF32 {
    #[new]
    #[pyo3(signature = (frames, bins, magnitude, phase, window_weight, normalization))]
    pub fn binding_new(
        frames: usize,
        bins: usize,
        magnitude: Option<Vec<f32>>,
        phase: Option<Vec<f32>>,
        window_weight: Option<f32>,
        normalization: Option<f32>,
    ) -> pyo3::PyResult<Self> {
        Ok(StftAnalysisF32 {
            inner: Some(crate::api::StftAnalysisF32::new(
                frames,
                bins,
                magnitude,
                phase,
                window_weight,
                normalization,
            )),
        })
    }
    #[getter(frames)]
    pub fn binding_frames(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frames())
    }
    #[setter(frames)]
    pub fn binding_set_frames(&mut self, value: usize) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_frames(value);
        Ok(())
    }
    #[getter(bins)]
    pub fn binding_bins(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).bins())
    }
    #[setter(bins)]
    pub fn binding_set_bins(&mut self, value: usize) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_bins(value);
        Ok(())
    }
    #[getter(magnitude)]
    pub fn binding_magnitude(&self) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).magnitude())
    }
    #[setter(magnitude)]
    pub fn binding_set_magnitude(&mut self, value: Option<Vec<f32>>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_magnitude(value);
        Ok(())
    }
    #[getter(phase)]
    pub fn binding_phase(&self) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).phase())
    }
    #[setter(phase)]
    pub fn binding_set_phase(&mut self, value: Option<Vec<f32>>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_phase(value);
        Ok(())
    }
    #[getter(window_weight)]
    pub fn binding_window_weight(&self) -> pyo3::PyResult<Option<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).window_weight())
    }
    #[setter(window_weight)]
    pub fn binding_set_window_weight(&mut self, value: Option<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_window_weight(value);
        Ok(())
    }
    #[getter(normalization)]
    pub fn binding_normalization(&self) -> pyo3::PyResult<Option<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).normalization())
    }
    #[setter(normalization)]
    pub fn binding_set_normalization(&mut self, value: Option<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_normalization(value);
        Ok(())
    }
    #[pyo3(name = "window_weight_storage")]
    #[pyo3(signature = ())]
    pub fn binding_window_weight_storage(&self) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).window_weight_storage())
    }
    #[pyo3(name = "set_window_weight_storage")]
    #[pyo3(signature = (values))]
    pub fn binding_set_window_weight_storage(
        &mut self,
        values: Option<Vec<f32>>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_window_weight_storage(values)
    }
    #[pyo3(name = "normalization_storage")]
    #[pyo3(signature = ())]
    pub fn binding_normalization_storage(&self) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).normalization_storage())
    }
    #[pyo3(name = "set_normalization_storage")]
    #[pyo3(signature = (values))]
    pub fn binding_set_normalization_storage(
        &mut self,
        values: Option<Vec<f32>>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_normalization_storage(values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StftAnalysisF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
