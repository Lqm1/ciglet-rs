use super::*;
#[napi]
pub struct WaveWriteOptions {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::WaveWriteOptions>>,
}
#[napi]
impl WaveWriteOptions {
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
impl WaveWriteOptions {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(WaveWriteOptions {
            inner: std::cell::RefCell::new(Some(crate::api::WaveWriteOptions::new())),
        })
    }
    #[napi(js_name = "legacy")]
    pub fn binding_legacy(bits_per_sample: u16) -> napi::Result<Self> {
        Ok(WaveWriteOptions {
            inner: std::cell::RefCell::new(Some(crate::api::WaveWriteOptions::legacy(
                bits_per_sample,
            ))),
        })
    }
    #[napi(getter, js_name = "bits_per_sample")]
    pub fn binding_bits_per_sample(&self) -> napi::Result<u16> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .bits_per_sample())
    }
    #[napi(setter, js_name = "bits_per_sample")]
    pub fn binding_set_bits_per_sample(&self, value: u16) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_bits_per_sample(value);
        Ok(())
    }
    #[napi(getter, js_name = "pcm8_quantization")]
    pub fn binding_pcm8_quantization(&self) -> napi::Result<u32> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .pcm8_quantization())
    }
    #[napi(setter, js_name = "pcm8_quantization")]
    pub fn binding_set_pcm8_quantization(&self, code: u32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_pcm8_quantization(code)
    }
    #[napi(getter, js_name = "pad_odd_data")]
    pub fn binding_pad_odd_data(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .pad_odd_data())
    }
    #[napi(setter, js_name = "pad_odd_data")]
    pub fn binding_set_pad_odd_data(&self, value: bool) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_pad_odd_data(value);
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(WaveWriteOptions {
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
