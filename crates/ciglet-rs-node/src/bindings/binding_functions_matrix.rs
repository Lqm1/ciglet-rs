use super::*;
#[napi(js_name = "matrix_apply_swaps_f32")]
pub fn binding_matrix_apply_swaps_f32(
    values: napi::bindgen_prelude::Float32Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Float32Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_f32(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Float32Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_f64")]
pub fn binding_matrix_apply_swaps_f64(
    values: napi::bindgen_prelude::Float64Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Float64Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_f64(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Float64Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_u8")]
pub fn binding_matrix_apply_swaps_u8(
    values: napi::bindgen_prelude::Uint8Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Uint8Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_u8(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Uint8Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_u16")]
pub fn binding_matrix_apply_swaps_u16(
    values: napi::bindgen_prelude::Uint16Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Uint16Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_u16(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Uint16Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_u32")]
pub fn binding_matrix_apply_swaps_u32(
    values: napi::bindgen_prelude::Uint32Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Uint32Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_u32(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Uint32Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_u64")]
pub fn binding_matrix_apply_swaps_u64(
    values: napi::bindgen_prelude::BigUint64Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::BigUint64Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_u64(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::BigUint64Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_i8")]
pub fn binding_matrix_apply_swaps_i8(
    values: napi::bindgen_prelude::Int8Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Int8Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_i8(&mut storage_values, &storage_partners))?;
        Ok(((), <napi::bindgen_prelude::Int8Array>::from(storage_values)))
    }
}
#[napi(js_name = "matrix_apply_swaps_i16")]
pub fn binding_matrix_apply_swaps_i16(
    values: napi::bindgen_prelude::Int16Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Int16Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_i16(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Int16Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_i32")]
pub fn binding_matrix_apply_swaps_i32(
    values: napi::bindgen_prelude::Int32Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::Int32Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_i32(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::Int32Array>::from(storage_values),
        ))
    }
}
#[napi(js_name = "matrix_apply_swaps_i64")]
pub fn binding_matrix_apply_swaps_i64(
    values: napi::bindgen_prelude::BigInt64Array,
    partners: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<((), napi::bindgen_prelude::BigInt64Array)> {
    let mut storage_values = values.to_vec();
    let storage_partners = partners.to_vec();
    {
        (crate::api::matrix::matrix_apply_swaps_i64(&mut storage_values, &storage_partners))?;
        Ok((
            (),
            <napi::bindgen_prelude::BigInt64Array>::from(storage_values),
        ))
    }
}
