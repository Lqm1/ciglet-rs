//! Liljencrants–Fant glottal flow derivative model and its spectrum.
pub use crate::approximation::Precision;
use crate::{
    Error, approximation,
    complex::{self, Complex},
    phase, root,
    statistics::Scalar,
    vector,
};

#[cfg(test)]
#[path = "glottal/tests.rs"]
mod tests;

/// Public LF configuration; timing fields other than period are period ratios.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Model<T> {
    /// Period in seconds.
    pub period: T,
    pub closure: T,
    pub peak: T,
    pub return_time: T,
    /// Finite signed amplitude; negative values invert the waveform.
    pub excitation: T,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum<T> {
    pub magnitude: Vec<T>,
    /// Radians, using the original accurate atan2.
    pub phase: Vec<T>,
}
/// Derived parameters are reused for multiple spectra or sampled periods.
#[derive(Debug, Clone)]
pub struct Prepared<T> {
    model: Model<T>,
    period: T,
    closure: T,
    return_time: T,
    angular_frequency: T,
    sine: T,
    cosine: T,
    decay: T,
    growth: T,
    initial_excitation: T,
    scale: T,
}
impl<T: Scalar> Model<T> {
    /// Construct the original piecewise Rd mapping. Rd must be positive.
    pub fn from_shape(shape: T, period: T, excitation: T) -> Result<Self, Error> {
        vector::validate(&[shape, period, excitation])?;
        let rd = shape.into();
        if rd <= 0.0 {
            return Err(Error("LF shape must be positive"));
        }
        let return_time: T = vector::scalar(if rd < 0.21 {
            1e-6
        } else if rd < 2.7 {
            (-1.0 + 4.8 * rd) / 100.0
        } else {
            0.323 / rd
        })?;
        let upper: T = vector::scalar(1.0 - 1.0 / (2.17 * rd))?;
        let (peak_ratio, growth_ratio): (T, T) = if rd < 2.7 {
            let ratio: T = vector::scalar((22.4 + 11.8 * rd) / 100.0)?;
            (
                ratio,
                vector::scalar(
                    0.25 * ratio.into()
                        / ((0.11 * rd) / (0.5 + 1.2 * ratio.into()) - return_time.into()),
                )?,
            )
        } else {
            let growth: T = vector::scalar(9.3552e-3 + 596e-2 / (7.96 - 2.0 * upper.into()))?;
            (
                vector::scalar(2.0 * growth.into() * upper.into() - 1.0428)?,
                growth,
            )
        };
        let peak: T = vector::scalar(1.0 / (2.0 * growth_ratio.into()))?;
        // Rkp + 1.0 is promoted to double in the original expression.
        let closure = vector::scalar(peak.into() * (peak_ratio.into() + 1.0))?;
        let model = Self {
            period,
            closure,
            peak,
            return_time,
            excitation,
        };
        model.validate()?;
        Ok(model)
    }
    fn validate(&self) -> Result<(), Error> {
        vector::validate(&[
            self.period,
            self.closure,
            self.peak,
            self.return_time,
            self.excitation,
        ])?;
        if self.period <= T::zero()
            || self.peak <= T::zero()
            || self.peak >= self.closure
            || self.closure >= T::one()
            || self.return_time <= T::zero()
            || self.return_time.into() >= 1.0 - self.closure.into()
        {
            return Err(Error(
                "LF model requires ordered physical timings and finite excitation",
            ));
        }
        Ok(())
    }
    pub fn prepare(self) -> Result<Prepared<T>, Error> {
        self.validate()?;
        let mut period = self.period;
        let scale: T = if period.into() < 1.0 / 800.0 {
            let scale = vector::scalar(1.0 / period.into() / 800.0)?;
            period = T::from_f64(1.0 / 800.0).unwrap();
            scale
        } else {
            T::one()
        };
        let closure = period * self.closure;
        let peak = period * self.peak;
        let return_time = period * self.return_time;
        vector::validate(&[closure, peak, return_time])?;
        let angular_frequency: T = vector::scalar(std::f64::consts::PI / peak.into())?;
        let angle = (angular_frequency * closure).into();
        let sine: T = vector::scalar(angle.sin())?;
        let cosine: T = vector::scalar(angle.cos())?;
        let end: T = vector::scalar(2.0 / (return_time.into() + 1e-9))?;
        let decay = root::find(
            |x: T| {
                T::from_f64(1.0 - ((closure - period) * x).into().exp() - (return_time * x).into())
                    .unwrap()
            },
            T::one(),
            end,
        )?;
        let exponential: T = vector::scalar((decay * (closure - period)).into().exp())?;
        let integral: T = vector::scalar(
            (1.0 - exponential.into()) / (decay * decay * return_time).into()
                + ((closure - period) * exponential / (decay * return_time)).into(),
        )?;
        let constant =
            angular_frequency * angular_frequency * sine * integral - angular_frequency * cosine;
        let mut growth = T::zero();
        for _ in 0..8 {
            let exponential = (-growth * closure).into().exp();
            let function: T = vector::scalar(
                (sine * integral * growth * growth + sine * growth).into()
                    + angular_frequency.into() * exponential
                    + constant.into(),
            )?;
            let derivative: T = vector::scalar(
                (T::from_f64(2.0).unwrap() * sine * integral * growth + sine).into()
                    - (angular_frequency * closure).into() * exponential,
            )?;
            growth = growth - function / derivative;
            vector::validate(&[growth])?;
        }
        let initial_excitation: T = vector::scalar(
            -self.excitation.into() / ((growth * closure).into().exp() * sine.into()),
        )?;
        Ok(Prepared {
            model: self,
            period,
            closure,
            return_time,
            angular_frequency,
            sine,
            cosine,
            decay,
            growth,
            initial_excitation,
            scale,
        })
    }
    pub fn spectrum(self, frequencies: &[T]) -> Result<Spectrum<T>, Error> {
        self.prepare()?.spectrum(frequencies)
    }
    pub fn sampled_period(self, sampling_rate: i32, count: usize) -> Result<Vec<T>, Error> {
        self.prepare()?.sampled_period(sampling_rate, count)
    }
}
impl<T: Scalar> Prepared<T> {
    pub fn model(&self) -> Model<T> {
        self.model
    }
    /// Evaluate the source analytic response at arbitrary finite hertz queries.
    /// DC is explicitly zero, avoiding the source 0/0 intermediate.
    pub fn spectrum(&self, frequencies: &[T]) -> Result<Spectrum<T>, Error> {
        self.spectrum_with(frequencies, Precision::Approximate)
    }
    /// Choose exponential/trig precision; Approximate is the original default.
    pub fn spectrum_with(
        &self,
        frequencies: &[T],
        precision: Precision,
    ) -> Result<Spectrum<T>, Error> {
        vector::validate(frequencies)?;
        let mut magnitude = vector::buffer(frequencies.len())?;
        let mut phases = vector::buffer(frequencies.len())?;
        let c = |real, imaginary| Complex::new(real, imaginary);
        let z = T::zero();
        let e = self.decay;
        let a = self.growth;
        let wg = self.angular_frequency;
        let te = self.closure;
        let ta = self.return_time;
        let stability: T = vector::scalar(e.into() * (1.0 - (e * ta).into()))?;
        for &frequency in frequencies {
            if frequency == z {
                magnitude.push(z);
                phases.push(z);
                continue;
            }
            let omega: T =
                vector::scalar(std::f64::consts::TAU * frequency.into() / self.scale.into())?;
            let pole = c(a, -omega);
            let p1 = complex::divide(c(self.initial_excitation, z), pole * pole + c(wg * wg, z))?;
            let p2 = c(wg, z)
                + complex::exponential(pole * c(te, z), precision)?
                    * (pole * c(self.sine, z) - c(wg * self.cosine, z));
            let p3 = c(self.model.excitation, z)
                * complex::divide(
                    complex::exponential(c(z, -omega * te), precision)?,
                    c(z, e * ta * omega) * c(e, omega),
                )?;
            let p4 = if stability.into() < 1.0 {
                c(z, -e * ta * omega)
            } else {
                c(stability, z)
                    * (c(T::one(), z)
                        - complex::exponential(c(z, -omega * (self.period - te)), precision)?)
                    - c(z, e * ta * omega)
            };
            let response = p1 * p2 + p3 * p4;
            magnitude.push(phase::magnitude_of(response)?);
            phases.push(phase::argument_of(response)?);
        }
        Ok(Spectrum {
            magnitude,
            phase: phases,
        })
    }
    /// Samples begin at time zero; entries after the period remain positive zero.
    pub fn sampled_period(&self, sampling_rate: i32, count: usize) -> Result<Vec<T>, Error> {
        if sampling_rate <= 0 {
            return Err(Error("LF sampling rate must be positive"));
        }
        let mut output = vector::buffer(count)?;
        output.resize(count, T::zero());
        let rate = T::from_i32(sampling_rate).unwrap();
        let closure_index = (self.closure * rate / self.scale).into().round();
        let period_index = (self.period * rate / self.scale).into().round();
        if !closure_index.is_finite() || !period_index.is_finite() {
            return Err(Error("LF sample index overflows"));
        }
        let end_open = ((closure_index as usize).saturating_add(1)).min(count);
        let end_period = (period_index as usize).min(count);
        for (index, value) in output.iter_mut().enumerate().take(end_open) {
            let time = T::from_usize(index).unwrap() / rate * self.scale;
            // Approximation outputs are binary32; C promotes them to the scalar.
            *value = self.initial_excitation
                * T::from_f32(approximation::exponential(
                    (self.growth * time).into() as f32
                )?)
                .unwrap()
                * T::from_f32(approximation::sine(
                    (self.angular_frequency * time).into() as f32
                )?)
                .unwrap();
        }
        for (index, value) in output
            .iter_mut()
            .enumerate()
            .take(end_period)
            .skip(end_open)
        {
            let time = T::from_usize(index).unwrap() / rate * self.scale;
            let first =
                approximation::exponential((-self.decay * (time - self.closure)).into() as f32)?;
            let last = approximation::exponential(
                (-self.decay * (self.period - self.closure)).into() as f32,
            )?;
            // The subtraction is binary32 before multiplication by the scalar.
            *value = -self.model.excitation / self.decay / self.return_time
                * T::from_f32(first - last).unwrap();
        }
        vector::validate(&output)?;
        Ok(output)
    }
}
