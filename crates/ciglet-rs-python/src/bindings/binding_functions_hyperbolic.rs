use super::*;
#[pyfunction(name = "hyperbolic_sine")]
#[pyo3(signature = (value, precision))]
pub fn binding_hyperbolic_sine(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::hyperbolic::hyperbolic_sine(value, precision)
}
#[pyfunction(name = "hyperbolic_sine_lanes")]
#[pyo3(signature = (values, precision))]
pub fn binding_hyperbolic_sine_lanes(
    values: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::hyperbolic::hyperbolic_sine_lanes(&values, precision)
}
#[pyfunction(name = "hyperbolic_cosine")]
#[pyo3(signature = (value, precision))]
pub fn binding_hyperbolic_cosine(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::hyperbolic::hyperbolic_cosine(value, precision)
}
#[pyfunction(name = "hyperbolic_cosine_lanes")]
#[pyo3(signature = (values, precision))]
pub fn binding_hyperbolic_cosine_lanes(
    values: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::hyperbolic::hyperbolic_cosine_lanes(&values, precision)
}
#[pyfunction(name = "hyperbolic_tangent")]
#[pyo3(signature = (value, precision))]
pub fn binding_hyperbolic_tangent(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::hyperbolic::hyperbolic_tangent(value, precision)
}
#[pyfunction(name = "hyperbolic_tangent_lanes")]
#[pyo3(signature = (values, precision))]
pub fn binding_hyperbolic_tangent_lanes(
    values: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::hyperbolic::hyperbolic_tangent_lanes(&values, precision)
}
#[pyfunction(name = "hyperbolic_sigmoid")]
#[pyo3(signature = (value, precision))]
pub fn binding_hyperbolic_sigmoid(
    value: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::hyperbolic::hyperbolic_sigmoid(value, precision)
}
#[pyfunction(name = "hyperbolic_sigmoid_lanes")]
#[pyo3(signature = (values, precision))]
pub fn binding_hyperbolic_sigmoid_lanes(
    values: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::hyperbolic::hyperbolic_sigmoid_lanes(&values, precision)
}
