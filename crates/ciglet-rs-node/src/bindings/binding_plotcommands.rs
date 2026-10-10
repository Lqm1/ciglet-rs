use super::*;
#[napi]
pub struct PlotCommands {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PlotCommands>>,
}
#[napi]
impl PlotCommands {
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
impl PlotCommands {
    #[napi(constructor)]
    pub fn binding_new(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        Ok(PlotCommands {
            inner: std::cell::RefCell::new(Some(crate::api::PlotCommands::new(bytes.to_vec()))),
        })
    }
    #[napi(getter, js_name = "bytes")]
    pub fn binding_bytes(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .as_mut()
                .ok_or_else(consumed)?)
            .bytes(),
        ))
    }
    #[napi(setter, js_name = "bytes")]
    pub fn binding_set_bytes(&self, bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_bytes(bytes.to_vec());
        Ok(())
    }
    #[napi(js_name = "append")]
    pub fn binding_append(&self, bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<()> {
        let storage_bytes = bytes.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .append(&storage_bytes)
    }
    #[napi(js_name = "into_inner")]
    pub fn binding_into_inner(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .take()
                .ok_or_else(consumed)?)
            .into_inner(),
        ))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PlotCommands {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .as_mut()
                    .ok_or_else(consumed)?)
                .cloned(),
            )),
        })
    }
    #[napi(js_name = "line_f32")]
    pub fn binding_line_f32(
        &self,
        x: Option<napi::bindgen_prelude::Float32Array>,
        y: napi::bindgen_prelude::Float32Array,
        code: u32,
    ) -> napi::Result<()> {
        let storage_y = y.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .line_f32(x.map(|value| value.to_vec()), &storage_y, code)
    }
    #[napi(js_name = "line_f64")]
    pub fn binding_line_f64(
        &self,
        x: Option<napi::bindgen_prelude::Float64Array>,
        y: napi::bindgen_prelude::Float64Array,
        code: u32,
    ) -> napi::Result<()> {
        let storage_y = y.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .line_f64(x.map(|value| value.to_vec()), &storage_y, code)
    }
    #[napi(js_name = "image_f32")]
    pub fn binding_image_f32(
        &self,
        rows: f64,
        columns: f64,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .image_f32(
            checked_usize(rows)?,
            checked_usize(columns)?,
            values.to_vec(),
        )
    }
    #[napi(js_name = "image_f64")]
    pub fn binding_image_f64(
        &self,
        rows: f64,
        columns: f64,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .image_f64(
            checked_usize(rows)?,
            checked_usize(columns)?,
            values.to_vec(),
        )
    }
}
