//! Owned column-major matrices and checked row exchanges.
mod lu;
pub use lu::Lu;

use crate::Error;
use rustfft::FftNum;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    columns: usize,
    values: Vec<T>,
}

fn shape(rows: usize, columns: usize) -> Result<usize, Error> {
    rows.checked_mul(columns)
        .filter(|&length| {
            rows <= i32::MAX as usize && columns <= i32::MAX as usize && length <= i32::MAX as usize
        })
        .ok_or(Error("matrix dimensions exceed the supported range"))
}

fn finite<T: FftNum + Into<f64>>(values: &[T]) -> Result<(), Error> {
    if values.iter().any(|&value| !value.into().is_finite()) {
        return Err(Error("matrix arithmetic requires finite values"));
    }
    Ok(())
}

fn buffer<T: FftNum>(length: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("matrix allocation failed"))?;
    values.resize(length, T::zero());
    Ok(values)
}

fn validate_swaps(length: usize, swaps: &[usize]) -> Result<(), Error> {
    if swaps.len() != length || swaps.iter().any(|&row| row >= length) {
        return Err(Error("row swaps require one in-bounds partner per row"));
    }
    Ok(())
}

/// Apply successive swaps `(i, swaps[i])` in their listed order.
/// Validate the complete plan before modifying the vector.
pub fn apply_swaps<T>(values: &mut [T], swaps: &[usize]) -> Result<(), Error> {
    validate_swaps(values.len(), swaps)?;
    for (row, &partner) in swaps.iter().enumerate() {
        values.swap(row, partner);
    }
    Ok(())
}

impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.columns
    }
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }
    pub fn into_column_major(self) -> Vec<T> {
        self.values
    }
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.values.get(row + column * self.rows)
    }
    fn swap_rows(&mut self, first: usize, second: usize) {
        for column in 0..self.columns {
            self.values
                .swap(first + column * self.rows, second + column * self.rows);
        }
    }
}

impl<T: FftNum + Into<f64>> Matrix<T> {
    /// Own a finite column-major buffer. A(row, column) is row + column * rows.
    pub fn from_column_major(rows: usize, columns: usize, values: Vec<T>) -> Result<Self, Error> {
        if shape(rows, columns)? != values.len() {
            return Err(Error("matrix buffer does not match its dimensions"));
        }
        finite(&values)?;
        Ok(Self {
            rows,
            columns,
            values,
        })
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, Error> {
        Ok(Self {
            rows,
            columns,
            values: buffer(shape(rows, columns)?)?,
        })
    }
    pub fn identity(size: usize) -> Result<Self, Error> {
        let mut matrix = Self::zeros(size, size)?;
        for i in 0..size {
            matrix.values[i + i * size] = T::one();
        }
        Ok(matrix)
    }
    /// Apply ciglet's ordered row-swap plan, with the correct rectangular stride.
    pub fn apply_row_swaps(&mut self, swaps: &[usize]) -> Result<(), Error> {
        validate_swaps(self.rows, swaps)?;
        for (row, &partner) in swaps.iter().enumerate() {
            self.swap_rows(row, partner);
        }
        Ok(())
    }
    /// Source-compatible preliminary column pivoting on a square matrix.
    /// [`Lu::factor`] pivots after each elimination step for robust solving.
    pub fn pivot_rows(&mut self) -> Result<Vec<usize>, Error> {
        if self.rows != self.columns {
            return Err(Error("preliminary pivoting requires a square matrix"));
        }
        let mut swaps = Vec::new();
        swaps
            .try_reserve_exact(self.rows)
            .map_err(|_| Error("matrix pivot allocation failed"))?;
        for column in 0..self.columns {
            let mut best = column;
            let mut largest = 0.0;
            for row in column..self.rows {
                let value = self.values[row + column * self.rows].into().abs();
                if value > largest {
                    largest = value;
                    best = row;
                }
            }
            swaps.push(best);
            self.swap_rows(column, best);
        }
        Ok(swaps)
    }
    /// Matrix multiplication, preserving C's column/inner/row accumulation order.
    pub fn multiply(&self, right: &Self) -> Result<Self, Error> {
        if self.columns != right.rows {
            return Err(Error("matrix product inner dimensions differ"));
        }
        let mut product = Self::zeros(self.rows, right.columns)?;
        for column in 0..right.columns {
            for inner in 0..self.columns {
                let weight = right.values[inner + column * right.rows];
                for row in 0..self.rows {
                    let index = row + column * self.rows;
                    product.values[index] =
                        product.values[index] + weight * self.values[row + inner * self.rows];
                }
            }
        }
        finite(&product.values)?;
        Ok(product)
    }
    /// Rectangular matrix-vector product. Output length is the number of rows.
    pub fn multiply_vector(&self, vector: &[T]) -> Result<Vec<T>, Error> {
        if self.columns != vector.len() {
            return Err(Error("matrix-vector dimensions differ"));
        }
        finite(vector)?;
        let mut output = buffer(self.rows)?;
        for (column, &weight) in vector.iter().enumerate() {
            for (row, value) in output.iter_mut().enumerate() {
                *value = *value + weight * self.values[row + column * self.rows];
            }
        }
        finite(&output)?;
        Ok(output)
    }
}
