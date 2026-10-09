use ciglet_rs::filtering::{self, Startup};
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
    fn values<T: FftNum>(&mut self, count: usize, width: usize) -> Vec<T> {
        (0..count)
            .map(|_| {
                T::from_f64(if width == 4 {
                    f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(self.bytes(8).try_into().unwrap())
                })
                .unwrap()
            })
            .collect()
    }
}

fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"IIR1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 210);
    for case in 0..210 {
        let nb = reader.integer();
        let na = reader.integer();
        let nx = reader.integer();
        let numerator = reader.values::<T>(nb, width);
        let denominator = reader.values::<T>(na, width);
        let signal = reader.values::<T>(nx, width);
        for startup in [Startup::LegacyZeroPrefix, Startup::ZeroState] {
            let expected = reader.values::<T>(nx, width);
            let actual =
                filtering::filter_with(&signal, &numerator, &denominator, startup).unwrap();
            assert_eq!(actual, expected, "causal case={case}, startup={startup:?}");
            let expected = reader.values::<T>(nx, width);
            let actual =
                filtering::forward_backward_with(&signal, &numerator, &denominator, startup)
                    .unwrap();
            assert_eq!(
                actual, expected,
                "two-pass case={case}, startup={startup:?}"
            );
        }
    }
    assert!(reader.0.is_empty());
}

#[test]
fn single_precision_matches_bounded_original_c() {
    compare::<f32>(
        include_bytes!("../../../tests/fixtures/recursive-filter-float.bin"),
        4,
    );
}

#[test]
fn double_precision_matches_bounded_original_c() {
    compare::<f64>(
        include_bytes!("../../../tests/fixtures/recursive-filter-double.bin"),
        8,
    );
}

#[test]
fn initial_impulse_short_inputs_and_forward_backward_are_correct() {
    assert_eq!(
        filtering::filter(&[1.0f64, 0.0, 0.0], &[0.5, 0.0], &[1.0, -0.5]).unwrap(),
        [0.5, 0.25, 0.125]
    );
    assert_eq!(
        filtering::filter_with(
            &[1.0f64, 0.0, 0.0],
            &[0.5, 0.0],
            &[1.0, -0.5],
            Startup::LegacyZeroPrefix
        )
        .unwrap(),
        [0.0; 3]
    );
    assert_eq!(
        filtering::filter(&[1.0f64], &[1.0; 8], &[1.0]).unwrap(),
        [1.0]
    );
    assert_eq!(
        filtering::filter(&[1.0f64, 2.0, 3.0], &[1.0, 2.0], &[]).unwrap(),
        [1.0, 4.0, 7.0]
    );
    assert_eq!(
        filtering::forward_backward(&[1.0f64, 2.0, 3.0], &[1.0, 2.0], &[]).unwrap(),
        [9.0, 18.0, 7.0]
    );
    // The original API ignores a[0], even when its value is not one.
    assert_eq!(filtering::filter(&[1.0f64], &[0.5], &[2.0]).unwrap(), [0.5]);
    assert_eq!(
        filtering::filter(&[1.0f64, 2.0], &[], &[]).unwrap(),
        [0.0, 0.0]
    );
    assert!(filtering::filter::<f64>(&[], &[], &[]).unwrap().is_empty());
}

#[test]
fn nonfinite_inputs_and_overflow_return_errors() {
    assert!(filtering::filter(&[f32::NAN], &[1.0], &[1.0]).is_err());
    assert!(filtering::filter(&[1.0f32], &[f32::NAN], &[1.0]).is_err());
    assert!(filtering::filter(&[1.0f32], &[1.0], &[f32::INFINITY]).is_err());
    assert!(filtering::filter(&[f32::MAX], &[2.0], &[]).is_err());
    assert!(filtering::forward_backward(&[f32::MAX], &[2.0], &[]).is_err());
}
