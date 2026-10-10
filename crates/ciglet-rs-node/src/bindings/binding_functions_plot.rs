use super::*;
#[napi(js_name = "plot_color_from_character")]
pub fn binding_plot_color_from_character(value: String) -> napi::Result<u32> {
    crate::api::plotting::plot_color_from_character(&value)
}
