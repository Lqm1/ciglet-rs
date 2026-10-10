use crate::array::Array2;
use napi::Error as BindingError;
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2U8 {
    pub(super) value: Array2<u8>,
}
impl Array2U8 {
    pub fn new(rows: usize, columns: usize, values: &[u8]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: u8) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[u8], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<u8> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[u8]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<u8> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: u8) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<u8>> {
        self.value.row(row).map(<[u8]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[u8]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<u8>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<u8> {
        self.value.into_row_major()
    }
}
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2U16 {
    pub(super) value: Array2<u16>,
}
impl Array2U16 {
    pub fn new(rows: usize, columns: usize, values: &[u16]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: u16) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[u16], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<u16> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[u16]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<u16> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: u16) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<u16>> {
        self.value.row(row).map(<[u16]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[u16]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<u16>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<u16> {
        self.value.into_row_major()
    }
}
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2U32 {
    pub(super) value: Array2<u32>,
}
impl Array2U32 {
    pub fn new(rows: usize, columns: usize, values: &[u32]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: u32) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[u32], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<u32> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[u32]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<u32> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: u32) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<u32>> {
        self.value.row(row).map(<[u32]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[u32]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<u32>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<u32> {
        self.value.into_row_major()
    }
}
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2U64 {
    pub(super) value: Array2<u64>,
}
impl Array2U64 {
    pub fn new(rows: usize, columns: usize, values: &[u64]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: u64) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[u64], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<u64> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[u64]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<u64> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: u64) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<u64>> {
        self.value.row(row).map(<[u64]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[u64]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<u64>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<u64> {
        self.value.into_row_major()
    }
}
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2F32 {
    pub(super) value: Array2<f32>,
}
impl Array2F32 {
    pub fn new(rows: usize, columns: usize, values: &[f32]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: f32) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[f32], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<f32> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<f32> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: f32) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<f32>> {
        self.value.row(row).map(<[f32]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[f32]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<f32>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<f32> {
        self.value.into_row_major()
    }
}
/// Complete owned row-major storage in the selected numeric element type.
pub struct Array2F64 {
    pub(super) value: Array2<f64>,
}
impl Array2F64 {
    pub fn new(rows: usize, columns: usize, values: &[f64]) -> Result<Self, BindingError> {
        Array2::from_slice(rows, columns, values)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn zeros(rows: usize, columns: usize) -> Result<Self, BindingError> {
        Array2::zeros(rows, columns)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn filled(rows: usize, columns: usize, element: f64) -> Result<Self, BindingError> {
        Array2::filled(rows, columns, element)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Reconstruct complete equally sized rows from packed typed storage.
    /// Row lengths make empty rows distinct from an empty row collection.
    pub fn from_rows(values: &[f64], row_lengths: &[u32]) -> Result<Self, BindingError> {
        let mut rows = Vec::new();
        rows.try_reserve_exact(row_lengths.len())
            .map_err(|_| napi::Error::from_reason("array row allocation failed"))?;
        let mut remaining = values;
        for &length in row_lengths {
            let length = length as usize;
            if length > remaining.len() {
                return Err(napi::Error::from_reason("array row storage is truncated"));
            }
            let (row, tail) = remaining.split_at(length);
            rows.push(row);
            remaining = tail;
        }
        if !remaining.is_empty() {
            return Err(napi::Error::from_reason(
                "array row storage has trailing values",
            ));
        }
        Array2::from_rows(&rows)
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn rows(&self) -> usize {
        self.value.rows()
    }
    pub fn columns(&self) -> usize {
        self.value.columns()
    }
    pub fn length(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn values(&self) -> Vec<f64> {
        self.value.as_slice().to_vec()
    }
    /// Replace mutable native storage without changing its shape.
    pub fn set_values(&mut self, values: &[f64]) -> Result<(), BindingError> {
        if values.len() != self.value.len() {
            return Err(napi::Error::from_reason(
                "array replacement must retain its length",
            ));
        }
        self.value.as_mut_slice().copy_from_slice(values);
        Ok(())
    }
    pub fn get(&self, row: usize, column: usize) -> Option<f64> {
        self.value.get(row, column).copied()
    }
    /// Return false for an out-of-range cell without changing storage.
    pub fn set(&mut self, row: usize, column: usize, value: f64) -> bool {
        if let Some(element) = self.value.get_mut(row, column) {
            *element = value;
            true
        } else {
            false
        }
    }
    pub fn row(&self, row: usize) -> Option<Vec<f64>> {
        self.value.row(row).map(<[f64]>::to_vec)
    }
    pub fn set_row(&mut self, row: usize, values: &[f64]) -> Result<(), BindingError> {
        let destination = self
            .value
            .row_mut(row)
            .ok_or_else(|| napi::Error::from_reason("array row is out of range"))?;
        if values.len() != destination.len() {
            return Err(napi::Error::from_reason(
                "array row replacement has a different length",
            ));
        }
        destination.copy_from_slice(values);
        Ok(())
    }
    pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), BindingError> {
        self.value
            .reshape(rows, columns)
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transpose(&mut self) -> Result<(), BindingError> {
        self.value
            .transpose()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn transposed(&self) -> Result<Self, BindingError> {
        self.value
            .transposed()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.value
            .try_clone()
            .map(|value| Self { value })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    pub fn flatten(&self) -> Result<Vec<f64>, BindingError> {
        self.value
            .flatten()
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Consume this owner and move the native buffer to the result.
    pub fn into_row_major(self) -> Vec<f64> {
        self.value.into_row_major()
    }
}
