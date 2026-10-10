use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct RealCepstrumPlanF32 {
    pub(crate) inner: Option<crate::api::RealCepstrumPlanF32>,
}
#[pymethods]
impl RealCepstrumPlanF32 {
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
impl RealCepstrumPlanF32 {
    #[new]
    #[pyo3(signature = (length))]
    pub fn binding_new(length: usize) -> pyo3::PyResult<Self> {
        (crate::api::RealCepstrumPlanF32::new(length))
            .map(|value| RealCepstrumPlanF32 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[getter(bins)]
    pub fn binding_bins(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).bins())
    }
    #[pyo3(name = "is_empty")]
    #[pyo3(signature = ())]
    pub fn binding_is_empty(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).is_empty())
    }
    #[pyo3(name = "analyze")]
    #[pyo3(signature = (logs, policy))]
    pub fn binding_analyze(&mut self, logs: Vec<f32>, policy: u32) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).analyze(&logs, policy)
    }
    #[pyo3(name = "reconstruct")]
    #[pyo3(signature = (coefficients, policy))]
    pub fn binding_reconstruct(
        &mut self,
        coefficients: Vec<f32>,
        policy: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).reconstruct(&coefficients, policy)
    }
    #[pyo3(name = "minimum_phase_with")]
    #[pyo3(signature = (logs, policy, lifter_code))]
    pub fn binding_minimum_phase_with(
        &mut self,
        logs: Vec<f32>,
        policy: u32,
        lifter_code: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).minimum_phase_with(&logs, policy, lifter_code)
    }
    #[pyo3(name = "spectrogram_to_cepstra")]
    #[pyo3(signature = (magnitudes, count, policy))]
    pub fn binding_spectrogram_to_cepstra(
        &mut self,
        magnitudes: Vec<f32>,
        count: usize,
        policy: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).spectrogram_to_cepstra(
            &magnitudes,
            count,
            policy,
        )
    }
    #[pyo3(name = "cepstra_to_spectrogram")]
    #[pyo3(signature = (coefficients, frames, count, policy))]
    pub fn binding_cepstra_to_spectrogram(
        &mut self,
        coefficients: Vec<f32>,
        frames: usize,
        count: usize,
        policy: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_mut().ok_or_else(consumed)?).cepstra_to_spectrogram(
            &coefficients,
            frames,
            count,
            policy,
        )
    }
}
