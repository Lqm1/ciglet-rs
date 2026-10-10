use super::*;
#[napi(js_name = "lanes_exponential2")]
pub fn binding_lanes_exponential2(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_exponential2(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_exponential2_fast")]
pub fn binding_lanes_exponential2_fast(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_exponential2_fast(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_exponential")]
pub fn binding_lanes_exponential(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_exponential(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_exponential_fast")]
pub fn binding_lanes_exponential_fast(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_exponential_fast(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_logarithm2")]
pub fn binding_lanes_logarithm2(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_logarithm2(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_logarithm2_fast")]
pub fn binding_lanes_logarithm2_fast(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_logarithm2_fast(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_logarithm")]
pub fn binding_lanes_logarithm(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_logarithm(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_logarithm_fast")]
pub fn binding_lanes_logarithm_fast(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_logarithm_fast(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_power")]
pub fn binding_lanes_power(
    bases: napi::bindgen_prelude::Float32Array,
    exponents: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_bases = bases.to_vec();
    let storage_exponents = exponents.to_vec();
    (crate::api::lanes::lanes_power(&storage_bases, &storage_exponents))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_power_fast")]
pub fn binding_lanes_power_fast(
    bases: napi::bindgen_prelude::Float32Array,
    exponents: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_bases = bases.to_vec();
    let storage_exponents = exponents.to_vec();
    (crate::api::lanes::lanes_power_fast(&storage_bases, &storage_exponents))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_to_integers")]
pub fn binding_lanes_to_integers(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Int32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_to_integers(&storage_values))
        .map(<napi::bindgen_prelude::Int32Array>::from)
}
#[napi(js_name = "lanes_from_integers")]
pub fn binding_lanes_from_integers(
    values: napi::bindgen_prelude::Int32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_from_integers(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_to_bits")]
pub fn binding_lanes_to_bits(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_to_bits(&storage_values))
        .map(<napi::bindgen_prelude::Uint32Array>::from)
}
#[napi(js_name = "lanes_from_bits")]
pub fn binding_lanes_from_bits(
    values: napi::bindgen_prelude::Uint32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_from_bits(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_absolute")]
pub fn binding_lanes_absolute(
    values: napi::bindgen_prelude::Float32Array,
) -> napi::Result<napi::bindgen_prelude::Float32Array> {
    let storage_values = values.to_vec();
    (crate::api::lanes::lanes_absolute(&storage_values))
        .map(<napi::bindgen_prelude::Float32Array>::from)
}
#[napi(js_name = "lanes_from_integer64")]
pub fn binding_lanes_from_integer64(
    first: napi::bindgen_prelude::BigInt,
    second: napi::bindgen_prelude::BigInt,
) -> napi::Result<napi::bindgen_prelude::Int32Array> {
    Ok(<napi::bindgen_prelude::Int32Array>::from(
        crate::api::lanes::lanes_from_integer64(checked_i64(&first)?, checked_i64(&second)?),
    ))
}
