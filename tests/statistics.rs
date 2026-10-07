use ciglet_rs::statistics::{self as stats, Scalar};
use rustfft::FftNum;
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
    fn scalar<T: FftNum>(&mut self) -> T {
        let value = if self.width == 4 {
            f32::from_le_bytes(self.bytes[self.cursor..self.cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes[self.cursor..self.cursor + 8].try_into().unwrap())
        };
        self.cursor += self.width;
        T::from_f64(value).unwrap()
    }
    fn vector<T: FftNum>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.scalar()).collect()
    }
}
fn check<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"STA1");
    let mut r = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    r.width = r.integer();
    let count = r.integer();
    for case in 0..count {
        let n = r.integer();
        let lags = r.integer();
        let x = r.vector::<T>(n);
        let y = r.vector::<T>(n);
        let expected = r.vector::<T>(9);
        let actual = [
            stats::sum(&x),
            stats::sum_squares(&x),
            stats::mean_with(&x, stats::Policy::Legacy).unwrap(),
            stats::variance_with(&x, stats::Policy::Legacy).unwrap(),
            stats::minimum(&x).unwrap(),
            stats::maximum(&x).unwrap(),
            stats::median(&x).unwrap(),
            stats::dot(&x, &y).unwrap(),
            stats::covariance_with(&x, &y, stats::Policy::Legacy).unwrap(),
        ];
        assert_eq!(actual.as_slice(), expected, "statistics case {case}");
        let valid = r.integer() != 0;
        let correlation = r.scalar::<T>();
        if valid {
            assert_eq!(
                stats::correlation_with(&x, &y, stats::Policy::Legacy).unwrap(),
                correlation,
                "correlation case {case}"
            );
        } else {
            assert!(
                stats::correlation_with(&x, &y, stats::Policy::Legacy).is_err(),
                "undefined correlation {case}"
            );
        }
        for index in 0..n {
            let expected = r.scalar::<T>();
            let mut copy = x.clone();
            assert_eq!(stats::select_nth(&mut copy, index).unwrap(), expected);
        }
        let sorted = r.vector::<T>(n);
        let indices: Vec<_> = (0..n).map(|_| r.integer()).collect();
        assert_eq!(stats::sort(&x), (sorted, indices));
        let cross = r.vector::<T>(lags);
        assert_eq!(stats::cross_correlation(&x, &y, lags).unwrap(), cross);
    }
    assert_eq!(r.cursor, bytes.len());
}
#[test]
fn c_float() {
    check::<f32>(include_bytes!("fixtures/statistics-float.bin"));
}
#[test]
fn c_double() {
    check::<f64>(include_bytes!("fixtures/statistics-double.bin"));
}
#[test]
fn ieee_precision_degenerate_inputs_and_ordering() {
    assert_eq!(stats::sum::<f32>(&[]).to_bits(), 0);
    assert_eq!(stats::sum(&[-0.0f64]).to_bits(), 0);
    assert_eq!(stats::dot(&[-0.0f32], &[1.0]).unwrap().to_bits(), 0);
    assert_eq!(stats::sum(&[16_777_216.0f64, 1.0]), 16_777_217.0);
    assert_eq!(stats::sum(&[16_777_216.0f32, 1.0]), 16_777_216.0);
    assert_eq!(stats::median(&[f32::MAX, f32::MAX]).unwrap(), f32::MAX);
    assert_eq!(stats::median(&[f64::MAX, f64::MAX]).unwrap(), f64::MAX);
    let values = [
        3.0f64,
        -0.0,
        0.0,
        3.0,
        f64::NAN,
        f64::NEG_INFINITY,
        f64::INFINITY,
    ];
    let (sorted, indices) = stats::sort(&values);
    assert_eq!(indices, vec![5, 1, 2, 0, 3, 6, 4]);
    assert_eq!(sorted[1].to_bits(), (-0.0f64).to_bits());
    assert!(sorted[6].is_nan());
    let mut values = values;
    assert!(stats::select_nth(&mut values, 6).unwrap().is_nan());
    assert!(stats::correlation(&[1.0f64, 1.0], &[1.0, 2.0]).is_err());
    assert!(stats::correlation(&[f64::NAN, 1.0], &[1.0, 2.0]).is_err());
    assert!(stats::cross_correlation(&[1.0f32], &[1.0], usize::MAX).is_err());
    assert_eq!(
        stats::cross_correlation::<f64>(&[], &[], 3).unwrap(),
        vec![0.0; 3]
    );
    assert!(stats::minimum::<f64>(&[]).is_none());
    assert!(stats::maximum::<f32>(&[]).is_none());
    assert_eq!(stats::minimum(&[1.0f64, f64::NAN, 2.0]).unwrap(), 2.0);
    assert!(stats::maximum(&[1.0f32, f32::NAN]).unwrap().is_nan());
}

fn stable_reference<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"SMT1");
    let mut reader = Reader {
        bytes,
        cursor: 4,
        width: 0,
    };
    reader.width = reader.integer();
    assert_eq!(reader.integer(), 48);
    let mut maximum_error = [0.0_f64; 4];
    for case in 0..48 {
        let n = reader.integer();
        let left = reader.vector::<T>(n);
        let right = reader.vector::<T>(n);
        let legacy = [
            stats::mean_with(&left, stats::Policy::Legacy),
            stats::variance_with(&left, stats::Policy::Legacy),
            stats::covariance_with(&left, &right, stats::Policy::Legacy),
            stats::correlation_with(&left, &right, stats::Policy::Legacy),
        ];
        for (operation, actual) in legacy.iter().enumerate() {
            let code = reader.integer();
            let expected = reader.scalar::<T>();
            if code == 4 {
                assert!(actual.is_err(), "legacy correlation case {case}");
            } else {
                let actual = (*actual).unwrap();
                match code {
                    0 => assert_eq!(actual, expected, "legacy case {case}/{operation}"),
                    1 => assert_eq!(actual.into(), f64::INFINITY),
                    2 => assert_eq!(actual.into(), f64::NEG_INFINITY),
                    3 => assert!(actual.into().is_nan()),
                    _ => panic!("invalid source classification"),
                }
            }
        }
        let actual = [
            stats::mean(&left),
            stats::variance(&left),
            stats::covariance(&left, &right),
            stats::correlation(&left, &right),
        ];
        for (operation, actual) in actual.into_iter().enumerate() {
            let valid = reader.integer() != 0;
            let expected: f64 = reader.scalar::<T>().into();
            if !valid {
                assert!(
                    actual.is_err(),
                    "unrepresentable/undefined case {case}/{operation}"
                );
                continue;
            }
            let actual: f64 = actual.unwrap().into();
            if expected == 0.0 {
                assert_eq!(actual, expected, "zero moment case {case}/{operation}");
            } else {
                let error = (actual / expected - 1.0).abs();
                maximum_error[operation] = maximum_error[operation].max(error);
                assert!(
                    error <= if reader.width == 4 { 2e-7 } else { 4e-15 },
                    "stable case {case}/{operation}: {actual:e} vs {expected:e}, relative error {error:e}"
                );
            }
        }
    }
    assert_eq!(reader.cursor, bytes.len());
    eprintln!(
        "Stable statistics {}-byte maximum relative errors {maximum_error:?}",
        reader.width
    );
}

#[test]
fn stable_float_matches_independent_rational_moments() {
    stable_reference::<f32>(include_bytes!("fixtures/stable_statistics-float.bin"));
}
#[test]
fn stable_double_matches_independent_rational_moments() {
    stable_reference::<f64>(include_bytes!("fixtures/stable_statistics-double.bin"));
}
#[test]
fn stable_extremes_recover_representable_moments() {
    assert_eq!(stats::mean(&[f64::MAX, -f64::MAX, 1.0]).unwrap(), 1.0 / 3.0);
    assert_eq!(stats::mean(&[f64::MAX; 7]).unwrap(), f64::MAX);
    assert_eq!(stats::variance(&[f64::MAX; 7]).unwrap(), 0.0);
    let offset = [1e15 - 3.0, 1e15 - 1.0, 1e15 + 1.0, 1e15 + 3.0];
    assert_eq!(stats::variance(&offset).unwrap(), 5.0);
    let maximum = [f64::MAX, -f64::MAX];
    assert!(stats::variance(&maximum).is_err());
    assert!((stats::correlation(&maximum, &maximum).unwrap() - 1.0).abs() <= 4e-16);
    let tiny = f64::from_bits(1);
    assert_eq!(stats::variance(&[-tiny, tiny]).unwrap(), 0.0);
    assert!((stats::correlation(&[-tiny, tiny], &[-tiny, tiny]).unwrap() - 1.0).abs() <= 4e-16);
    assert_eq!(
        stats::covariance(&[-tiny, tiny], &[-f64::MAX, f64::MAX]).unwrap(),
        tiny * f64::MAX
    );
    assert!(stats::mean::<f64>(&[]).is_err());
    assert!(stats::variance(&[f64::INFINITY]).is_err());
    assert!(stats::covariance(&[1.0], &[f64::NAN]).is_err());
    assert!(stats::covariance::<f64>(&[], &[]).is_err());
    assert!(stats::correlation(&[1.0], &[1.0, 2.0]).is_err());
}

#[test]
fn stable_moments_preserve_constant_bounds_and_affine_relations() {
    for length in [1, 2, 3, 7, 17, 65, 1024] {
        for value in [f64::MAX, -f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
            let values = vec![value; length];
            assert_eq!(
                stats::mean(&values).unwrap(),
                value,
                "constant length {length}"
            );
            assert_eq!(stats::variance(&values).unwrap(), 0.0);
            assert_eq!(stats::covariance(&values, &values).unwrap(), 0.0);
            assert!(stats::correlation(&values, &values).is_err());
        }
    }
    let left: Vec<f64> = (0..65).map(|i| (i * 37 % 127) as f64 - 63.0).collect();
    let right: Vec<f64> = left.iter().map(|&v| 1e15 - 8.0 * v).collect();
    assert!((stats::correlation(&left, &right).unwrap() + 1.0).abs() < 5e-16);
    let expected = stats::variance(&left).unwrap();
    assert!((stats::covariance(&left, &right).unwrap() / (-8.0 * expected) - 1.0).abs() < 5e-16);
    let mut reversed = left.clone();
    reversed.reverse();
    assert_eq!(stats::mean(&reversed).unwrap(), stats::mean(&left).unwrap());
    assert_eq!(stats::variance(&reversed).unwrap(), expected);
    assert_eq!(stats::covariance(&left, &left).unwrap(), expected);
}
