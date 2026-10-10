use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct CorrelogramFrame {
    pub(crate) inner: Option<crate::api::CorrelogramFrame>,
}
#[pymethods]
impl CorrelogramFrame {
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
impl CorrelogramFrame {
    #[getter(center)]
    fn binding_get_center(&self) -> PyResult<i32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.center)
    }
    #[setter(center)]
    fn binding_set_center(&mut self, value: i32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.center = value;
        Ok(())
    }
    #[getter(length)]
    fn binding_get_length(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.length)
    }
    #[setter(length)]
    fn binding_set_length(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.length = value;
        Ok(())
    }
}
#[pymethods]
impl CorrelogramFrame {
    #[new]
    #[pyo3(signature = (center, length))]
    pub fn binding_new(center: i32, length: usize) -> pyo3::PyResult<Self> {
        Ok(CorrelogramFrame {
            inner: Some(crate::api::CorrelogramFrame::new(center, length)),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(CorrelogramFrame {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "compute_f32")]
    #[pyo3(signature = (samples, periods, method_code, policy_code))]
    pub fn binding_compute_f32(
        &self,
        samples: Vec<f32>,
        periods: usize,
        method_code: u32,
        policy_code: u32,
    ) -> pyo3::PyResult<Array2F32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).compute_f32(
            &samples,
            periods,
            method_code,
            policy_code,
        ))
        .map(|value| Array2F32 { inner: Some(value) })
    }
    #[pyo3(name = "compute_f64")]
    #[pyo3(signature = (samples, periods, method_code, policy_code))]
    pub fn binding_compute_f64(
        &self,
        samples: Vec<f64>,
        periods: usize,
        method_code: u32,
        policy_code: u32,
    ) -> pyo3::PyResult<Array2F64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).compute_f64(
            &samples,
            periods,
            method_code,
            policy_code,
        ))
        .map(|value| Array2F64 { inner: Some(value) })
    }
}
