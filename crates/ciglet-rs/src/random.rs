//! Owned random sampling and the original ciglet integer-draw transformations.
//!
//! C's `rand` sequence is platform dependent. These transformations preserve its
//! arithmetic, while [`Generator`] uses explicitly seeded ChaCha8 instead.
use crate::{Error, approximation, statistics::Scalar, vector};
use rand::{Rng, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Gaussian arithmetic; uniform and white-noise arithmetic are unchanged.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Policy {
    /// Double-precision logarithm/cosine, with a positive first draw.
    #[default]
    Stable,
    /// Original binary32 approximate logarithm/cosine, including at zero.
    Legacy,
}

fn maximum_valid(maximum: u32) -> Result<(), Error> {
    if maximum == 0 || maximum > i32::MAX as u32 {
        return Err(Error("random maximum must be in 1..=i32::MAX"));
    }
    Ok(())
}

fn unit<T: Scalar>(draw: u32, maximum: u32) -> Result<f64, Error> {
    maximum_valid(maximum)?;
    if draw > maximum {
        return Err(Error("random draw exceeds its maximum"));
    }
    // Division occurs in FP_TYPE before C promotes the result to double.
    Ok((T::from_u32(draw).unwrap() / T::from_u32(maximum).unwrap()).into())
}

/// Map an inclusive integer draw to [-1, 1], retaining selected-scalar division.
pub fn uniform_from_draw<T: Scalar>(draw: u32, maximum: u32) -> Result<T, Error> {
    vector::scalar((unit::<T>(draw, maximum)? - 0.5) * 2.0)
}

/// White-noise sample; amplitude may be positive, negative or zero.
pub fn white_from_draw<T: Scalar>(amplitude: T, draw: u32, maximum: u32) -> Result<T, Error> {
    vector::validate(&[amplitude])?;
    vector::scalar((unit::<T>(draw, maximum)? - 0.5) * amplitude.into() * 2.0)
}

fn parameters<T: Scalar>(mean: T, variance: T) -> Result<(), Error> {
    vector::validate(&[mean, variance])?;
    if variance.into() < 0.0 {
        return Err(Error("Gaussian variance must be nonnegative"));
    }
    Ok(())
}

/// Transform two inclusive integer draws; `variance` is not standard deviation.
/// Stable arithmetic rejects a zero first draw for positive variance. Legacy
/// retains the original finite fast-log artifact at that endpoint.
pub fn normal_from_draws<T: Scalar>(
    mean: T,
    variance: T,
    draws: [u32; 2],
    maximum: u32,
    policy: Policy,
) -> Result<T, Error> {
    parameters(mean, variance)?;
    let first = unit::<T>(draws[0], maximum)?;
    let second = unit::<T>(draws[1], maximum)?;
    let variance = variance.into();
    let angle = std::f64::consts::TAU * second;
    let deviation = match policy {
        Policy::Legacy => {
            let logarithm = approximation::logarithm_legacy_bits(first as f32) as f64;
            (-2.0 * logarithm * variance).sqrt() * approximation::cosine(angle as f32)? as f64
        }
        Policy::Stable => {
            if variance == 0.0 {
                return Ok(mean);
            }
            if first == 0.0 {
                return Err(Error("Gaussian first draw must be positive"));
            }
            // Avoid overflow in variance times the Box-Muller radial factor.
            (-2.0 * first.ln()).sqrt() * variance.sqrt() * angle.cos()
        }
    };
    vector::scalar(deviation + mean.into())
}

/// Caller-owned state with unbiased inclusive integer draws. Range sampling
/// itself may consume more than one underlying RNG word.
#[derive(Debug, Clone)]
pub struct Sampler<R> {
    rng: R,
    maximum: u32,
}

/// Portable named ChaCha algorithm; no operating-system entropy is requested.
pub type Generator = Sampler<ChaCha8Rng>;

impl Generator {
    /// Reproduce the stream from a 256-bit seed and the default i32 draw grid.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            rng: ChaCha8Rng::from_seed(seed),
            maximum: i32::MAX as u32,
        }
    }
}

impl<R: Rng> Sampler<R> {
    /// Supply an RNG and its inclusive output-grid maximum.
    pub fn new(rng: R, maximum: u32) -> Result<Self, Error> {
        maximum_valid(maximum)?;
        Ok(Self { rng, maximum })
    }

    pub fn into_rng(self) -> R {
        self.rng
    }

    pub fn maximum(&self) -> u32 {
        self.maximum
    }

    pub fn draw(&mut self) -> u32 {
        self.rng.random_range(0..=self.maximum)
    }

    pub fn uniform<T: Scalar>(&mut self) -> Result<T, Error> {
        let draw = self.draw();
        uniform_from_draw(draw, self.maximum)
    }

    pub fn normal<T: Scalar>(&mut self, mean: T, variance: T) -> Result<T, Error> {
        self.normal_with(mean, variance, Policy::Stable)
    }

    /// Legacy consumes exactly two grid draws. Stable redraws a zero first draw
    /// for positive variance, rejecting pathological RNGs after 128 attempts.
    /// Zero variance still consumes two draws. Invalid parameters consume none.
    pub fn normal_with<T: Scalar>(
        &mut self,
        mean: T,
        variance: T,
        policy: Policy,
    ) -> Result<T, Error> {
        parameters(mean, variance)?;
        let mut first = self.draw();
        if policy == Policy::Stable && variance.into() > 0.0 {
            for _ in 1..128 {
                if first != 0 {
                    break;
                }
                first = self.draw();
            }
            if first == 0 {
                return Err(Error("Gaussian generator repeatedly returned zero"));
            }
        }
        let second = self.draw();
        normal_from_draws(mean, variance, [first, second], self.maximum, policy)
    }

    /// One grid draw per output, even for zero amplitude. Validation and
    /// allocation precede drawing; arithmetic errors may advance the stream.
    pub fn white_noise<T: Scalar>(&mut self, amplitude: T, count: usize) -> Result<Vec<T>, Error> {
        vector::validate(&[amplitude])?;
        let mut output = vector::buffer(count)?;
        for _ in 0..count {
            let draw = self.draw();
            output.push(white_from_draw(amplitude, draw, self.maximum)?);
        }
        Ok(output)
    }
}
