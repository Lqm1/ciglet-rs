//! Owned Python callback references. Python exceptions retain their identity.
use pyo3::prelude::*;
use pyo3::types::PyFloat;

pub(crate) struct Value {
    inner: Py<PyAny>,
}
impl Value {
    pub(crate) fn new(inner: Py<PyAny>) -> Self {
        Self { inner }
    }
    pub(crate) fn number(value: f64) -> Self {
        Python::attach(|py| Self {
            inner: PyFloat::new(py, value).into_any().unbind(),
        })
    }
    pub(crate) fn as_f64(&self) -> Option<f64> {
        Python::attach(|py| self.inner.bind(py).extract::<f64>().ok())
    }
}
pub(crate) struct Function {
    callable: Py<PyAny>,
}
impl Clone for Function {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            callable: self.callable.clone_ref(py),
        })
    }
}
impl Function {
    pub(crate) fn new(callable: Py<PyAny>) -> PyResult<Self> {
        Python::attach(|py| {
            if !callable.bind(py).is_callable() {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "callback must be callable",
                ));
            }
            Ok(Self { callable })
        })
    }
    pub(crate) fn into_callable(self) -> Py<PyAny> {
        self.callable
    }
    pub(crate) fn call0(&self, _receiver: &()) -> PyResult<Value> {
        Python::attach(|py| {
            self.callable
                .bind(py)
                .call0()
                .map(|value| Value::new(value.unbind()))
        })
    }
    pub(crate) fn call1(&self, _receiver: &(), value: &Value) -> PyResult<Value> {
        Python::attach(|py| {
            self.callable
                .bind(py)
                .call1((value.inner.bind(py),))
                .map(|value| Value::new(value.unbind()))
        })
    }
    pub(crate) fn call3(
        &self,
        _receiver: &(),
        first: &Value,
        second: &Value,
        context: &Value,
    ) -> PyResult<Value> {
        Python::attach(|py| {
            self.callable
                .bind(py)
                .call1((
                    first.inner.bind(py),
                    second.inner.bind(py),
                    context.inner.bind(py),
                ))
                .map(|value| Value::new(value.unbind()))
        })
    }
}
