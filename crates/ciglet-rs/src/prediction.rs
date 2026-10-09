//! Autoregressive prediction with stable and source-compatible arithmetic.
pub use crate::phase::NyquistPolicy;
mod scaled;
use crate::{
    Error,
    accumulation::{Scaled, Sum},
    fourier::{Complex, Direction, FftPlan},
    polynomial,
    statistics::{self, Scalar},
    vector,
};
pub use scaled::ScaledPrediction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticPolicy {
    /// Scaled correlations, normalized recursion and compensated gain products.
    Stable,
    /// Preserve source selected-scalar recursion and gain arithmetic.
    Legacy,
}

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
        self.estimate_with_arithmetic(magnitudes, order, policy, ArithmeticPolicy::Stable)
    }

    pub fn estimate_with_arithmetic(
        &mut self,
        magnitudes: &[T],
        order: usize,
        policy: NyquistPolicy,
        arithmetic: ArithmeticPolicy,
    ) -> Result<Prediction<T>, Error> {
        if arithmetic == ArithmeticPolicy::Stable {
            return self
                .estimate_scaled(magnitudes, order, policy)?
                .into_prediction();
        }
        let autocorrelation = self.magnitude_correlations(magnitudes, order, policy)?;
        let coefficients = levinson_with(&autocorrelation, arithmetic)?;
        Ok(Prediction {
            coefficients,
            autocorrelation,
        })
    }

    /// Retain each lag's exponent when squared magnitudes cannot fit the scalar.
    pub fn estimate_scaled(
        &mut self,
        magnitudes: &[T],
        order: usize,
        policy: NyquistPolicy,
    ) -> Result<ScaledPrediction<T>, Error> {
        let n = self.len();
        if magnitudes.len() < 2 || magnitudes.len() > n / 2 + 1 {
            return Err(Error(
                "magnitude spectrum does not fit the prediction transform",
            ));
        }
        let used = used_magnitudes(magnitudes, n, policy);
        vector::validate(used)?;
        let count = order
            .checked_add(1)
            .filter(|&count| count <= n)
            .ok_or(Error("prediction order exceeds the transform lag range"))?;
        let minimum_normal = if std::mem::size_of::<T>() == 4 {
            f32::MIN_POSITIVE as f64
        } else {
            f64::MIN_POSITIVE
        };
        let risky = used.iter().any(|&magnitude| {
            let square: f64 = (magnitude * magnitude).into();
            !square.is_finite() || (magnitude != T::zero() && square < minimum_normal)
        });
        if !risky {
            let values = self.magnitude_correlations(magnitudes, order, policy)?;
            if values.iter().all(|&value| value.into().is_finite())
                && (values[0].into() >= minimum_normal
                    || used.iter().all(|&value| value == T::zero()))
            {
                let mut scaled = vector::buffer(count)?;
                scaled.extend(values.into_iter().map(|value| Scaled::new(value.into(), 0)));
                return ScaledPrediction::new(scaled);
            }
        }
        let mut correlations = vector::buffer(count)?;
        for lag in 0..count {
            let mut sum = Sum::new();
            for bin in 0..=n / 2 {
                let source_bin = if bin == n / 2 && policy == NyquistPolicy::Legacy {
                    bin - 1
                } else {
                    bin
                };
                let Some(&magnitude) = used.get(source_bin) else {
                    continue;
                };
                let weight =
                    grid_phase(bin, lag, n).1 * if bin == 0 || bin == n / 2 { 1.0 } else { 2.0 };
                sum.weighted_square(magnitude.into(), weight);
            }
            correlations.push(sum.finish(n as f64));
        }
        ScaledPrediction::new(correlations)
    }

    fn magnitude_correlations(
        &mut self,
        magnitudes: &[T],
        order: usize,
        policy: NyquistPolicy,
    ) -> Result<Vec<T>, Error> {
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
        Ok(autocorrelation)
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
    for (index, &coefficient) in coefficients.iter().enumerate() {
        let (sine, cosine) = grid_phase(index, bin, length);
        real.product(Scaled::new(coefficient.into(), 0), Scaled::new(cosine, 0));
        imaginary.product(Scaled::new(coefficient.into(), 0), Scaled::new(-sine, 0));
    }
    real.finish(1.0).hypot(imaginary.finish(1.0))
}

fn grid_phase(index: usize, bin: usize, length: usize) -> (f64, f64) {
    let length = length as u64;
    let turn = (index as u64 * bin as u64) % length;
    if (turn * 4).is_multiple_of(length) {
        match turn * 4 / length {
            0 => (0.0, 1.0),
            1 => (1.0, 0.0),
            2 => (0.0, -1.0),
            _ => (-1.0, 0.0),
        }
    } else {
        (std::f64::consts::TAU * turn as f64 / length as f64).sin_cos()
    }
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
    from_magnitude_with_arithmetic(magnitudes, order, policy, ArithmeticPolicy::Stable)
}

pub fn from_magnitude_with_arithmetic<T: Scalar>(
    magnitudes: &[T],
    order: usize,
    policy: NyquistPolicy,
    arithmetic: ArithmeticPolicy,
) -> Result<Prediction<T>, Error> {
    let length = magnitude_length(magnitudes.len())?;
    vector::validate(used_magnitudes(magnitudes, length, policy))?;
    SpectrumPlan::new(length)?.estimate_with_arithmetic(magnitudes, order, policy, arithmetic)
}

pub fn from_magnitude_scaled<T: Scalar>(
    magnitudes: &[T],
    order: usize,
    policy: NyquistPolicy,
) -> Result<ScaledPrediction<T>, Error> {
    SpectrumPlan::new(magnitude_length(magnitudes.len())?)?
        .estimate_scaled(magnitudes, order, policy)
}

fn magnitude_length(bins: usize) -> Result<usize, Error> {
    bins.checked_sub(1)
        .filter(|&length| length > 0)
        .and_then(|length| length.checked_mul(2))
        .and_then(usize::checked_next_power_of_two)
        .ok_or(Error(
            "prediction magnitude spectrum requires at least two bounded bins",
        ))
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
    levinson_with(autocorrelation, ArithmeticPolicy::Stable)
}

pub fn levinson_with<T: Scalar>(
    autocorrelation: &[T],
    policy: ArithmeticPolicy,
) -> Result<Vec<T>, Error> {
    if policy == ArithmeticPolicy::Legacy {
        return legacy_levinson(autocorrelation);
    }
    vector::validate(autocorrelation)?;
    let Some(&energy) = autocorrelation.first() else {
        return Err(Error(
            "prediction requires at least one autocorrelation lag",
        ));
    };
    if energy <= T::zero() {
        return legacy_levinson(autocorrelation);
    }
    let n = autocorrelation.len();
    let energy: f64 = energy.into();
    let mut correlation = vector::buffer(n)?;
    for &value in autocorrelation {
        let normalized = value.into() / energy;
        if !normalized.is_finite() || normalized.abs() > 1.000001 {
            return Err(Error("autocorrelation is not positive definite"));
        }
        correlation.push(normalized);
    }
    let mut coefficients = vector::buffer(n)?;
    coefficients.resize(n, 0.0_f64);
    coefficients[0] = 1.0;
    let mut scratch = vector::buffer(n)?;
    scratch.resize(n, 0.0_f64);
    let mut residual = 1.000001;
    for order in 1..n {
        let mut sum = Sum::new();
        sum.scalar(correlation[order], 0);
        for lag in 1..order {
            sum.product(
                Scaled::new(correlation[lag], 0),
                Scaled::new(coefficients[order - lag], 0),
            );
        }
        let reflection = -sum.finish(1.0).value() / residual;
        if !reflection.is_finite() || reflection.abs() >= 1.0 {
            return Err(Error("autocorrelation is not positive definite"));
        }
        for lag in 1..order {
            scratch[lag] = reflection.mul_add(coefficients[order - lag], coefficients[lag]);
        }
        coefficients[1..order].copy_from_slice(&scratch[1..order]);
        coefficients[order] = reflection;
        residual *= (-reflection).mul_add(reflection, 1.0);
        if !residual.is_finite() || residual <= 0.0 {
            return Err(Error("autocorrelation is not positive definite"));
        }
    }
    let mut output = vector::buffer(n)?;
    for coefficient in coefficients {
        output.push(vector::scalar(coefficient)?);
    }
    Ok(output)
}

fn legacy_levinson<T: Scalar>(autocorrelation: &[T]) -> Result<Vec<T>, Error> {
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
    from_samples_with(samples, order, ArithmeticPolicy::Stable)
}

pub fn from_samples_with<T: Scalar>(
    samples: &[T],
    order: usize,
    policy: ArithmeticPolicy,
) -> Result<Prediction<T>, Error> {
    if policy == ArithmeticPolicy::Stable {
        return from_samples_scaled(samples, order)?.into_prediction();
    }
    vector::validate(samples)?;
    let count = order
        .checked_add(1)
        .filter(|&count| count <= samples.len())
        .ok_or(Error("prediction order must be smaller than sample count"))?;
    let autocorrelation = statistics::cross_correlation(samples, samples, count)?;
    let coefficients = levinson_with(&autocorrelation, policy)?;
    Ok(Prediction {
        coefficients,
        autocorrelation,
    })
}

/// Preserve individually scaled powers when ordinary correlations cannot fit.
pub fn from_samples_scaled<T: Scalar>(
    samples: &[T],
    order: usize,
) -> Result<ScaledPrediction<T>, Error> {
    scaled::waveform(samples, order)
}

/// Gain from compensated scaled products; unrepresentable results are errors.
pub fn gain<T: Scalar>(coefficients: &[T], autocorrelation: &[T]) -> Result<T, Error> {
    gain_with(coefficients, autocorrelation, ArithmeticPolicy::Stable)
}

pub fn gain_with<T: Scalar>(
    coefficients: &[T],
    autocorrelation: &[T],
    policy: ArithmeticPolicy,
) -> Result<T, Error> {
    vector::validate(coefficients)?;
    vector::validate(autocorrelation)?;
    if coefficients.is_empty() || coefficients.len() != autocorrelation.len() {
        return Err(Error("prediction gain requires matching nonempty arrays"));
    }
    if policy == ArithmeticPolicy::Stable {
        let mut sum = Sum::new();
        for (&coefficient, &correlation) in coefficients.iter().zip(autocorrelation) {
            sum.product(
                Scaled::new(coefficient.into(), 0),
                Scaled::new(correlation.into(), 0),
            );
        }
        let energy = sum.finish(1.0);
        if energy.fraction < 0.0 {
            return Err(Error("prediction residual energy must be nonnegative"));
        }
        return vector::scalar(energy.sqrt());
    }
    let energy = statistics::dot(coefficients, autocorrelation)?;
    if energy < T::zero() {
        return Err(Error("prediction residual energy must be nonnegative"));
    }
    vector::scalar(energy.into().sqrt())
}
