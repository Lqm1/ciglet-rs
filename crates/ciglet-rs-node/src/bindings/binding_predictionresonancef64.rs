use super::*;
#[napi]
pub struct PredictionResonanceF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PredictionResonanceF64>>,
}
#[napi]
impl PredictionResonanceF64 {
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
impl PredictionResonanceF64 {
    #[napi(constructor)]
    pub fn binding_new(frequency_nyquist: f64, real: f64, imaginary: f64) -> napi::Result<Self> {
        Ok(PredictionResonanceF64 {
            inner: std::cell::RefCell::new(Some(crate::api::PredictionResonanceF64::new(
                frequency_nyquist,
                real,
                imaginary,
            ))),
        })
    }
    #[napi(js_name = "from_components")]
    pub fn binding_from_components(
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::PredictionResonanceF64::from_components(&storage_values)).map(|value| {
            PredictionResonanceF64 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "components")]
    pub fn binding_components(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .components(),
        ))
    }
    #[napi(js_name = "set_components")]
    pub fn binding_set_components(
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
        .set_components(&storage_values)
    }
    #[napi(getter, js_name = "frequency_nyquist")]
    pub fn binding_frequency_nyquist(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .frequency_nyquist())
    }
    #[napi(setter, js_name = "frequency_nyquist")]
    pub fn binding_set_frequency_nyquist(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_frequency_nyquist(value);
        Ok(())
    }
    #[napi(js_name = "pole")]
    pub fn binding_pole(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .pole(),
        ))
    }
    #[napi(js_name = "set_pole")]
    pub fn binding_set_pole(
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
        .set_pole(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PredictionResonanceF64 {
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
