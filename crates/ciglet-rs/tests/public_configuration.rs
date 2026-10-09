use ciglet_rs::{
    Error,
    correlogram::Method,
    scalar,
    statistics::Scalar,
    window::{Precision, Window},
};

const WINDOWS: [Window; 6] = [
    Window::Hann,
    Window::Hamming,
    Window::MltSine,
    Window::Blackman,
    Window::BlackmanHarris,
    Window::Nuttall98,
];

struct Reader<'a> {
    bytes: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (value, remaining) = self.bytes.split_at(n);
        self.bytes = remaining;
        value
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar<T: Scalar>(&mut self) -> T {
        let value = if self.width == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        };
        T::from_f64(value).unwrap()
    }
}
fn custom<T: Scalar, R: Scalar>(window: Window, length: usize) -> (Vec<T>, usize) {
    let mut calls = 0;
    let values = window
        .generate_custom::<T, R>(length, |angle| {
            calls += 1;
            Ok(R::from_f64(0.75 - angle * 0.0625 + angle * angle * 0.00390625).unwrap())
        })
        .unwrap();
    (values, calls)
}
fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    let mut reader = Reader { bytes, width };
    assert_eq!(reader.take(4), b"CUS1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 60);
    for case in 0..60 {
        let window = WINDOWS[reader.integer()];
        let callback_width = reader.integer();
        let length = reader.integer();
        let calls = reader.integer();
        let (actual, observed) = if callback_width == 4 {
            custom::<T, f32>(window, length)
        } else {
            custom::<T, f64>(window, length)
        };
        assert_eq!(observed, calls);
        for (index, value) in actual.into_iter().enumerate() {
            let expected: f64 = reader.scalar::<T>().into();
            assert_eq!(
                value.into().to_bits(),
                expected.to_bits(),
                "case {case}/{index}"
            );
        }
    }
    assert_eq!(reader.take(4), b"RED1");
    assert_eq!(reader.integer(), 12);
    for _ in 0..12 {
        let length = reader.integer();
        let expected_calls = reader.integer();
        let initial: T = reader.scalar();
        let values: Vec<T> = (0..length).map(|_| reader.scalar()).collect();
        let expected: T = reader.scalar();
        let mut calls = 0;
        let actual = values.iter().copied().fold(initial, |accumulator, value| {
            calls += 1;
            accumulator * T::from_f64(0.5).unwrap() - value * T::from_f64(0.125).unwrap()
        });
        assert_eq!(calls, expected_calls);
        assert_eq!(actual.into().to_bits(), expected.into().to_bits());
    }
    assert!(reader.bytes.is_empty());
}

#[test]
fn custom_macro_reference_and_standard_fold() {
    reference::<f32>(
        include_bytes!("../../../tests/fixtures/custom_window-float.bin"),
        4,
    );
    reference::<f64>(
        include_bytes!("../../../tests/fixtures/custom_window-double.bin"),
        8,
    );
}

#[test]
fn callback_order_state_errors_and_blackman_fix() {
    let mut angles = Vec::new();
    let weights = Window::Blackman
        .generate_custom::<f64, f64>(2, |angle| {
            angles.push(angle);
            Ok(0.0)
        })
        .unwrap();
    assert_eq!(weights, [0.42, 0.42]);
    assert_eq!(
        angles,
        [0.0, 0.0, std::f64::consts::PI, std::f64::consts::TAU]
    );
    // The caller's custom zero cosine produces the constant term, rather than
    // the original macro's hard-coded cosine window.
    assert_ne!(
        weights,
        Window::Blackman
            .generate_with::<f64>(2, Precision::Accurate)
            .unwrap()
    );
    for window in WINDOWS.into_iter().chain([Window::Boxcar]) {
        let mut calls = 0;
        for length in [0, 1] {
            let values = window
                .generate_custom::<f64, f32>(length, |_| {
                    calls += 1;
                    Ok(0.0)
                })
                .unwrap();
            assert_eq!(values, if length == 0 { vec![] } else { vec![1.0] });
        }
        assert_eq!(calls, 0);
        assert!(
            window
                .generate_custom::<f32, f32>(usize::MAX, |_| {
                    calls += 1;
                    Ok(0.0)
                })
                .is_err()
        );
        assert_eq!(calls, 0);
    }
    let mut calls = 0;
    let error = Window::BlackmanHarris
        .generate_custom::<f64, f64>(8, |_| {
            calls += 1;
            if calls == 2 {
                Err(Error("caller failure"))
            } else {
                Ok(1.0)
            }
        })
        .unwrap_err();
    assert_eq!(error, Error("caller failure"));
    assert_eq!(calls, 2);
    assert!(
        Window::Hann
            .generate_custom::<f64, f64>(2, |_| Ok(f64::NAN))
            .is_err()
    );
    assert!(
        Window::MltSine
            .generate_custom::<f32, f64>(2, |_| Ok(f64::MAX))
            .is_err()
    );
    let boxcar = Window::Boxcar
        .generate_custom::<f64, f64>(4, |_| Err(Error("must not be called")))
        .unwrap();
    assert_eq!(boxcar, [1.0; 4]);
}

#[test]
fn builtin_equivalence_and_public_codes() {
    for window in WINDOWS {
        let standard32 = window
            .generate_with::<f32>(31, Precision::Accurate)
            .unwrap();
        let custom32 = window
            .generate_custom::<f32, f64>(31, |angle| {
                Ok(if window == Window::MltSine {
                    angle.sin()
                } else {
                    angle.cos()
                })
            })
            .unwrap();
        assert_eq!(standard32, custom32);
        let standard = window
            .generate_with::<f64>(31, Precision::Accurate)
            .unwrap();
        let custom = window
            .generate_custom::<f64, f64>(31, |angle| {
                Ok(if window == Window::MltSine {
                    angle.sin()
                } else {
                    angle.cos()
                })
            })
            .unwrap();
        assert_eq!(standard, custom);
        if window != Window::Blackman {
            let approximate = |angle: f64| {
                if window == Window::MltSine {
                    ciglet_rs::approximation::sine(angle as f32)
                } else {
                    ciglet_rs::approximation::cosine(angle as f32)
                }
            };
            assert_eq!(
                window
                    .generate_with::<f32>(31, Precision::Approximate)
                    .unwrap(),
                window.generate_custom::<f32, f32>(31, approximate).unwrap()
            );
            assert_eq!(
                window
                    .generate_with::<f64>(31, Precision::Approximate)
                    .unwrap(),
                window.generate_custom::<f64, f32>(31, approximate).unwrap()
            );
        }
    }
    for (code, method) in [
        Method::Autocorrelation,
        Method::AbsoluteDifference,
        Method::SquaredDifference,
        Method::Yin,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(method as i32, code as i32);
        assert_eq!(Method::try_from(code as i32).unwrap(), method);
    }
    assert!(Method::try_from(-1).is_err());
    assert!(Method::try_from(4).is_err());
    for level in [i32::MIN, -1, 0, 1, 2] {
        assert_eq!(Precision::from_legacy_level(level), Precision::Approximate);
    }
    for level in [3, 4, i32::MAX] {
        assert_eq!(Precision::from_legacy_level(level), Precision::Accurate);
    }
    assert_eq!(scalar::LEGACY_EPSILON, 1e-15);
    assert_eq!(scalar::LEGACY_INFINITY_SENTINEL, 1e15);
    assert!(scalar::LEGACY_INFINITY_SENTINEL.is_finite());
}
