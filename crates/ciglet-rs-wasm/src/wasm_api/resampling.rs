use super::{BoundaryPolicy, KernelPolicy};
use wasm_bindgen::prelude::*;

macro_rules! resampling_api {
    ($scalar:ty, $default:ident, $with:ident, $kernel:ident, $length:ident) => {
        /// Antialiased Lanczos resampling with corrected native defaults.
        #[wasm_bindgen]
        pub fn $default(signal: &[$scalar], ratio: $scalar) -> Result<Vec<$scalar>, JsValue> {
            crate::resampling::resample(signal, ratio).map_err(|error| JsValue::from_str(error.0))
        }

        /// Select sample-zero handling while retaining original kernel arithmetic.
        #[wasm_bindgen]
        pub fn $with(
            signal: &[$scalar],
            ratio: $scalar,
            boundary: BoundaryPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::resampling::resample_with(signal, ratio, boundary.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Select sample-zero handling and kernel arithmetic independently.
        #[wasm_bindgen]
        pub fn $kernel(
            signal: &[$scalar],
            ratio: $scalar,
            boundary: BoundaryPolicy,
            kernel: KernelPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::resampling::resample_with_kernel(signal, ratio, boundary.into(), kernel.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Compute the native checked output length without allocating a signal.
        #[wasm_bindgen]
        pub fn $length(count: usize, ratio: $scalar) -> Result<usize, JsValue> {
            crate::resampling::output_length(count, ratio)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

resampling_api!(
    f32,
    resample_f32,
    resample_with_f32,
    resample_with_kernel_f32,
    resample_output_length_f32
);
resampling_api!(
    f64,
    resample_f64,
    resample_with_f64,
    resample_with_kernel_f64,
    resample_output_length_f64
);
