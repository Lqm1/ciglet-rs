use crate::fourier::{Complex, Direction, FftPlan};
use wasm_bindgen::prelude::*;

/// Frequency order and inverse normalization follow the native Fourier API.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum FftDirection {
    Forward,
    Inverse,
}

impl From<FftDirection> for Direction {
    fn from(value: FftDirection) -> Self {
        match value {
            FftDirection::Forward => Self::Forward,
            FftDirection::Inverse => Self::Inverse,
        }
    }
}

// Validate within the method's Result path so errors release the owner guard.
pub(super) fn fft_direction(value: u32) -> Result<Direction, JsValue> {
    match value {
        0 => Ok(FftDirection::Forward.into()),
        1 => Ok(FftDirection::Inverse.into()),
        _ => Err(JsValue::from_str("invalid FFT direction")),
    }
}

macro_rules! fourier_api {
    ($scalar:ty, $values:ident, $plan:ident, $fft:ident, $idft:ident, $shift:ident) => {
        /// Complete independently owned complex result. Numeric getters copy
        /// into JavaScript typed arrays, which outlive this owner's `free()`.
        #[wasm_bindgen]
        pub struct $values {
            pub(super) values: Vec<Complex<$scalar>>,
        }

        #[wasm_bindgen]
        impl $values {
            /// Construct complete complex values from equally sized planes.
            #[wasm_bindgen(constructor)]
            pub fn new(real: Vec<$scalar>, imaginary: Vec<$scalar>) -> Result<Self, JsValue> {
                if real.len() != imaginary.len() {
                    return Err(JsValue::from_str("complex component lengths differ"));
                }
                Ok(Self {
                    values: real
                        .into_iter()
                        .zip(imaginary)
                        .map(|(re, im)| Complex::new(re, im))
                        .collect(),
                })
            }

            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.values.len()
            }

            #[wasm_bindgen(getter)]
            pub fn real(&self) -> Vec<$scalar> {
                self.values.iter().map(|value| value.re).collect()
            }

            #[wasm_bindgen(getter)]
            pub fn imaginary(&self) -> Vec<$scalar> {
                self.values.iter().map(|value| value.im).collect()
            }

            /// Alternating real and imaginary values, retaining every bit.
            #[wasm_bindgen(getter)]
            pub fn interleaved(&self) -> Vec<$scalar> {
                self.values
                    .iter()
                    .flat_map(|value| [value.re, value.im])
                    .collect()
            }
        }

        /// Reusable native forward/inverse FFT plans and scratch storage.
        #[wasm_bindgen]
        pub struct $plan {
            plan: FftPlan<$scalar>,
        }

        #[wasm_bindgen]
        impl $plan {
            #[wasm_bindgen(constructor)]
            pub fn new(length: usize) -> Result<Self, JsValue> {
                FftPlan::new(length)
                    .map(|plan| Self { plan })
                    .map_err(|error| JsValue::from_str(error.0))
            }

            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.plan.len()
            }

            /// Positive power-of-two plans are never empty.
            pub fn is_empty(&self) -> bool {
                self.plan.is_empty()
            }

            /// Missing planes are zero; present planes must match plan length.
            pub fn transform(
                &mut self,
                real: Option<Vec<$scalar>>,
                imaginary: Option<Vec<$scalar>>,
                direction: u32,
            ) -> Result<$values, JsValue> {
                self.plan
                    .transform(
                        real.as_deref(),
                        imaginary.as_deref(),
                        fft_direction(direction)?,
                    )
                    .map(|values| $values { values })
                    .map_err(|error| JsValue::from_str(error.0))
            }

            /// Transform a copied alternating real/imaginary buffer using the
            /// same native plan and scratch. Input arrays remain unchanged.
            pub fn process(
                &mut self,
                interleaved: &[$scalar],
                direction: u32,
            ) -> Result<$values, JsValue> {
                let direction = fft_direction(direction)?;
                if interleaved.len() != self.plan.len() * 2 {
                    return Err(JsValue::from_str(
                        "interleaved FFT input length differs from the plan",
                    ));
                }
                let mut values = Vec::new();
                values
                    .try_reserve_exact(self.plan.len())
                    .map_err(|_| JsValue::from_str("FFT input allocation failed"))?;
                values.extend(
                    interleaved
                        .chunks_exact(2)
                        .map(|pair| Complex::new(pair[0], pair[1])),
                );
                self.plan
                    .process(&mut values, direction)
                    .map_err(|error| JsValue::from_str(error.0))?;
                Ok($values { values })
            }
        }

        /// One-shot complete forward or normalized inverse FFT.
        #[wasm_bindgen]
        pub fn $fft(
            length: usize,
            real: Option<Vec<$scalar>>,
            imaginary: Option<Vec<$scalar>>,
            direction: u32,
        ) -> Result<$values, JsValue> {
            let mut plan = $plan::new(length)?;
            plan.transform(real, imaginary, direction)
        }

        /// Native direct normalized inverse DFT, including non-power-of-two lengths.
        #[wasm_bindgen]
        pub fn $idft(
            length: usize,
            real: Option<Vec<$scalar>>,
            imaginary: Option<Vec<$scalar>>,
        ) -> Result<$values, JsValue> {
            crate::fourier::inverse_dft(length, real.as_deref(), imaginary.as_deref())
                .map(|values| $values { values })
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Move DC to the center with the original odd-length ordering.
        #[wasm_bindgen]
        pub fn $shift(values: &[$scalar]) -> Vec<$scalar> {
            crate::fourier::fft_shift(values)
        }
    };
}

fourier_api!(
    f32,
    ComplexValuesF32,
    FftPlanF32,
    fft_f32,
    inverse_dft_f32,
    fft_shift_f32
);
fourier_api!(
    f64,
    ComplexValuesF64,
    FftPlanF64,
    fft_f64,
    inverse_dft_f64,
    fft_shift_f64
);
