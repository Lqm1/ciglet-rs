use super::*;
#[napi]
pub struct ComplexF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ComplexF32>>,
}
#[napi]
impl ComplexF32 {
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
impl ComplexF32 {
    #[napi(constructor)]
    pub fn binding_new(real: f64, imaginary: f64) -> napi::Result<Self> {
        Ok(ComplexF32 {
            inner: std::cell::RefCell::new(Some(crate::api::ComplexF32::new(
                real as f32,
                imaginary as f32,
            ))),
        })
    }
    #[napi(js_name = "from_components")]
    pub fn binding_from_components(
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::ComplexF32::from_components(&storage_values)).map(|value| ComplexF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "real")]
    pub fn binding_real(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .real() as f64)
    }
    #[napi(setter, js_name = "real")]
    pub fn binding_set_real(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_real(value as f32);
        Ok(())
    }
    #[napi(getter, js_name = "imaginary")]
    pub fn binding_imaginary(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .imaginary() as f64)
    }
    #[napi(setter, js_name = "imaginary")]
    pub fn binding_set_imaginary(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_imaginary(value as f32);
        Ok(())
    }
    #[napi(js_name = "components")]
    pub fn binding_components(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .components(),
        ))
    }
    #[napi(js_name = "set_components")]
    pub fn binding_set_components(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_components(&storage_values)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ComplexF32 {
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
    #[napi(js_name = "conjugate")]
    pub fn binding_conjugate(&self) -> napi::Result<Self> {
        Ok(ComplexF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .conjugate(),
            )),
        })
    }
    #[napi(js_name = "add")]
    pub fn binding_add(&self, other: &ComplexF32) -> napi::Result<Self> {
        Ok(ComplexF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .add(
                    other
                        .inner
                        .try_borrow()
                        .map_err(borrowed)?
                        .as_ref()
                        .ok_or_else(consumed)?,
                ),
            )),
        })
    }
    #[napi(js_name = "subtract")]
    pub fn binding_subtract(&self, other: &ComplexF32) -> napi::Result<Self> {
        Ok(ComplexF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .subtract(
                    other
                        .inner
                        .try_borrow()
                        .map_err(borrowed)?
                        .as_ref()
                        .ok_or_else(consumed)?,
                ),
            )),
        })
    }
    #[napi(js_name = "multiply")]
    pub fn binding_multiply(&self, other: &ComplexF32) -> napi::Result<Self> {
        Ok(ComplexF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .multiply(
                    other
                        .inner
                        .try_borrow()
                        .map_err(borrowed)?
                        .as_ref()
                        .ok_or_else(consumed)?,
                ),
            )),
        })
    }
    #[napi(js_name = "divide")]
    pub fn binding_divide(&self, other: &ComplexF32) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .divide(
            other
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| ComplexF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "exponential")]
    pub fn binding_exponential(&self, mode: u32) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .exponential(mode))
        .map(|value| ComplexF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "argument")]
    pub fn binding_argument(&self, mode: u32) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .argument(mode))
        .map(|value| value as f64)
    }
}
