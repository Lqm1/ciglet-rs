use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct GlottalModelF64 {
    pub(crate) inner: Option<crate::api::GlottalModelF64>,
}
#[pymethods]
impl GlottalModelF64 {
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
impl GlottalModelF64 {
    #[getter(period)]
    fn binding_get_period(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.period)
    }
    #[setter(period)]
    fn binding_set_period(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.period = value;
        Ok(())
    }
    #[getter(closure)]
    fn binding_get_closure(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.closure)
    }
    #[setter(closure)]
    fn binding_set_closure(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.closure = value;
        Ok(())
    }
    #[getter(peak)]
    fn binding_get_peak(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.peak)
    }
    #[setter(peak)]
    fn binding_set_peak(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.peak = value;
        Ok(())
    }
    #[getter(return_time)]
    fn binding_get_return_time(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.return_time)
    }
    #[setter(return_time)]
    fn binding_set_return_time(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.return_time = value;
        Ok(())
    }
    #[getter(excitation)]
    fn binding_get_excitation(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.excitation)
    }
    #[setter(excitation)]
    fn binding_set_excitation(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.excitation = value;
        Ok(())
    }
}
#[pymethods]
impl GlottalModelF64 {
    #[new]
    #[pyo3(signature = (period, closure, peak, return_time, excitation))]
    pub fn binding_new(
        period: f64,
        closure: f64,
        peak: f64,
        return_time: f64,
        excitation: f64,
    ) -> pyo3::PyResult<Self> {
        Ok(GlottalModelF64 {
            inner: Some(crate::api::GlottalModelF64::new(
                period,
                closure,
                peak,
                return_time,
                excitation,
            )),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_shape")]
    #[pyo3(signature = (shape, period, excitation))]
    pub fn binding_from_shape(shape: f64, period: f64, excitation: f64) -> pyo3::PyResult<Self> {
        (crate::api::GlottalModelF64::from_shape(shape, period, excitation))
            .map(|value| GlottalModelF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_components")]
    #[pyo3(signature = (values))]
    pub fn binding_from_components(values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::GlottalModelF64::from_components(&values))
            .map(|value| GlottalModelF64 { inner: Some(value) })
    }
    #[pyo3(name = "components")]
    #[pyo3(signature = ())]
    pub fn binding_components(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).components())
    }
    #[pyo3(name = "set_components")]
    #[pyo3(signature = (values))]
    pub fn binding_set_components(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_components(&values)
    }
    #[pyo3(name = "prepare")]
    #[pyo3(signature = ())]
    pub fn binding_prepare(&self) -> pyo3::PyResult<GlottalPreparedF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).prepare())
            .map(|value| GlottalPreparedF64 { inner: Some(value) })
    }
    #[pyo3(name = "spectrum")]
    #[pyo3(signature = (frequencies))]
    pub fn binding_spectrum(&self, frequencies: Vec<f64>) -> pyo3::PyResult<GlottalSpectrumF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).spectrum(&frequencies))
            .map(|value| GlottalSpectrumF64 { inner: Some(value) })
    }
    #[pyo3(name = "sampled_period")]
    #[pyo3(signature = (sampling_rate, count))]
    pub fn binding_sampled_period(
        &self,
        sampling_rate: i32,
        count: usize,
    ) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_ref().ok_or_else(consumed)?).sampled_period(sampling_rate, count)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GlottalModelF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
