use super::*;
#[napi]
pub struct GlottalPreparedF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GlottalPreparedF32>>,
}
#[napi]
impl GlottalPreparedF32 {
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
impl GlottalPreparedF32 {
    #[napi(js_name = "model")]
    pub fn binding_model(&self) -> napi::Result<GlottalModelF32> {
        Ok(GlottalModelF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .model(),
            )),
        })
    }
    #[napi(js_name = "spectrum")]
    pub fn binding_spectrum(
        &self,
        frequencies: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<GlottalSpectrumF32> {
        let storage_frequencies = frequencies.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .spectrum(&storage_frequencies))
        .map(|value| GlottalSpectrumF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "spectrum_with")]
    pub fn binding_spectrum_with(
        &self,
        frequencies: napi::bindgen_prelude::Float32Array,
        code: u32,
    ) -> napi::Result<GlottalSpectrumF32> {
        let storage_frequencies = frequencies.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .spectrum_with(&storage_frequencies, code))
        .map(|value| GlottalSpectrumF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "sampled_period")]
    pub fn binding_sampled_period(
        &self,
        sampling_rate: i32,
        count: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .sampled_period(sampling_rate, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GlottalPreparedF32 {
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
