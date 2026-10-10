use super::*;
#[napi]
pub struct PredictionF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PredictionF32>>,
}
#[napi]
impl PredictionF32 {
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
impl PredictionF32 {
    #[napi(constructor)]
    pub fn binding_new(
        coefficients: napi::bindgen_prelude::Float32Array,
        autocorrelation: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        Ok(PredictionF32 {
            inner: std::cell::RefCell::new(Some(crate::api::PredictionF32::new(
                coefficients.to_vec(),
                autocorrelation.to_vec(),
            ))),
        })
    }
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
    #[napi(setter, js_name = "coefficients")]
    pub fn binding_set_coefficients(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_coefficients(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "autocorrelation")]
    pub fn binding_autocorrelation(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .autocorrelation(),
        ))
    }
    #[napi(setter, js_name = "autocorrelation")]
    pub fn binding_set_autocorrelation(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_autocorrelation(values.to_vec());
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PredictionF32 {
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
