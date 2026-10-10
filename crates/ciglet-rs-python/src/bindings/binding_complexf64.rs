use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct ComplexF64 {
    pub(crate) inner: Option<crate::api::ComplexF64>,
}
#[pymethods]
impl ComplexF64 {
    fn close(&mut self) {
        self.inner = None;
    }
    fn free(&mut self) {
        self.inner = None;
    }
    #[getter]
    fn is_closed(&self) -> bool {
        self.inner.is_none()
    }
}
#[pymethods]
impl ComplexF64 {
    #[new]
    #[pyo3(signature = (real, imaginary))]
    pub fn binding_new(real: f64, imaginary: f64) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some(crate::api::ComplexF64::new(real, imaginary)),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_components")]
    #[pyo3(signature = (values))]
    pub fn binding_from_components(values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::ComplexF64::from_components(&values))
            .map(|value| ComplexF64 { inner: Some(value) })
    }
    #[getter(real)]
    pub fn binding_real(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).real())
    }
    #[setter(real)]
    pub fn binding_set_real(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_real(value);
        Ok(())
    }
    #[getter(imaginary)]
    pub fn binding_imaginary(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).imaginary())
    }
    #[setter(imaginary)]
    pub fn binding_set_imaginary(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_imaginary(value);
        Ok(())
    }
    #[pyo3(name = "components")]
    #[pyo3(signature = ())]
    pub fn binding_components(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).components())
    }
    #[pyo3(name = "set_components")]
    #[pyo3(signature = (values))]
    pub fn binding_set_components(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_components(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "conjugate")]
    #[pyo3(signature = ())]
    pub fn binding_conjugate(&self) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).conjugate()),
        })
    }
    #[pyo3(name = "add")]
    #[pyo3(signature = (other))]
    pub fn binding_add(&self, other: pyo3::PyRef<'_, ComplexF64>) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some(
                (self.inner.as_ref().ok_or_else(consumed)?)
                    .add(other.inner.as_ref().ok_or_else(consumed)?),
            ),
        })
    }
    #[pyo3(name = "subtract")]
    #[pyo3(signature = (other))]
    pub fn binding_subtract(&self, other: pyo3::PyRef<'_, ComplexF64>) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some(
                (self.inner.as_ref().ok_or_else(consumed)?)
                    .subtract(other.inner.as_ref().ok_or_else(consumed)?),
            ),
        })
    }
    #[pyo3(name = "multiply")]
    #[pyo3(signature = (other))]
    pub fn binding_multiply(&self, other: pyo3::PyRef<'_, ComplexF64>) -> pyo3::PyResult<Self> {
        Ok(ComplexF64 {
            inner: Some(
                (self.inner.as_ref().ok_or_else(consumed)?)
                    .multiply(other.inner.as_ref().ok_or_else(consumed)?),
            ),
        })
    }
    #[pyo3(name = "divide")]
    #[pyo3(signature = (other))]
    pub fn binding_divide(&self, other: pyo3::PyRef<'_, ComplexF64>) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .divide(other.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| ComplexF64 { inner: Some(value) })
    }
    #[pyo3(name = "exponential")]
    #[pyo3(signature = (mode))]
    pub fn binding_exponential(&self, mode: u32) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).exponential(mode))
            .map(|value| ComplexF64 { inner: Some(value) })
    }
    #[pyo3(name = "argument")]
    #[pyo3(signature = (mode))]
    pub fn binding_argument(&self, mode: u32) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).argument(mode)
    }
}
