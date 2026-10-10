use super::*;
#[napi]
pub struct FftPlanF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::FftPlanF32>>,
}
#[napi]
impl FftPlanF32 {
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
impl FftPlanF32 {
    #[napi(constructor)]
    pub fn binding_new(length: f64) -> napi::Result<Self> {
        (crate::api::FftPlanF32::new(checked_usize(length)?)).map(|value| FftPlanF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "transform")]
    pub fn binding_transform(
        &self,
        real: Option<napi::bindgen_prelude::Float32Array>,
        imaginary: Option<napi::bindgen_prelude::Float32Array>,
        direction: u32,
    ) -> napi::Result<ComplexValuesF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .transform(
            real.map(|value| value.to_vec()),
            imaginary.map(|value| value.to_vec()),
            direction,
        ))
        .map(|value| ComplexValuesF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "process")]
    pub fn binding_process(
        &self,
        interleaved: napi::bindgen_prelude::Float32Array,
        direction: u32,
    ) -> napi::Result<ComplexValuesF32> {
        let storage_interleaved = interleaved.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .process(&storage_interleaved, direction))
        .map(|value| ComplexValuesF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
