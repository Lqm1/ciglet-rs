use crate::matrix::{self, Lu, Matrix};
use napi::Error as BindingError;
fn swaps(values: &[u32]) -> Vec<usize> {
    values.iter().map(|&value| value as usize).collect()
}
/// Complete native finite column-major matrix and row operations.
pub struct MatrixF32 {
    matrix: Matrix<f32>,
}
impl MatrixF32 {
    pub fn new(rows: usize, columns: usize, values: Vec<f32>) -> Result<Self, BindingError> {
        Matrix::from_column_major(rows, columns, values)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Matrix::zeros(rows, columns)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn identity(size: usize) -> Result<Self, BindingError> {
        Matrix::identity(size)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.matrix.rows()
    }
    pub fn columns(&self) -> usize {
        self.matrix.columns()
    }
    pub fn as_slice(&self) -> Vec<f32> {
        self.matrix.as_slice().to_vec()
    }
    pub fn into_column_major(self) -> Vec<f32> {
        self.matrix.into_column_major()
    }
    pub fn get(&self, row: usize, column: usize) -> Option<f32> {
        self.matrix.get(row, column).copied()
    }
    pub fn apply_row_swaps(&mut self, partners: &[u32]) -> Result<(), BindingError> {
        self.matrix
            .apply_row_swaps(&swaps(partners))
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn pivot_rows(&mut self) -> Result<Vec<u32>, BindingError> {
        self.matrix
            .pivot_rows()
            .map(|values| values.into_iter().map(|v| v as u32).collect())
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn multiply(&self, right: &Self) -> Result<Self, BindingError> {
        self.matrix
            .multiply(&right.matrix)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn multiply_vector(&self, values: &[f32]) -> Result<Vec<f32>, BindingError> {
        self.matrix
            .multiply_vector(values)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
        }
    }
}
/// Full packed LU factors and ordered swaps, including robust pivoting.
pub struct LuF32 {
    lu: Lu<f32>,
}
impl LuF32 {
    /// Consume the matrix exactly as native Lu::factor does.
    pub fn factor(matrix: MatrixF32) -> Result<Self, BindingError> {
        Lu::factor(matrix.matrix)
            .map(|lu| Self { lu })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn factor_unpivoted(matrix: MatrixF32) -> Result<Self, BindingError> {
        Lu::factor_unpivoted(matrix.matrix)
            .map(|lu| Self { lu })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn packed(&self) -> MatrixF32 {
        MatrixF32 {
            matrix: self.lu.packed().clone(),
        }
    }
    pub fn row_swaps(&self) -> Vec<u32> {
        self.lu.row_swaps().iter().map(|&v| v as u32).collect()
    }
    pub fn solve(&self, right: &[f32]) -> Result<Vec<f32>, BindingError> {
        self.lu
            .solve(right)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            lu: self.lu.clone(),
        }
    }
}
/// Complete native finite column-major matrix and row operations.
pub struct MatrixF64 {
    matrix: Matrix<f64>,
}
impl MatrixF64 {
    pub fn new(rows: usize, columns: usize, values: Vec<f64>) -> Result<Self, BindingError> {
        Matrix::from_column_major(rows, columns, values)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Matrix::zeros(rows, columns)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn identity(size: usize) -> Result<Self, BindingError> {
        Matrix::identity(size)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.matrix.rows()
    }
    pub fn columns(&self) -> usize {
        self.matrix.columns()
    }
    pub fn as_slice(&self) -> Vec<f64> {
        self.matrix.as_slice().to_vec()
    }
    pub fn into_column_major(self) -> Vec<f64> {
        self.matrix.into_column_major()
    }
    pub fn get(&self, row: usize, column: usize) -> Option<f64> {
        self.matrix.get(row, column).copied()
    }
    pub fn apply_row_swaps(&mut self, partners: &[u32]) -> Result<(), BindingError> {
        self.matrix
            .apply_row_swaps(&swaps(partners))
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn pivot_rows(&mut self) -> Result<Vec<u32>, BindingError> {
        self.matrix
            .pivot_rows()
            .map(|values| values.into_iter().map(|v| v as u32).collect())
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn multiply(&self, right: &Self) -> Result<Self, BindingError> {
        self.matrix
            .multiply(&right.matrix)
            .map(|matrix| Self { matrix })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn multiply_vector(&self, values: &[f64]) -> Result<Vec<f64>, BindingError> {
        self.matrix
            .multiply_vector(values)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
        }
    }
}
/// Full packed LU factors and ordered swaps, including robust pivoting.
pub struct LuF64 {
    lu: Lu<f64>,
}
impl LuF64 {
    /// Consume the matrix exactly as native Lu::factor does.
    pub fn factor(matrix: MatrixF64) -> Result<Self, BindingError> {
        Lu::factor(matrix.matrix)
            .map(|lu| Self { lu })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn factor_unpivoted(matrix: MatrixF64) -> Result<Self, BindingError> {
        Lu::factor_unpivoted(matrix.matrix)
            .map(|lu| Self { lu })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn packed(&self) -> MatrixF64 {
        MatrixF64 {
            matrix: self.lu.packed().clone(),
        }
    }
    pub fn row_swaps(&self) -> Vec<u32> {
        self.lu.row_swaps().iter().map(|&v| v as u32).collect()
    }
    pub fn solve(&self, right: &[f64]) -> Result<Vec<f64>, BindingError> {
        self.lu
            .solve(right)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Self {
        Self {
            lu: self.lu.clone(),
        }
    }
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_f32(values: &mut [f32], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_f64(values: &mut [f64], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_u8(values: &mut [u8], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_u16(values: &mut [u16], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_u32(values: &mut [u32], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_u64(values: &mut [u64], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_i8(values: &mut [i8], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_i16(values: &mut [i16], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_i32(values: &mut [i32], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
/// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
pub fn matrix_apply_swaps_i64(values: &mut [i64], partners: &[u32]) -> Result<(), BindingError> {
    matrix::apply_swaps(values, &swaps(partners)).map_err(|error| napi::Error::from_reason(error.0))
}
