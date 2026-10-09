use ciglet_rs::smoothing;
use ciglet_rs::statistics::Scalar;
use rustfft::FftNum;
fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
    let value = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
    *cursor += 4;
    value as usize
}
fn scalar<T: FftNum>(bytes: &[u8], cursor: &mut usize, width: usize) -> T {
    let value = if width == 4 {
        f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
    } else {
        f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
    };
    *cursor += width;
    T::from_f64(value).unwrap()
}
fn check<T: Scalar>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"SMO1");
    let mut cursor = 4;
    let width = integer(bytes, &mut cursor);
    let count = integer(bytes, &mut cursor);
    for case in 0..count {
        let n = integer(bytes, &mut cursor);
        let radius = scalar::<T>(bytes, &mut cursor, width);
        let mut read = || {
            (0..n)
                .map(|_| scalar::<T>(bytes, &mut cursor, width))
                .collect::<Vec<_>>()
        };
        let signal = read();
        let mean = read();
        let rms = read();
        assert_eq!(
            smoothing::moving_average_with(&signal, radius, smoothing::Policy::Legacy).unwrap(),
            mean,
            "mean case {case}"
        );
        assert_eq!(
            smoothing::moving_rms_with(
                &signal,
                radius.into() as usize + 1,
                smoothing::Policy::Legacy
            )
            .unwrap(),
            rms,
            "RMS case {case}"
        );
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!(
        "../../../tests/fixtures/smoothing-float.bin"
    ));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!(
        "../../../tests/fixtures/smoothing-double.bin"
    ));
}
#[test]
fn zero_radius_empty_and_invalid_samples() {
    assert_eq!(
        smoothing::moving_average(&[-3.0, 4.0], 0.0).unwrap(),
        [-3.0, 4.0]
    );
    assert_eq!(smoothing::moving_rms(&[-3.0, 4.0], 0).unwrap(), [3.0, 4.0]);
    assert!(
        smoothing::moving_average::<f32>(&[], 1.0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        smoothing::moving_average(&[1.0, 2.0, 8.0], 1.0).unwrap(),
        [1.5, 5.0, 8.0]
    );
    assert_eq!(smoothing::moving_average(&[2.0], 1.0).unwrap(), [2.0]);
    assert!(smoothing::moving_average(&[1.0], -1.0).is_err());
    assert!(smoothing::moving_average(&[f32::NAN], 1.0).is_err());
    assert_eq!(smoothing::moving_rms(&[f32::MAX], 1).unwrap(), [f32::MAX]);
    assert!(smoothing::moving_rms_with(&[f32::MAX], 1, smoothing::Policy::Legacy).is_err());
}

#[test]
fn fractional_windows_preserve_dc_and_positive_tiny_width_limit() {
    assert_eq!(
        smoothing::moving_average(&[1.0_f64; 9], 1.5).unwrap(),
        [1.0; 9]
    );
    assert_eq!(
        smoothing::moving_average(&[1.0_f64, 2.0, 3.0], 1e-300).unwrap(),
        [1.5, 2.5, 3.0]
    );
}

fn stable_reference<T: Scalar>(bytes: &[u8], tolerance: f64) {
    assert_eq!(&bytes[..4], b"SSW1");
    let mut cursor = 4;
    let width = integer(bytes, &mut cursor);
    assert_eq!(integer(bytes, &mut cursor), 330);
    let mut outputs = 0;
    let mut maximum = 0.0_f64;
    for case in 0..330 {
        let n = integer(bytes, &mut cursor);
        let rms_width = integer(bytes, &mut cursor);
        let half_width = scalar::<T>(bytes, &mut cursor, width);
        let mut read = || {
            (0..n)
                .map(|_| scalar::<T>(bytes, &mut cursor, width))
                .collect::<Vec<_>>()
        };
        let signal = read();
        let mean = read();
        let rms = read();
        for (field, (actual, expected)) in [
            (
                smoothing::moving_average(&signal, half_width).unwrap(),
                mean,
            ),
            (smoothing::moving_rms(&signal, rms_width).unwrap(), rms),
        ]
        .into_iter()
        .enumerate()
        {
            for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
                let actual: f64 = actual.into();
                let expected: f64 = expected.into();
                let error = if expected == 0.0 {
                    actual.abs()
                } else {
                    (actual / expected - 1.0).abs()
                };
                // A rounded binary64 mean-square can land on a subnormal
                // root's rounding boundary. Permit one quantum only there;
                // normal results retain the measured relative-error bound.
                let subnormal_quantum = if width == 4 {
                    f32::from_bits(1) as f64
                } else {
                    f64::from_bits(1)
                };
                let minimum_normal = if width == 4 {
                    f32::MIN_POSITIVE as f64
                } else {
                    f64::MIN_POSITIVE
                };
                let subnormal = field == 1
                    && expected.abs() < minimum_normal
                    && (actual - expected).abs() <= subnormal_quantum;
                assert!(
                    (error <= tolerance && (expected != 0.0 || actual == 0.0)) || subnormal,
                    "case={case} field={field} index={index}: actual={actual:e}, expected={expected:e}, error={error:e}"
                );
                maximum = maximum.max(error);
                outputs += 1;
            }
        }
    }
    assert_eq!(outputs, 10_920);
    assert_eq!(cursor, bytes.len());
    println!(
        "stable smoothing width={width}, outputs={outputs}, maximum relative error={maximum:e}"
    );
}

#[test]
fn exact_integral_float_reference() {
    stable_reference::<f32>(
        include_bytes!("../../../tests/fixtures/stable_smoothing-float.bin"),
        2e-7,
    );
}

#[test]
fn exact_integral_double_reference() {
    stable_reference::<f64>(
        include_bytes!("../../../tests/fixtures/stable_smoothing-double.bin"),
        4e-15,
    );
}
