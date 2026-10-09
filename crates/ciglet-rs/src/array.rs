//! Owned row-major arrays replacing ciglet's raw two-dimensional buffers.
use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array2<T> {
    rows: usize,
    columns: usize,
    values: Vec<T>,
}
fn length(rows: usize, columns: usize) -> Result<usize, Error> {
    rows.checked_mul(columns)
        .ok_or(Error("array dimensions overflow"))
}
fn buffer<T>(length: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error("array allocation failed"))?;
    Ok(values)
}
impl<T> Array2<T> {
    /// Take ownership without copying. Elements use row * columns + column.
    pub fn from_row_major(rows: usize, columns: usize, values: Vec<T>) -> Result<Self, Error> {
        if length(rows, columns)? != values.len() {
            return Err(Error("array buffer does not match its dimensions"));
        }
        Ok(Self {
            rows,
            columns,
            values,
        })
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.columns
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }
    /// Move out the existing flat buffer without allocating.
    pub fn into_row_major(self) -> Vec<T> {
        self.values
    }
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.values.get(row * self.columns + column)
    }
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        self.values.get_mut(row * self.columns + column)
    }
    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.columns;
        Some(&self.values[start..start + self.columns])
    }
    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.columns;
        Some(&mut self.values[start..start + self.columns])
    }
    /// Change dimensions while retaining the same row-major element sequence.
    /// Invalid dimensions leave both the shape and buffer unchanged.
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), Error> {
        if length(rows, columns)? != self.len() {
            return Err(Error("reshaped array must retain its element count"));
        }
        self.rows = rows;
        self.columns = columns;
        Ok(())
    }
    /// Transpose in place by permutation cycles, without requiring Clone.
    /// The visitation allocation is completed before any value is moved.
    pub fn transpose(&mut self) -> Result<(), Error> {
        if self.rows > 1 && self.columns > 1 {
            let mut visited = buffer(self.len())?;
            visited.resize(self.len(), false);
            for start in 0..self.len() {
                if visited[start] {
                    continue;
                }
                let mut current = start;
                loop {
                    visited[current] = true;
                    let next = (current % self.columns) * self.rows + current / self.columns;
                    if next == start {
                        break;
                    }
                    self.values.swap(start, next);
                    current = next;
                }
            }
        }
        std::mem::swap(&mut self.rows, &mut self.columns);
        Ok(())
    }
}
impl<T: Clone> Array2<T> {
    /// Allocate one flat buffer initialized with the supplied value.
    pub fn filled(rows: usize, columns: usize, value: T) -> Result<Self, Error> {
        let n = length(rows, columns)?;
        let mut values = buffer(n)?;
        values.resize(n, value);
        Ok(Self {
            rows,
            columns,
            values,
        })
    }
    /// Copy an existing flat buffer, retaining an independent allocation.
    pub fn from_slice(rows: usize, columns: usize, values: &[T]) -> Result<Self, Error> {
        if length(rows, columns)? != values.len() {
            return Err(Error("array slice does not match its dimensions"));
        }
        let mut copy = buffer(values.len())?;
        copy.extend_from_slice(values);
        Self::from_row_major(rows, columns, copy)
    }
    /// Copy equally sized rows. Empty row collections have shape 0 by 0.
    pub fn from_rows<R: AsRef<[T]>>(rows: &[R]) -> Result<Self, Error> {
        let columns = rows.first().map_or(0, |row| row.as_ref().len());
        if rows.iter().any(|row| row.as_ref().len() != columns) {
            return Err(Error("array rows must have equal lengths"));
        }
        let mut values = buffer(length(rows.len(), columns)?)?;
        for row in rows {
            values.extend_from_slice(row.as_ref());
        }
        Self::from_row_major(rows.len(), columns, values)
    }
    pub fn try_clone(&self) -> Result<Self, Error> {
        Self::from_slice(self.rows, self.columns, &self.values)
    }
    /// Copy all values in row-major order, matching upstream flatten.
    pub fn flatten(&self) -> Result<Vec<T>, Error> {
        Ok(self.try_clone()?.into_row_major())
    }
    /// Return an independent transposed copy, preserving the original array.
    pub fn transposed(&self) -> Result<Self, Error> {
        if self.is_empty() {
            return Self::from_row_major(self.columns, self.rows, Vec::new());
        }
        let mut values = buffer(self.len())?;
        for column in 0..self.columns {
            for row in 0..self.rows {
                values.push(self.values[row * self.columns + column].clone());
            }
        }
        Self::from_row_major(self.columns, self.rows, values)
    }
}
impl<T: Clone + Default> Array2<T> {
    /// Initialize numeric elements to zero, or other types to their defaults.
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, Error> {
        Self::filled(rows, columns, T::default())
    }
}
