use super::*;
#[napi(js_name = "select_nth_f32")]
pub fn binding_select_nth_f32(
    values: napi::bindgen_prelude::Float32Array,
    index: f64,
) -> napi::Result<(f64, napi::bindgen_prelude::Float32Array)> {
    let mut storage_values = values.to_vec();
    {
        let result =
            (crate::api::ordering::select_nth_f32(&mut storage_values, checked_usize(index)?))
                .map(|value| value as f64)?;
        Ok((
            result,
            <napi::bindgen_prelude::Float32Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "select_nth_f64")]
pub fn binding_select_nth_f64(
    values: napi::bindgen_prelude::Float64Array,
    index: f64,
) -> napi::Result<(f64, napi::bindgen_prelude::Float64Array)> {
    let mut storage_values = values.to_vec();
    {
        let result =
            (crate::api::ordering::select_nth_f64(&mut storage_values, checked_usize(index)?))?;
        Ok((
            result,
            <napi::bindgen_prelude::Float64Array>::from(storage_values),
        ))
    }
}
