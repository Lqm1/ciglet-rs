use ciglet_rs::fourier::{Complex, Direction, FftPlan, fft_shift, fft_shift_in_place};
use rustfft::FftNum;

fn inputs<T: FftNum>(length: usize, scenario: usize) -> (Vec<T>, Vec<T>) {
    let mut real = Vec::new();
    let mut imaginary = Vec::new();
    for index in 0..length {
        let (re, im) = match scenario {
            0 => (
                if index == length / 3 { 0.75 } else { 0.0 },
                if index == length / 4 { -0.25 } else { 0.0 },
            ),
            1 => (
                ((index % 17) as i32 - 8) as f64 * 0.125,
                ((index % 11) as i32 - 5) as f64 * 0.0625,
            ),
            2 => (
                2048.0 + (index % 7) as f64 * 0.125,
                -1024.0 + (index % 5) as f64 * 0.0625,
            ),
            _ => panic!("unknown C scenario"),
        };
        real.push(T::from_f64(re).unwrap());
        imaginary.push(T::from_f64(im).unwrap());
    }
    (real, imaginary)
}

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (value, remaining) = self.0.split_at(N);
        self.0 = remaining;
        value.try_into().unwrap()
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take()) as usize
    }
    fn scalar(&mut self, bytes: usize) -> f64 {
        match bytes {
            4 => f32::from_le_bytes(self.take()) as f64,
            8 => f64::from_le_bytes(self.take()),
            _ => panic!("unknown C scalar size"),
        }
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize, tolerance: f64) {
    let mut reader = Reader(fixture);
    assert_eq!(&reader.take::<4>(), b"FFT1");
    assert_eq!(reader.integer(), precision);
    let records = reader.integer();
    assert_eq!(records, 240);
    let mut maximum_normalized_error = 0.0f64;
    let mut maximum_absolute_error = 0.0f64;
    let mut values_checked = 0;
    for _ in 0..records {
        let length = reader.integer();
        let scenario = reader.integer();
        let mask = reader.integer();
        let inverse = reader.integer();
        let (real, imaginary) = inputs::<T>(length, scenario);
        let real = (mask & 1 != 0).then_some(real.as_slice());
        let imaginary = (mask & 2 != 0).then_some(imaginary.as_slice());
        let direction = if inverse == 0 {
            Direction::Forward
        } else {
            Direction::Inverse
        };
        let mut plan = FftPlan::<T>::new(length).unwrap();
        let result = plan.transform(real, imaginary, direction).unwrap();
        let input_scale = real
            .into_iter()
            .flatten()
            .chain(imaginary.into_iter().flatten())
            .map(|&value| value.into().abs())
            .fold(1.0f64, f64::max);
        let scale = input_scale * if inverse == 0 { length as f64 } else { 1.0 };
        for (index, value) in result.iter().enumerate() {
            for actual in [value.re.into(), value.im.into()] {
                let expected = reader.scalar(precision);
                let error = (actual - expected).abs();
                assert!(
                    actual.is_finite(),
                    "nonfinite FFT result at {length}/{scenario}/{mask}/{inverse}/{index}"
                );
                maximum_absolute_error = maximum_absolute_error.max(error);
                maximum_normalized_error = maximum_normalized_error.max(error / scale);
                assert!(
                    error / scale <= tolerance,
                    "FFT difference at {length}/{scenario}/{mask}/{inverse}/{index}: {actual} versus {expected}, normalized error {} exceeds {tolerance}",
                    error / scale
                );
                values_checked += 1;
            }
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C FFT {precision}-byte precision: {values_checked} scalar values, maximum absolute error {maximum_absolute_error:e}, normalized error {maximum_normalized_error:e}"
    );
}

#[test]
fn float_fft_matches_c() {
    compare_c::<f32>(include_bytes!("fixtures/c-fft-f32.bin"), 4, 2e-7);
}

#[test]
fn double_fft_matches_c() {
    compare_c::<f64>(include_bytes!("fixtures/c-fft-f64.bin"), 8, 1e-15);
}

#[test]
fn inverse_normalization_round_trips_and_preserves_energy() {
    for length in [1, 2, 8, 64, 1024] {
        let (real, imaginary) = inputs::<f64>(length, 1);
        let original: Vec<_> = real
            .iter()
            .zip(&imaginary)
            .map(|(&re, &im)| Complex::new(re, im))
            .collect();
        let mut values = original.clone();
        let mut plan = FftPlan::new(length).unwrap();
        assert_eq!(plan.len(), length);
        assert!(!plan.is_empty());
        plan.process(&mut values, Direction::Forward).unwrap();
        let input_energy: f64 = original.iter().map(|v| v.norm_sqr()).sum();
        let output_energy: f64 = values.iter().map(|v| v.norm_sqr()).sum::<f64>() / length as f64;
        assert!((input_energy - output_energy).abs() <= input_energy.max(1.0) * 1e-12);
        plan.process(&mut values, Direction::Inverse).unwrap();
        for (actual, expected) in values.iter().zip(&original) {
            assert!((*actual - *expected).norm() < 1e-12);
        }
        assert_eq!(
            plan.transform(None, None, Direction::Forward).unwrap(),
            vec![Complex::new(0.0, 0.0); length]
        );
    }
}

#[test]
fn fft_shift_handles_odd_even_and_empty_lengths() {
    assert_eq!(fft_shift(&[0, 1, 2, 3, 4]), [3, 4, 0, 1, 2]);
    assert_eq!(fft_shift(&[0, 1, 2, 3]), [2, 3, 0, 1]);
    assert_eq!(fft_shift(&[7]), [7]);
    assert!(fft_shift::<f32>(&[]).is_empty());
    let mut values = [0, 1, 2, 3, 4];
    fft_shift_in_place(&mut values);
    assert_eq!(values, [3, 4, 0, 1, 2]);
}

#[test]
fn invalid_fft_shapes_are_errors_and_leave_the_buffer_unchanged() {
    for length in [0, 3, 7, usize::MAX] {
        assert!(FftPlan::<f32>::new(length).is_err());
    }
    let mut plan = FftPlan::<f32>::new(8).unwrap();
    let mut values = vec![Complex::new(2.0, -1.0); 7];
    let original = values.clone();
    assert!(plan.process(&mut values, Direction::Forward).is_err());
    assert_eq!(values, original);
    assert!(
        plan.transform(Some(&[1.0; 7]), None, Direction::Forward)
            .is_err()
    );
    assert!(
        plan.transform(None, Some(&[1.0; 9]), Direction::Inverse)
            .is_err()
    );
}
