use ciglet_rs::smoothing;
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
fn check<T: FftNum + Into<f64>>(bytes: &[u8]) {
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
            smoothing::moving_average(&signal, radius).unwrap(),
            mean,
            "mean case {case}"
        );
        assert_eq!(
            smoothing::moving_rms(&signal, radius.into() as usize + 1).unwrap(),
            rms,
            "RMS case {case}"
        );
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!("fixtures/smoothing-float.bin"));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!("fixtures/smoothing-double.bin"));
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
    assert!(smoothing::moving_rms(&[f32::MAX], 1).is_err());
}
