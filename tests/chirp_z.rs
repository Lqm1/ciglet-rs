use ciglet_rs::{
    chirp_z::CztPlan,
    fourier::{Complex, Direction, FftPlan},
};
use rustfft::FftNum;

struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let (value, rest) = self.0.split_at(N);
        self.0 = rest;
        value.try_into().unwrap()
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take()) as usize
    }
    fn scalar(&mut self, precision: usize) -> f64 {
        match precision {
            4 => f32::from_le_bytes(self.take()) as f64,
            8 => f64::from_le_bytes(self.take()),
            _ => panic!("invalid precision"),
        }
    }
}

fn compare_c<T: FftNum + Into<f64>>(fixture: &[u8], precision: usize, tolerance: f64) {
    let mut reader = Reader(fixture);
    assert_eq!(&reader.take::<4>(), b"CZT1");
    assert_eq!(reader.integer(), precision);
    assert_eq!(reader.integer(), 1200);
    let mut maximum_error = 0.0f64;
    let mut maximum_absolute_error = 0.0f64;
    let mut count = 0;
    for _ in 0..1200 {
        let length = reader.integer();
        let scenario = reader.integer();
        let mask = reader.integer();
        let inverse = reader.integer();
        let step = T::from_f64(reader.scalar(precision)).unwrap();
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
                _ => panic!("invalid scenario"),
            };
            real.push(T::from_f64(re).unwrap());
            imaginary.push(T::from_f64(im).unwrap());
        }
        let real = (mask & 1 != 0).then_some(real.as_slice());
        let imaginary = (mask & 2 != 0).then_some(imaginary.as_slice());
        let scale = real
            .into_iter()
            .flatten()
            .chain(imaginary.into_iter().flatten())
            .map(|&x| x.into().abs())
            .fold(1.0, f64::max)
            * if inverse == 0 { length as f64 } else { 1.0 };
        let direction = if inverse == 0 {
            Direction::Forward
        } else {
            Direction::Inverse
        };
        let mut plan = CztPlan::new(length, step, direction).unwrap();
        for (index, value) in plan.transform(real, imaginary).unwrap().iter().enumerate() {
            for actual in [value.re.into(), value.im.into()] {
                let expected = reader.scalar(precision);
                let error = (actual - expected).abs();
                maximum_error = maximum_error.max(error / scale);
                maximum_absolute_error = maximum_absolute_error.max(error);
                assert!(
                    actual.is_finite() && error / scale <= tolerance,
                    "CZT mismatch {length}/{scenario}/{mask}/{inverse}/{index}: {actual} versus {expected}, normalized error {}",
                    error / scale
                );
                count += 1;
            }
        }
    }
    assert!(reader.0.is_empty());
    eprintln!(
        "C CZT {precision}-byte precision: {count} scalar values, maximum absolute error {maximum_absolute_error:e}, normalized error {maximum_error:e}"
    );
}

#[test]
fn float_czt_matches_c() {
    compare_c::<f32>(include_bytes!("fixtures/c-czt-f32.bin"), 4, 5e-7);
}
#[test]
fn double_czt_matches_c() {
    compare_c::<f64>(include_bytes!("fixtures/c-czt-f64.bin"), 8, 2e-15);
}

#[test]
fn ordinary_dft_grid_matches_fft_and_round_trips() {
    for length in [1, 2, 8, 32] {
        let real: Vec<_> = (0..length).map(|i| (i as f64 - 3.0) * 0.125).collect();
        let step = std::f64::consts::TAU / length as f64;
        let mut forward = CztPlan::new(length, step, Direction::Forward).unwrap();
        assert_eq!(forward.len(), length);
        assert!(!forward.is_empty());
        let spectrum = forward.transform(Some(&real), None).unwrap();
        let expected = FftPlan::new(length)
            .unwrap()
            .transform(Some(&real), None, Direction::Forward)
            .unwrap();
        for (a, b) in spectrum.iter().zip(expected) {
            assert!((*a - b).norm() < 1e-11);
        }
        let (re, im): (Vec<_>, Vec<_>) = spectrum.iter().map(|x| (x.re, x.im)).unzip();
        let result = CztPlan::new(length, step, Direction::Inverse)
            .unwrap()
            .transform(Some(&re), Some(&im))
            .unwrap();
        for (a, b) in result.iter().zip(&real) {
            assert!((a.re - b).abs() < 1e-12 && a.im.abs() < 1e-12);
        }
        assert_eq!(
            forward.transform(None, None).unwrap(),
            vec![Complex::new(0.0, 0.0); length]
        );
    }
}

#[test]
fn arbitrary_grid_matches_direct_sum_and_validates_shapes() {
    let real = [0.25f64, -0.5, 1.0, 0.125, -0.25];
    let imaginary = [-0.125, 0.25, 0.5, -0.25, 0.125];
    for step in [0.0, 0.125, -0.25] {
        let mut plan = CztPlan::new(5, step, Direction::Forward).unwrap();
        for _ in 0..2 {
            let output = plan.transform(Some(&real), Some(&imaginary)).unwrap();
            for (k, actual) in output.iter().enumerate() {
                let expected: Complex<f64> = (0..5)
                    .map(|i| {
                        Complex::new(real[i], imaginary[i])
                            * Complex::from_polar(1.0, -step * i as f64 * k as f64)
                    })
                    .sum();
                assert!((*actual - expected).norm() < 1e-13);
            }
        }
        let mut output = [Complex::new(7.0, 3.0); 5];
        let original = output;
        assert!(
            plan.transform_into(Some(&real[..4]), None, &mut output)
                .is_err()
        );
        assert_eq!(output, original);
        assert!(plan.transform_into(None, None, &mut output[..4]).is_err());
    }
    for length in [0, usize::MAX, i32::MAX as usize] {
        assert!(CztPlan::new(length, 0.125f32, Direction::Forward).is_err());
    }
    for step in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX] {
        assert!(CztPlan::new(5, step, Direction::Forward).is_err());
    }
}
