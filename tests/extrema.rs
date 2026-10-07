use ciglet_rs::extrema::{self, Direction};
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
    assert_eq!(&bytes[..4], b"EXT2");
    let mut cursor = 4;
    let width = integer(bytes, &mut cursor);
    let count = integer(bytes, &mut cursor);
    for case in 0..count {
        let n = integer(bytes, &mut cursor);
        let start = integer(bytes, &mut cursor);
        let end = integer(bytes, &mut cursor);
        let direction = if integer(bytes, &mut cursor) == 0 {
            Direction::Maximum
        } else {
            Direction::Minimum
        };
        let signal: Vec<_> = (0..n)
            .map(|_| scalar::<T>(bytes, &mut cursor, width))
            .collect();
        let peak = integer(bytes, &mut cursor);
        let extreme = integer(bytes, &mut cursor);
        assert_eq!(
            extrema::peak(&signal, start..=end, direction).unwrap(),
            peak,
            "peak case {case}"
        );
        assert_eq!(
            extrema::extremum(&signal, start..=end, direction).unwrap(),
            extreme,
            "extremum case {case}"
        );
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!("fixtures/extrema-float.bin"));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!("fixtures/extrema-double.bin"));
}
#[test]
fn boundaries_fallback_ties_and_inspected_ranges() {
    let x = [8.0, 1.0, 5.0, 0.0, 5.0, 1.0, 9.0];
    assert_eq!(extrema::peak(&x, 0..=6, Direction::Maximum).unwrap(), 0);
    assert_eq!(extrema::peak(&x, 1..=5, Direction::Maximum).unwrap(), 2);
    assert_eq!(extrema::extremum(&x, 0..=6, Direction::Maximum).unwrap(), 6);
    assert_eq!(extrema::peak(&[1.0], 0..=0, Direction::Minimum).unwrap(), 0);
    assert_eq!(
        extrema::extremum(&[1.0], 0..=0, Direction::Minimum).unwrap(),
        0
    );
    assert!(extrema::peak::<f32>(&[], 0..=0, Direction::Maximum).is_err());
    assert!(extrema::peak(&x, std::ops::RangeInclusive::new(5, 3), Direction::Maximum).is_err());
    assert!(extrema::peak(&x, 0..=7, Direction::Maximum).is_err());
    assert_eq!(
        extrema::peak(&[f32::NAN, 2.0], 1..=1, Direction::Maximum).unwrap(),
        1
    );
    let values = [f32::NAN, 1.0, 3.0, 2.0, f32::NAN];
    assert_eq!(
        extrema::extremum(&values, 1..=3, Direction::Maximum).unwrap(),
        2
    );
    assert_eq!(
        extrema::peak(&values, 2..=2, Direction::Maximum).unwrap(),
        2
    );
    assert!(extrema::peak(&values, 1..=3, Direction::Maximum).is_err());
}
