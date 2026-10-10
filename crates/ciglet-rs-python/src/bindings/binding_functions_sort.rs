use super::*;
#[pyfunction(name = "sort_f32")]
#[pyo3(signature = (values))]
pub fn binding_sort_f32(values: Vec<f32>) -> pyo3::PyResult<SortedValuesF32> {
    Ok(SortedValuesF32 {
        inner: Some(crate::api::ordering::sort_f32(&values)),
    })
}
#[pyfunction(name = "sort_f64")]
#[pyo3(signature = (values))]
pub fn binding_sort_f64(values: Vec<f64>) -> pyo3::PyResult<SortedValuesF64> {
    Ok(SortedValuesF64 {
        inner: Some(crate::api::ordering::sort_f64(&values)),
    })
}
