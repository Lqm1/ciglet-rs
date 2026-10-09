use ciglet_rs::instantaneous_frequency::{Detector, Kernels, Policy};
use ciglet_rs::statistics::Scalar;
fn reference<T: Scalar>(bytes: &[u8], width: usize, tolerance: f64, estimate_tolerance: f64) {
    assert_eq!(&bytes[..4], b"IFD1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 24);
    let mut cursor = 12;
    let mut maximum_estimate_error = 0.0_f64;
    fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
        let result = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as usize;
        *cursor += 4;
        result
    }
    fn scalar<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize) -> T {
        let value = if width == 4 {
            f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
        };
        *cursor += width;
        T::from_f64(value).unwrap()
    }
    for _ in 0..24 {
        let center = scalar::<T>(bytes, &mut cursor, width);
        let resolution = scalar::<T>(bytes, &mut cursor, width);
        let length = integer(bytes, &mut cursor);
        let detector = Detector::new(center, resolution).unwrap();
        assert_eq!(detector.kernels().real.len(), length);
        for filter in [
            &detector.kernels().real,
            &detector.kernels().imaginary,
            &detector.kernels().derivative_real,
            &detector.kernels().derivative_imaginary,
        ] {
            for &actual in filter {
                let expected = scalar::<T>(bytes, &mut cursor, width);
                if width == 4 {
                    assert_eq!(actual.into().to_bits(), expected.into().to_bits());
                }
                assert!(
                    (actual.into() - expected.into()).abs()
                        <= tolerance * (1.0 + expected.into().abs()),
                    "kernel {} != {}",
                    actual.into(),
                    expected.into()
                );
            }
        }
        for _ in 0..4 {
            let nx = integer(bytes, &mut cursor);
            let samples: Vec<T> = (0..nx).map(|_| scalar(bytes, &mut cursor, width)).collect();
            let expected = scalar::<T>(bytes, &mut cursor, width);
            let actual = detector.estimate_with(&samples, Policy::Legacy).unwrap();
            maximum_estimate_error = maximum_estimate_error
                .max((actual.into() - expected.into()).abs() / (1.0 + expected.into().abs()));
            assert!(
                (actual.into() - expected.into()).abs()
                    <= estimate_tolerance * (1.0 + expected.into().abs()),
                "estimate {} != {}",
                actual.into(),
                expected.into()
            );
            if width == 4 {
                assert_eq!(actual.into().to_bits(), expected.into().to_bits());
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("width={width}, maximum normalized estimate error={maximum_estimate_error:e}");
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(
        include_bytes!("../../../tests/fixtures/instantaneous-frequency-float.bin"),
        4,
        2e-7,
        0.0,
    );
    reference::<f64>(
        include_bytes!("../../../tests/fixtures/instantaneous-frequency-double.bin"),
        8,
        3e-15,
        1e-12,
    );
}
#[test]
fn analytic_custom_filters_and_extreme_scale() {
    let detector = Detector::from_kernels(
        0.125_f64,
        Kernels {
            real: vec![1.0, 0.0],
            imaginary: vec![0.0, 1.0],
            derivative_real: vec![0.0, -2.0],
            derivative_imaginary: vec![2.0, 0.0],
        },
    )
    .unwrap();
    for samples in [
        [1.0, 2.0],
        [f64::MAX, f64::MAX / 2.0],
        [f64::from_bits(1), f64::from_bits(2)],
    ] {
        assert!((detector.estimate(&samples).unwrap() - 1.0 / std::f64::consts::PI).abs() < 2e-16);
    }
    assert_eq!(detector.estimate(&[f64::NAN]).unwrap(), 0.0);
    assert!(detector.estimate(&[0.0, 0.0]).is_err());
    assert!(detector.estimate_with(&[0.0, 0.0], Policy::Legacy).is_err());
    assert!(detector.estimate(&[f64::NAN, 1.0]).is_err());
    let surrounded = [f64::NAN, 1.0, 2.0, f64::NAN];
    assert_eq!(
        detector.estimate(&surrounded),
        detector.estimate(&[1.0, 2.0])
    );
    let mut edited = detector.clone();
    edited.kernels_mut().real.pop();
    assert!(edited.estimate(&[1.0, 2.0]).is_err());
    assert_eq!(detector.kernels().real.len(), 2);
}
#[test]
fn sinusoid_and_parameters() {
    let detector = Detector::new(0.125_f64, 0.015625).unwrap();
    let samples: Vec<_> = (0..512)
        .map(|i| (std::f64::consts::TAU * 0.125 * i as f64 + 0.3).cos())
        .collect();
    assert!((detector.estimate(&samples).unwrap() - 0.125).abs() < 1e-12);
    for resolution in [0.0, -1.0, f64::NAN, f64::INFINITY, 5.0, f64::from_bits(1)] {
        assert!(Detector::new(0.125_f64, resolution).is_err());
    }
    assert!(Detector::new(f64::INFINITY, 0.125).is_err());
    assert_eq!(
        Detector::new(0.125_f32, 0.3).unwrap().kernels().real.len(),
        13
    );
}
