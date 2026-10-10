use super::*;
#[napi]
pub struct MelOptionsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MelOptionsF32>>,
}
#[napi]
impl MelOptionsF32 {
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
impl MelOptionsF32 {
    #[napi(getter, js_name = "minimum_hz")]
    pub fn binding_get_minimum_hz(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .minimum_hz as f64)
    }
    #[napi(setter, js_name = "minimum_hz")]
    pub fn binding_set_minimum_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .minimum_hz = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "maximum_hz")]
    pub fn binding_get_maximum_hz(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .maximum_hz as f64)
    }
    #[napi(setter, js_name = "maximum_hz")]
    pub fn binding_set_maximum_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .maximum_hz = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "scale")]
    pub fn binding_get_scale(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .scale as f64)
    }
    #[napi(setter, js_name = "scale")]
    pub fn binding_set_scale(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .scale = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "minimum_width_hz")]
    pub fn binding_get_minimum_width_hz(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .minimum_width_hz as f64)
    }
    #[napi(setter, js_name = "minimum_width_hz")]
    pub fn binding_set_minimum_width_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .minimum_width_hz = value as f32;
        Ok(())
    }
}
#[napi]
impl MelOptionsF32 {
    #[napi(constructor)]
    pub fn binding_new(
        minimum_hz: f64,
        maximum_hz: f64,
        scale: f64,
        minimum_width_hz: f64,
    ) -> napi::Result<Self> {
        Ok(MelOptionsF32 {
            inner: std::cell::RefCell::new(Some(crate::api::MelOptionsF32::new(
                minimum_hz as f32,
                maximum_hz as f32,
                scale as f32,
                minimum_width_hz as f32,
            ))),
        })
    }
}
