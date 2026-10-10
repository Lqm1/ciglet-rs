use super::*;
#[napi]
pub struct ComplexValuesF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ComplexValuesF64>>,
}
#[napi]
impl ComplexValuesF64 {
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
impl ComplexValuesF64 {
    #[napi(constructor)]
    pub fn binding_new(
        real: napi::bindgen_prelude::Float64Array,
        imaginary: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<Self> {
        (crate::api::ComplexValuesF64::new(real.to_vec(), imaginary.to_vec())).map(|value| {
            ComplexValuesF64 {
                inner: std::cell::RefCell::new(Some(value)),
            }
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
    #[napi(getter, js_name = "real")]
    pub fn binding_real(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .real(),
        ))
    }
    #[napi(getter, js_name = "imaginary")]
    pub fn binding_imaginary(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .imaginary(),
        ))
    }
    #[napi(getter, js_name = "interleaved")]
    pub fn binding_interleaved(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .interleaved(),
        ))
    }
}
