use super::*;
#[napi]
pub struct FilterBankF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::FilterBankF32>>,
}
#[napi]
impl FilterBankF32 {
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
impl FilterBankF32 {
    #[napi(constructor)]
    pub fn binding_new(bins: f64, nyquist_hz: f64, channels: f64) -> napi::Result<Self> {
        (crate::api::FilterBankF32::new(
            checked_usize(bins)?,
            nyquist_hz as f32,
            checked_usize(channels)?,
        ))
        .map(|value| FilterBankF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "mel")]
    pub fn binding_mel(
        bins: f64,
        nyquist_hz: f64,
        channels: f64,
        options: &MelOptionsF32,
    ) -> napi::Result<Self> {
        (crate::api::FilterBankF32::mel(
            checked_usize(bins)?,
            nyquist_hz as f32,
            checked_usize(channels)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| FilterBankF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "mel_default")]
    pub fn binding_mel_default(
        bins: f64,
        nyquist_hz: f64,
        channels: f64,
        minimum_hz: f64,
        maximum_hz: f64,
    ) -> napi::Result<Self> {
        (crate::api::FilterBankF32::mel_default(
            checked_usize(bins)?,
            nyquist_hz as f32,
            checked_usize(channels)?,
            minimum_hz as f32,
            maximum_hz as f32,
        ))
        .map(|value| FilterBankF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "plp")]
    pub fn binding_plp(bins: f64, nyquist_hz: f64, channels: f64) -> napi::Result<Self> {
        (crate::api::FilterBankF32::plp(
            checked_usize(bins)?,
            nyquist_hz as f32,
            checked_usize(channels)?,
        ))
        .map(|value| FilterBankF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(FilterBankF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .cloned(),
            )),
        })
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
    #[napi(getter, js_name = "channels")]
    pub fn binding_channels(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .channels() as f64)
    }
    #[napi(getter, js_name = "nyquist_hz")]
    pub fn binding_nyquist_hz(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .nyquist_hz() as f64)
    }
    #[napi(js_name = "set_nyquist_hz")]
    pub fn binding_set_nyquist_hz(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_nyquist_hz(value as f32);
        Ok(())
    }
    #[napi(js_name = "response")]
    pub fn binding_response(
        &self,
        channel: f64,
    ) -> napi::Result<Option<napi::bindgen_prelude::Float32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .response(checked_usize(channel)?))
        .map(<napi::bindgen_prelude::Float32Array>::from))
    }
    #[napi(js_name = "support")]
    pub fn binding_support(
        &self,
        channel: f64,
    ) -> napi::Result<Option<napi::bindgen_prelude::Uint32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .support(checked_usize(channel)?))
        .map(<napi::bindgen_prelude::Uint32Array>::from))
    }
    #[napi(js_name = "set_channel")]
    pub fn binding_set_channel(
        &self,
        channel: f64,
        response: napi::bindgen_prelude::Float32Array,
        start: f64,
        end: f64,
    ) -> napi::Result<()> {
        let storage_response = response.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_channel(
            checked_usize(channel)?,
            &storage_response,
            checked_usize(start)?,
            checked_usize(end)?,
        )
    }
    #[napi(js_name = "apply")]
    pub fn binding_apply(
        &self,
        spectrum: napi::bindgen_prelude::Float32Array,
        compression_kind: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_spectrum = spectrum.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .apply(&storage_spectrum, compression_kind))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "apply_spectrogram")]
    pub fn binding_apply_spectrogram(
        &self,
        spectra: napi::bindgen_prelude::Float32Array,
        compression_kind: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_spectra = spectra.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .apply_spectrogram(&storage_spectra, compression_kind))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "apply_frames")]
    pub fn binding_apply_frames(
        &self,
        spectra: napi::bindgen_prelude::Float32Array,
        frames: f64,
        compression_kind: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_spectra = spectra.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .apply_frames(&storage_spectra, checked_usize(frames)?, compression_kind))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
}
