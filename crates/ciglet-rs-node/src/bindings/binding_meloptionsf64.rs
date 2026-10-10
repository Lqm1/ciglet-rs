use super::*;
#[napi]
pub struct MelOptionsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MelOptionsF64>>,
}
#[napi]
impl MelOptionsF64 {
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
impl MelOptionsF64 {
    #[napi(getter, js_name = "minimum_hz")]
    pub fn binding_get_minimum_hz(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .minimum_hz)
    }
    #[napi(setter, js_name = "minimum_hz")]
    pub fn binding_set_minimum_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .minimum_hz = value;
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
            .maximum_hz)
    }
    #[napi(setter, js_name = "maximum_hz")]
    pub fn binding_set_maximum_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .maximum_hz = value;
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
            .scale)
    }
    #[napi(setter, js_name = "scale")]
    pub fn binding_set_scale(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .scale = value;
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
            .minimum_width_hz)
    }
    #[napi(setter, js_name = "minimum_width_hz")]
    pub fn binding_set_minimum_width_hz(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .minimum_width_hz = value;
        Ok(())
    }
}
#[napi]
impl MelOptionsF64 {
    #[napi(constructor)]
    pub fn binding_new(
        minimum_hz: f64,
        maximum_hz: f64,
        scale: f64,
        minimum_width_hz: f64,
    ) -> napi::Result<Self> {
        Ok(MelOptionsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::MelOptionsF64::new(
                minimum_hz,
                maximum_hz,
                scale,
                minimum_width_hz,
            ))),
        })
    }
}
