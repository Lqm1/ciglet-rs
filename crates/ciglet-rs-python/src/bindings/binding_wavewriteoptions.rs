use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct WaveWriteOptions {
    pub(crate) inner: Option<crate::api::WaveWriteOptions>,
}
#[pymethods]
impl WaveWriteOptions {
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
impl WaveWriteOptions {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(WaveWriteOptions {
            inner: Some(crate::api::WaveWriteOptions::new()),
        })
    }
    #[staticmethod]
    #[pyo3(name = "legacy")]
    #[pyo3(signature = (bits_per_sample))]
    pub fn binding_legacy(bits_per_sample: u16) -> pyo3::PyResult<Self> {
        Ok(WaveWriteOptions {
            inner: Some(crate::api::WaveWriteOptions::legacy(bits_per_sample)),
        })
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
    #[getter(pcm8_quantization)]
    pub fn binding_pcm8_quantization(&self) -> pyo3::PyResult<u32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).pcm8_quantization())
    }
    #[setter(pcm8_quantization)]
    pub fn binding_set_pcm8_quantization(&mut self, code: u32) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_pcm8_quantization(code)
    }
    #[getter(pad_odd_data)]
    pub fn binding_pad_odd_data(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).pad_odd_data())
    }
    #[setter(pad_odd_data)]
    pub fn binding_set_pad_odd_data(&mut self, value: bool) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_pad_odd_data(value);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(WaveWriteOptions {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
