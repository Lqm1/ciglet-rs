use super::*;
#[napi]
pub struct GlottalModelF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GlottalModelF32>>,
}
#[napi]
impl GlottalModelF32 {
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
impl GlottalModelF32 {
    #[napi(getter, js_name = "period")]
    pub fn binding_get_period(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .period as f64)
    }
    #[napi(setter, js_name = "period")]
    pub fn binding_set_period(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .period = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "closure")]
    pub fn binding_get_closure(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .closure as f64)
    }
    #[napi(setter, js_name = "closure")]
    pub fn binding_set_closure(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .closure = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "peak")]
    pub fn binding_get_peak(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .peak as f64)
    }
    #[napi(setter, js_name = "peak")]
    pub fn binding_set_peak(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .peak = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "return_time")]
    pub fn binding_get_return_time(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .return_time as f64)
    }
    #[napi(setter, js_name = "return_time")]
    pub fn binding_set_return_time(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .return_time = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "excitation")]
    pub fn binding_get_excitation(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .excitation as f64)
    }
    #[napi(setter, js_name = "excitation")]
    pub fn binding_set_excitation(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .excitation = value as f32;
        Ok(())
    }
}
#[napi]
impl GlottalModelF32 {
    #[napi(constructor)]
    pub fn binding_new(
        period: f64,
        closure: f64,
        peak: f64,
        return_time: f64,
        excitation: f64,
    ) -> napi::Result<Self> {
        Ok(GlottalModelF32 {
            inner: std::cell::RefCell::new(Some(crate::api::GlottalModelF32::new(
                period as f32,
                closure as f32,
                peak as f32,
                return_time as f32,
                excitation as f32,
            ))),
        })
    }
    #[napi(js_name = "from_shape")]
    pub fn binding_from_shape(shape: f64, period: f64, excitation: f64) -> napi::Result<Self> {
        (crate::api::GlottalModelF32::from_shape(shape as f32, period as f32, excitation as f32))
            .map(|value| GlottalModelF32 {
                inner: std::cell::RefCell::new(Some(value)),
            })
    }
    #[napi(js_name = "from_components")]
    pub fn binding_from_components(
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::GlottalModelF32::from_components(&storage_values)).map(|value| {
            GlottalModelF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "components")]
    pub fn binding_components(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
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
        values: napi::bindgen_prelude::Float32Array,
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
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(&self) -> napi::Result<GlottalPreparedF32> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare())
        .map(|value| GlottalPreparedF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "spectrum")]
    pub fn binding_spectrum(
        &self,
        frequencies: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<GlottalSpectrumF32> {
        let storage_frequencies = frequencies.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .spectrum(&storage_frequencies))
        .map(|value| GlottalSpectrumF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "sampled_period")]
    pub fn binding_sampled_period(
        &self,
        sampling_rate: i32,
        count: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .sampled_period(sampling_rate, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GlottalModelF32 {
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
