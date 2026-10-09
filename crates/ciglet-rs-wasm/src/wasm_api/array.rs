use crate::array::Array2;
use wasm_bindgen::prelude::*;

macro_rules! array_api {
    ($scalar:ty, $owner:ident) => {
        /// Complete owned row-major storage in the selected numeric element type.
        #[wasm_bindgen]
        pub struct $owner {
            pub(super) value: Array2<$scalar>,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(rows: usize, columns: usize, values: &[$scalar]) -> Result<Self, JsValue> {
                Array2::from_slice(rows, columns, values)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn zeros(rows: usize, columns: usize) -> Result<Self, JsValue> {
                Array2::zeros(rows, columns)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn filled(rows: usize, columns: usize, element: $scalar) -> Result<Self, JsValue> {
                Array2::filled(rows, columns, element)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            /// Reconstruct complete equally sized rows from packed typed storage.
            /// Row lengths make empty rows distinct from an empty row collection.
            pub fn from_rows(values: &[$scalar], row_lengths: &[u32]) -> Result<Self, JsValue> {
                let mut rows = Vec::new();
                rows.try_reserve_exact(row_lengths.len())
                    .map_err(|_| JsValue::from_str("array row allocation failed"))?;
                let mut remaining = values;
                for &length in row_lengths {
                    let length = length as usize;
                    if length > remaining.len() {
                        return Err(JsValue::from_str("array row storage is truncated"));
                    }
                    let (row, tail) = remaining.split_at(length);
                    rows.push(row);
                    remaining = tail;
                }
                if !remaining.is_empty() {
                    return Err(JsValue::from_str("array row storage has trailing values"));
                }
                Array2::from_rows(&rows)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            #[wasm_bindgen(getter)]
            pub fn rows(&self) -> usize {
                self.value.rows()
            }
            #[wasm_bindgen(getter)]
            pub fn columns(&self) -> usize {
                self.value.columns()
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.value.len()
            }
            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
            pub fn values(&self) -> Vec<$scalar> {
                self.value.as_slice().to_vec()
            }
            /// Replace mutable native storage without changing its shape.
            pub fn set_values(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                if values.len() != self.value.len() {
                    return Err(JsValue::from_str(
                        "array replacement must retain its length",
                    ));
                }
                self.value.as_mut_slice().copy_from_slice(values);
                Ok(())
            }
            pub fn get(&self, row: usize, column: usize) -> Option<$scalar> {
                self.value.get(row, column).copied()
            }
            /// Return false for an out-of-range cell without changing storage.
            pub fn set(&mut self, row: usize, column: usize, value: $scalar) -> bool {
                if let Some(element) = self.value.get_mut(row, column) {
                    *element = value;
                    true
                } else {
                    false
                }
            }
            pub fn row(&self, row: usize) -> Option<Vec<$scalar>> {
                self.value.row(row).map(<[$scalar]>::to_vec)
            }
            pub fn set_row(&mut self, row: usize, values: &[$scalar]) -> Result<(), JsValue> {
                let destination = self
                    .value
                    .row_mut(row)
                    .ok_or_else(|| JsValue::from_str("array row is out of range"))?;
                if values.len() != destination.len() {
                    return Err(JsValue::from_str(
                        "array row replacement has a different length",
                    ));
                }
                destination.copy_from_slice(values);
                Ok(())
            }
            pub fn reshape(&mut self, rows: usize, columns: usize) -> Result<(), JsValue> {
                self.value
                    .reshape(rows, columns)
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn transpose(&mut self) -> Result<(), JsValue> {
                self.value
                    .transpose()
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn transposed(&self) -> Result<Self, JsValue> {
                self.value
                    .transposed()
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn cloned(&self) -> Result<Self, JsValue> {
                self.value
                    .try_clone()
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn flatten(&self) -> Result<Vec<$scalar>, JsValue> {
                self.value
                    .flatten()
                    .map_err(|error| JsValue::from_str(error.0))
            }
            /// Consume this owner and move the native buffer to the result.
            pub fn into_row_major(self) -> Vec<$scalar> {
                self.value.into_row_major()
            }
        }
    };
}
array_api!(u8, Array2U8);
array_api!(u16, Array2U16);
array_api!(u32, Array2U32);
array_api!(u64, Array2U64);
array_api!(f32, Array2F32);
array_api!(f64, Array2F64);
