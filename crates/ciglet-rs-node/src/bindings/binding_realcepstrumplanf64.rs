use super::*;
#[napi]
pub struct RealCepstrumPlanF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::RealCepstrumPlanF64>>,
}
#[napi]
impl RealCepstrumPlanF64 {
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
impl RealCepstrumPlanF64 {
    #[napi(constructor)]
    pub fn binding_new(length: f64) -> napi::Result<Self> {
        (crate::api::RealCepstrumPlanF64::new(checked_usize(length)?)).map(|value| {
            RealCepstrumPlanF64 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(getter, js_name = "bins")]
    pub fn binding_bins(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .bins() as f64)
    }
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "analyze")]
    pub fn binding_analyze(
        &self,
        logs: napi::bindgen_prelude::Float64Array,
        policy: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_logs = logs.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .analyze(&storage_logs, policy))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
    #[napi(js_name = "reconstruct")]
    pub fn binding_reconstruct(
        &self,
        coefficients: napi::bindgen_prelude::Float64Array,
        policy: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_coefficients = coefficients.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .reconstruct(&storage_coefficients, policy))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
    #[napi(js_name = "minimum_phase_with")]
    pub fn binding_minimum_phase_with(
        &self,
        logs: napi::bindgen_prelude::Float64Array,
        policy: u32,
        lifter_code: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_logs = logs.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .minimum_phase_with(&storage_logs, policy, lifter_code))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
    #[napi(js_name = "spectrogram_to_cepstra")]
    pub fn binding_spectrogram_to_cepstra(
        &self,
        magnitudes: napi::bindgen_prelude::Float64Array,
        count: f64,
        policy: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_magnitudes = magnitudes.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .spectrogram_to_cepstra(&storage_magnitudes, checked_usize(count)?, policy))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
    #[napi(js_name = "cepstra_to_spectrogram")]
    pub fn binding_cepstra_to_spectrogram(
        &self,
        coefficients: napi::bindgen_prelude::Float64Array,
        frames: f64,
        count: f64,
        policy: u32,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_coefficients = coefficients.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .cepstra_to_spectrogram(
            &storage_coefficients,
            checked_usize(frames)?,
            checked_usize(count)?,
            policy,
        ))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
}
