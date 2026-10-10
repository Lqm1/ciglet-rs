use super::*;
#[napi]
pub struct ProcessOutput {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ProcessOutput>>,
}
#[napi]
impl ProcessOutput {
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
impl ProcessOutput {
    #[napi(getter, js_name = "success")]
    pub fn binding_get_success(&self) -> napi::Result<bool> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .success)
    }
    #[napi(setter, js_name = "success")]
    pub fn binding_set_success(&self, value: bool) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .success = value;
        Ok(())
    }
    #[napi(getter, js_name = "code")]
    pub fn binding_get_code(&self) -> napi::Result<Option<i32>> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .code)
    }
    #[napi(setter, js_name = "code")]
    pub fn binding_set_code(&self, value: Option<i32>) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .code = value;
        Ok(())
    }
}
#[napi]
impl ProcessOutput {
    #[napi(js_name = "stdout")]
    pub fn binding_stdout(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .stdout(),
        ))
    }
    #[napi(js_name = "stderr")]
    pub fn binding_stderr(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .stderr(),
        ))
    }
}
