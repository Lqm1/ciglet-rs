use super::*;
#[napi]
pub struct StftFrameGrid {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StftFrameGrid>>,
}
#[napi]
impl StftFrameGrid {
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
impl StftFrameGrid {
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
    #[napi(getter, js_name = "frames")]
    pub fn binding_get_frames(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .frames as f64)
    }
    #[napi(setter, js_name = "frames")]
    pub fn binding_set_frames(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .frames = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "window_factor")]
    pub fn binding_get_window_factor(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .window_factor as f64)
    }
    #[napi(setter, js_name = "window_factor")]
    pub fn binding_set_window_factor(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .window_factor = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "zero_padding_factor")]
    pub fn binding_get_zero_padding_factor(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .zero_padding_factor as f64)
    }
    #[napi(setter, js_name = "zero_padding_factor")]
    pub fn binding_set_zero_padding_factor(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .zero_padding_factor = checked_usize(value)?;
        Ok(())
    }
}
#[napi]
impl StftFrameGrid {
    #[napi(constructor)]
    pub fn binding_new(
        hop: f64,
        frames: f64,
        window_factor: f64,
        zero_padding_factor: f64,
    ) -> napi::Result<Self> {
        Ok(StftFrameGrid {
            inner: std::cell::RefCell::new(Some(crate::api::StftFrameGrid::new(
                checked_usize(hop)?,
                checked_usize(frames)?,
                checked_usize(window_factor)?,
                checked_usize(zero_padding_factor)?,
            ))),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StftFrameGrid {
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
