use super::*;
#[napi]
pub struct PredictionResonanceEstimateF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PredictionResonanceEstimateF64>>,
}
#[napi]
impl PredictionResonanceEstimateF64 {
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
impl PredictionResonanceEstimateF64 {
    #[napi(constructor)]
    pub fn binding_new(
        resonances: napi::bindgen_prelude::Float64Array,
        iterations: f64,
        converged: bool,
        maximum_residual: f64,
    ) -> napi::Result<Self> {
        let storage_resonances = resonances.to_vec();
        (crate::api::PredictionResonanceEstimateF64::new(
            &storage_resonances,
            checked_usize(iterations)?,
            converged,
            maximum_residual,
        ))
        .map(|value| PredictionResonanceEstimateF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "resonances")]
    pub fn binding_resonances(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .resonances(),
        ))
    }
    #[napi(setter, js_name = "resonances")]
    pub fn binding_set_resonances(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_resonances(&storage_values)
    }
    #[napi(js_name = "resonance")]
    pub fn binding_resonance(&self, index: f64) -> napi::Result<Option<PredictionResonanceF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .resonance(checked_usize(index)?))
        .map(|value| PredictionResonanceF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(getter, js_name = "iterations")]
    pub fn binding_iterations(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .iterations() as f64)
    }
    #[napi(setter, js_name = "iterations")]
    pub fn binding_set_iterations(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_iterations(checked_usize(value)?);
        Ok(())
    }
    #[napi(getter, js_name = "converged")]
    pub fn binding_converged(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .converged())
    }
    #[napi(setter, js_name = "converged")]
    pub fn binding_set_converged(&self, value: bool) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_converged(value);
        Ok(())
    }
    #[napi(getter, js_name = "maximum_residual")]
    pub fn binding_maximum_residual(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .maximum_residual())
    }
    #[napi(setter, js_name = "maximum_residual")]
    pub fn binding_set_maximum_residual(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_maximum_residual(value);
        Ok(())
    }
    #[napi(js_name = "residual_storage")]
    pub fn binding_residual_storage(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .residual_storage(),
        ))
    }
    #[napi(js_name = "set_residual_storage")]
    pub fn binding_set_residual_storage(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_residual_storage(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PredictionResonanceEstimateF64 {
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
