use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct SpectralPeakF64 {
    pub(crate) inner: Option<crate::api::SpectralPeakF64>,
}
#[pymethods]
impl SpectralPeakF64 {
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
impl SpectralPeakF64 {
    #[new]
    #[pyo3(signature = (bin, magnitude))]
    pub fn binding_new(bin: f64, magnitude: f64) -> pyo3::PyResult<Self> {
        Ok(SpectralPeakF64 {
            inner: Some(crate::api::SpectralPeakF64::new(bin, magnitude)),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_values")]
    #[pyo3(signature = (values))]
    pub fn binding_from_values(values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::SpectralPeakF64::from_values(&values))
            .map(|value| SpectralPeakF64 { inner: Some(value) })
    }
    #[getter(bin)]
    pub fn binding_bin(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).bin())
    }
    #[setter(bin)]
    pub fn binding_set_bin(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_bin(value);
        Ok(())
    }
    #[getter(magnitude)]
    pub fn binding_magnitude(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).magnitude())
    }
    #[setter(magnitude)]
    pub fn binding_set_magnitude(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_magnitude(value);
        Ok(())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "set_values")]
    #[pyo3(signature = (values))]
    pub fn binding_set_values(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_values(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SpectralPeakF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
