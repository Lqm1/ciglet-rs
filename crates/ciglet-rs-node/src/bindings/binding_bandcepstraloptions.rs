use super::*;
#[napi]
pub struct BandCepstralOptions {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::BandCepstralOptions>>,
}
#[napi]
impl BandCepstralOptions {
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
impl BandCepstralOptions {
    #[napi(getter, js_name = "order")]
    pub fn binding_get_order(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .order as f64)
    }
    #[napi(setter, js_name = "order")]
    pub fn binding_set_order(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .order = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "include_dc")]
    pub fn binding_get_include_dc(&self) -> napi::Result<bool> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .include_dc)
    }
    #[napi(setter, js_name = "include_dc")]
    pub fn binding_set_include_dc(&self, value: bool) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .include_dc = value;
        Ok(())
    }
}
#[napi]
impl BandCepstralOptions {
    #[napi(constructor)]
    pub fn binding_new(order: f64, include_dc: bool) -> napi::Result<Self> {
        Ok(BandCepstralOptions {
            inner: std::cell::RefCell::new(Some(crate::api::BandCepstralOptions::new(
                checked_usize(order)?,
                include_dc,
            ))),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(BandCepstralOptions {
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
    #[napi(js_name = "coefficient_count")]
    pub fn binding_coefficient_count(&self, bands: f64) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .coefficient_count(checked_usize(bands)?))
        .map(|value| value as f64)
    }
}
