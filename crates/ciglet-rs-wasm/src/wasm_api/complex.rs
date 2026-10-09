use crate::complex::{self, Complex, Precision};
use wasm_bindgen::prelude::*;

fn precision(value: u32) -> Result<Precision, JsValue> {
    match value {
        0 => Ok(Precision::Fast),
        1 => Ok(Precision::Approximate),
        2 => Ok(Precision::Exact),
        _ => Err(JsValue::from_str("invalid complex precision")),
    }
}

macro_rules! complex_api {
    ($scalar:ty, $owner:ident) => {
        /// Complete owned native complex value, including arbitrary component bits.
        #[wasm_bindgen]
        pub struct $owner {
            value: Complex<$scalar>,
        }

        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(real: $scalar, imaginary: $scalar) -> Self {
                Self {
                    value: Complex::new(real, imaginary),
                }
            }

            /// Construct from two typed-array components without Number conversion.
            pub fn from_components(values: &[$scalar]) -> Result<Self, JsValue> {
                let [real, imaginary]: [$scalar; 2] = values.try_into().map_err(|_| {
                    JsValue::from_str("complex value requires exactly two components")
                })?;
                Ok(Self::new(real, imaginary))
            }

            #[wasm_bindgen(getter)]
            pub fn real(&self) -> $scalar {
                self.value.re
            }
            #[wasm_bindgen(setter)]
            pub fn set_real(&mut self, value: $scalar) {
                self.value.re = value;
            }
            #[wasm_bindgen(getter)]
            pub fn imaginary(&self) -> $scalar {
                self.value.im
            }
            #[wasm_bindgen(setter)]
            pub fn set_imaginary(&mut self, value: $scalar) {
                self.value.im = value;
            }

            /// Independently owned real/imaginary storage retaining full payload bits.
            pub fn components(&self) -> Vec<$scalar> {
                vec![self.value.re, self.value.im]
            }

            /// Replace both native fields atomically using complete typed storage.
            pub fn set_components(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                self.value = Self::from_components(values)?.value;
                Ok(())
            }

            pub fn cloned(&self) -> Self {
                Self { value: self.value }
            }
            pub fn conjugate(&self) -> Self {
                Self {
                    value: self.value.conj(),
                }
            }
            pub fn add(&self, other: &Self) -> Self {
                Self {
                    value: self.value + other.value,
                }
            }
            pub fn subtract(&self, other: &Self) -> Self {
                Self {
                    value: self.value - other.value,
                }
            }
            pub fn multiply(&self, other: &Self) -> Self {
                Self {
                    value: self.value * other.value,
                }
            }
            pub fn divide(&self, other: &Self) -> Result<Self, JsValue> {
                complex::divide(self.value, other.value)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn exponential(&self, mode: u32) -> Result<Self, JsValue> {
                complex::exponential(self.value, precision(mode)?)
                    .map(|value| Self { value })
                    .map_err(|error| JsValue::from_str(error.0))
            }
            pub fn argument(&self, mode: u32) -> Result<$scalar, JsValue> {
                complex::argument(self.value, precision(mode)?)
                    .map_err(|error| JsValue::from_str(error.0))
            }
        }
    };
}
complex_api!(f32, ComplexF32);
complex_api!(f64, ComplexF64);
