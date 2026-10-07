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
            stats::mean(&x).unwrap(),
            stats::variance(&x).unwrap(),
            stats::minimum(&x).unwrap(),
            stats::maximum(&x).unwrap(),
            stats::median(&x).unwrap(),
            stats::dot(&x, &y).unwrap(),
            stats::covariance(&x, &y).unwrap(),
        ];
        assert_eq!(actual.as_slice(), expected, "statistics case {case}");
        let valid = r.integer() != 0;
        let correlation = r.scalar::<T>();
        if valid {
            assert_eq!(
                stats::correlation(&x, &y).unwrap(),
                correlation,
                "correlation case {case}"
            );
        } else {
            assert!(
                stats::correlation(&x, &y).is_err(),
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
