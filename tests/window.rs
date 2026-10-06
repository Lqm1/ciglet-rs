use ciglet_rs::window::{Precision, Window};
use rustfft::FftNum;

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (value, rest) = self.0.split_at(n);
        self.0 = rest;
        value
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self, precision: usize) -> f64 {
        match precision {
            4 => f32::from_le_bytes(self.bytes(4).try_into().unwrap()) as f64,
            8 => f64::from_le_bytes(self.bytes(8).try_into().unwrap()),
            _ => panic!("invalid precision"),
        }
    }
}
const WINDOWS: [Window; 7] = [
    Window::Boxcar,
    Window::Hann,
    Window::Hamming,
    Window::Blackman,
    Window::BlackmanHarris,
    Window::Nuttall98,
    Window::MltSine,
];
fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize, tolerance: f64) {
    let mut reader = Reader(fixture);
    assert_eq!(reader.bytes(4), b"WIN1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 112);
    let mut maximum_error = 0.0f64;
    let mut count = 0;
    for _ in 0..112 {
        let window = WINDOWS[reader.integer()];
        let approximate = reader.integer() != 0;
        let length = reader.integer();
        let mode = if approximate {
            Precision::Approximate
        } else {
            Precision::Accurate
        };
        for (index, value) in window
            .generate_with::<T>(length, mode)
            .unwrap()
            .into_iter()
            .enumerate()
        {
            let expected = reader.scalar(precision);
            let actual = value.into();
            let error = (actual - expected).abs();
            maximum_error = maximum_error.max(error);
            assert!(
                error <= tolerance,
                "window mismatch {window:?}/{mode:?}/{length}/{index}: {actual} versus {expected}"
            );
            count += 1;
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C windows {precision}-byte precision: {count} weights, maximum absolute error {maximum_error:e}"
    );
}
#[test]
fn float_windows_match_c() {
    compare_c::<f32>(include_bytes!("fixtures/c-window-f32.bin"), 4, 1e-7);
}
#[test]
fn double_windows_match_c() {
    compare_c::<f64>(include_bytes!("fixtures/c-window-f64.bin"), 8, 1e-15);
}

#[test]
fn degeneracy_symmetry_and_blackman_alias_behavior() {
    for window in WINDOWS {
        for precision in [Precision::Accurate, Precision::Approximate] {
            assert!(
                window
                    .generate_with::<f64>(0, precision)
                    .unwrap()
                    .is_empty()
            );
            assert_eq!(window.generate_with::<f64>(1, precision).unwrap(), [1.0]);
        }
        assert!(
            window
                .generate_with::<f32>(usize::MAX, Precision::Accurate)
                .is_err()
        );
        assert_eq!(
            window.generate(16),
            window
                .generate_with::<f32>(16, Precision::Accurate)
                .unwrap()
        );
    }
    for window in [Window::Hann, Window::Hamming, Window::MltSine] {
        let values = window
            .generate_with::<f64>(31, Precision::Accurate)
            .unwrap();
        for (&a, &b) in values.iter().zip(values.iter().rev()) {
            assert!((a - b).abs() < 1e-14);
        }
    }
    let precise = Window::Blackman
        .generate_with::<f64>(128, Precision::Accurate)
        .unwrap();
    assert_eq!(
        precise,
        Window::Blackman
            .generate_with::<f64>(128, Precision::Approximate)
            .unwrap()
    );
    let exact = Window::Hann
        .generate_with::<f32>(31, Precision::Accurate)
        .unwrap();
    let approximate = Window::Hann
        .generate_with::<f32>(31, Precision::Approximate)
        .unwrap();
    assert_ne!(exact, approximate);
    for (&a, &b) in exact.iter().zip(&approximate) {
        assert!((a - b).abs() < 1e-4);
    }
    assert_eq!(Precision::default(), Precision::Accurate);
}
