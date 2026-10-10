use super::*;
#[napi]
pub struct WaveF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::WaveF64>>,
}
#[napi]
impl WaveF64 {
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
impl WaveF64 {
    #[napi(constructor)]
    pub fn binding_new(
        sample_rate: u32,
        bits_per_sample: u16,
        channels: u16,
        encoding_code: u32,
        samples: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<Self> {
        (crate::api::WaveF64::new(
            sample_rate,
            bits_per_sample,
            channels,
            encoding_code,
            samples.to_vec(),
        ))
        .map(|value| WaveF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "sample_rate")]
    pub fn binding_sample_rate(&self) -> napi::Result<u32> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .sample_rate())
    }
    #[napi(setter, js_name = "sample_rate")]
    pub fn binding_set_sample_rate(&self, value: u32) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_sample_rate(value);
        Ok(())
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
    #[napi(getter, js_name = "channels")]
    pub fn binding_channels(&self) -> napi::Result<u16> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .channels())
    }
    #[napi(setter, js_name = "channels")]
    pub fn binding_set_channels(&self, value: u16) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_channels(value);
        Ok(())
    }
    #[napi(getter, js_name = "encoding")]
    pub fn binding_encoding(&self) -> napi::Result<u32> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .encoding())
    }
    #[napi(setter, js_name = "encoding")]
    pub fn binding_set_encoding(&self, code: u32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_encoding(code)
    }
    #[napi(getter, js_name = "samples")]
    pub fn binding_samples(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .samples(),
        ))
    }
    #[napi(setter, js_name = "samples")]
    pub fn binding_set_samples(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_samples(values.to_vec());
        Ok(())
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
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(WaveF64 {
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
