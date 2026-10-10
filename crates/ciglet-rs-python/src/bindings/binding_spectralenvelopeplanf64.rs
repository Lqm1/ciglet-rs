use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct SpectralEnvelopePlanF64 {
    pub(crate) inner: Option<crate::api::SpectralEnvelopePlanF64>,
}
#[pymethods]
impl SpectralEnvelopePlanF64 {
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
impl SpectralEnvelopePlanF64 {
    #[new]
    #[pyo3(signature = (length))]
    pub fn binding_new(length: usize) -> pyo3::PyResult<Self> {
        (crate::api::SpectralEnvelopePlanF64::new(length))
            .map(|value| SpectralEnvelopePlanF64 { inner: Some(value) })
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
    #[pyo3(name = "estimate")]
    #[pyo3(signature = (spectrum, fundamental, harmonics))]
    pub fn binding_estimate(
        &mut self,
        spectrum: Vec<f64>,
        fundamental: f64,
        harmonics: Option<usize>,
    ) -> pyo3::PyResult<SpectralEnvelopeF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate(&spectrum, fundamental, harmonics))
            .map(|value| SpectralEnvelopeF64 { inner: Some(value) })
    }
    #[pyo3(name = "estimate_with")]
    #[pyo3(signature = (spectrum, fundamental, harmonics, nyquist_code))]
    pub fn binding_estimate_with(
        &mut self,
        spectrum: Vec<f64>,
        fundamental: f64,
        harmonics: Option<usize>,
        nyquist_code: u32,
    ) -> pyo3::PyResult<SpectralEnvelopeF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate_with(
            &spectrum,
            fundamental,
            harmonics,
            nyquist_code,
        ))
        .map(|value| SpectralEnvelopeF64 { inner: Some(value) })
    }
    #[pyo3(name = "estimate_with_smoothing")]
    #[pyo3(signature = (spectrum, fundamental, harmonics, nyquist_code, smoothing_code))]
    pub fn binding_estimate_with_smoothing(
        &mut self,
        spectrum: Vec<f64>,
        fundamental: f64,
        harmonics: Option<usize>,
        nyquist_code: u32,
        smoothing_code: u32,
    ) -> pyo3::PyResult<SpectralEnvelopeF64> {
        ((self.inner.as_mut().ok_or_else(consumed)?).estimate_with_smoothing(
            &spectrum,
            fundamental,
            harmonics,
            nyquist_code,
            smoothing_code,
        ))
        .map(|value| SpectralEnvelopeF64 { inner: Some(value) })
    }
}
