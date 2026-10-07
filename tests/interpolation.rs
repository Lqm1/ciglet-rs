use ciglet_rs::{interpolation, resampling};
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
    assert_eq!(&bytes[..4], b"INT1");
    let mut cursor = 4;
    let width = integer(bytes, &mut cursor);
    let cases = integer(bytes, &mut cursor);
    for case in 0..cases {
        let kind = integer(bytes, &mut cursor);
        let count = integer(bytes, &mut cursor);
        let queries = integer(bytes, &mut cursor);
        let start = scalar::<T>(bytes, &mut cursor, width);
        let end = scalar::<T>(bytes, &mut cursor, width);
        let mut read = |length| {
            (0..length)
                .map(|_| scalar::<T>(bytes, &mut cursor, width))
                .collect::<Vec<_>>()
        };
        let coordinates = read(count);
        let values = read(count);
        let positions = read(queries);
        let expected = read(queries);
        let actual = if kind == 0 {
            interpolation::linear(&coordinates, &values, &positions)
        } else {
            interpolation::linear_uniform(start, end, &values, &positions)
        }
        .unwrap();
        assert_eq!(actual, expected, "case {case}");
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn original_c_float() {
    check::<f32>(include_bytes!("fixtures/interpolation-linear-float.bin"));
}
#[test]
fn original_c_double() {
    check::<f64>(include_bytes!("fixtures/interpolation-linear-double.bin"));
}
#[test]
fn unordered_queries_and_bounds() {
    assert_eq!(
        interpolation::linear(&[0.0, 2.0, 5.0], &[1.0, 5.0, -1.0], &[4.0, 1.0, -9.0, 10.0])
            .unwrap(),
        [1.0, 3.0, 1.0, -1.0]
    );
    assert_eq!(
        interpolation::linear_uniform(0.0, 3.0, &[1.0, 3.0, 5.0], &[1.5, -1.0, 8.0, 0.0]).unwrap(),
        [4.0, 1.0, 5.0, 1.0]
    );
    assert!(interpolation::linear::<f32>(&[], &[], &[]).is_err());
    assert!(interpolation::linear(&[1.0, 1.0], &[0.0, 2.0], &[1.0]).is_err());
    assert_eq!(
        interpolation::linear_uniform(0.0, 2.0, &[4.0], &[-2.0, 0.0, 1.0, 9.0]).unwrap(),
        [4.0; 4]
    );
    assert!(
        interpolation::linear_uniform(0.0, 2.0, &[0.0, 1.0], &[])
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        interpolation::linear_uniform(0.0, 2.0, &[1.0, 3.0], &[2.0]).unwrap(),
        [3.0]
    );
}

#[test]
fn missing_gaps_and_sinc_boundaries() {
    use resampling::BoundaryPolicy;
    assert_eq!(
        interpolation::fill_missing(&[-99.0, 2.0, -99.0, 6.0, -99.0], -99.0).unwrap(),
        [2.0, 2.0, 4.0, 6.0, 6.0]
    );
    assert_eq!(
        interpolation::fill_missing(&[f64::NAN, 2.0, f64::NAN, 6.0, f64::NAN], f64::NAN).unwrap(),
        [2.0, 2.0, 4.0, 6.0, 6.0]
    );
    assert_eq!(
        interpolation::fill_missing(&[f32::NAN; 3], f32::NAN).unwrap(),
        [0.0; 3]
    );
    assert!(
        interpolation::fill_missing::<f32>(&[], f32::NAN)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        interpolation::sinc_uniform(0.0, 2.0, &[7.0, 0.0], &[0.0, 100.0, -100.0]).unwrap(),
        [7.0, 0.0, 0.0]
    );
    assert_eq!(
        interpolation::sinc_uniform_with(
            0.0,
            2.0,
            &[7.0, 0.0],
            &[0.0],
            BoundaryPolicy::LegacySkipFirst
        )
        .unwrap(),
        [0.0]
    );
}

fn extended<T: FftNum + Into<f64>>(bytes: &[u8], policy: resampling::BoundaryPolicy) {
    assert_eq!(&bytes[..4], b"EXT1");
    let mut cursor = 4;
    let width = integer(bytes, &mut cursor);
    let cases = integer(bytes, &mut cursor);
    for case in 0..cases {
        let kind = integer(bytes, &mut cursor);
        let count = integer(bytes, &mut cursor);
        let queries = integer(bytes, &mut cursor);
        let start = scalar::<T>(bytes, &mut cursor, width);
        let end = scalar::<T>(bytes, &mut cursor, width);
        let mut read = |length| {
            (0..length)
                .map(|_| scalar::<T>(bytes, &mut cursor, width))
                .collect::<Vec<_>>()
        };
        let values = read(count);
        let positions = if kind == 0 { read(queries) } else { Vec::new() };
        let expected = read(queries);
        let actual = if kind == 0 {
            interpolation::sinc_uniform_with(start, end, &values, &positions, policy)
        } else {
            interpolation::fill_missing(&values, start)
        }
        .unwrap();
        assert_eq!(actual, expected, "extended case {case}");
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn corrected_c_float() {
    extended::<f32>(
        include_bytes!("fixtures/interpolation-extended-float.bin"),
        resampling::BoundaryPolicy::LegacySkipFirst,
    );
}
#[test]
fn corrected_c_double() {
    extended::<f64>(
        include_bytes!("fixtures/interpolation-extended-double.bin"),
        resampling::BoundaryPolicy::LegacySkipFirst,
    );
}

#[test]
fn corrected_including_c_float() {
    extended::<f32>(
        include_bytes!("fixtures/interpolation-extended-float-include.bin"),
        resampling::BoundaryPolicy::IncludeFirst,
    );
}
#[test]
fn corrected_including_c_double() {
    extended::<f64>(
        include_bytes!("fixtures/interpolation-extended-double-include.bin"),
        resampling::BoundaryPolicy::IncludeFirst,
    );
}
