use pyo3::PyErr as BindingError;
/// Complete sort result with independently copied values and original indices.
pub struct SortedValuesF32 {
    values: Vec<f32>,
    indices: Vec<u32>,
}
impl SortedValuesF32 {
    /// Construct both complete result fields without normalizing their contents.
    pub fn new(values: Vec<f32>, indices: Vec<u32>) -> Self {
        Self { values, indices }
    }
    pub fn length(&self) -> usize {
        self.values.len()
    }
    pub fn values(&self) -> Vec<f32> {
        self.values.clone()
    }
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
/// Native comparison fold; empty arrays return undefined, not a NaN sentinel.
pub fn minimum_f32(values: &[f32]) -> Option<f32> {
    crate::statistics::minimum(values)
}
/// Native comparison fold, retaining second-operand ties and NaN behavior.
pub fn maximum_f32(values: &[f32]) -> Option<f32> {
    crate::statistics::maximum(values)
}
/// Native total-order median with finite overflow-safe even averaging.
pub fn median_f32(values: &[f32]) -> Result<f32, BindingError> {
    crate::statistics::median(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Partition the caller's typed array in place and return the selected value.
/// Native validation rejects an invalid index before any mutation.
pub fn select_nth_f32(values: &mut [f32], index: usize) -> Result<f32, BindingError> {
    crate::statistics::select_nth(values, index)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Stable ascending IEEE total order with original indices for ties.
pub fn sort_f32(values: &[f32]) -> SortedValuesF32 {
    let (values, indices) = crate::statistics::sort(values);
    SortedValuesF32 {
        values,
        indices: indices.into_iter().map(|index| index as u32).collect(),
    }
}
/// Complete sort result with independently copied values and original indices.
pub struct SortedValuesF64 {
    values: Vec<f64>,
    indices: Vec<u32>,
}
impl SortedValuesF64 {
    /// Construct both complete result fields without normalizing their contents.
    pub fn new(values: Vec<f64>, indices: Vec<u32>) -> Self {
        Self { values, indices }
    }
    pub fn length(&self) -> usize {
        self.values.len()
    }
    pub fn values(&self) -> Vec<f64> {
        self.values.clone()
    }
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
/// Native comparison fold; empty arrays return undefined, not a NaN sentinel.
pub fn minimum_f64(values: &[f64]) -> Option<f64> {
    crate::statistics::minimum(values)
}
/// Native comparison fold, retaining second-operand ties and NaN behavior.
pub fn maximum_f64(values: &[f64]) -> Option<f64> {
    crate::statistics::maximum(values)
}
/// Native total-order median with finite overflow-safe even averaging.
pub fn median_f64(values: &[f64]) -> Result<f64, BindingError> {
    crate::statistics::median(values)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Partition the caller's typed array in place and return the selected value.
/// Native validation rejects an invalid index before any mutation.
pub fn select_nth_f64(values: &mut [f64], index: usize) -> Result<f64, BindingError> {
    crate::statistics::select_nth(values, index)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Stable ascending IEEE total order with original indices for ties.
pub fn sort_f64(values: &[f64]) -> SortedValuesF64 {
    let (values, indices) = crate::statistics::sort(values);
    SortedValuesF64 {
        values,
        indices: indices.into_iter().map(|index| index as u32).collect(),
    }
}
