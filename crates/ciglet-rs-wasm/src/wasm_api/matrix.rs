use crate::matrix::{self, Lu, Matrix};
use wasm_bindgen::prelude::*;

fn swaps(values: &[u32]) -> Vec<usize> {
    values.iter().map(|&value| value as usize).collect()
}

macro_rules! matrix_api {
    ($scalar:ty, $matrix:ident, $lu:ident) => {
        /// Complete native finite column-major matrix and row operations.
        #[wasm_bindgen]
        pub struct $matrix {
            matrix: Matrix<$scalar>,
        }
        #[wasm_bindgen]
        impl $matrix {
            #[wasm_bindgen(constructor)]
            pub fn new(rows: usize, columns: usize, values: Vec<$scalar>) -> Result<Self, JsValue> {
                Matrix::from_column_major(rows, columns, values)
                    .map(|matrix| Self { matrix })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn zeros(rows: usize, columns: usize) -> Result<Self, JsValue> {
                Matrix::zeros(rows, columns)
                    .map(|matrix| Self { matrix })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn identity(size: usize) -> Result<Self, JsValue> {
                Matrix::identity(size)
                    .map(|matrix| Self { matrix })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn rows(&self) -> usize {
                self.matrix.rows()
            }
            #[wasm_bindgen(getter)]
            pub fn columns(&self) -> usize {
                self.matrix.columns()
            }
            pub fn as_slice(&self) -> Vec<$scalar> {
                self.matrix.as_slice().to_vec()
            }
            pub fn into_column_major(self) -> Vec<$scalar> {
                self.matrix.into_column_major()
            }
            pub fn get(&self, row: usize, column: usize) -> Option<$scalar> {
                self.matrix.get(row, column).copied()
            }
            pub fn apply_row_swaps(&mut self, partners: &[u32]) -> Result<(), JsValue> {
                self.matrix
                    .apply_row_swaps(&swaps(partners))
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn pivot_rows(&mut self) -> Result<Vec<u32>, JsValue> {
                self.matrix
                    .pivot_rows()
                    .map(|values| values.into_iter().map(|v| v as u32).collect())
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn multiply(&self, right: &Self) -> Result<Self, JsValue> {
                self.matrix
                    .multiply(&right.matrix)
                    .map(|matrix| Self { matrix })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn multiply_vector(&self, values: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
                self.matrix
                    .multiply_vector(values)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                Self {
                    matrix: self.matrix.clone(),
                }
            }
        }
        /// Full packed LU factors and ordered swaps, including robust pivoting.
        #[wasm_bindgen]
        pub struct $lu {
            lu: Lu<$scalar>,
        }
        #[wasm_bindgen]
        impl $lu {
            /// Consume the matrix exactly as native Lu::factor does.
            pub fn factor(matrix: $matrix) -> Result<Self, JsValue> {
                Lu::factor(matrix.matrix)
                    .map(|lu| Self { lu })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn factor_unpivoted(matrix: $matrix) -> Result<Self, JsValue> {
                Lu::factor_unpivoted(matrix.matrix)
                    .map(|lu| Self { lu })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn packed(&self) -> $matrix {
                $matrix {
                    matrix: self.lu.packed().clone(),
                }
            }
            pub fn row_swaps(&self) -> Vec<u32> {
                self.lu.row_swaps().iter().map(|&v| v as u32).collect()
            }
            pub fn solve(&self, right: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
                self.lu
                    .solve(right)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Self {
                Self {
                    lu: self.lu.clone(),
                }
            }
        }
    };
}
matrix_api!(f32, MatrixF32, LuF32);
matrix_api!(f64, MatrixF64, LuF64);

macro_rules! vector_swaps_api {
    ($scalar:ty, $name:ident) => {
        /// Ordered in-place swaps, retaining arbitrary bits and validating before writes.
        #[wasm_bindgen]
        pub fn $name(values: &mut [$scalar], partners: &[u32]) -> Result<(), JsValue> {
            matrix::apply_swaps(values, &swaps(partners))
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
vector_swaps_api!(f32, matrix_apply_swaps_f32);
vector_swaps_api!(f64, matrix_apply_swaps_f64);
vector_swaps_api!(u8, matrix_apply_swaps_u8);
vector_swaps_api!(u16, matrix_apply_swaps_u16);
vector_swaps_api!(u32, matrix_apply_swaps_u32);
vector_swaps_api!(u64, matrix_apply_swaps_u64);
vector_swaps_api!(i8, matrix_apply_swaps_i8);
vector_swaps_api!(i16, matrix_apply_swaps_i16);
vector_swaps_api!(i32, matrix_apply_swaps_i32);
vector_swaps_api!(i64, matrix_apply_swaps_i64);
