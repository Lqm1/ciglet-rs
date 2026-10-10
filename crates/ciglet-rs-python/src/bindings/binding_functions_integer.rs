use super::*;
#[pyfunction(name = "integer_sequence")]
#[pyo3(signature = (start, step, count))]
pub fn binding_integer_sequence(start: i32, step: i32, count: usize) -> pyo3::PyResult<Vec<i32>> {
    crate::api::vector::integer_sequence(start, step, count)
}
