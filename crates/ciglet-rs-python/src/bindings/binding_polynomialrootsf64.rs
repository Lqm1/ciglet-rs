use super::*;
#[pyclass(module = "ciglet_rs")]
pub struct PolynomialRootsF64 {
    pub(crate) inner: Option<crate::api::PolynomialRootsF64>,
}
#[pymethods]
impl PolynomialRootsF64 {
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
impl PolynomialRootsF64 {
    #[new]
    #[pyo3(signature = (values, iterations, converged, maximum_residual))]
    pub fn binding_new(
        values: pyo3::PyRef<'_, ComplexValuesF64>,
        iterations: usize,
        converged: bool,
        maximum_residual: f64,
    ) -> pyo3::PyResult<Self> {
        Ok(PolynomialRootsF64 {
            inner: Some(crate::api::PolynomialRootsF64::new(
                values.inner.as_ref().ok_or_else(consumed)?,
                iterations,
                converged,
                maximum_residual,
            )),
        })
    }
    #[getter(values)]
    pub fn binding_values(&self) -> pyo3::PyResult<ComplexValuesF64> {
        Ok(ComplexValuesF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).values()),
        })
    }
    #[setter(values)]
    pub fn binding_set_values(
        &mut self,
        values: pyo3::PyRef<'_, ComplexValuesF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_values(values.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[getter(iterations)]
    pub fn binding_iterations(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).iterations())
    }
    #[setter(iterations)]
    pub fn binding_set_iterations(&mut self, value: usize) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_iterations(value);
        Ok(())
    }
    #[getter(converged)]
    pub fn binding_converged(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).converged())
    }
    #[setter(converged)]
    pub fn binding_set_converged(&mut self, value: bool) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_converged(value);
        Ok(())
    }
    #[getter(maximum_residual)]
    pub fn binding_maximum_residual(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).maximum_residual())
    }
    #[setter(maximum_residual)]
    pub fn binding_set_maximum_residual(&mut self, value: f64) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_maximum_residual(value);
        Ok(())
    }
    #[pyo3(name = "residual_storage")]
    #[pyo3(signature = ())]
    pub fn binding_residual_storage(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).residual_storage())
    }
    #[pyo3(name = "set_residual_storage")]
    #[pyo3(signature = (values))]
    pub fn binding_set_residual_storage(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_residual_storage(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PolynomialRootsF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
