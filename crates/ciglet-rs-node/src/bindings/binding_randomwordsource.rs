use super::*;
#[napi]
pub struct RandomWordSource {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::RandomWordSource>>,
}
#[napi]
impl RandomWordSource {
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
impl RandomWordSource {
    #[napi(js_name = "from_seed")]
    pub fn binding_from_seed(seed: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_seed = seed.to_vec();
        (crate::api::RandomWordSource::from_seed(&storage_seed)).map(|value| RandomWordSource {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(constructor)]
    pub fn binding_new(
        #[napi(ts_arg_type = "() => number")] callback: napi::bindgen_prelude::Unknown<'_>,
        env: napi::Env,
    ) -> napi::Result<Self> {
        Ok(RandomWordSource {
            inner: std::cell::RefCell::new(Some(crate::api::RandomWordSource::new(
                crate::callbacks::Function::new(env, callback)?,
            ))),
        })
    }
    #[napi(js_name = "callback", ts_return_type = "(() => number) | null")]
    pub fn binding_callback(&self) -> napi::Result<Option<crate::callbacks::ReturnedFunction>> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .callback())
        .map(|value| (value).map(crate::callbacks::ReturnedFunction))
    }
    #[napi(js_name = "next_u32")]
    pub fn binding_next_u32(&self) -> napi::Result<u32> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .next_u32()
    }
    #[napi(js_name = "next_u64")]
    pub fn binding_next_u64(&self) -> napi::Result<napi::bindgen_prelude::BigInt> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .next_u64())
        .map(napi::bindgen_prelude::BigInt::from)
    }
    #[napi(js_name = "fill_bytes")]
    pub fn binding_fill_bytes(
        &self,
        count: f64,
    ) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .fill_bytes(checked_usize(count)?))
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
    #[napi(js_name = "seed")]
    pub fn binding_seed(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .seed())
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
    #[napi(js_name = "stream")]
    pub fn binding_stream(&self) -> napi::Result<napi::bindgen_prelude::BigInt> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .stream())
        .map(napi::bindgen_prelude::BigInt::from)
    }
    #[napi(js_name = "set_stream")]
    pub fn binding_set_stream(&self, stream: napi::bindgen_prelude::BigInt) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .set_stream(checked_u64(&stream)?)
    }
    #[napi(js_name = "word_position")]
    pub fn binding_word_position(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .word_position())
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
    #[napi(js_name = "set_word_position")]
    pub fn binding_set_word_position(
        &self,
        position: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<()> {
        let storage_position = position.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .set_word_position(&storage_position)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .cloned())
        .map(|value| RandomWordSource {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
