use super::*;
#[napi]
pub struct Figure {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::Figure>>,
}
#[napi]
impl Figure {
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
impl Figure {
    #[napi(js_name = "open")]
    pub fn binding_open() -> napi::Result<Self> {
        (crate::api::Figure::open()).map(|value| Figure {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "open_with")]
    pub fn binding_open_with(program: String) -> napi::Result<Self> {
        (crate::api::Figure::open_with(&program)).map(|value| Figure {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "spawn")]
    pub fn binding_spawn(program: String, arguments: Vec<String>) -> napi::Result<Self> {
        (crate::api::Figure::spawn(&program, arguments.to_vec())).map(|value| Figure {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "id")]
    pub fn binding_id(&self) -> napi::Result<u32> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .id())
    }
    #[napi(js_name = "append")]
    pub fn binding_append(&self, bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<()> {
        let storage_bytes = bytes.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .append(&storage_bytes)
    }
    #[napi(js_name = "flush")]
    pub fn binding_flush(&self) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .flush()
    }
    #[napi(js_name = "close_with_output")]
    pub fn binding_close_with_output(&self) -> napi::Result<ProcessOutput> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .close_with_output())
        .map(|value| ProcessOutput {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
