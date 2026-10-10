use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct WaveF64 {
    pub(crate) inner: Option<crate::api::WaveF64>,
}
#[pymethods]
impl WaveF64 {
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
impl WaveF64 {
    #[new]
    #[pyo3(signature = (sample_rate, bits_per_sample, channels, encoding_code, samples))]
    pub fn binding_new(
        sample_rate: u32,
        bits_per_sample: u16,
        channels: u16,
        encoding_code: u32,
        samples: Vec<f64>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::WaveF64::new(
            sample_rate,
            bits_per_sample,
            channels,
            encoding_code,
            samples,
        ))
        .map(|value| WaveF64 { inner: Some(value) })
    }
    #[getter(sample_rate)]
    pub fn binding_sample_rate(&self) -> pyo3::PyResult<u32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).sample_rate())
    }
    #[setter(sample_rate)]
    pub fn binding_set_sample_rate(&mut self, value: u32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_sample_rate(value);
        Ok(())
    }
    #[getter(bits_per_sample)]
    pub fn binding_bits_per_sample(&self) -> pyo3::PyResult<u16> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).bits_per_sample())
    }
    #[setter(bits_per_sample)]
    pub fn binding_set_bits_per_sample(&mut self, value: u16) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_bits_per_sample(value);
        Ok(())
    }
    #[getter(channels)]
    pub fn binding_channels(&self) -> pyo3::PyResult<u16> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).channels())
    }
    #[setter(channels)]
    pub fn binding_set_channels(&mut self, value: u16) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_channels(value);
        Ok(())
    }
    #[getter(encoding)]
    pub fn binding_encoding(&self) -> pyo3::PyResult<u32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).encoding())
    }
    #[setter(encoding)]
    pub fn binding_set_encoding(&mut self, code: u32) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_encoding(code)
    }
    #[getter(samples)]
    pub fn binding_samples(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).samples())
    }
    #[setter(samples)]
    pub fn binding_set_samples(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_samples(values);
        Ok(())
    }
    #[getter(position)]
    pub fn binding_position(&self) -> pyo3::PyResult<u64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).position())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(WaveF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
