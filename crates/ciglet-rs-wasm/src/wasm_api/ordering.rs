use wasm_bindgen::prelude::*;

macro_rules! ordering_api {
    ($scalar:ty, $owner:ident, $minimum:ident, $maximum:ident, $median:ident, $select:ident, $sort:ident) => {
        /// Complete sort result with independently copied values and original indices.
        #[wasm_bindgen]
        pub struct $owner {
            values: Vec<$scalar>,
            indices: Vec<u32>,
        }

        #[wasm_bindgen]
        impl $owner {
            /// Construct both complete result fields without normalizing their contents.
            #[wasm_bindgen(constructor)]
            pub fn new(values: Vec<$scalar>, indices: Vec<u32>) -> Self {
                Self { values, indices }
            }

            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.values.len()
            }

            #[wasm_bindgen(getter)]
            pub fn values(&self) -> Vec<$scalar> {
                self.values.clone()
            }

            #[wasm_bindgen(getter)]
            pub fn indices(&self) -> Vec<u32> {
                self.indices.clone()
            }
        }

        /// Native comparison fold; empty arrays return undefined, not a NaN sentinel.
        #[wasm_bindgen]
        pub fn $minimum(values: &[$scalar]) -> Option<$scalar> {
            crate::statistics::minimum(values)
        }

        /// Native comparison fold, retaining second-operand ties and NaN behavior.
        #[wasm_bindgen]
        pub fn $maximum(values: &[$scalar]) -> Option<$scalar> {
            crate::statistics::maximum(values)
        }

        /// Native total-order median with finite overflow-safe even averaging.
        #[wasm_bindgen]
        pub fn $median(values: &[$scalar]) -> Result<$scalar, JsValue> {
            crate::statistics::median(values).map_err(|error| JsValue::from_str(error.0))
        }

        /// Partition the caller's typed array in place and return the selected value.
        /// Native validation rejects an invalid index before any mutation.
        #[wasm_bindgen]
        pub fn $select(values: &mut [$scalar], index: usize) -> Result<$scalar, JsValue> {
            crate::statistics::select_nth(values, index).map_err(|error| JsValue::from_str(error.0))
        }

        /// Stable ascending IEEE total order with original indices for ties.
        #[wasm_bindgen]
        pub fn $sort(values: &[$scalar]) -> $owner {
            let (values, indices) = crate::statistics::sort(values);
            // WASM usize is 32-bit; every original slice index fits u32.
            $owner {
                values,
                indices: indices.into_iter().map(|index| index as u32).collect(),
            }
        }
    };
}

ordering_api!(
    f32,
    SortedValuesF32,
    minimum_f32,
    maximum_f32,
    median_f32,
    select_nth_f32,
    sort_f32
);
ordering_api!(
    f64,
    SortedValuesF64,
    minimum_f64,
    maximum_f64,
    median_f64,
    select_nth_f64,
    sort_f64
);
