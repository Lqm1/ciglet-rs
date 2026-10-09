use ciglet_rs::{approximation, cosine::dct};
use rustfft::FftNum;

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
    fn scalar(&mut self, precision: usize) -> f64 {
        match precision {
            4 => f32::from_le_bytes(self.take()) as f64,
            8 => f64::from_le_bytes(self.take()),
            _ => panic!("invalid scalar precision"),
        }
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize, tolerance: f64) {
    let mut reader = Reader(fixture);
    assert_eq!(&reader.take::<4>(), b"DCT1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 32);
    let mut maximum_error = 0.0f64;
    let mut count = 0;
    for _ in 0..32 {
        let length = reader.integer();
        let scenario = reader.integer();
        let signal: Vec<T> = (0..length)
            .map(|index| {
                T::from_f64(match scenario {
                    0 => {
                        if index == length / 3 {
                            0.75
                        } else {
                            0.0
                        }
                    }
                    1 => ((index % 17) as i32 - 8) as f64 * 0.125,
                    2 => 2048.0,
                    3 => {
                        if index % 2 == 1 {
                            -0.5
                        } else {
                            0.5
                        }
                    }
                    _ => panic!("invalid C scenario"),
                })
                .unwrap()
            })
            .collect();
        let input_scale = signal.iter().map(|&v| v.into().abs()).fold(1.0, f64::max);
        let scale = input_scale * (length as f64).sqrt();
        for (frequency, actual) in dct(&signal).unwrap().into_iter().enumerate() {
            let expected = reader.scalar(precision);
            let error = (actual.into() - expected).abs() / scale;
            maximum_error = maximum_error.max(error);
            assert!(
                error <= tolerance,
                "DCT mismatch {length}/{scenario}/{frequency}: normalized error {error}"
            );
            count += 1;
        }
    }
    assert_eq!(reader.integer(), 12);
    for _ in 0..12 {
        let angle = reader.scalar(4) as f32;
        let sine = reader.scalar(4) as f32;
        let cosine = reader.scalar(4) as f32;
        assert_eq!(
            approximation::sine(angle).unwrap().to_bits(),
            sine.to_bits()
        );
        assert_eq!(
            approximation::cosine(angle).unwrap().to_bits(),
            cosine.to_bits()
        );
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C DCT {precision}-byte precision: {count} coefficients, maximum normalized error {maximum_error:e}"
    );
}

#[test]
fn float_dct_matches_c() {
    compare_c::<f32>(
        include_bytes!("../../../tests/fixtures/c-dct-f32.bin"),
        4,
        2e-7,
    );
}

#[test]
fn double_dct_matches_c() {
    compare_c::<f64>(
        include_bytes!("../../../tests/fixtures/c-dct-f64.bin"),
        8,
        1e-15,
    );
}

#[test]
fn dct_scaling_and_invalid_inputs() {
    assert!(dct::<f32>(&[]).is_err());
    for length in [1, 2, 3, 5, 16, 32] {
        let result = dct(&vec![1.0f64; length]).unwrap();
        // The original approximate cos(0) is about 0.9999864, not exactly one.
        assert!((result[0] - (length as f64).sqrt()).abs() < 1e-4 * (length as f64).sqrt());
        for &coefficient in &result[1..] {
            assert!(coefficient.abs() < 0.002 * (length as f64).sqrt());
        }
    }
    for angle in [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MAX,
        -f32::MAX,
    ] {
        assert!(approximation::sine(angle).is_err());
        assert!(approximation::cosine(angle).is_err());
    }
}

#[test]
fn trigonometric_approximations_retain_small_angle_accuracy() {
    let mut maximum_error = 0.0f32;
    for index in -1000..=1000 {
        let angle = index as f32 * std::f32::consts::PI / 250.0;
        maximum_error =
            maximum_error.max((approximation::sine(angle).unwrap() - angle.sin()).abs());
        maximum_error =
            maximum_error.max((approximation::cosine(angle).unwrap() - angle.cos()).abs());
    }
    eprintln!("Trigonometric approximation maximum absolute error {maximum_error:e}");
    assert!(maximum_error < 1e-4);
}
