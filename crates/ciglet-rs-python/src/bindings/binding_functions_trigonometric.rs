use super::*;
#[pyfunction(name = "trigonometric_sine")]
#[pyo3(signature = (angle, precision))]
pub fn binding_trigonometric_sine(
    angle: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::trigonometric::trigonometric_sine(angle, precision)
}
#[pyfunction(name = "trigonometric_cosine")]
#[pyo3(signature = (angle, precision))]
pub fn binding_trigonometric_cosine(
    angle: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::trigonometric::trigonometric_cosine(angle, precision)
}
#[pyfunction(name = "trigonometric_tangent")]
#[pyo3(signature = (angle, precision))]
pub fn binding_trigonometric_tangent(
    angle: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::trigonometric::trigonometric_tangent(angle, precision)
}
#[pyfunction(name = "trigonometric_tangent_full")]
#[pyo3(signature = (angle, precision))]
pub fn binding_trigonometric_tangent_full(
    angle: f32,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<f32> {
    crate::api::trigonometric::trigonometric_tangent_full(angle, precision)
}
#[pyfunction(name = "trigonometric_sine_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_sine_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_sine_lanes(&angles, precision)
}
#[pyfunction(name = "trigonometric_cosine_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_cosine_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_cosine_lanes(&angles, precision)
}
#[pyfunction(name = "trigonometric_tangent_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_tangent_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_tangent_lanes(&angles, precision)
}
#[pyfunction(name = "trigonometric_sine_full_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_sine_full_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_sine_full_lanes(&angles, precision)
}
#[pyfunction(name = "trigonometric_cosine_full_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_cosine_full_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_cosine_full_lanes(&angles, precision)
}
#[pyfunction(name = "trigonometric_tangent_full_lanes")]
#[pyo3(signature = (angles, precision))]
pub fn binding_trigonometric_tangent_full_lanes(
    angles: Vec<f32>,
    precision: ApproximationPrecision,
) -> pyo3::PyResult<Vec<f32>> {
    crate::api::trigonometric::trigonometric_tangent_full_lanes(&angles, precision)
}
