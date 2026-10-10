use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct StftPlanF32 {
    pub(crate) inner: Option<crate::api::StftPlanF32>,
}
#[pymethods]
impl StftPlanF32 {
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
impl StftPlanF32 {
    #[new]
    #[pyo3(signature = (length, window_code, precision_code))]
    pub fn binding_new(
        length: usize,
        window_code: u32,
        precision_code: u32,
    ) -> pyo3::PyResult<Self> {
        (crate::api::StftPlanF32::new(length, window_code, precision_code))
            .map(|value| StftPlanF32 { inner: Some(value) })
    }
    #[pyo3(name = "analyze")]
    #[pyo3(signature = (signal, centers, window_lengths, options))]
    pub fn binding_analyze(
        &mut self,
        signal: Vec<f32>,
        centers: Vec<i32>,
        window_lengths: Vec<u32>,
        options: pyo3::PyRef<'_, StftAnalysisOptions>,
    ) -> pyo3::PyResult<StftAnalysisF32> {
        ((self.inner.as_mut().ok_or_else(consumed)?).analyze(
            &signal,
            &centers,
            &window_lengths,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| StftAnalysisF32 { inner: Some(value) })
    }
    #[pyo3(name = "synthesize")]
    #[pyo3(signature = (magnitude, phase, options))]
    pub fn binding_synthesize(
        &mut self,
        magnitude: Vec<f32>,
        phase: Vec<f32>,
        options: pyo3::PyRef<'_, StftSynthesisOptionsF32>,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).synthesize(
            &magnitude,
            &phase,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
}
