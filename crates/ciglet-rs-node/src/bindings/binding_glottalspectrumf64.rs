use super::*;
#[napi]
pub struct GlottalSpectrumF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GlottalSpectrumF64>>,
}
#[napi]
impl GlottalSpectrumF64 {
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
impl GlottalSpectrumF64 {
    #[napi(constructor)]
    pub fn binding_new(
        magnitude: napi::bindgen_prelude::Float64Array,
        phase: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<Self> {
        Ok(GlottalSpectrumF64 {
            inner: std::cell::RefCell::new(Some(crate::api::GlottalSpectrumF64::new(
                magnitude.to_vec(),
                phase.to_vec(),
            ))),
        })
    }
    #[napi(getter, js_name = "magnitude")]
    pub fn binding_magnitude(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .magnitude(),
        ))
    }
    #[napi(setter, js_name = "magnitude")]
    pub fn binding_set_magnitude(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_magnitude(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "phase")]
    pub fn binding_phase(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .phase(),
        ))
    }
    #[napi(setter, js_name = "phase")]
    pub fn binding_set_phase(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_phase(values.to_vec());
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GlottalSpectrumF64 {
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
