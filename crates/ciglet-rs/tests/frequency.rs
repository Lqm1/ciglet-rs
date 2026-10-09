use ciglet_rs::{approximation, frequency::*};
use rustfft::FftNum;
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self, n: usize) -> f64 {
        if n == 4 {
            self.single() as f64
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        }
    }
    fn single(&mut self) -> f32 {
        f32::from_le_bytes(self.bytes(4).try_into().unwrap())
    }
}
fn compare<T: FftNum + Into<f64>>(bytes: &[u8], width: usize) {
    let mut r = Reader(bytes);
    assert_eq!(r.bytes(4), b"FRQ1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 12);
    let mut maximum = 0.0f64;
    let mut count = 0;
    for record in 0..12 {
        let input = T::from_f64(r.scalar(width)).unwrap();
        let mel = frequency_to_mel(input).unwrap();
        let bark = frequency_to_bark(input).unwrap();
        for value in [
            mel,
            mel_to_frequency(mel).unwrap(),
            bark,
            bark_to_frequency(bark).unwrap(),
            equal_loudness(input).unwrap(),
        ] {
            let expected = r.scalar(width);
            let value: f64 = value.into();
            let error = (value - expected).abs() / expected.abs().max(1.0);
            maximum = maximum.max(error);
            assert!(
                error < if width == 4 { 1e-7 } else { 1e-15 },
                "frequency {record}: {value} vs {expected}"
            );
            count += 1;
        }
    }
    assert_eq!(r.integer(), 5);
    for _ in 0..5 {
        let n = r.integer();
        for value in mel_space(T::from_f64(50.0).unwrap(), T::from_f64(8000.0).unwrap(), n).unwrap()
        {
            let expected = r.scalar(width);
            let value: f64 = value.into();
            let error = (value - expected).abs() / expected.abs().max(1.0);
            maximum = maximum.max(error);
            assert_eq!(value, expected);
            count += 1;
        }
    }
    assert_eq!(r.integer(), 12);
    for _ in 0..12 {
        let x = r.single();
        assert_eq!(
            approximation::logarithm2(x).unwrap().to_bits(),
            r.single().to_bits()
        );
        assert_eq!(
            approximation::logarithm(x).unwrap().to_bits(),
            r.single().to_bits()
        );
    }
    assert_eq!(r.integer(), 12);
    for _ in 0..12 {
        let x = r.single();
        assert_eq!(
            approximation::exponential2(x).unwrap().to_bits(),
            r.single().to_bits()
        );
        assert_eq!(
            approximation::exponential(x).unwrap().to_bits(),
            r.single().to_bits()
        );
        assert_eq!(
            approximation::power(10.0, x / 40.0).unwrap().to_bits(),
            r.single().to_bits()
        );
    }
    assert!(r.0.is_empty());
    eprintln!(
        "Frequency {width}-byte: {count} values, normalized error {maximum:e}; 60 approximation values bit-exact"
    );
}
#[test]
fn float_frequency_matches_c() {
    compare::<f32>(
        include_bytes!("../../../tests/fixtures/c-frequency-f32.bin"),
        4,
    );
}
#[test]
fn double_frequency_matches_c() {
    compare::<f64>(
        include_bytes!("../../../tests/fixtures/c-frequency-f64.bin"),
        8,
    );
}
#[test]
fn frequency_domains_round_trips_and_spacing() {
    assert_eq!(equal_loudness(0.0f64).unwrap(), 0.0);
    for f in [-8000.0f64, -440.0, 0.0, 440.0, 8000.0] {
        let bark = frequency_to_bark(f).unwrap();
        assert!((bark_to_frequency(bark).unwrap() - f).abs() < 1e-10);
    }
    let points = mel_space(50.0f64, 8000.0, 40).unwrap();
    assert_eq!(points.len(), 41);
    assert!(points.windows(2).all(|w| w[0] < w[1]));
    assert!(mel_space(0.0f64, 8000.0, 0).is_err());
    assert!(mel_space(1000.0f64, 50.0, 4).is_err());
    assert!(frequency_to_mel(-700.0f64).is_err());
    assert!(frequency_to_bark(f64::INFINITY).is_err());
    assert!(mel_to_frequency(f64::NAN).is_err());
    assert!(equal_loudness(f64::MAX).is_err());
}
#[test]
fn approximation_accuracy_and_conversion_domains() {
    let mut log_error = 0.0f32;
    let mut exp_error = 0.0f32;
    for index in 0..=1000 {
        let x = 0.125 + index as f32 / 100.0;
        log_error = log_error.max((approximation::logarithm(x).unwrap() - x.ln()).abs());
        let exponent = -5.0 + index as f32 / 100.0;
        let expected = exponent.exp();
        exp_error = exp_error
            .max((approximation::exponential(exponent).unwrap() - expected).abs() / expected);
    }
    eprintln!(
        "Approximate logarithm absolute error {log_error:e}, exponential relative error {exp_error:e}"
    );
    assert!(log_error < 2e-4);
    assert!(exp_error < 2e-4);
    assert!(approximation::logarithm(0.0).is_err());
    assert!(approximation::logarithm(-1.0).is_err());
    assert!(approximation::exponential2(128.0).is_err());
    assert!(approximation::exponential2(f32::NAN).is_err());
    assert_eq!(
        approximation::exponential2(-200.0).unwrap(),
        approximation::exponential2(-126.0).unwrap()
    );
}
