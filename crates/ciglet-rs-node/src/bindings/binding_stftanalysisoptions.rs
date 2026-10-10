use super::*;
#[napi]
pub struct StftAnalysisOptions {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StftAnalysisOptions>>,
}
#[napi]
impl StftAnalysisOptions {
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
impl StftAnalysisOptions {
    #[napi(getter, js_name = "subtract_mean")]
    pub fn binding_get_subtract_mean(&self) -> napi::Result<bool> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .subtract_mean)
    }
    #[napi(setter, js_name = "subtract_mean")]
    pub fn binding_set_subtract_mean(&self, value: bool) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .subtract_mean = value;
        Ok(())
    }
    #[napi(getter, js_name = "magnitude")]
    pub fn binding_get_magnitude(&self) -> napi::Result<bool> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .magnitude)
    }
    #[napi(setter, js_name = "magnitude")]
    pub fn binding_set_magnitude(&self, value: bool) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .magnitude = value;
        Ok(())
    }
    #[napi(getter, js_name = "phase")]
    pub fn binding_get_phase(&self) -> napi::Result<bool> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .phase)
    }
    #[napi(setter, js_name = "phase")]
    pub fn binding_set_phase(&self, value: bool) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .phase = value;
        Ok(())
    }
    #[napi(getter, js_name = "normalization_hop")]
    pub fn binding_get_normalization_hop(&self) -> napi::Result<Option<f64>> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .normalization_hop
            .map(|value| value as f64))
    }
    #[napi(setter, js_name = "normalization_hop")]
    pub fn binding_set_normalization_hop(&self, value: Option<f64>) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .normalization_hop = value.map(checked_usize).transpose()?;
        Ok(())
    }
}
#[napi]
impl StftAnalysisOptions {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(StftAnalysisOptions {
            inner: std::cell::RefCell::new(Some(crate::api::StftAnalysisOptions::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StftAnalysisOptions {
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
