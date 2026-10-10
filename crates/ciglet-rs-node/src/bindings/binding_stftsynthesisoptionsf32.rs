use super::*;
#[napi]
pub struct StftSynthesisOptionsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StftSynthesisOptionsF32>>,
}
#[napi]
impl StftSynthesisOptionsF32 {
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
impl StftSynthesisOptionsF32 {
    #[napi(getter, js_name = "hop")]
    pub fn binding_get_hop(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .hop as f64)
    }
    #[napi(setter, js_name = "hop")]
    pub fn binding_set_hop(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .hop = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "offset")]
    pub fn binding_get_offset(&self) -> napi::Result<i32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .offset)
    }
    #[napi(setter, js_name = "offset")]
    pub fn binding_set_offset(&self, value: i32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .offset = value;
        Ok(())
    }
    #[napi(getter, js_name = "fade_samples")]
    pub fn binding_get_fade_samples(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .fade_samples as f64)
    }
    #[napi(setter, js_name = "fade_samples")]
    pub fn binding_set_fade_samples(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .fade_samples = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "normalization")]
    pub fn binding_get_normalization(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .normalization as f64)
    }
    #[napi(setter, js_name = "normalization")]
    pub fn binding_set_normalization(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .normalization = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "nyquist")]
    pub fn binding_get_nyquist(&self) -> napi::Result<u32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .nyquist)
    }
    #[napi(setter, js_name = "nyquist")]
    pub fn binding_set_nyquist(&self, value: u32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .nyquist = value;
        Ok(())
    }
}
#[napi]
impl StftSynthesisOptionsF32 {
    #[napi(constructor)]
    pub fn binding_new(
        hop: f64,
        offset: i32,
        fade_samples: f64,
        normalization: f64,
        nyquist: u32,
    ) -> napi::Result<Self> {
        Ok(StftSynthesisOptionsF32 {
            inner: std::cell::RefCell::new(Some(crate::api::StftSynthesisOptionsF32::new(
                checked_usize(hop)?,
                offset,
                checked_usize(fade_samples)?,
                normalization as f32,
                nyquist,
            ))),
        })
    }
    #[napi(js_name = "normalization_storage")]
    pub fn binding_normalization_storage(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .normalization_storage(),
        ))
    }
    #[napi(js_name = "set_normalization_storage")]
    pub fn binding_set_normalization_storage(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_normalization_storage(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StftSynthesisOptionsF32 {
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
