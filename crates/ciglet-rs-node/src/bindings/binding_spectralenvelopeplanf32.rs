use super::*;
#[napi]
pub struct SpectralEnvelopePlanF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SpectralEnvelopePlanF32>>,
}
#[napi]
impl SpectralEnvelopePlanF32 {
    #[napi]
    pub fn close(&self) -> napi::Result<()> {
        *self.inner.try_borrow_mut().map_err(borrowed)? = None;
        Ok(())
    }
    #[napi]
    pub fn free(&self) -> napi::Result<()> {
        self.close()
    }
    #[napi(getter, js_name = "is_closed")]
    pub fn is_closed(&self) -> napi::Result<bool> {
        Ok(self.inner.try_borrow().map_err(borrowed)?.is_none())
    }
}
#[napi]
impl SpectralEnvelopePlanF32 {
    #[napi(constructor)]
    pub fn binding_new(length: f64) -> napi::Result<Self> {
        (crate::api::SpectralEnvelopePlanF32::new(checked_usize(length)?)).map(|value| {
            SpectralEnvelopePlanF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(getter, js_name = "bins")]
    pub fn binding_bins(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .bins() as f64)
    }
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "estimate")]
    pub fn binding_estimate(
        &self,
        spectrum: napi::bindgen_prelude::Float32Array,
        fundamental: f64,
        harmonics: Option<f64>,
    ) -> napi::Result<SpectralEnvelopeF32> {
        let storage_spectrum = spectrum.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate(
            &storage_spectrum,
            fundamental as f32,
            harmonics.map(checked_usize).transpose()?,
        ))
        .map(|value| SpectralEnvelopeF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "estimate_with")]
    pub fn binding_estimate_with(
        &self,
        spectrum: napi::bindgen_prelude::Float32Array,
        fundamental: f64,
        harmonics: Option<f64>,
        nyquist_code: u32,
    ) -> napi::Result<SpectralEnvelopeF32> {
        let storage_spectrum = spectrum.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_with(
            &storage_spectrum,
            fundamental as f32,
            harmonics.map(checked_usize).transpose()?,
            nyquist_code,
        ))
        .map(|value| SpectralEnvelopeF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "estimate_with_smoothing")]
    pub fn binding_estimate_with_smoothing(
        &self,
        spectrum: napi::bindgen_prelude::Float32Array,
        fundamental: f64,
        harmonics: Option<f64>,
        nyquist_code: u32,
        smoothing_code: u32,
    ) -> napi::Result<SpectralEnvelopeF32> {
        let storage_spectrum = spectrum.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_with_smoothing(
            &storage_spectrum,
            fundamental as f32,
            harmonics.map(checked_usize).transpose()?,
            nyquist_code,
            smoothing_code,
        ))
        .map(|value| SpectralEnvelopeF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
