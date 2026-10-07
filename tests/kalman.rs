use ciglet_rs::kalman;
use rustfft::FftNum;

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, count: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar<T: FftNum>(&mut self, width: usize) -> T {
        T::from_f64(if width == 4 {
            f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        })
        .unwrap()
    }
    fn values<T: FftNum>(&mut self, count: usize, width: usize) -> Vec<T> {
        (0..count).map(|_| self.scalar(width)).collect()
    }
}

fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"KLM1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 108);
    for case in 0..108 {
        let count = reader.integer();
        let initial = reader.scalar::<T>(width);
        let measurements = reader.values::<T>(count, width);
        let process = reader.values::<T>(count, width);
        let noise = reader.values::<T>(count, width);
        let means = reader.values::<T>(count, width);
        let variances = reader.values::<T>(count, width);
        let smoothed = reader.values::<T>(count, width);
        let likelihood = reader.scalar::<T>(width);
        let actual = kalman::filter_with_initial(&measurements, &process, &noise, initial).unwrap();
        assert_eq!(actual.means, means, "means case={case}");
        assert_eq!(actual.variances, variances, "variance case={case}");
        assert_eq!(actual.log_likelihood, likelihood, "likelihood case={case}");
        assert_eq!(
            kalman::smooth(&actual.means, &actual.variances, &process).unwrap(),
            smoothed,
            "smooth case={case}"
        );
        if initial == measurements[0] {
            assert_eq!(
                kalman::filter(&measurements, &process, &noise).unwrap(),
                actual
            );
        }
    }
    assert!(reader.0.is_empty());
}

#[test]
fn single_precision_matches_original_c() {
    compare::<f32>(include_bytes!("fixtures/kalman-float.bin"), 4);
}
#[test]
fn double_precision_matches_original_c() {
    compare::<f64>(include_bytes!("fixtures/kalman-double.bin"), 8);
}
#[test]
fn independent_posterior_smoothing_and_deterministic_limits() {
    let estimate =
        kalman::filter_with_initial(&[1.0f64, 0.0], &[1.0, 1.0], &[1.0, 1.0], 0.0).unwrap();
    assert_eq!(estimate.means[0], 0.5);
    assert_eq!(estimate.variances[0], 0.5);
    assert!((estimate.means[1] - 0.2).abs() < 1e-15);
    assert!((estimate.variances[1] - 0.6).abs() < 1e-15);
    let smooth = kalman::smooth(&estimate.means, &estimate.variances, &[1.0, 1.0]).unwrap();
    assert!((smooth[0] - 0.4).abs() < 1e-15);
    assert_eq!(smooth[1], estimate.means[1]);
    let fixed =
        kalman::filter_with_initial(&[1.0f64, 2.0], &[0.0, 0.0], &[1.0, 1.0], 0.25).unwrap();
    assert_eq!(fixed.means, [0.25, 0.25]);
    assert_eq!(fixed.variances, [0.0, 0.0]);
    assert_eq!(
        kalman::smooth(&fixed.means, &fixed.variances, &[0.0, 0.0]).unwrap(),
        fixed.means
    );
    let empty = kalman::filter::<f64>(&[], &[], &[]).unwrap();
    assert!(empty.means.is_empty());
    assert!(empty.variances.is_empty());
    assert_eq!(empty.log_likelihood, 0.0);
    assert!(kalman::smooth::<f64>(&[], &[], &[]).unwrap().is_empty());
}
#[test]
fn invalid_variances_shapes_and_overflow_return_errors() {
    assert!(kalman::filter(&[1.0f64], &[], &[1.0]).is_err());
    assert!(kalman::filter(&[1.0f64], &[-1.0], &[1.0]).is_err());
    assert!(kalman::filter(&[1.0f64], &[0.0], &[0.0]).is_err());
    assert!(kalman::filter(&[f64::NAN], &[1.0], &[1.0]).is_err());
    assert!(kalman::filter_with_initial(&[1.0f64], &[1.0], &[1.0], f64::INFINITY).is_err());
    assert!(kalman::filter(&[1.0f64], &[f64::MAX], &[f64::MAX]).is_err());
    assert!(kalman::smooth(&[1.0f64], &[-1.0], &[1.0]).is_err());
    assert!(kalman::smooth(&[1.0f64, 2.0], &[f64::MAX, 1.0], &[1.0, f64::MAX]).is_err());
    for variance in [1e-100, 1e100] {
        let actual = kalman::filter_with_initial(&[0.0f64], &[variance], &[variance], 0.0).unwrap();
        assert!(actual.log_likelihood.is_finite());
        assert!(
            (actual.log_likelihood + 0.5 * (std::f64::consts::TAU * 2.0 * variance).ln()).abs()
                < 1e-12
        );
    }
}
