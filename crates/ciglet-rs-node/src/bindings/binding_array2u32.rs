use super::*;
#[napi]
pub struct Array2U32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::Array2U32>>,
}
#[napi]
impl Array2U32 {
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
impl Array2U32 {
    #[napi(constructor)]
    pub fn binding_new(
        rows: f64,
        columns: f64,
        values: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::Array2U32::new(
            checked_usize(rows)?,
            checked_usize(columns)?,
            &storage_values,
        ))
        .map(|value| Array2U32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "zeros")]
    pub fn binding_zeros(rows: f64, columns: f64) -> napi::Result<Self> {
        (crate::api::Array2U32::zeros(checked_usize(rows)?, checked_usize(columns)?)).map(|value| {
            Array2U32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "filled")]
    pub fn binding_filled(rows: f64, columns: f64, element: u32) -> napi::Result<Self> {
        (crate::api::Array2U32::filled(checked_usize(rows)?, checked_usize(columns)?, element)).map(
            |value| Array2U32 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
    }
    #[napi(js_name = "from_rows")]
    pub fn binding_from_rows(
        values: napi::bindgen_prelude::Uint32Array,
        row_lengths: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        let storage_row_lengths = row_lengths.to_vec();
        (crate::api::Array2U32::from_rows(&storage_values, &storage_row_lengths)).map(|value| {
            Array2U32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
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
    #[napi(js_name = "values")]
    pub fn binding_values(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .values(),
        ))
    }
    #[napi(js_name = "set_values")]
    pub fn binding_set_values(
        &self,
        values: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_values(&storage_values)
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, row: f64, column: f64) -> napi::Result<Option<u32>> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(row)?, checked_usize(column)?))
    }
    #[napi(js_name = "set")]
    pub fn binding_set(&self, row: f64, column: f64, value: u32) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set(checked_usize(row)?, checked_usize(column)?, value))
    }
    #[napi(js_name = "row")]
    pub fn binding_row(
        &self,
        row: f64,
    ) -> napi::Result<Option<napi::bindgen_prelude::Uint32Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .row(checked_usize(row)?))
        .map(<napi::bindgen_prelude::Uint32Array>::from))
    }
    #[napi(js_name = "set_row")]
    pub fn binding_set_row(
        &self,
        row: f64,
        values: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_row(checked_usize(row)?, &storage_values)
    }
    #[napi(js_name = "reshape")]
    pub fn binding_reshape(&self, rows: f64, columns: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .reshape(checked_usize(rows)?, checked_usize(columns)?)
    }
    #[napi(js_name = "transpose")]
    pub fn binding_transpose(&self) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .transpose()
    }
    #[napi(js_name = "transposed")]
    pub fn binding_transposed(&self) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .transposed())
        .map(|value| Array2U32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .cloned())
        .map(|value| Array2U32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "flatten")]
    pub fn binding_flatten(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .flatten())
        .map(<napi::bindgen_prelude::Uint32Array>::from)
    }
    #[napi(js_name = "into_row_major")]
    pub fn binding_into_row_major(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?)
            .into_row_major(),
        ))
    }
}
