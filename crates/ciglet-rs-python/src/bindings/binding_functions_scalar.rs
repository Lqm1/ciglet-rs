use super::*;
#[pyfunction(name = "scalar_minimum_f32")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_minimum_f32(first: f32, second: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::approximation::scalar_minimum_f32(first, second))
}
#[pyfunction(name = "scalar_maximum_f32")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_maximum_f32(first: f32, second: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::approximation::scalar_maximum_f32(first, second))
}
#[pyfunction(name = "scalar_minimum_f64")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_minimum_f64(first: f64, second: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::approximation::scalar_minimum_f64(first, second))
}
#[pyfunction(name = "scalar_maximum_f64")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_maximum_f64(first: f64, second: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::approximation::scalar_maximum_f64(first, second))
}
#[pyfunction(name = "scalar_math_constants")]
#[pyo3(signature = ())]
pub fn binding_scalar_math_constants() -> pyo3::PyResult<Vec<f64>> {
    Ok(crate::api::scalar::scalar_math_constants())
}
#[pyfunction(name = "scalar_add_f32")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_add_f32(first: f32, second: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::scalar::scalar_add_f32(first, second))
}
#[pyfunction(name = "scalar_accumulate_square_f32")]
#[pyo3(signature = (accumulator, value))]
pub fn binding_scalar_accumulate_square_f32(accumulator: f32, value: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::scalar::scalar_accumulate_square_f32(
        accumulator,
        value,
    ))
}
#[pyfunction(name = "scalar_fold_f32")]
#[pyo3(signature = (values, initial, callback, context))]
pub fn binding_scalar_fold_f32(
    values: Vec<f32>,
    initial: f32,
    callback: Option<Py<PyAny>>,
    context: Py<PyAny>,
) -> pyo3::PyResult<f32> {
    crate::api::scalar::scalar_fold_f32(
        &values,
        initial,
        callback.map(crate::callbacks::Function::new).transpose()?,
        &crate::callbacks::Value::new(context),
    )
}
#[pyfunction(name = "scalar_add_f64")]
#[pyo3(signature = (first, second))]
pub fn binding_scalar_add_f64(first: f64, second: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::scalar::scalar_add_f64(first, second))
}
#[pyfunction(name = "scalar_accumulate_square_f64")]
#[pyo3(signature = (accumulator, value))]
pub fn binding_scalar_accumulate_square_f64(accumulator: f64, value: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::scalar::scalar_accumulate_square_f64(
        accumulator,
        value,
    ))
}
#[pyfunction(name = "scalar_fold_f64")]
#[pyo3(signature = (values, initial, callback, context))]
pub fn binding_scalar_fold_f64(
    values: Vec<f64>,
    initial: f64,
    callback: Option<Py<PyAny>>,
    context: Py<PyAny>,
) -> pyo3::PyResult<f64> {
    crate::api::scalar::scalar_fold_f64(
        &values,
        initial,
        callback.map(crate::callbacks::Function::new).transpose()?,
        &crate::callbacks::Value::new(context),
    )
}
