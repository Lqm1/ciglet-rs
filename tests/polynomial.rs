use ciglet_rs::{approximation, fourier::Complex, polynomial};
use rustfft::FftNum;

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> usize {
        let n = u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        n as usize
    }
    fn scalar<T: FftNum>(&mut self) -> T {
        let n = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(n).unwrap()
    }
    fn complex<T: FftNum>(&mut self) -> Complex<T> {
        Complex::new(self.scalar(), self.scalar())
    }
}
fn check<T: FftNum + Into<f64>>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"POL1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer();
    let cases = reader.integer();
    for case in 0..cases {
        let n = reader.integer();
        let coefficients: Vec<_> = (0..n).map(|_| reader.complex::<T>()).collect();
        for _ in 0..3 {
            let x = reader.complex();
            let expected = reader.complex();
            assert_eq!(
                polynomial::evaluate(&coefficients, x).unwrap(),
                expected,
                "evaluation {case}"
            );
            if coefficients.iter().all(|c| c.im == T::zero()) {
                let real: Vec<_> = coefficients.iter().map(|c| c.re).collect();
                assert_eq!(polynomial::evaluate_real(&real, x).unwrap(), expected);
            }
        }
        let expected: Vec<_> = (0..n - 1).map(|_| reader.complex::<T>()).collect();
        let result = polynomial::roots(&coefficients).unwrap();
        assert_eq!(result.values, expected, "roots {case}");
        assert!(result.iterations <= 50 && result.iterations > 0);
        assert!(result.maximum_residual.is_finite());
        assert_eq!(result.converged, result.maximum_residual < 1e-6);
        if coefficients.iter().all(|c| c.im == T::zero()) {
            let real: Vec<_> = coefficients.iter().map(|c| c.re).collect();
            assert_eq!(polynomial::roots_real(&real).unwrap().values, result.values);
        }
    }
    let trig = reader.integer();
    for _ in 0..trig {
        let x: f64 = reader.scalar();
        let sine: f64 = reader.scalar();
        let cosine: f64 = reader.scalar();
        assert_eq!(approximation::sine_fast(x as f32).unwrap() as f64, sine);
        assert_eq!(approximation::cosine_fast(x as f32).unwrap() as f64, cosine);
    }
    assert_eq!(reader.cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!("fixtures/polynomial-float.bin"));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!("fixtures/polynomial-double.bin"));
}
#[test]
fn constants_known_roots_and_invalid_inputs() {
    let z = Complex::new(0.0f64, 0.0);
    let one = Complex::new(1.0, 0.0);
    assert_eq!(polynomial::evaluate(&[one], z).unwrap(), one);
    assert!(polynomial::roots(&[one]).unwrap().values.is_empty());
    assert!(polynomial::evaluate::<f64>(&[], z).is_err());
    assert!(polynomial::roots(&[z, one]).is_err());
    assert!(polynomial::roots_real(&[f32::NAN, 1.0]).is_err());
    assert!(polynomial::evaluate_real(&[1.0], Complex::new(f64::INFINITY, 0.0)).is_err());
    let result = polynomial::roots_real(&[1.0f64, 0.0, 1.0]).unwrap();
    assert!(result.converged);
    assert!(result.maximum_residual < 1e-6);
    for root in result.values {
        assert!(root.re.abs() < 1e-6);
        assert!((root.im.abs() - 1.0).abs() < 1e-6);
    }
    assert_eq!(
        polynomial::roots_real(&[1.0f64, -1.0]).unwrap().values,
        vec![one]
    );
    assert!(approximation::sine_fast(f32::NAN).is_err());
    assert!(approximation::cosine_fast(f32::INFINITY).is_err());
}

#[test]
fn limited_iteration_report() {
    let coefficients = [
        1.0f64, -36.0, 546.0, -4536.0, 22449.0, -67284.0, 118124.0, -109584.0, 40320.0,
    ];
    let result = polynomial::roots_real(&coefficients).unwrap();
    assert_eq!(result.values.len(), 8);
    assert_eq!(result.iterations, 50);
    assert!(!result.converged);
    assert!(result.maximum_residual > 1e-6);
    let independently_checked = result
        .values
        .iter()
        .map(|&x| {
            // This polynomial is the product of (x-k), k=1 through 8.
            let product = (1..=8).fold(Complex::new(1.0, 0.0), |p, k| {
                p * (x - Complex::new(k as f64, 0.0))
            });
            product.re.hypot(product.im)
        })
        .fold(0.0f64, f64::max);
    assert!((independently_checked - result.maximum_residual).abs() / independently_checked < 1e-8);
}
