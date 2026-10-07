//! Autoregressive prediction with source-compatible Levinson recursion.
pub use crate::phase::NyquistPolicy;
use crate::{
    Error,
    accumulation::{Scaled, Sum},
    fourier::{Complex, Direction, FftPlan},
    polynomial,
    statistics::{self, Scalar},
    vector,
};

fn used_magnitudes<T>(values: &[T], length: usize, policy: NyquistPolicy) -> &[T] {
    if policy == NyquistPolicy::Legacy && values.len() == length / 2 + 1 {
        &values[..values.len() - 1]
    } else {
        values
    }
}

/// Reusable transform/workspace for magnitude-domain estimation and spectra.
#[derive(Debug)]
pub struct SpectrumPlan<T: Scalar> {
    fft: FftPlan<T>,
    workspace: Vec<Complex<T>>,
}
impl<T: Scalar> SpectrumPlan<T> {
    pub fn new(length: usize) -> Result<Self, Error> {
        let fft = FftPlan::new(length)?;
        let mut workspace = vector::buffer(length)?;
        workspace.resize(length, Complex::new(T::zero(), T::zero()));
        Ok(Self { fft, workspace })
    }
    pub fn len(&self) -> usize {
        self.fft.len()
    }
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Zero-pad squared magnitudes, mirror their upper spectrum and invert.
    pub fn estimate(
        &mut self,
        magnitudes: &[T],
        order: usize,
        policy: NyquistPolicy,
    ) -> Result<Prediction<T>, Error> {
        let n = self.len();
        if magnitudes.len() < 2 || magnitudes.len() > n / 2 + 1 {
            return Err(Error(
                "magnitude spectrum does not fit the prediction transform",
            ));
        }
        let magnitudes = used_magnitudes(magnitudes, n, policy);
        vector::validate(magnitudes)?;
        let count = order
            .checked_add(1)
            .filter(|&count| count <= n)
            .ok_or(Error("prediction order exceeds the transform lag range"))?;
        self.workspace.fill(Complex::new(T::zero(), T::zero()));
        for (value, &magnitude) in self.workspace.iter_mut().zip(magnitudes) {
            value.re = magnitude * magnitude;
            if !value.re.into().is_finite() {
                return Err(Error("squared magnitude overflowed"));
            }
        }
        if policy == NyquistPolicy::Legacy {
            self.workspace[n / 2].re = self.workspace[n / 2 - 1].re;
        }
        for i in n / 2 + 1..n {
            self.workspace[i].re = self.workspace[n - i].re;
        }
        self.fft.process(&mut self.workspace, Direction::Inverse)?;
        let mut autocorrelation = vector::buffer(count)?;
        autocorrelation.extend(self.workspace[..count].iter().map(|value| value.re));
        let coefficients = levinson(&autocorrelation)?;
        Ok(Prediction {
            coefficients,
            autocorrelation,
        })
    }

    /// Evaluate the full FFT-grid prediction magnitude, retaining signed gain.
    pub fn spectrum(&mut self, coefficients: &[T], gain: T) -> Result<Vec<T>, Error> {
        vector::validate(coefficients)?;
        vector::validate(&[gain])?;
        let n = self.len();
        if coefficients.is_empty()
            || coefficients.len() > n
            || coefficients.iter().all(|&value| value == T::zero())
        {
            return Err(Error(
                "prediction spectrum requires a nonzero polynomial fitting the transform",
            ));
        }
        let mut output = vector::buffer(n)?;
        if gain == T::zero() {
            output.resize(n, gain);
            return Ok(output);
        }
        self.workspace.fill(Complex::new(T::zero(), T::zero()));
        for (value, &coefficient) in self.workspace.iter_mut().zip(coefficients) {
            value.re = coefficient;
        }
        self.fft.process(&mut self.workspace, Direction::Forward)?;
        for (index, value) in self.workspace.iter().enumerate() {
            let square = value.re * value.re + value.im * value.im;
            let magnitude = if !square.into().is_finite()
                || (square == T::zero() && (value.re != T::zero() || value.im != T::zero()))
            {
                value.re.into().hypot(value.im.into())
            } else {
                square.into().sqrt()
            };
            let ratio = if !magnitude.is_finite() || magnitude == 0.0 {
                let response = scaled_response(coefficients, index, n);
                if response.fraction == 0.0 {
                    return Err(Error(
                        "prediction spectrum has a pole on the transform grid",
                    ));
                }
                Scaled::new(gain.into(), 0).divided_by(response).value()
            } else {
                gain.into() / magnitude
            };
            output.push(vector::scalar(ratio)?);
        }
        Ok(output)
    }
}

/// Recover only exceptional FFT bins without normalizing away small coefficients.
fn scaled_response<T: Scalar>(coefficients: &[T], bin: usize, length: usize) -> Scaled {
    let mut real = Sum::new();
    let mut imaginary = Sum::new();
    let length = length as u64;
    for (index, &coefficient) in coefficients.iter().enumerate() {
        let turn = (index as u64 * bin as u64) % length;
        let (sine, cosine) = if (turn * 4).is_multiple_of(length) {
            match turn * 4 / length {
                0 => (0.0, 1.0),
                1 => (1.0, 0.0),
                2 => (0.0, -1.0),
                _ => (-1.0, 0.0),
            }
        } else {
            (std::f64::consts::TAU * turn as f64 / length as f64).sin_cos()
        };
        real.product(Scaled::new(coefficient.into(), 0), Scaled::new(cosine, 0));
        imaginary.product(Scaled::new(coefficient.into(), 0), Scaled::new(-sine, 0));
    }
    real.finish(1.0).hypot(imaginary.finish(1.0))
}

/// Source transform sizing: next power of two at least twice bins minus one.
pub fn from_magnitude<T: Scalar>(magnitudes: &[T], order: usize) -> Result<Prediction<T>, Error> {
    from_magnitude_with(magnitudes, order, NyquistPolicy::Preserve)
}
pub fn from_magnitude_with<T: Scalar>(
    magnitudes: &[T],
    order: usize,
    policy: NyquistPolicy,
) -> Result<Prediction<T>, Error> {
    let length = magnitudes
        .len()
        .checked_sub(1)
        .filter(|&length| length > 0)
        .and_then(|length| length.checked_mul(2))
        .and_then(usize::checked_next_power_of_two)
        .ok_or(Error(
            "prediction magnitude spectrum requires at least two bounded bins",
        ))?;
    vector::validate(used_magnitudes(magnitudes, length, policy))?;
    SpectrumPlan::new(length)?.estimate(magnitudes, order, policy)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resonance<T> {
    pub frequency_nyquist: T,
    pub pole: Complex<T>,
}
#[derive(Debug, Clone)]
pub struct ResonanceEstimate<T> {
    pub resonances: Vec<Resonance<T>>,
    pub iterations: usize,
    pub converged: bool,
    pub maximum_residual: f64,
}
/// Keep source positive-frequency poles strictly inside 0.001..0.999 Nyquist.
pub fn resonances<T: Scalar>(coefficients: &[T]) -> Result<ResonanceEstimate<T>, Error> {
    let roots = polynomial::roots_real(coefficients)?;
    let mut resonances = vector::buffer(roots.values.len())?;
    for &pole in &roots.values {
        let frequency_nyquist: T =
            vector::scalar(pole.im.into().atan2(pole.re.into()) / std::f64::consts::PI)?;
        if frequency_nyquist.into() > 0.001 && frequency_nyquist.into() < 0.999 {
            resonances.push(Resonance {
                frequency_nyquist,
                pole,
            });
        }
    }
    resonances
        .sort_by(|first, second| T::total_cmp(&first.frequency_nyquist, &second.frequency_nyquist));
    Ok(ResonanceEstimate {
        resonances,
        iterations: roots.iterations,
        converged: roots.converged,
        maximum_residual: roots.maximum_residual,
    })
}

/// Owned prediction coefficients and the autocorrelation used to estimate them.
#[derive(Debug, Clone, PartialEq)]
pub struct Prediction<T> {
    pub coefficients: Vec<T>,
    pub autocorrelation: Vec<T>,
}

/// Estimate coefficients from nonnegative-lag autocorrelation.
/// The leading coefficient is one; the order is `autocorrelation.len() - 1`.
pub fn levinson<T: Scalar>(autocorrelation: &[T]) -> Result<Vec<T>, Error> {
    vector::validate(autocorrelation)?;
    let Some(&energy) = autocorrelation.first() else {
        return Err(Error(
            "prediction requires at least one autocorrelation lag",
        ));
    };
    if energy < T::zero() {
        return Err(Error("autocorrelation energy must be nonnegative"));
    }
    let n = autocorrelation.len();
    let mut coefficients = vector::buffer(n)?;
    coefficients.resize(n, T::zero());
    coefficients[0] = T::one();
    if n == 1 {
        return Ok(coefficients);
    }
    if energy == T::zero() {
        if autocorrelation.iter().any(|&value| value != T::zero()) {
            return Err(Error("zero energy requires zero autocorrelation"));
        }
        return Ok(coefficients);
    }
    let mut scratch = vector::buffer(n)?;
    scratch.resize(n, T::zero());
    let reflection: T = vector::scalar(-autocorrelation[1].into() / (energy.into() * 1.000001))?;
    coefficients[1] = reflection;
    let mut residual: T =
        vector::scalar(energy.into() * 1.000001 * (T::one() - reflection * reflection).into())?;
    if residual <= T::zero() {
        return Err(Error("autocorrelation is not positive definite"));
    }
    for order in 2..n {
        let mut sum = autocorrelation[order];
        for lag in 1..order {
            sum = sum + autocorrelation[lag] * coefficients[order - lag];
        }
        let reflection = -sum / residual;
        for lag in 1..order {
            scratch[lag] = coefficients[lag] + reflection * coefficients[order - lag];
        }
        coefficients[1..order].copy_from_slice(&scratch[1..order]);
        coefficients[order] = reflection;
        residual = vector::scalar(residual.into() * (1.0 - (reflection * reflection).into()))?;
        if residual <= T::zero() {
            return Err(Error("autocorrelation is not positive definite"));
        }
    }
    vector::validate(&coefficients)?;
    Ok(coefficients)
}

/// Unnormalized waveform autocorrelation followed by Levinson recursion.
pub fn from_samples<T: Scalar>(samples: &[T], order: usize) -> Result<Prediction<T>, Error> {
    vector::validate(samples)?;
    let count = order
        .checked_add(1)
        .filter(|&count| count <= samples.len())
        .ok_or(Error("prediction order must be smaller than sample count"))?;
    let autocorrelation = statistics::cross_correlation(samples, samples, count)?;
    let coefficients = levinson(&autocorrelation)?;
    Ok(Prediction {
        coefficients,
        autocorrelation,
    })
}

/// Source gain: square root of the sequential coefficient/autocorrelation dot.
pub fn gain<T: Scalar>(coefficients: &[T], autocorrelation: &[T]) -> Result<T, Error> {
    vector::validate(coefficients)?;
    vector::validate(autocorrelation)?;
    if coefficients.is_empty() || coefficients.len() != autocorrelation.len() {
        return Err(Error("prediction gain requires matching nonempty arrays"));
    }
    let energy = statistics::dot(coefficients, autocorrelation)?;
    if energy < T::zero() {
        return Err(Error("prediction residual energy must be nonnegative"));
    }
    vector::scalar(energy.into().sqrt())
}
