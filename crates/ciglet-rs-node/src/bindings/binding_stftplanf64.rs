use super::*;
#[napi]
pub struct StftPlanF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StftPlanF64>>,
}
#[napi]
impl StftPlanF64 {
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
impl StftPlanF64 {
    #[napi(constructor)]
    pub fn binding_new(length: f64, window_code: u32, precision_code: u32) -> napi::Result<Self> {
        (crate::api::StftPlanF64::new(checked_usize(length)?, window_code, precision_code)).map(
            |value| StftPlanF64 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
    }
    #[napi(js_name = "analyze")]
    pub fn binding_analyze(
        &self,
        signal: napi::bindgen_prelude::Float64Array,
        centers: napi::bindgen_prelude::Int32Array,
        window_lengths: napi::bindgen_prelude::Uint32Array,
        options: &StftAnalysisOptions,
    ) -> napi::Result<StftAnalysisF64> {
        let storage_signal = signal.to_vec();
        let storage_centers = centers.to_vec();
        let storage_window_lengths = window_lengths.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .analyze(
            &storage_signal,
            &storage_centers,
            &storage_window_lengths,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| StftAnalysisF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "synthesize")]
    pub fn binding_synthesize(
        &self,
        magnitude: napi::bindgen_prelude::Float64Array,
        phase: napi::bindgen_prelude::Float64Array,
        options: &StftSynthesisOptionsF64,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_magnitude = magnitude.to_vec();
        let storage_phase = phase.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .synthesize(
            &storage_magnitude,
            &storage_phase,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
}
