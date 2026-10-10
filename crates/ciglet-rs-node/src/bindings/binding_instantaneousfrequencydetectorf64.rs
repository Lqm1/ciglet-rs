use super::*;
#[napi]
pub struct InstantaneousFrequencyDetectorF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::InstantaneousFrequencyDetectorF64>>,
}
#[napi]
impl InstantaneousFrequencyDetectorF64 {
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
impl InstantaneousFrequencyDetectorF64 {
    #[napi(constructor)]
    pub fn binding_new(center_frequency: f64, resolution: f64) -> napi::Result<Self> {
        (crate::api::InstantaneousFrequencyDetectorF64::new(center_frequency, resolution)).map(
            |value| InstantaneousFrequencyDetectorF64 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
    }
    #[napi(js_name = "from_kernels")]
    pub fn binding_from_kernels(
        center_frequency: f64,
        kernels: &InstantaneousFrequencyKernelsF64,
    ) -> napi::Result<Self> {
        (crate::api::InstantaneousFrequencyDetectorF64::from_kernels(
            center_frequency,
            kernels
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| InstantaneousFrequencyDetectorF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "center_frequency")]
    pub fn binding_center_frequency(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .center_frequency())
    }
    #[napi(js_name = "set_center_frequency")]
    pub fn binding_set_center_frequency(&self, value: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_center_frequency(value)
    }
    #[napi(getter, js_name = "kernels")]
    pub fn binding_kernels(&self) -> napi::Result<InstantaneousFrequencyKernelsF64> {
        Ok(InstantaneousFrequencyKernelsF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .kernels(),
            )),
        })
    }
    #[napi(setter, js_name = "kernels")]
    pub fn binding_set_kernels(
        &self,
        kernels: &InstantaneousFrequencyKernelsF64,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_kernels(
            kernels
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "estimate")]
    pub fn binding_estimate(
        &self,
        samples: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<f64> {
        let storage_samples = samples.to_vec();
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .estimate(&storage_samples)
    }
    #[napi(js_name = "estimate_with")]
    pub fn binding_estimate_with(
        &self,
        samples: napi::bindgen_prelude::Float64Array,
        policy_code: u32,
    ) -> napi::Result<f64> {
        let storage_samples = samples.to_vec();
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .estimate_with(&storage_samples, policy_code)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(InstantaneousFrequencyDetectorF64 {
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
