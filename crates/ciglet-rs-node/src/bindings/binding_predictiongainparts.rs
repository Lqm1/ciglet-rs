use super::*;
#[napi]
pub struct PredictionGainParts {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PredictionGainParts>>,
}
#[napi]
impl PredictionGainParts {
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
impl PredictionGainParts {
    #[napi(getter, js_name = "fraction")]
    pub fn binding_get_fraction(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .fraction)
    }
    #[napi(setter, js_name = "fraction")]
    pub fn binding_set_fraction(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .fraction = value;
        Ok(())
    }
    #[napi(getter, js_name = "exponent")]
    pub fn binding_get_exponent(&self) -> napi::Result<i32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .exponent)
    }
    #[napi(setter, js_name = "exponent")]
    pub fn binding_set_exponent(&self, value: i32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .exponent = value;
        Ok(())
    }
}
#[napi]
impl PredictionGainParts {
    #[napi(js_name = "fraction_storage")]
    pub fn binding_fraction_storage(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .fraction_storage(),
        ))
    }
}
