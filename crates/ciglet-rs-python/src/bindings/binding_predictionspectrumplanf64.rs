use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PredictionSpectrumPlanF64 {
    pub(crate) inner: Option<crate::api::PredictionSpectrumPlanF64>,
}
#[pymethods]
impl PredictionSpectrumPlanF64 {
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
impl PredictionSpectrumPlanF64 {
    #[new]
    #[pyo3(signature = (length))]
    pub fn binding_new(length: usize) -> pyo3::PyResult<Self> {
        (crate::api::PredictionSpectrumPlanF64::new(length))
            .map(|value| PredictionSpectrumPlanF64 { inner: Some(value) })
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
    #[pyo3(name = "estimate")]
    #[pyo3(signature = (magnitudes, order, code))]
    pub fn binding_estimate(
        &mut self,
        magnitudes: Vec<f64>,
        order: usize,
        code: u32,
    ) -> pyo3::PyResult<PredictionF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate(&magnitudes, order, code))
            .map(|value| PredictionF64 { inner: Some(value) })
    }
    #[pyo3(name = "estimate_with_arithmetic")]
    #[pyo3(signature = (magnitudes, order, nyquist_code, arithmetic_code))]
    pub fn binding_estimate_with_arithmetic(
        &mut self,
        magnitudes: Vec<f64>,
        order: usize,
        nyquist_code: u32,
        arithmetic_code: u32,
    ) -> pyo3::PyResult<PredictionF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate_with_arithmetic(
            &magnitudes,
            order,
            nyquist_code,
            arithmetic_code,
        ))
        .map(|value| PredictionF64 { inner: Some(value) })
    }
    #[pyo3(name = "estimate_scaled")]
    #[pyo3(signature = (magnitudes, order, code))]
    pub fn binding_estimate_scaled(
        &mut self,
        magnitudes: Vec<f64>,
        order: usize,
        code: u32,
    ) -> pyo3::PyResult<ScaledPredictionF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate_scaled(&magnitudes, order, code))
            .map(|value| ScaledPredictionF64 { inner: Some(value) })
    }
    #[pyo3(name = "spectrum")]
    #[pyo3(signature = (coefficients, gain))]
    pub fn binding_spectrum(
        &mut self,
        coefficients: Vec<f64>,
        gain: f64,
    ) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_mut().ok_or_else(consumed)?).spectrum(&coefficients, gain)
    }
}
