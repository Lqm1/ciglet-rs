use ciglet_rs::{
    approximation::{self, Precision},
    complex::{self, Complex},
    statistics::Scalar,
};

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> u32 {
        let value =
            u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        value
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        let value = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(value).unwrap()
    }
    fn point<T: Scalar>(&mut self) -> Complex<T> {
        Complex::new(self.scalar(), self.scalar())
    }
}
fn exact<T: Scalar>(actual: T, expected: T) {
    assert_eq!(
        actual.into().to_bits(),
        expected.into().to_bits(),
        "{actual:?} != {expected:?}"
    );
}
fn compare_host<T: Scalar>(actual: T, expected: T, maximum: &mut f64, maximum_ulp: &mut u64) {
    let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
    assert!(error.is_finite());
    *maximum = maximum.max(error);
    let distance = if std::mem::size_of::<T>() == 8 {
        actual.into().to_bits().abs_diff(expected.into().to_bits())
    } else {
        u64::from(
            (actual.into() as f32)
                .to_bits()
                .abs_diff((expected.into() as f32).to_bits()),
        )
    };
    *maximum_ulp = (*maximum_ulp).max(distance);
    if actual == T::zero() || expected == T::zero() {
        exact(actual, expected);
    }
}
fn reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"CPX1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer() as usize;
    let count = reader.integer();
    let mut maximum = [0.0f64; 4];
    let mut maximum_ulp = [0u64; 4];
    for _ in 0..count {
        let first = reader.point::<T>();
        let second = reader.point::<T>();
        let actual = [
            first.conj(),
            first + second,
            first - second,
            first * second,
            complex::divide(first, second).unwrap(),
            complex::exponential(first, Precision::Fast).unwrap(),
            complex::exponential(first, Precision::Approximate).unwrap(),
            complex::exponential(first, Precision::Exact).unwrap(),
        ];
        for (field, value) in actual.into_iter().enumerate() {
            let expected = reader.point::<T>();
            if field == 7 {
                compare_host(value.re, expected.re, &mut maximum[0], &mut maximum_ulp[0]);
                compare_host(value.im, expected.im, &mut maximum[0], &mut maximum_ulp[0]);
            } else {
                exact(value.re, expected.re);
                exact(value.im, expected.im);
            }
        }
        exact(
            complex::argument(first, Precision::Fast).unwrap(),
            reader.scalar(),
        );
        compare_host(
            complex::argument(first, Precision::Exact).unwrap(),
            reader.scalar(),
            &mut maximum[1],
            &mut maximum_ulp[1],
        );
    }
    let scalars = reader.integer();
    for _ in 0..scalars {
        let value = reader.scalar::<T>();
        let exponent = reader.scalar::<T>();
        for (field, precision) in [Precision::Fast, Precision::Approximate, Precision::Exact]
            .into_iter()
            .enumerate()
        {
            let actual = approximation::logarithm_with(value, precision).unwrap();
            let expected = reader.scalar::<T>();
            if field == 2 {
                compare_host(actual, expected, &mut maximum[2], &mut maximum_ulp[2]);
            } else {
                exact(actual, expected);
            }
        }
        for (field, precision) in [Precision::Fast, Precision::Approximate, Precision::Exact]
            .into_iter()
            .enumerate()
        {
            let actual = approximation::exponential_with(exponent, precision).unwrap();
            let expected = reader.scalar::<T>();
            if field == 2 {
                compare_host(actual, expected, &mut maximum[3], &mut maximum_ulp[3]);
            } else {
                exact(actual, expected);
            }
        }
        exact(
            T::from_f32(approximation::logarithm2_fast(value.into() as f32).unwrap()).unwrap(),
            reader.scalar(),
        );
        exact(
            T::from_f32(approximation::exponential2_fast(exponent.into() as f32).unwrap()).unwrap(),
            reader.scalar(),
        );
    }
    assert_eq!(reader.cursor, bytes.len());
    println!(
        "{}-byte libm maximum normalized errors (complex exp, argument, log, exp): {maximum:?}",
        reader.width
    );
    println!("host libm maximum ULP distances: {maximum_ulp:?}");
    let epsilon = if reader.width == 4 {
        f64::from(f32::EPSILON)
    } else {
        f64::EPSILON
    };
    for (field, (&error, &ulp)) in maximum.iter().zip(&maximum_ulp).enumerate() {
        assert!(
            ulp <= 1 && error <= epsilon,
            "host libm field {field}: {ulp} ULP, normalized error {error}"
        );
    }
}
#[test]
fn c_float_reference() {
    reference::<f32>(include_bytes!("../../../tests/fixtures/complex-float.bin"));
}
#[test]
fn c_double_reference() {
    reference::<f64>(include_bytes!("../../../tests/fixtures/complex-double.bin"));
}
#[test]
fn division_range_loss_and_precision_boundaries() {
    let largest = Complex::new(f64::MAX, f64::MAX);
    assert_eq!(
        complex::divide(largest, largest).unwrap(),
        Complex::new(1.0, 0.0)
    );
    assert_eq!(
        complex::divide(largest, Complex::new(1.0, 1.0)).unwrap(),
        Complex::new(f64::MAX, 0.0)
    );
    let tiny = Complex::new(f64::from_bits(1), f64::from_bits(1));
    assert_eq!(complex::divide(tiny, tiny).unwrap(), Complex::new(1.0, 0.0));
    let quotient =
        complex::divide(Complex::new(1e-250f64, 0.0), Complex::new(1e-100, 0.0)).unwrap();
    assert!((quotient.re / 1e-150 - 1.0).abs() < 4.0 * f64::EPSILON);
    let largest = Complex::new(f32::MAX, f32::MAX);
    assert_eq!(
        complex::divide(largest, largest).unwrap(),
        Complex::new(1.0, 0.0)
    );
    let tiny = Complex::new(f32::from_bits(1), f32::from_bits(1));
    assert_eq!(complex::divide(tiny, tiny).unwrap(), Complex::new(1.0, 0.0));
    assert!(complex::divide(Complex::new(1.0f32, 0.0), Complex::new(0.0, 0.0)).is_err());
    assert!(complex::divide(Complex::new(f64::INFINITY, 0.0), Complex::new(1.0, 1.0)).is_err());
    assert_eq!(
        complex::exponential(Complex::new(0.0f64, 0.0), Precision::Exact).unwrap(),
        Complex::new(1.0, 0.0)
    );
    assert_eq!(
        approximation::atan2_fast(-0.0f64, -1.0).unwrap(),
        std::f64::consts::PI
    );
    assert_eq!(
        approximation::atan2_fast(f64::MAX, f64::from_bits(1)).unwrap(),
        std::f64::consts::FRAC_PI_2
    );
    assert!(approximation::atan2_fast(f32::NAN, 1.0).is_err());
    assert!(approximation::logarithm_fast(0.0).is_err());
    assert!(approximation::logarithm_with(-1.0f64, Precision::Exact).is_err());
    assert!(approximation::exponential2_fast(128.0).is_err());
    assert!(complex::exponential(Complex::new(1000.0f64, 0.0), Precision::Exact).is_err());
    assert_eq!(
        approximation::exponential2_fast(-200.0).unwrap(),
        approximation::exponential2_fast(-126.0).unwrap()
    );
}
