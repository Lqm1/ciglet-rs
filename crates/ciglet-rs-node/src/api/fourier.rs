use crate::fourier::{Complex, Direction, FftPlan};
use napi::Error as BindingError;
/// Frequency order and inverse normalization follow the native Fourier API.
#[derive(Clone, Copy, Debug)]
#[napi_derive::napi]
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
pub(super) fn fft_direction(value: u32) -> Result<Direction, BindingError> {
    match value {
        0 => Ok(FftDirection::Forward.into()),
        1 => Ok(FftDirection::Inverse.into()),
        _ => Err(napi::Error::from_reason("invalid FFT direction")),
    }
}
/// Complete independently owned complex result. Numeric getters copy
/// into JavaScript typed arrays, which outlive this owner's `free()`.
pub struct ComplexValuesF32 {
    pub(super) values: Vec<Complex<f32>>,
}
impl ComplexValuesF32 {
    /// Construct complete complex values from equally sized planes.
    pub fn new(real: Vec<f32>, imaginary: Vec<f32>) -> Result<Self, BindingError> {
        if real.len() != imaginary.len() {
            return Err(napi::Error::from_reason("complex component lengths differ"));
        }
        Ok(Self {
            values: real
                .into_iter()
                .zip(imaginary)
                .map(|(re, im)| Complex::new(re, im))
                .collect(),
        })
    }
    pub fn length(&self) -> usize {
        self.values.len()
    }
    pub fn real(&self) -> Vec<f32> {
        self.values.iter().map(|value| value.re).collect()
    }
    pub fn imaginary(&self) -> Vec<f32> {
        self.values.iter().map(|value| value.im).collect()
    }
    /// Alternating real and imaginary values, retaining every bit.
    pub fn interleaved(&self) -> Vec<f32> {
        self.values
            .iter()
            .flat_map(|value| [value.re, value.im])
            .collect()
    }
}
/// Reusable native forward/inverse FFT plans and scratch storage.
pub struct FftPlanF32 {
    plan: FftPlan<f32>,
}
impl FftPlanF32 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        FftPlan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
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
        real: Option<Vec<f32>>,
        imaginary: Option<Vec<f32>>,
        direction: u32,
    ) -> Result<ComplexValuesF32, BindingError> {
        self.plan
            .transform(
                real.as_deref(),
                imaginary.as_deref(),
                fft_direction(direction)?,
            )
            .map(|values| ComplexValuesF32 { values })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Transform a copied alternating real/imaginary buffer using the
    /// same native plan and scratch. Input arrays remain unchanged.
    pub fn process(
        &mut self,
        interleaved: &[f32],
        direction: u32,
    ) -> Result<ComplexValuesF32, BindingError> {
        let direction = fft_direction(direction)?;
        if interleaved.len() != self.plan.len() * 2 {
            return Err(napi::Error::from_reason(
                "interleaved FFT input length differs from the plan",
            ));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.plan.len())
            .map_err(|_| napi::Error::from_reason("FFT input allocation failed"))?;
        values.extend(
            interleaved
                .chunks_exact(2)
                .map(|pair| Complex::new(pair[0], pair[1])),
        );
        self.plan
            .process(&mut values, direction)
            .map_err(|error| napi::Error::from_reason(error.0))?;
        Ok(ComplexValuesF32 { values })
    }
}
/// One-shot complete forward or normalized inverse FFT.
pub fn fft_f32(
    length: usize,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
    direction: u32,
) -> Result<ComplexValuesF32, BindingError> {
    let mut plan = FftPlanF32::new(length)?;
    plan.transform(real, imaginary, direction)
}
/// Native direct normalized inverse DFT, including non-power-of-two lengths.
pub fn inverse_dft_f32(
    length: usize,
    real: Option<Vec<f32>>,
    imaginary: Option<Vec<f32>>,
) -> Result<ComplexValuesF32, BindingError> {
    crate::fourier::inverse_dft(length, real.as_deref(), imaginary.as_deref())
        .map(|values| ComplexValuesF32 { values })
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Move DC to the center with the original odd-length ordering.
pub fn fft_shift_f32(values: &[f32]) -> Vec<f32> {
    crate::fourier::fft_shift(values)
}
/// Complete independently owned complex result. Numeric getters copy
/// into JavaScript typed arrays, which outlive this owner's `free()`.
pub struct ComplexValuesF64 {
    pub(super) values: Vec<Complex<f64>>,
}
impl ComplexValuesF64 {
    /// Construct complete complex values from equally sized planes.
    pub fn new(real: Vec<f64>, imaginary: Vec<f64>) -> Result<Self, BindingError> {
        if real.len() != imaginary.len() {
            return Err(napi::Error::from_reason("complex component lengths differ"));
        }
        Ok(Self {
            values: real
                .into_iter()
                .zip(imaginary)
                .map(|(re, im)| Complex::new(re, im))
                .collect(),
        })
    }
    pub fn length(&self) -> usize {
        self.values.len()
    }
    pub fn real(&self) -> Vec<f64> {
        self.values.iter().map(|value| value.re).collect()
    }
    pub fn imaginary(&self) -> Vec<f64> {
        self.values.iter().map(|value| value.im).collect()
    }
    /// Alternating real and imaginary values, retaining every bit.
    pub fn interleaved(&self) -> Vec<f64> {
        self.values
            .iter()
            .flat_map(|value| [value.re, value.im])
            .collect()
    }
}
/// Reusable native forward/inverse FFT plans and scratch storage.
pub struct FftPlanF64 {
    plan: FftPlan<f64>,
}
impl FftPlanF64 {
    pub fn new(length: usize) -> Result<Self, BindingError> {
        FftPlan::new(length)
            .map(|plan| Self { plan })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
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
        real: Option<Vec<f64>>,
        imaginary: Option<Vec<f64>>,
        direction: u32,
    ) -> Result<ComplexValuesF64, BindingError> {
        self.plan
            .transform(
                real.as_deref(),
                imaginary.as_deref(),
                fft_direction(direction)?,
            )
            .map(|values| ComplexValuesF64 { values })
            .map_err(|error| napi::Error::from_reason(error.0))
    }
    /// Transform a copied alternating real/imaginary buffer using the
    /// same native plan and scratch. Input arrays remain unchanged.
    pub fn process(
        &mut self,
        interleaved: &[f64],
        direction: u32,
    ) -> Result<ComplexValuesF64, BindingError> {
        let direction = fft_direction(direction)?;
        if interleaved.len() != self.plan.len() * 2 {
            return Err(napi::Error::from_reason(
                "interleaved FFT input length differs from the plan",
            ));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.plan.len())
            .map_err(|_| napi::Error::from_reason("FFT input allocation failed"))?;
        values.extend(
            interleaved
                .chunks_exact(2)
                .map(|pair| Complex::new(pair[0], pair[1])),
        );
        self.plan
            .process(&mut values, direction)
            .map_err(|error| napi::Error::from_reason(error.0))?;
        Ok(ComplexValuesF64 { values })
    }
}
/// One-shot complete forward or normalized inverse FFT.
pub fn fft_f64(
    length: usize,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
    direction: u32,
) -> Result<ComplexValuesF64, BindingError> {
    let mut plan = FftPlanF64::new(length)?;
    plan.transform(real, imaginary, direction)
}
/// Native direct normalized inverse DFT, including non-power-of-two lengths.
pub fn inverse_dft_f64(
    length: usize,
    real: Option<Vec<f64>>,
    imaginary: Option<Vec<f64>>,
) -> Result<ComplexValuesF64, BindingError> {
    crate::fourier::inverse_dft(length, real.as_deref(), imaginary.as_deref())
        .map(|values| ComplexValuesF64 { values })
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Move DC to the center with the original odd-length ordering.
pub fn fft_shift_f64(values: &[f64]) -> Vec<f64> {
    crate::fourier::fft_shift(values)
}
