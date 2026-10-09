use ciglet_rs::median;
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
    assert_eq!(&bytes[..4], b"MED1");
    let mut cursor = 4;
    let scalar_width = integer(bytes, &mut cursor);
    let count = integer(bytes, &mut cursor);
    for case in 0..count {
        let n = integer(bytes, &mut cursor);
        let width = integer(bytes, &mut cursor);
        let mut read = || {
            (0..n)
                .map(|_| scalar::<T>(bytes, &mut cursor, scalar_width))
                .collect::<Vec<_>>()
        };
        let signal = read();
        let expected = read();
        assert_eq!(
            median::filter(&signal, width).unwrap(),
            expected,
            "case {case}"
        );
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!(
        "../../../tests/fixtures/median-filter-float.bin"
    ));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!(
        "../../../tests/fixtures/median-filter-double.bin"
    ));
}
#[test]
fn endpoint_rule_invalid_widths_and_large_finite_medians() {
    assert_eq!(
        median::filter(&[4.0, 1.0, 9.0, 2.0], 2).unwrap(),
        [4.0, 2.5, 5.0, 2.0]
    );
    assert_eq!(
        median::filter(&[4.0, 1.0, 9.0, 2.0, 7.0], 3).unwrap(),
        [4.0, 4.0, 2.0, 7.0, 7.0]
    );
    assert_eq!(median::filter(&[3.0], 1).unwrap(), [3.0]);
    assert!(median::filter::<f32>(&[], 3).unwrap().is_empty());
    assert!(median::filter(&[1.0], 3).is_err());
    assert!(median::filter(&[1.0], 0).is_err());
    assert!(median::filter(&[f32::NAN], 1).is_err());
    assert_eq!(median::filter(&[f64::MAX; 4], 2).unwrap(), [f64::MAX; 4]);
    assert_eq!(median::filter(&[f32::MAX; 4], 2).unwrap(), [f32::MAX; 4]);
}
