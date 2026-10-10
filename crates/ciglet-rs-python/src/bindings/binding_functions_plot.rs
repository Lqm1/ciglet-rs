use super::*;
#[pyfunction(name = "plot_color_from_character")]
#[pyo3(signature = (value))]
pub fn binding_plot_color_from_character(value: &str) -> pyo3::PyResult<u32> {
    crate::api::plotting::plot_color_from_character(value)
}
