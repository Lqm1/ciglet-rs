use super::*;
#[pyfunction(name = "matrix_apply_swaps_f32")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_f32(
    mut values: Vec<f32>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<f32>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_f32(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_f64")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_f64(
    mut values: Vec<f64>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<f64>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_f64(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_u8")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_u8(
    mut values: Vec<u8>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<u8>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_u8(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_u16")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_u16(
    mut values: Vec<u16>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<u16>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_u16(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_u32")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_u32(
    mut values: Vec<u32>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<u32>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_u32(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_u64")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_u64(
    mut values: Vec<u64>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<u64>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_u64(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_i8")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_i8(
    mut values: Vec<i8>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<i8>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_i8(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_i16")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_i16(
    mut values: Vec<i16>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<i16>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_i16(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_i32")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_i32(
    mut values: Vec<i32>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<i32>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_i32(&mut values, &partners))?;
        Ok(((), values))
    }
}
#[pyfunction(name = "matrix_apply_swaps_i64")]
#[pyo3(signature = (values, partners))]
pub fn binding_matrix_apply_swaps_i64(
    mut values: Vec<i64>,
    partners: Vec<u32>,
) -> pyo3::PyResult<((), Vec<i64>)> {
    {
        (crate::api::matrix::matrix_apply_swaps_i64(&mut values, &partners))?;
        Ok(((), values))
    }
}
