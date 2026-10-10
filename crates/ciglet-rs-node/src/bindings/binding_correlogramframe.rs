use super::*;
#[napi]
pub struct CorrelogramFrame {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::CorrelogramFrame>>,
}
#[napi]
impl CorrelogramFrame {
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
impl CorrelogramFrame {
    #[napi(getter, js_name = "center")]
    pub fn binding_get_center(&self) -> napi::Result<i32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .center)
    }
    #[napi(setter, js_name = "center")]
    pub fn binding_set_center(&self, value: i32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .center = value;
        Ok(())
    }
    #[napi(getter, js_name = "length")]
    pub fn binding_get_length(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .length as f64)
    }
    #[napi(setter, js_name = "length")]
    pub fn binding_set_length(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .length = checked_usize(value)?;
        Ok(())
    }
}
#[napi]
impl CorrelogramFrame {
    #[napi(constructor)]
    pub fn binding_new(center: i32, length: f64) -> napi::Result<Self> {
        Ok(CorrelogramFrame {
            inner: std::cell::RefCell::new(Some(crate::api::CorrelogramFrame::new(
                center,
                checked_usize(length)?,
            ))),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(CorrelogramFrame {
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
    #[napi(js_name = "compute_f32")]
    pub fn binding_compute_f32(
        &self,
        samples: napi::bindgen_prelude::Float32Array,
        periods: f64,
        method_code: u32,
        policy_code: u32,
    ) -> napi::Result<Array2F32> {
        let storage_samples = samples.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .compute_f32(
            &storage_samples,
            checked_usize(periods)?,
            method_code,
            policy_code,
        ))
        .map(|value| Array2F32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "compute_f64")]
    pub fn binding_compute_f64(
        &self,
        samples: napi::bindgen_prelude::Float64Array,
        periods: f64,
        method_code: u32,
        policy_code: u32,
    ) -> napi::Result<Array2F64> {
        let storage_samples = samples.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .compute_f64(
            &storage_samples,
            checked_usize(periods)?,
            method_code,
            policy_code,
        ))
        .map(|value| Array2F64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
