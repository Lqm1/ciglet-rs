use ciglet_rs::{frame::fetch_frame, statistics::Scalar};
fn check<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"FRM1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 25);
    let input: Vec<T> = [-2.0, 0.5, 4.0, 1.25, -0.75, 3.0]
        .into_iter()
        .map(|v| T::from_f64(v).unwrap())
        .collect();
    let mut cursor = 12;
    for _ in 0..25 {
        let center = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as isize;
        let n = u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        cursor += 8;
        for value in fetch_frame(&input, center, n).unwrap() {
            let expected = if width == 4 {
                f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap())
            };
            cursor += width;
            assert_eq!(value.into().to_bits(), expected.to_bits());
        }
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn frames_match_original_float() {
    check::<f32>(include_bytes!("../../../tests/fixtures/frame-float.bin"), 4);
}
#[test]
fn frames_match_original_double() {
    check::<f64>(
        include_bytes!("../../../tests/fixtures/frame-double.bin"),
        8,
    );
}
#[test]
fn frame_bits_and_invalid_arithmetic_are_preserved() {
    let input = [f64::from_bits(0x7ff8000000000042), -0.0, f64::NEG_INFINITY];
    let output = fetch_frame(&input, 1, 3).unwrap();
    for (a, b) in output.iter().zip(input) {
        assert_eq!(a.to_bits(), b.to_bits());
    }
    assert!(fetch_frame::<f64>(&[], isize::MIN, 4).is_err());
    assert!(fetch_frame::<f32>(&[], 0, usize::MAX).is_err());
    assert_eq!(fetch_frame::<f64>(&[], 0, 3).unwrap(), [0.0; 3]);
}
