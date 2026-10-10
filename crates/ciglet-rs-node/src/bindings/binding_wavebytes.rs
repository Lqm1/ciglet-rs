use super::*;
#[napi]
pub struct WaveBytes {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::WaveBytes>>,
}
#[napi]
impl WaveBytes {
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
impl WaveBytes {
    #[napi(getter, js_name = "bytes")]
    pub fn binding_bytes(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .bytes(),
        ))
    }
    #[napi(getter, js_name = "position")]
    pub fn binding_position(&self) -> napi::Result<napi::bindgen_prelude::BigInt> {
        Ok(napi::bindgen_prelude::BigInt::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .position(),
        ))
    }
    #[napi(js_name = "into_bytes")]
    pub fn binding_into_bytes(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?)
            .into_bytes(),
        ))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(WaveBytes {
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
