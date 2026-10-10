use super::*;
#[napi]
pub struct MatrixF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MatrixF32>>,
}
#[napi]
impl MatrixF32 {
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
impl MatrixF32 {
    #[napi(constructor)]
    pub fn binding_new(
        rows: f64,
        columns: f64,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        (crate::api::MatrixF32::new(
            checked_usize(rows)?,
            checked_usize(columns)?,
            values.to_vec(),
        ))
        .map(|value| MatrixF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "zeros")]
    pub fn binding_zeros(rows: f64, columns: f64) -> napi::Result<Self> {
        (crate::api::MatrixF32::zeros(checked_usize(rows)?, checked_usize(columns)?)).map(|value| {
            MatrixF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "identity")]
    pub fn binding_identity(size: f64) -> napi::Result<Self> {
        (crate::api::MatrixF32::identity(checked_usize(size)?)).map(|value| MatrixF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "rows")]
    pub fn binding_rows(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .rows() as f64)
    }
    #[napi(getter, js_name = "columns")]
    pub fn binding_columns(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .columns() as f64)
    }
    #[napi(js_name = "as_slice")]
    pub fn binding_as_slice(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .as_slice(),
        ))
    }
    #[napi(js_name = "into_column_major")]
    pub fn binding_into_column_major(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?)
            .into_column_major(),
        ))
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, row: f64, column: f64) -> napi::Result<Option<f64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(row)?, checked_usize(column)?))
        .map(|value| value as f64))
    }
    #[napi(js_name = "apply_row_swaps")]
    pub fn binding_apply_row_swaps(
        &self,
        partners: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<()> {
        let storage_partners = partners.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .apply_row_swaps(&storage_partners)
    }
    #[napi(js_name = "pivot_rows")]
    pub fn binding_pivot_rows(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .pivot_rows())
        .map(<napi::bindgen_prelude::Uint32Array>::from)
    }
    #[napi(js_name = "multiply")]
    pub fn binding_multiply(&self, right: &MatrixF32) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .multiply(
            right
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| MatrixF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "multiply_vector")]
    pub fn binding_multiply_vector(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        let storage_values = values.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .multiply_vector(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(MatrixF32 {
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
