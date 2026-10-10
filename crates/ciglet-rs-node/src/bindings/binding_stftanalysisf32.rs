use super::*;
#[napi]
pub struct StftAnalysisF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StftAnalysisF32>>,
}
#[napi]
impl StftAnalysisF32 {
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
impl StftAnalysisF32 {
    #[napi(constructor)]
    pub fn binding_new(
        frames: f64,
        bins: f64,
        magnitude: Option<napi::bindgen_prelude::Float32Array>,
        phase: Option<napi::bindgen_prelude::Float32Array>,
        window_weight: Option<f64>,
        normalization: Option<f64>,
    ) -> napi::Result<Self> {
        Ok(StftAnalysisF32 {
            inner: std::cell::RefCell::new(Some(crate::api::StftAnalysisF32::new(
                checked_usize(frames)?,
                checked_usize(bins)?,
                magnitude.map(|value| value.to_vec()),
                phase.map(|value| value.to_vec()),
                window_weight.map(|value| value as f32),
                normalization.map(|value| value as f32),
            ))),
        })
    }
    #[napi(getter, js_name = "frames")]
    pub fn binding_frames(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .frames() as f64)
    }
    #[napi(setter, js_name = "frames")]
    pub fn binding_set_frames(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_frames(checked_usize(value)?);
        Ok(())
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
    #[napi(setter, js_name = "bins")]
    pub fn binding_set_bins(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_bins(checked_usize(value)?);
        Ok(())
    }
    #[napi(getter, js_name = "magnitude")]
    pub fn binding_magnitude(&self) -> napi::Result<Option<napi::bindgen_prelude::Float32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .magnitude())
        .map(<napi::bindgen_prelude::Float32Array>::from))
    }
    #[napi(setter, js_name = "magnitude")]
    pub fn binding_set_magnitude(
        &self,
        value: Option<napi::bindgen_prelude::Float32Array>,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_magnitude(value.map(|value| value.to_vec()));
        Ok(())
    }
    #[napi(getter, js_name = "phase")]
    pub fn binding_phase(&self) -> napi::Result<Option<napi::bindgen_prelude::Float32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .phase())
        .map(<napi::bindgen_prelude::Float32Array>::from))
    }
    #[napi(setter, js_name = "phase")]
    pub fn binding_set_phase(
        &self,
        value: Option<napi::bindgen_prelude::Float32Array>,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_phase(value.map(|value| value.to_vec()));
        Ok(())
    }
    #[napi(getter, js_name = "window_weight")]
    pub fn binding_window_weight(&self) -> napi::Result<Option<f64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .window_weight())
        .map(|value| value as f64))
    }
    #[napi(setter, js_name = "window_weight")]
    pub fn binding_set_window_weight(&self, value: Option<f64>) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_window_weight(value.map(|value| value as f32));
        Ok(())
    }
    #[napi(getter, js_name = "normalization")]
    pub fn binding_normalization(&self) -> napi::Result<Option<f64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normalization())
        .map(|value| value as f64))
    }
    #[napi(setter, js_name = "normalization")]
    pub fn binding_set_normalization(&self, value: Option<f64>) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_normalization(value.map(|value| value as f32));
        Ok(())
    }
    #[napi(js_name = "window_weight_storage")]
    pub fn binding_window_weight_storage(
        &self,
    ) -> napi::Result<Option<napi::bindgen_prelude::Float32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .window_weight_storage())
        .map(<napi::bindgen_prelude::Float32Array>::from))
    }
    #[napi(js_name = "set_window_weight_storage")]
    pub fn binding_set_window_weight_storage(
        &self,
        values: Option<napi::bindgen_prelude::Float32Array>,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_window_weight_storage(values.map(|value| value.to_vec()))
    }
    #[napi(js_name = "normalization_storage")]
    pub fn binding_normalization_storage(
        &self,
    ) -> napi::Result<Option<napi::bindgen_prelude::Float32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normalization_storage())
        .map(<napi::bindgen_prelude::Float32Array>::from))
    }
    #[napi(js_name = "set_normalization_storage")]
    pub fn binding_set_normalization_storage(
        &self,
        values: Option<napi::bindgen_prelude::Float32Array>,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_normalization_storage(values.map(|value| value.to_vec()))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StftAnalysisF32 {
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
