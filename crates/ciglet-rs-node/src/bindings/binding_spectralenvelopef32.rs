use super::*;
#[napi]
pub struct SpectralEnvelopeF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SpectralEnvelopeF32>>,
}
#[napi]
impl SpectralEnvelopeF32 {
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
impl SpectralEnvelopeF32 {
    #[napi(constructor)]
    pub fn binding_new(
        log_spectrum: napi::bindgen_prelude::Float32Array,
        cepstrum: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        Ok(SpectralEnvelopeF32 {
            inner: std::cell::RefCell::new(Some(crate::api::SpectralEnvelopeF32::new(
                log_spectrum.to_vec(),
                cepstrum.to_vec(),
            ))),
        })
    }
    #[napi(getter, js_name = "log_spectrum")]
    pub fn binding_log_spectrum(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .log_spectrum(),
        ))
    }
    #[napi(setter, js_name = "log_spectrum")]
    pub fn binding_set_log_spectrum(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_log_spectrum(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "cepstrum")]
    pub fn binding_cepstrum(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .cepstrum(),
        ))
    }
    #[napi(setter, js_name = "cepstrum")]
    pub fn binding_set_cepstrum(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_cepstrum(values.to_vec());
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(SpectralEnvelopeF32 {
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
}
