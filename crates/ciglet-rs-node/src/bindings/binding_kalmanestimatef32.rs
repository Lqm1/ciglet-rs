use super::*;
#[napi]
pub struct KalmanEstimateF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::KalmanEstimateF32>>,
}
#[napi]
impl KalmanEstimateF32 {
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
impl KalmanEstimateF32 {
    #[napi(constructor)]
    pub fn binding_new(
        means: napi::bindgen_prelude::Float32Array,
        variances: napi::bindgen_prelude::Float32Array,
        log_likelihood: f64,
    ) -> napi::Result<Self> {
        let storage_means = means.to_vec();
        let storage_variances = variances.to_vec();
        Ok(KalmanEstimateF32 {
            inner: std::cell::RefCell::new(Some(crate::api::KalmanEstimateF32::new(
                &storage_means,
                &storage_variances,
                log_likelihood as f32,
            ))),
        })
    }
    #[napi(getter, js_name = "means")]
    pub fn binding_means(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .means(),
        ))
    }
    #[napi(setter, js_name = "means")]
    pub fn binding_set_means(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_means(&storage_values);
        Ok(())
    }
    #[napi(getter, js_name = "variances")]
    pub fn binding_variances(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .variances(),
        ))
    }
    #[napi(setter, js_name = "variances")]
    pub fn binding_set_variances(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_variances(&storage_values);
        Ok(())
    }
    #[napi(getter, js_name = "log_likelihood")]
    pub fn binding_log_likelihood(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_likelihood() as f64)
    }
    #[napi(setter, js_name = "log_likelihood")]
    pub fn binding_set_log_likelihood(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_log_likelihood(value as f32);
        Ok(())
    }
    #[napi(js_name = "likelihood_value")]
    pub fn binding_likelihood_value(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .likelihood_value(),
        ))
    }
    #[napi(js_name = "set_likelihood_value")]
    pub fn binding_set_likelihood_value(
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
        .set_likelihood_value(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(KalmanEstimateF32 {
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
