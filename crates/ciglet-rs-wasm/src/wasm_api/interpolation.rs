use wasm_bindgen::prelude::*;

/// Whether the Lanczos kernel includes sample zero.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum BoundaryPolicy {
    IncludeFirst,
    LegacySkipFirst,
}

impl From<BoundaryPolicy> for crate::resampling::BoundaryPolicy {
    fn from(value: BoundaryPolicy) -> Self {
        match value {
            BoundaryPolicy::IncludeFirst => Self::IncludeFirst,
            BoundaryPolicy::LegacySkipFirst => Self::LegacySkipFirst,
        }
    }
}

/// Corrected near-zero evaluation or original approximate kernel arithmetic.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum KernelPolicy {
    Stable,
    Legacy,
}

impl From<KernelPolicy> for crate::resampling::KernelPolicy {
    fn from(value: KernelPolicy) -> Self {
        match value {
            KernelPolicy::Stable => Self::Stable,
            KernelPolicy::Legacy => Self::Legacy,
        }
    }
}

macro_rules! interpolation_api {
    ($scalar:ty, $linear:ident, $uniform:ident, $sinc:ident, $with:ident, $kernel:ident, $fill:ident) => {
        /// Linear interpolation on strictly increasing coordinates with endpoint clamping.
        #[wasm_bindgen]
        pub fn $linear(
            coordinates: &[$scalar],
            values: &[$scalar],
            positions: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::linear(coordinates, values, positions)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Original uniform-source coordinate convention and near-final-sample clamp.
        #[wasm_bindgen]
        pub fn $uniform(
            start: $scalar,
            end: $scalar,
            values: &[$scalar],
            positions: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::linear_uniform(start, end, values, positions)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Four-lobe Lanczos interpolation with corrected boundary and kernel defaults.
        #[wasm_bindgen]
        pub fn $sinc(
            start: $scalar,
            end: $scalar,
            values: &[$scalar],
            positions: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::sinc_uniform(start, end, values, positions)
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Explicit sample-zero policy with original kernel arithmetic.
        #[wasm_bindgen]
        pub fn $with(
            start: $scalar,
            end: $scalar,
            values: &[$scalar],
            positions: &[$scalar],
            boundary: BoundaryPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::sinc_uniform_with(start, end, values, positions, boundary.into())
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Independent sample-zero and near-zero kernel policies.
        #[wasm_bindgen]
        pub fn $kernel(
            start: $scalar,
            end: $scalar,
            values: &[$scalar],
            positions: &[$scalar],
            boundary: BoundaryPolicy,
            kernel: KernelPolicy,
        ) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::sinc_uniform_with_kernel(
                start,
                end,
                values,
                positions,
                boundary.into(),
                kernel.into(),
            )
            .map_err(|error| JsValue::from_str(error.0))
        }

        /// Interpolate sentinel gaps, including NaN sentinels, without modifying the input.
        #[wasm_bindgen]
        pub fn $fill(signal: &[$scalar], blank: $scalar) -> Result<Vec<$scalar>, JsValue> {
            crate::interpolation::fill_missing(signal, blank)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}

interpolation_api!(
    f32,
    interpolate_linear_f32,
    interpolate_uniform_f32,
    interpolate_sinc_f32,
    interpolate_sinc_with_f32,
    interpolate_sinc_with_kernel_f32,
    interpolate_fill_missing_f32
);
interpolation_api!(
    f64,
    interpolate_linear_f64,
    interpolate_uniform_f64,
    interpolate_sinc_f64,
    interpolate_sinc_with_f64,
    interpolate_sinc_with_kernel_f64,
    interpolate_fill_missing_f64
);
