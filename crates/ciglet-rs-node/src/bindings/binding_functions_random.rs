use super::*;
#[napi(js_name = "random_uniform_from_draw_f32")]
pub fn binding_random_uniform_from_draw_f32(draw: u32, maximum: u32) -> napi::Result<f64> {
    (crate::api::random::random_uniform_from_draw_f32(draw, maximum)).map(|value| value as f64)
}
#[napi(js_name = "random_white_from_draw_f32")]
pub fn binding_random_white_from_draw_f32(
    amplitude: f64,
    draw: u32,
    maximum: u32,
) -> napi::Result<f64> {
    (crate::api::random::random_white_from_draw_f32(amplitude as f32, draw, maximum))
        .map(|value| value as f64)
}
#[napi(js_name = "random_normal_from_draws_f32")]
pub fn binding_random_normal_from_draws_f32(
    mean: f64,
    variance: f64,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> napi::Result<f64> {
    (crate::api::random::random_normal_from_draws_f32(
        mean as f32,
        variance as f32,
        first,
        second,
        maximum,
        code,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "random_uniform_from_draw_f64")]
pub fn binding_random_uniform_from_draw_f64(draw: u32, maximum: u32) -> napi::Result<f64> {
    crate::api::random::random_uniform_from_draw_f64(draw, maximum)
}
#[napi(js_name = "random_white_from_draw_f64")]
pub fn binding_random_white_from_draw_f64(
    amplitude: f64,
    draw: u32,
    maximum: u32,
) -> napi::Result<f64> {
    crate::api::random::random_white_from_draw_f64(amplitude, draw, maximum)
}
#[napi(js_name = "random_normal_from_draws_f64")]
pub fn binding_random_normal_from_draws_f64(
    mean: f64,
    variance: f64,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> napi::Result<f64> {
    crate::api::random::random_normal_from_draws_f64(mean, variance, first, second, maximum, code)
}
