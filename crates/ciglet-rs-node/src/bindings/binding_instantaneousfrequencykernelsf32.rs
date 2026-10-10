use super::*;
#[napi]
pub struct InstantaneousFrequencyKernelsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::InstantaneousFrequencyKernelsF32>>,
}
#[napi]
impl InstantaneousFrequencyKernelsF32 {
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
impl InstantaneousFrequencyKernelsF32 {
    #[napi(constructor)]
    pub fn binding_new(
        real: napi::bindgen_prelude::Float32Array,
        imaginary: napi::bindgen_prelude::Float32Array,
        derivative_real: napi::bindgen_prelude::Float32Array,
        derivative_imaginary: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        Ok(InstantaneousFrequencyKernelsF32 {
            inner: std::cell::RefCell::new(Some(
                crate::api::InstantaneousFrequencyKernelsF32::new(
                    real.to_vec(),
                    imaginary.to_vec(),
                    derivative_real.to_vec(),
                    derivative_imaginary.to_vec(),
                ),
            )),
        })
    }
    #[napi(getter, js_name = "real")]
    pub fn binding_real(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .real(),
        ))
    }
    #[napi(setter, js_name = "real")]
    pub fn binding_set_real(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_real(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "imaginary")]
    pub fn binding_imaginary(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .imaginary(),
        ))
    }
    #[napi(setter, js_name = "imaginary")]
    pub fn binding_set_imaginary(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_imaginary(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "derivative_real")]
    pub fn binding_derivative_real(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .derivative_real(),
        ))
    }
    #[napi(setter, js_name = "derivative_real")]
    pub fn binding_set_derivative_real(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_derivative_real(values.to_vec());
        Ok(())
    }
    #[napi(getter, js_name = "derivative_imaginary")]
    pub fn binding_derivative_imaginary(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .derivative_imaginary(),
        ))
    }
    #[napi(setter, js_name = "derivative_imaginary")]
    pub fn binding_set_derivative_imaginary(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_derivative_imaginary(values.to_vec());
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(InstantaneousFrequencyKernelsF32 {
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
