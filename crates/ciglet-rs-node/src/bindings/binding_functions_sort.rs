use super::*;
#[napi(js_name = "sort_f32")]
pub fn binding_sort_f32(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<SortedValuesF32> {
    let storage_values = values.to_vec();
    Ok(SortedValuesF32 {
        inner: std::cell::RefCell::new(Some(crate::api::ordering::sort_f32(&storage_values))),
    })
}
#[napi(js_name = "sort_f64")]
pub fn binding_sort_f64(
    values: napi::bindgen_prelude::Float64Array,
) -> napi::Result<SortedValuesF64> {
    let storage_values = values.to_vec();
    Ok(SortedValuesF64 {
        inner: std::cell::RefCell::new(Some(crate::api::ordering::sort_f64(&storage_values))),
    })
}
