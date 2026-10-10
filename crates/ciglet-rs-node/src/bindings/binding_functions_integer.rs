use super::*;
#[napi(js_name = "integer_sequence")]
pub fn binding_integer_sequence(
    start: i32,
    step: i32,
    count: f64,
) -> napi::Result<napi::bindgen_prelude::Int32Array> {
    (crate::api::vector::integer_sequence(start, step, checked_usize(count)?))
        .map(<napi::bindgen_prelude::Int32Array>::from)
}
