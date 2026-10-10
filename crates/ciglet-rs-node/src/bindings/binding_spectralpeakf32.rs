use super::*;
#[napi]
pub struct SpectralPeakF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SpectralPeakF32>>,
}
#[napi]
impl SpectralPeakF32 {
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
impl SpectralPeakF32 {
    #[napi(constructor)]
    pub fn binding_new(bin: f64, magnitude: f64) -> napi::Result<Self> {
        Ok(SpectralPeakF32 {
            inner: std::cell::RefCell::new(Some(crate::api::SpectralPeakF32::new(
                bin as f32,
                magnitude as f32,
            ))),
        })
    }
    #[napi(js_name = "from_values")]
    pub fn binding_from_values(values: napi::bindgen_prelude::Float32Array) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::SpectralPeakF32::from_values(&storage_values)).map(|value| SpectralPeakF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "bin")]
    pub fn binding_bin(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .bin() as f64)
    }
    #[napi(setter, js_name = "bin")]
    pub fn binding_set_bin(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_bin(value as f32);
        Ok(())
    }
    #[napi(getter, js_name = "magnitude")]
    pub fn binding_magnitude(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .magnitude() as f64)
    }
    #[napi(setter, js_name = "magnitude")]
    pub fn binding_set_magnitude(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_magnitude(value as f32);
        Ok(())
    }
    #[napi(js_name = "values")]
    pub fn binding_values(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .values(),
        ))
    }
    #[napi(js_name = "set_values")]
    pub fn binding_set_values(
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
        .set_values(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(SpectralPeakF32 {
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
