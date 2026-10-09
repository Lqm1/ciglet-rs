//! Bracketed scalar zero finding with ciglet's interpolation and bisection.
use crate::Error;
use rustfft::FftNum;

fn evaluate<T: FftNum + Into<f64>>(function: &mut impl FnMut(T) -> T, x: T) -> Result<T, Error> {
    let value = function(x);
    if !value.into().is_finite() {
        return Err(Error("root function returned a nonfinite value"));
    }
    Ok(value)
}

/// Find a scalar zero with the source's absolute tolerance of 1e-8.
/// The closure can capture the environment formerly passed as a C pointer.
/// Exact endpoint roots are returned directly. A non-bracketing interval retains
/// ciglet's midpoint fallback. Nonfinite function values and failure to converge
/// within 4096 iterations return an error.
pub fn find<T: FftNum + Into<f64>>(
    mut function: impl FnMut(T) -> T,
    start: T,
    end: T,
) -> Result<T, Error> {
    if !start.into().is_finite() || !end.into().is_finite() || start.into() > end.into() {
        return Err(Error("root search requires finite ordered bounds"));
    }
    let epsilon = T::from_f64(1e-8).unwrap();
    let mut a = start;
    let mut b = end;
    let mut fa = evaluate(&mut function, a)?;
    let mut fb = evaluate(&mut function, b)?;
    if fa == T::zero() {
        return Ok(a);
    }
    if fb == T::zero() {
        return Ok(b);
    }
    if (fa.into() > 0.0) == (fb.into() > 0.0) {
        let sum = a + b;
        let midpoint = if sum.into().is_finite() {
            sum.into() / 2.0
        } else {
            a.into() * 0.5 + b.into() * 0.5
        };
        return Ok(T::from_f64(midpoint).unwrap());
    }
    if fa.into().abs() < fb.into().abs() {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut fa, &mut fb);
    }
    let mut c = a;
    let mut fc = fa;
    let mut d = c;
    let mut bisected = true;
    for _ in 0..4096 {
        if fb.into().abs() <= epsilon.into() || (a - b).into().abs() <= epsilon.into() {
            return Ok(b);
        }
        let mut candidate = if fa != fc && fb != fc {
            a * fb * fc / (fa - fb) / (fa - fc)
                + b * fa * fc / (fb - fa) / (fb - fc)
                + c * fa * fb / (fc - fa) / (fc - fb)
        } else {
            b - fb * (b - a) / (fb - fa)
        };
        if !candidate.into().is_finite()
            || candidate.into() < (3.0 * a.into() + b.into()) / 4.0
            || candidate.into() > b.into()
            || (bisected && (candidate - b).into().abs() >= (b - c).into().abs() * 0.5)
            || (!bisected && (candidate - b).into().abs() >= (c - d).into().abs() * 0.5)
            || (bisected && (b - c).into().abs() < epsilon.into())
            || (!bisected && (c - d).into().abs() < epsilon.into())
        {
            let sum = a + b;
            candidate = if sum.into().is_finite() {
                T::from_f64(sum.into() / 2.0).unwrap()
            } else {
                T::from_f64(a.into() * 0.5 + b.into() * 0.5).unwrap()
            };
            if candidate == a || candidate == b {
                return Ok(b);
            }
            bisected = true;
        } else {
            bisected = false;
        }
        let value = evaluate(&mut function, candidate)?;
        d = c;
        c = b;
        fc = fb;
        if (fa.into() > 0.0) != (value.into() > 0.0) {
            b = candidate;
            fb = value;
        } else {
            a = candidate;
            fa = value;
        }
        if fa.into().abs() < fb.into().abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }
    }
    Err(Error("root search exceeded its iteration limit"))
}
