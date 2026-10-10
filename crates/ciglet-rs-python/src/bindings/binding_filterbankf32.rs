use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct FilterBankF32 {
    pub(crate) inner: Option<crate::api::FilterBankF32>,
}
#[pymethods]
impl FilterBankF32 {
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
impl FilterBankF32 {
    #[new]
    #[pyo3(signature = (bins, nyquist_hz, channels))]
    pub fn binding_new(bins: usize, nyquist_hz: f32, channels: usize) -> pyo3::PyResult<Self> {
        (crate::api::FilterBankF32::new(bins, nyquist_hz, channels))
            .map(|value| FilterBankF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "mel")]
    #[pyo3(signature = (bins, nyquist_hz, channels, options))]
    pub fn binding_mel(
        bins: usize,
        nyquist_hz: f32,
        channels: usize,
        options: pyo3::PyRef<'_, MelOptionsF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::FilterBankF32::mel(
            bins,
            nyquist_hz,
            channels,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| FilterBankF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "mel_default")]
    #[pyo3(signature = (bins, nyquist_hz, channels, minimum_hz, maximum_hz))]
    pub fn binding_mel_default(
        bins: usize,
        nyquist_hz: f32,
        channels: usize,
        minimum_hz: f32,
        maximum_hz: f32,
    ) -> pyo3::PyResult<Self> {
        (crate::api::FilterBankF32::mel_default(bins, nyquist_hz, channels, minimum_hz, maximum_hz))
            .map(|value| FilterBankF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "plp")]
    #[pyo3(signature = (bins, nyquist_hz, channels))]
    pub fn binding_plp(bins: usize, nyquist_hz: f32, channels: usize) -> pyo3::PyResult<Self> {
        (crate::api::FilterBankF32::plp(bins, nyquist_hz, channels))
            .map(|value| FilterBankF32 { inner: Some(value) })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(FilterBankF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[getter(bins)]
    pub fn binding_bins(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).bins())
    }
    #[getter(channels)]
    pub fn binding_channels(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).channels())
    }
    #[getter(nyquist_hz)]
    pub fn binding_nyquist_hz(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).nyquist_hz())
    }
    #[pyo3(name = "set_nyquist_hz")]
    #[pyo3(signature = (value))]
    pub fn binding_set_nyquist_hz(&mut self, value: f32) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_nyquist_hz(value);
        Ok(())
    }
    #[pyo3(name = "response")]
    #[pyo3(signature = (channel))]
    pub fn binding_response(&self, channel: usize) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).response(channel))
    }
    #[pyo3(name = "support")]
    #[pyo3(signature = (channel))]
    pub fn binding_support(&self, channel: usize) -> pyo3::PyResult<Option<Vec<u32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).support(channel))
    }
    #[pyo3(name = "set_channel")]
    #[pyo3(signature = (channel, response, start, end))]
    pub fn binding_set_channel(
        &mut self,
        channel: usize,
        response: Vec<f32>,
        start: usize,
        end: usize,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_channel(channel, &response, start, end)
    }
    #[pyo3(name = "apply")]
    #[pyo3(signature = (spectrum, compression_kind))]
    pub fn binding_apply(
        &self,
        spectrum: Vec<f32>,
        compression_kind: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).apply(&spectrum, compression_kind)
    }
    #[pyo3(name = "apply_spectrogram")]
    #[pyo3(signature = (spectra, compression_kind))]
    pub fn binding_apply_spectrogram(
        &self,
        spectra: Vec<f32>,
        compression_kind: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).apply_spectrogram(&spectra, compression_kind)
    }
    #[pyo3(name = "apply_frames")]
    #[pyo3(signature = (spectra, frames, compression_kind))]
    pub fn binding_apply_frames(
        &self,
        spectra: Vec<f32>,
        frames: usize,
        compression_kind: u32,
    ) -> pyo3::PyResult<Vec<f32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).apply_frames(&spectra, frames, compression_kind)
    }
}
