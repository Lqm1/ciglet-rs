use ciglet_rs::{
    approximation::{self, Precision},
    scalar,
    statistics::Scalar,
};
struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> usize {
        let value =
            u32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap());
        self.cursor += 4;
        value as usize
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        let bytes = &self.bytes[self.cursor..self.cursor + self.width];
        let value = if self.width == 4 {
            f32::from_le_bytes(bytes.try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(bytes.try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(value).unwrap()
    }
}
fn exact<T: Scalar>(a: T, b: T) {
    assert_eq!(a.into().to_bits(), b.into().to_bits());
}
fn host<T: Scalar>(a: T, b: T, error: &mut f64, ulp: &mut u64) {
    if a == T::zero() || b == T::zero() {
        exact(a, b);
        return;
    }
    let difference = (a.into() - b.into()).abs() / (1.0 + b.into().abs());
    *error = error.max(difference);
    let distance = a.into().to_bits().abs_diff(b.into().to_bits());
    *ulp = (*ulp).max(distance);
    assert!(
        difference <= f64::EPSILON && distance <= 1,
        "{a:?} != {b:?}"
    );
}
fn reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"SCM1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer();
    let cases = reader.integer();
    let mut maximum = [0.0; 3];
    let mut ulps = [0; 3];
    for _ in 0..cases {
        let angle: T = reader.scalar();
        let y: T = reader.scalar();
        let x: T = reader.scalar();
        for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
            for (field, actual) in [
                approximation::sine_with(angle, precision).unwrap(),
                approximation::cosine_with(angle, precision).unwrap(),
            ]
            .into_iter()
            .enumerate()
            {
                let expected = reader.scalar();
                if precision == Precision::Exact {
                    host(actual, expected, &mut maximum[field], &mut ulps[field]);
                } else {
                    exact(actual, expected);
                }
            }
        }
        for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
            let actual = approximation::atan2_with(y, x, precision).unwrap();
            let expected = reader.scalar();
            if precision == Precision::Fast {
                exact(actual, expected);
            } else {
                host(actual, expected, &mut maximum[2], &mut ulps[2]);
            }
        }
    }
    let pairs = reader.integer();
    for _ in 0..pairs {
        let a: T = reader.scalar();
        let b: T = reader.scalar();
        exact(scalar::minimum(a, b), reader.scalar());
        exact(scalar::maximum(a, b), reader.scalar());
    }
    assert_eq!(reader.cursor, bytes.len());
    println!(
        "{}-byte scalar math host errors {maximum:?}, ULPs {ulps:?}",
        reader.width
    );
}
#[test]
fn c_float_reference() {
    reference::<f32>(include_bytes!("fixtures/scalar-math-float.bin"));
}
#[test]
fn c_double_reference() {
    reference::<f64>(include_bytes!("fixtures/scalar-math-double.bin"));
}
#[test]
fn precision_and_comparison_boundaries() {
    // The source bit approximation rounds the last exponent below 128 to
    // infinity. A checked scalar result must reject it, like the lane API.
    assert!(approximation::exponential2(f32::from_bits(0x42ff_ffff)).is_err());
    assert!(approximation::exponential2(127.75).unwrap().is_finite());
    assert!(
        approximation::exponential2_fast(f32::from_bits(0x42ff_ffff))
            .unwrap()
            .is_finite()
    );
    assert_eq!(
        approximation::sine_with(-0.0_f64, Precision::Exact)
            .unwrap()
            .to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(
        approximation::cosine_with(0.0_f64, Precision::Exact).unwrap(),
        1.0
    );
    assert_eq!(
        approximation::atan2_with(-0.0_f64, -0.0, Precision::Exact).unwrap(),
        -std::f64::consts::PI
    );
    assert_eq!(
        approximation::atan2_with(1.0_f64, -1.0, Precision::Approximate).unwrap(),
        3.0 * std::f64::consts::FRAC_PI_4
    );
    for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
        assert!(approximation::sine_with(f64::NAN, precision).is_err());
        assert!(approximation::cosine_with(f64::INFINITY, precision).is_err());
        assert!(approximation::atan2_with(f64::NAN, 1.0, precision).is_err());
    }
    assert!(approximation::sine_with(f64::MAX, Precision::Fast).is_err());
    assert!(approximation::cosine_with(f64::MAX, Precision::Approximate).is_err());
    assert!(
        approximation::sine_with(f64::MAX, Precision::Exact)
            .unwrap()
            .is_finite()
    );
    assert_eq!(scalar::minimum(3, 2), 2);
    assert_eq!(
        scalar::maximum(String::from("alpha"), String::from("beta")),
        "beta"
    );
    assert_eq!(
        scalar::minimum(0.0_f32, -0.0).to_bits(),
        (-0.0_f32).to_bits()
    );
    assert_eq!(scalar::maximum(-0.0_f64, 0.0).to_bits(), 0.0_f64.to_bits());
}
