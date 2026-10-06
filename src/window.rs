//! High-precision upstream window definitions.
//! Hann/Hamming are symmetric; Blackman/Harris/Nuttall use a periodic divisor.
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    Boxcar,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    Nuttall98,
    MltSine,
}

impl Window {
    /// Generate a window. Singleton windows are defined as unity; empty windows
    /// are empty. This avoids the original symmetric-window division by zero.
    pub fn generate(self, length: usize) -> Vec<f32> {
        if length == 1 {
            return vec![1.0];
        }
        (0..length)
            .map(|index| {
                let periodic_angle = 2.0 * PI * index as f64 / length as f64;
                match self {
                    Self::Boxcar => 1.0,
                    Self::Hann => {
                        (0.5 * (1.0 - (2.0 * PI * index as f64 / (length - 1) as f64).cos())) as f32
                    }
                    Self::Hamming => {
                        (0.54 - 0.46 * (2.0 * PI * index as f64 / (length - 1) as f64).cos()) as f32
                    }
                    Self::MltSine => (PI / length as f64 * (index as f64 + 0.5)).sin() as f32,
                    Self::Blackman => cosine_sum(periodic_angle, &[0.42_f32, 0.5, 0.08]),
                    Self::BlackmanHarris => {
                        cosine_sum(periodic_angle, &[0.35875_f32, 0.48829, 0.14128, 0.01168])
                    }
                    Self::Nuttall98 => cosine_sum(
                        periodic_angle,
                        &[0.3635819_f32, 0.4891775, 0.1365995, 0.0106411],
                    ),
                }
            })
            .collect()
    }
}

fn cosine_sum(angle: f64, coefficients: &[f32]) -> f32 {
    coefficients
        .iter()
        .enumerate()
        .map(|(order, &coefficient)| {
            let sign = if order % 2 == 0 { 1.0 } else { -1.0 };
            sign * f64::from(coefficient) * (angle * order as f64).cos()
        })
        .sum::<f64>() as f32
}
