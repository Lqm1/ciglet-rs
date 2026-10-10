use super::*;
#[napi]
pub struct SortedValuesF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SortedValuesF32>>,
}
#[napi]
impl SortedValuesF32 {
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
impl SortedValuesF32 {
    #[napi(constructor)]
    pub fn binding_new(
        values: napi::bindgen_prelude::Float32Array,
        indices: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<Self> {
        Ok(SortedValuesF32 {
            inner: std::cell::RefCell::new(Some(crate::api::SortedValuesF32::new(
                values.to_vec(),
                indices.to_vec(),
            ))),
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
    #[napi(getter, js_name = "values")]
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
    #[napi(getter, js_name = "indices")]
    pub fn binding_indices(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .indices(),
        ))
    }
}
