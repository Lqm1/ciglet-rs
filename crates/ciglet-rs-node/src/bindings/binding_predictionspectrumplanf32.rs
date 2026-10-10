use super::*;
#[napi]
pub struct PredictionSpectrumPlanF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PredictionSpectrumPlanF32>>,
}
#[napi]
impl PredictionSpectrumPlanF32 {
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
impl PredictionSpectrumPlanF32 {
    #[napi(constructor)]
    pub fn binding_new(length: f64) -> napi::Result<Self> {
        (crate::api::PredictionSpectrumPlanF32::new(checked_usize(length)?)).map(|value| {
            PredictionSpectrumPlanF32 {
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
    #[napi(js_name = "estimate")]
    pub fn binding_estimate(
        &self,
        magnitudes: napi::bindgen_prelude::Float32Array,
        order: f64,
        code: u32,
    ) -> napi::Result<PredictionF32> {
        let storage_magnitudes = magnitudes.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate(&storage_magnitudes, checked_usize(order)?, code))
        .map(|value| PredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "estimate_with_arithmetic")]
    pub fn binding_estimate_with_arithmetic(
        &self,
        magnitudes: napi::bindgen_prelude::Float32Array,
        order: f64,
        nyquist_code: u32,
        arithmetic_code: u32,
    ) -> napi::Result<PredictionF32> {
        let storage_magnitudes = magnitudes.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_with_arithmetic(
            &storage_magnitudes,
            checked_usize(order)?,
            nyquist_code,
            arithmetic_code,
        ))
        .map(|value| PredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "estimate_scaled")]
    pub fn binding_estimate_scaled(
        &self,
        magnitudes: napi::bindgen_prelude::Float32Array,
        order: f64,
        code: u32,
    ) -> napi::Result<ScaledPredictionF32> {
        let storage_magnitudes = magnitudes.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_scaled(&storage_magnitudes, checked_usize(order)?, code))
        .map(|value| ScaledPredictionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "spectrum")]
    pub fn binding_spectrum(
        &self,
        coefficients: napi::bindgen_prelude::Float32Array,
        gain: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_coefficients = coefficients.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .spectrum(&storage_coefficients, gain as f32))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
}
