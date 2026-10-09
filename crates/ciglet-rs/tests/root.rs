use ciglet_rs::root;
use rustfft::FftNum;
fn check<T: FftNum + Into<f64>>(bytes: &[u8]) {
    assert_eq!(&bytes[..4], b"ROT1");
    let width = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let count = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let mut cursor = 12;
    for case in 0..count {
        let kind = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;
        let mut scalar = || {
            let value = if width == 4 {
                f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap())
            };
            cursor += width;
            T::from_f64(value).unwrap()
        };
        let a = scalar();
        let b = scalar();
        let expected = scalar();
        let result = root::find(
            |x: T| match kind {
                0 => x * x - T::from_f64(2.0).unwrap(),
                1 => x * x * x - x - T::from_f64(2.0).unwrap(),
                2 => T::from_f64(x.into().cos() - x.into()).unwrap(),
                3 => T::from_f64(x.into().exp() - 3.0).unwrap(),
                _ => x - T::from_f64(0.12345).unwrap(),
            },
            a,
            b,
        )
        .unwrap();
        assert_eq!(result, expected, "C root case {case}");
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_float() {
    check::<f32>(include_bytes!("../../../tests/fixtures/root-float.bin"));
}
#[test]
fn original_double() {
    check::<f64>(include_bytes!("../../../tests/fixtures/root-double.bin"));
}
#[test]
fn endpoints_fallback_invalid_values_and_captured_environment() {
    assert_eq!(root::find(|x: f64| x - 1.0, 1.0, 3.0).unwrap(), 1.0);
    assert_eq!(root::find(|x: f32| x - 3.0, 1.0, 3.0).unwrap(), 3.0);
    assert_eq!(root::find(|x: f64| x * x + 1.0, 1.0, 3.0).unwrap(), 2.0);
    assert!(root::find(|x: f64| x, 3.0, 1.0).is_err());
    assert!(root::find(|_: f64| f64::NAN, 0.0, 1.0).is_err());
    assert!(root::find(|x: f64| x, f64::NEG_INFINITY, 1.0).is_err());
    assert!(root::find(|x: f64| if x == 0.0 { f64::NAN } else { x }, -1.0, 1.0).is_err());
    assert_eq!(
        root::find(|_: f64| 1.0, f64::MAX, f64::MAX).unwrap(),
        f64::MAX
    );
    let target = 0.25;
    let mut calls = 0;
    let value = root::find(
        |x: f64| {
            calls += 1;
            (x - target) * 1e-200
        },
        0.0,
        1.0,
    )
    .unwrap();
    assert!(calls >= 2); // The source absolute residual tolerance applies even at tiny scales.
    assert_eq!(value, 0.0);
    assert_eq!(root::find(|x: f64| x - target, 0.0, 1.0).unwrap(), target);
}
