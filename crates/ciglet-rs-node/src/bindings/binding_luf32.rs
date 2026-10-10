use super::*;
#[napi]
pub struct LuF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::LuF32>>,
}
#[napi]
impl LuF32 {
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
impl LuF32 {
    #[napi(js_name = "factor")]
    pub fn binding_factor(matrix: &MatrixF32) -> napi::Result<Self> {
        (crate::api::LuF32::factor(
            matrix
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?,
        ))
        .map(|value| LuF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "factor_unpivoted")]
    pub fn binding_factor_unpivoted(matrix: &MatrixF32) -> napi::Result<Self> {
        (crate::api::LuF32::factor_unpivoted(
            matrix
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?,
        ))
        .map(|value| LuF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "packed")]
    pub fn binding_packed(&self) -> napi::Result<MatrixF32> {
        Ok(MatrixF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .packed(),
            )),
        })
    }
    #[napi(js_name = "row_swaps")]
    pub fn binding_row_swaps(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .row_swaps(),
        ))
    }
    #[napi(js_name = "solve")]
    pub fn binding_solve(
        &self,
        right: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_right = right.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .solve(&storage_right))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(LuF32 {
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
