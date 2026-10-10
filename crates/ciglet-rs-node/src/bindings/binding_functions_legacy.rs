use super::*;
#[napi(js_name = "legacy_infinity_sentinel")]
pub fn binding_legacy_infinity_sentinel() -> napi::Result<f64> {
    Ok(crate::api::approximation::legacy_infinity_sentinel())
}
#[napi(js_name = "legacy_epsilon")]
pub fn binding_legacy_epsilon() -> napi::Result<f64> {
    Ok(crate::api::approximation::legacy_epsilon())
}
