use super::*;
#[pyfunction(name = "random_uniform_from_draw_f32")]
#[pyo3(signature = (draw, maximum))]
pub fn binding_random_uniform_from_draw_f32(draw: u32, maximum: u32) -> pyo3::PyResult<f32> {
    crate::api::random::random_uniform_from_draw_f32(draw, maximum)
}
#[pyfunction(name = "random_white_from_draw_f32")]
#[pyo3(signature = (amplitude, draw, maximum))]
pub fn binding_random_white_from_draw_f32(
    amplitude: f32,
    draw: u32,
    maximum: u32,
) -> pyo3::PyResult<f32> {
    crate::api::random::random_white_from_draw_f32(amplitude, draw, maximum)
}
#[pyfunction(name = "random_normal_from_draws_f32")]
#[pyo3(signature = (mean, variance, first, second, maximum, code))]
pub fn binding_random_normal_from_draws_f32(
    mean: f32,
    variance: f32,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> pyo3::PyResult<f32> {
    crate::api::random::random_normal_from_draws_f32(mean, variance, first, second, maximum, code)
}
#[pyfunction(name = "random_uniform_from_draw_f64")]
#[pyo3(signature = (draw, maximum))]
pub fn binding_random_uniform_from_draw_f64(draw: u32, maximum: u32) -> pyo3::PyResult<f64> {
    crate::api::random::random_uniform_from_draw_f64(draw, maximum)
}
#[pyfunction(name = "random_white_from_draw_f64")]
#[pyo3(signature = (amplitude, draw, maximum))]
pub fn binding_random_white_from_draw_f64(
    amplitude: f64,
    draw: u32,
    maximum: u32,
) -> pyo3::PyResult<f64> {
    crate::api::random::random_white_from_draw_f64(amplitude, draw, maximum)
}
#[pyfunction(name = "random_normal_from_draws_f64")]
#[pyo3(signature = (mean, variance, first, second, maximum, code))]
pub fn binding_random_normal_from_draws_f64(
    mean: f64,
    variance: f64,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> pyo3::PyResult<f64> {
    crate::api::random::random_normal_from_draws_f64(mean, variance, first, second, maximum, code)
}
