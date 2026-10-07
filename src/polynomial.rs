//! Descending-order polynomial coefficients and simultaneous complex roots.
use crate::{Error, approximation, fourier::Complex};
use rustfft::FftNum;

fn finite<T: FftNum + Into<f64>>(value: Complex<T>) -> bool {
    value.re.into().is_finite() && value.im.into().is_finite()
}

fn validate<T: FftNum + Into<f64>>(coefficients: &[Complex<T>]) -> Result<(), Error> {
    if coefficients.is_empty() || coefficients.len() > i32::MAX as usize {
        return Err(Error(
            "polynomial requires a nonempty bounded coefficient array",
        ));
    }
    if coefficients.iter().any(|&coefficient| !finite(coefficient)) {
        return Err(Error("polynomial coefficients must be finite"));
    }
    Ok(())
}

fn evaluate_inner<T: FftNum>(coefficients: &[Complex<T>], x: Complex<T>) -> Complex<T> {
    let mut value = coefficients[coefficients.len() - 1];
    let mut power = x;
    for &coefficient in coefficients[..coefficients.len() - 1].iter().rev() {
        value = value + coefficient * power;
        power = power * x;
    }
    value
}

/// Evaluate complex coefficients in descending order, preserving C's power
/// accumulation rather than changing rounding through Horner evaluation.
pub fn evaluate<T: FftNum + Into<f64>>(
    coefficients: &[Complex<T>],
    x: Complex<T>,
) -> Result<Complex<T>, Error> {
    validate(coefficients)?;
    if !finite(x) {
        return Err(Error("polynomial argument must be finite"));
    }
    let value = evaluate_inner(coefficients, x);
    if !finite(value) {
        return Err(Error("polynomial evaluation overflowed"));
    }
    Ok(value)
}

/// Evaluate real coefficients at a complex argument without allocating a copy.
pub fn evaluate_real<T: FftNum + Into<f64>>(
    coefficients: &[T],
    x: Complex<T>,
) -> Result<Complex<T>, Error> {
    if coefficients.is_empty()
        || coefficients.len() > i32::MAX as usize
        || coefficients.iter().any(|&value| !value.into().is_finite())
        || !finite(x)
    {
        return Err(Error(
            "real polynomial requires finite nonempty coefficients and argument",
        ));
    }
    let mut value = Complex::new(coefficients[coefficients.len() - 1], T::zero());
    let mut power = x;
    for &coefficient in coefficients[..coefficients.len() - 1].iter().rev() {
        value = value + Complex::new(coefficient, T::zero()) * power;
        power = power * x;
    }
    if !finite(value) {
        return Err(Error("polynomial evaluation overflowed"));
    }
    Ok(value)
}

/// Root estimates retain C's iteration order. Convergence is reported rather
/// than silently treating the 50-iteration limit as a successful solve.
#[derive(Debug, Clone)]
pub struct RootEstimate<T> {
    pub values: Vec<Complex<T>>,
    pub iterations: usize,
    /// Whether all returned roots have normalized residual below 1e-6.
    pub converged: bool,
    /// Largest residual of the returned roots in the normalized polynomial.
    pub maximum_residual: f64,
}

fn magnitude<T: FftNum + Into<f64>>(value: Complex<T>) -> T {
    T::from_f64((value.re * value.re + value.im * value.im).into().sqrt()).unwrap()
}

fn divide<T: FftNum + Into<f64>>(a: Complex<T>, b: Complex<T>) -> Result<Complex<T>, Error> {
    let denominator = b.re * b.re + b.im * b.im;
    let value = Complex::new(
        (a.re * b.re + a.im * b.im) / denominator,
        (a.im * b.re - a.re * b.im) / denominator,
    );
    if !finite(value) {
        return Err(Error("polynomial root division is singular or overflowed"));
    }
    Ok(value)
}

/// Simultaneous Durand-Kerner iteration with the source's initial points,
/// level-one trigonometry, radius clamp, 50 steps and 1e-6 residual threshold.
/// Constant nonzero polynomials have no roots. A zero leading coefficient is
/// rejected. High-degree, repeated or ill-conditioned roots may not converge.
pub fn roots<T: FftNum + Into<f64>>(coefficients: &[Complex<T>]) -> Result<RootEstimate<T>, Error> {
    validate(coefficients)?;
    let zero = Complex::new(T::zero(), T::zero());
    let one = Complex::new(T::one(), T::zero());
    if coefficients[0] == zero {
        return Err(Error("polynomial leading coefficient must be nonzero"));
    }
    let degree = coefficients.len() - 1;
    if degree == 0 {
        return Ok(RootEstimate {
            values: Vec::new(),
            iterations: 0,
            converged: true,
            maximum_residual: 0.0,
        });
    }
    let normalized = coefficients
        .iter()
        .map(|&value| divide(value, coefficients[0]))
        .collect::<Result<Vec<_>, _>>()?;
    let mut radius = T::zero();
    for &value in &normalized {
        let length = magnitude(value);
        if !length.into().is_finite() {
            return Err(Error("polynomial root radius overflowed"));
        }
        if length.into() > radius.into() {
            radius = length;
        }
    }
    radius = radius + T::one();
    let mut previous = vec![one; degree];
    let mut next = vec![zero; degree];
    if degree > 2 {
        let angle = (std::f64::consts::TAU / degree as f64) as f32;
        let rotation = Complex::new(
            T::from_f32(approximation::cosine_fast(angle)?).unwrap(),
            T::from_f32(approximation::sine_fast(angle)?).unwrap(),
        );
        for i in 1..degree {
            previous[i] = previous[i - 1] * rotation;
            next[i] = previous[i];
        }
    } else {
        for i in 1..degree {
            previous[i] = Complex::new(T::from_f64(0.4).unwrap(), T::from_f64(0.6).unwrap());
            next[i] = previous[i];
        }
    }
    let mut iterations = 0;
    for _ in 0..50 {
        iterations += 1;
        let mut maximum = T::zero();
        for (i, &point) in previous.iter().enumerate() {
            let value = evaluate_inner(&normalized, point);
            let error = magnitude(value);
            if !finite(value) || !error.into().is_finite() {
                return Err(Error("polynomial root evaluation overflowed"));
            }
            if error.into() > maximum.into() {
                maximum = error;
            }
            let mut denominator = one;
            for (j, &other) in previous.iter().enumerate() {
                if i != j {
                    denominator = denominator * (point - other);
                }
            }
            // An exact root needs no update, including colliding exact roots.
            next[i] = if value == zero {
                point
            } else {
                point - divide(value, denominator)?
            };
            if !finite(next[i]) {
                return Err(Error("polynomial root update overflowed"));
            }
        }
        if maximum.into() < 1e-6 {
            break;
        }
        for point in &mut next {
            let length = magnitude(*point);
            if !length.into().is_finite() {
                return Err(Error("polynomial root magnitude overflowed"));
            }
            if length.into() > radius.into() {
                let scale = radius / length;
                point.re = point.re * scale;
                point.im = point.im * scale;
            }
        }
        previous.copy_from_slice(&next);
    }
    let mut maximum_residual = 0.0f64;
    for &point in &next {
        let residual: f64 = magnitude(evaluate_inner(&normalized, point)).into();
        if !residual.is_finite() {
            return Err(Error("polynomial final residual overflowed"));
        }
        maximum_residual = maximum_residual.max(residual);
    }
    Ok(RootEstimate {
        values: next,
        iterations,
        converged: maximum_residual < 1e-6,
        maximum_residual,
    })
}

/// Find roots of real coefficients using the same complex solver.
pub fn roots_real<T: FftNum + Into<f64>>(coefficients: &[T]) -> Result<RootEstimate<T>, Error> {
    let complex: Vec<_> = coefficients
        .iter()
        .map(|&value| Complex::new(value, T::zero()))
        .collect();
    roots(&complex)
}
