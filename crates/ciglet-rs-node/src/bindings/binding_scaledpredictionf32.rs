use super::*;
#[napi]
pub struct ScaledPredictionF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ScaledPredictionF32>>,
}
#[napi]
impl ScaledPredictionF32 {
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
impl ScaledPredictionF32 {
    #[napi(getter, js_name = "coefficients")]
    pub fn binding_coefficients(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .coefficients(),
        ))
    }
    #[napi(js_name = "autocorrelation_parts")]
    pub fn binding_autocorrelation_parts(&self) -> napi::Result<PredictionCorrelationParts> {
        Ok(PredictionCorrelationParts {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .autocorrelation_parts(),
            )),
        })
    }
    #[napi(js_name = "into_prediction")]
    pub fn binding_into_prediction(&self) -> napi::Result<PredictionF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .into_prediction())
        .map(|value| PredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "gain_parts")]
    pub fn binding_gain_parts(&self) -> napi::Result<PredictionGainParts> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .gain_parts())
        .map(|value| PredictionGainParts {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "gain")]
    pub fn binding_gain(&self) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .gain())
        .map(|value| value as f64)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ScaledPredictionF32 {
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
