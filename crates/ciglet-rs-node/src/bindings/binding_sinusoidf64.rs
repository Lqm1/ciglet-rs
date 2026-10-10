use super::*;
#[napi]
pub struct SinusoidF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SinusoidF64>>,
}
#[napi]
impl SinusoidF64 {
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
impl SinusoidF64 {
    #[napi(constructor)]
    pub fn binding_new(
        frequency_hz: f64,
        amplitude: f64,
        phase_radians: f64,
    ) -> napi::Result<Self> {
        Ok(SinusoidF64 {
            inner: std::cell::RefCell::new(Some(crate::api::SinusoidF64::new(
                frequency_hz,
                amplitude,
                phase_radians,
            ))),
        })
    }
    #[napi(js_name = "from_values")]
    pub fn binding_from_values(values: napi::bindgen_prelude::Float64Array) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::SinusoidF64::from_values(&storage_values)).map(|value| SinusoidF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "frequency_hz")]
    pub fn binding_frequency_hz(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .frequency_hz())
    }
    #[napi(setter, js_name = "frequency_hz")]
    pub fn binding_set_frequency_hz(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_frequency_hz(value);
        Ok(())
    }
    #[napi(getter, js_name = "amplitude")]
    pub fn binding_amplitude(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .amplitude())
    }
    #[napi(setter, js_name = "amplitude")]
    pub fn binding_set_amplitude(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_amplitude(value);
        Ok(())
    }
    #[napi(getter, js_name = "phase_radians")]
    pub fn binding_phase_radians(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .phase_radians())
    }
    #[napi(setter, js_name = "phase_radians")]
    pub fn binding_set_phase_radians(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_phase_radians(value);
        Ok(())
    }
    #[napi(js_name = "values")]
    pub fn binding_values(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
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
        values: napi::bindgen_prelude::Float64Array,
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
        Ok(SinusoidF64 {
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
