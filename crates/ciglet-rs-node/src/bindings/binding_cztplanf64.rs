use super::*;
#[napi]
pub struct CztPlanF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::CztPlanF64>>,
}
#[napi]
impl CztPlanF64 {
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
impl CztPlanF64 {
    #[napi(constructor)]
    pub fn binding_new(length: f64, angular_step: f64, direction: u32) -> napi::Result<Self> {
        (crate::api::CztPlanF64::new(checked_usize(length)?, angular_step, direction)).map(
            |value| CztPlanF64 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
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
        real: Option<napi::bindgen_prelude::Float64Array>,
        imaginary: Option<napi::bindgen_prelude::Float64Array>,
    ) -> napi::Result<ComplexValuesF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .transform(
            real.map(|value| value.to_vec()),
            imaginary.map(|value| value.to_vec()),
        ))
        .map(|value| ComplexValuesF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "transform_into")]
    pub fn binding_transform_into(
        &self,
        real: Option<napi::bindgen_prelude::Float64Array>,
        imaginary: Option<napi::bindgen_prelude::Float64Array>,
        output: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<((), napi::bindgen_prelude::Float64Array)> {
        let mut storage_output = output.to_vec();
        {
            ((self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .as_mut()
                .ok_or_else(consumed)?)
            .transform_into(
                real.map(|value| value.to_vec()),
                imaginary.map(|value| value.to_vec()),
                &mut storage_output,
            ))?;
            Ok((
                (),
                <napi::bindgen_prelude::Float64Array>::from(storage_output),
            ))
        }
    }
}
