use super::*;
#[pyfunction(name = "select_nth_f32")]
#[pyo3(signature = (values, index))]
pub fn binding_select_nth_f32(
    mut values: Vec<f32>,
    index: usize,
) -> pyo3::PyResult<(f32, Vec<f32>)> {
    {
        let result = (crate::api::ordering::select_nth_f32(&mut values, index))?;
        Ok((result, values))
    }
}
#[pyfunction(name = "select_nth_f64")]
#[pyo3(signature = (values, index))]
pub fn binding_select_nth_f64(
    mut values: Vec<f64>,
    index: usize,
) -> pyo3::PyResult<(f64, Vec<f64>)> {
    {
        let result = (crate::api::ordering::select_nth_f64(&mut values, index))?;
        Ok((result, values))
    }
}
