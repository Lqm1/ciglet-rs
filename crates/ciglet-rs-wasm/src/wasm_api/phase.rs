use crate::{Error, fourier::Complex, phase, statistics::Scalar};
use wasm_bindgen::prelude::*;

/// Complete spectrum mirroring choices.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum SpectrumSymmetry {
    Symmetric,
    Antisymmetric,
}

/// Native preserved Nyquist or original substituted-bin behavior.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum SpectrumNyquist {
    Preserve,
    Legacy,
}

fn components<T: Scalar>(real: &[T], imaginary: &[T]) -> Result<Vec<Complex<T>>, Error> {
    if real.len() != imaginary.len() {
        return Err(Error("complex component lengths differ"));
    }
    let mut values = Vec::new();
    values
        .try_reserve_exact(real.len())
        .map_err(|_| Error("complex input allocation failed"))?;
    values.extend(
        real.iter()
            .zip(imaginary)
            .map(|(&re, &im)| Complex::new(re, im)),
    );
    Ok(values)
}

macro_rules! phase_api {
    ($scalar:ty, $dct:ident, $wrap:ident, $unwrap:ident, $difference:ident, $magnitude:ident, $argument:ident, $real:ident, $imaginary:ident, $complete:ident) => {
        /// Original orthonormal DCT-II, retaining approximate binary32 cosine.
        #[wasm_bindgen]
        pub fn $dct(values: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            crate::cosine::dct(values).map_err(|error| JsValue::from_str(error.0))
        }

        /// Native phase wrapping, including source rounding at half turns.
        #[wasm_bindgen]
        pub fn $wrap(angle: $scalar) -> Result<$scalar, JsValue> {
            phase::wrap(angle).map_err(|error| JsValue::from_str(error.0))
        }

        /// Native single-turn unwrapping; returned arrays own their storage.
        #[wasm_bindgen]
        pub fn $unwrap(angles: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            phase::unwrap(angles).map_err(|error| JsValue::from_str(error.0))
        }

        /// Directed phase difference, preserving the original integer range.
        #[wasm_bindgen]
        pub fn $difference(first: $scalar, second: $scalar) -> Result<$scalar, JsValue> {
            phase::difference(first, second).map_err(|error| JsValue::from_str(error.0))
        }

        /// Complete complex magnitudes; scalar aliases use one-element planes.
        #[wasm_bindgen]
        pub fn $magnitude(
            real: &[$scalar],
            imaginary: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            components(real, imaginary)
                .and_then(|values| phase::magnitude(&values))
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Complete complex arguments; scalar aliases use one-element planes.
        #[wasm_bindgen]
        pub fn $argument(real: &[$scalar], imaginary: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            components(real, imaginary)
                .and_then(|values| phase::argument(&values))
                .map_err(|error| JsValue::from_str(error.0))
        }

        /// Polar real components with original approximate trigonometry.
        #[wasm_bindgen]
        pub fn $real(magnitudes: &[$scalar], angles: &[$scalar]) -> Result<Vec<$scalar>, JsValue> {
            phase::polar_real(magnitudes, angles).map_err(|error| JsValue::from_str(error.0))
        }

        /// Polar imaginary components with original approximate trigonometry.
        #[wasm_bindgen]
        pub fn $imaginary(
            magnitudes: &[$scalar],
            angles: &[$scalar],
        ) -> Result<Vec<$scalar>, JsValue> {
            phase::polar_imaginary(magnitudes, angles).map_err(|error| JsValue::from_str(error.0))
        }

        /// Copy and complete the spectrum. Omitted policy preserves Nyquist;
        /// Legacy retains the original even-bin substitution. Input is unchanged.
        #[wasm_bindgen]
        pub fn $complete(
            mut values: Vec<$scalar>,
            symmetry: SpectrumSymmetry,
            policy: Option<SpectrumNyquist>,
        ) -> Result<Vec<$scalar>, JsValue> {
            let symmetry = match symmetry {
                SpectrumSymmetry::Symmetric => phase::Symmetry::Symmetric,
                SpectrumSymmetry::Antisymmetric => phase::Symmetry::Antisymmetric,
            };
            let policy = match policy.unwrap_or(SpectrumNyquist::Preserve) {
                SpectrumNyquist::Preserve => phase::NyquistPolicy::Preserve,
                SpectrumNyquist::Legacy => phase::NyquistPolicy::Legacy,
            };
            phase::complete_spectrum_with(&mut values, symmetry, policy)
                .map_err(|error| JsValue::from_str(error.0))?;
            Ok(values)
        }
    };
}

phase_api!(
    f32,
    dct_f32,
    phase_wrap_f32,
    phase_unwrap_f32,
    phase_difference_f32,
    magnitude_f32,
    argument_f32,
    polar_real_f32,
    polar_imaginary_f32,
    complete_spectrum_f32
);
phase_api!(
    f64,
    dct_f64,
    phase_wrap_f64,
    phase_unwrap_f64,
    phase_difference_f64,
    magnitude_f64,
    argument_f64,
    polar_real_f64,
    polar_imaginary_f64,
    complete_spectrum_f64
);
