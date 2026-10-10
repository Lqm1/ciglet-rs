use super::*;
#[napi]
pub struct RandomSampler {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::RandomSampler>>,
}
#[napi]
impl RandomSampler {
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
impl RandomSampler {
    #[napi(constructor)]
    pub fn binding_new(source: &RandomWordSource, maximum: u32) -> napi::Result<Self> {
        (crate::api::RandomSampler::new(
            source
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?,
            maximum,
        ))
        .map(|value| RandomSampler {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "from_seed")]
    pub fn binding_from_seed(seed: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_seed = seed.to_vec();
        (crate::api::RandomSampler::from_seed(&storage_seed)).map(|value| RandomSampler {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "maximum")]
    pub fn binding_maximum(&self) -> napi::Result<u32> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .maximum()
    }
    #[napi(js_name = "draw")]
    pub fn binding_draw(&self) -> napi::Result<u32> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .draw()
    }
    #[napi(js_name = "into_rng")]
    pub fn binding_into_rng(&self) -> napi::Result<RandomWordSource> {
        Ok(RandomWordSource {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .take()
                    .ok_or_else(consumed)?)
                .into_rng(),
            )),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .cloned())
        .map(|value| RandomSampler {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "uniform_f32")]
    pub fn binding_uniform_f32(&self) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .uniform_f32())
        .map(|value| value as f64)
    }
    #[napi(js_name = "uniform_f64")]
    pub fn binding_uniform_f64(&self) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .uniform_f64()
    }
    #[napi(js_name = "normal_f32")]
    pub fn binding_normal_f32(&self, mean: f64, variance: f64) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normal_f32(mean as f32, variance as f32))
        .map(|value| value as f64)
    }
    #[napi(js_name = "normal_f64")]
    pub fn binding_normal_f64(&self, mean: f64, variance: f64) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normal_f64(mean, variance)
    }
    #[napi(js_name = "normal_with_f32")]
    pub fn binding_normal_with_f32(
        &self,
        mean: f64,
        variance: f64,
        code: u32,
    ) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normal_with_f32(mean as f32, variance as f32, code))
        .map(|value| value as f64)
    }
    #[napi(js_name = "normal_with_f64")]
    pub fn binding_normal_with_f64(
        &self,
        mean: f64,
        variance: f64,
        code: u32,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .normal_with_f64(mean, variance, code)
    }
    #[napi(js_name = "white_noise_f32")]
    pub fn binding_white_noise_f32(
        &self,
        amplitude: f64,
        count: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .white_noise_f32(amplitude as f32, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "white_noise_f64")]
    pub fn binding_white_noise_f64(
        &self,
        amplitude: f64,
        count: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .white_noise_f64(amplitude, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
}
