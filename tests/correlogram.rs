use ciglet_rs::{
    array::Array2,
    correlogram::{self, Frame, Method, Policy},
    statistics::Scalar,
};
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("fixtures/correlogram-float.bin"), 4);
    reference::<f64>(include_bytes!("fixtures/correlogram-double.bin"), 8);
}
fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"CRG1");
    let mut cursor = 4;
    fn integer(bytes: &[u8], cursor: &mut usize) -> u32 {
        let v = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        v
    }
    fn values<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let v = if width == 4 {
                    f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
                };
                *cursor += width;
                T::from_f64(v).unwrap()
            })
            .collect()
    }
    assert_eq!(integer(bytes, &mut cursor) as usize, width);
    assert_eq!(integer(bytes, &mut cursor), 32);
    let mut maximum_error = 0.0_f64;
    for case in 0..32 {
        let nx = integer(bytes, &mut cursor) as usize;
        let count = integer(bytes, &mut cursor) as usize;
        let periods = integer(bytes, &mut cursor) as usize;
        let method = [
            Method::Autocorrelation,
            Method::AbsoluteDifference,
            Method::SquaredDifference,
            Method::Yin,
        ][integer(bytes, &mut cursor) as usize];
        let samples = values::<T>(bytes, &mut cursor, width, nx);
        let centers: Vec<_> = (0..count)
            .map(|_| integer(bytes, &mut cursor) as i32)
            .collect();
        let frames: Vec<_> = centers
            .into_iter()
            .map(|center| Frame {
                center,
                length: integer(bytes, &mut cursor) as usize,
            })
            .collect();
        let expected = values::<T>(bytes, &mut cursor, width, count * periods);
        let actual =
            correlogram::compute_with(&samples, &frames, periods, method, Policy::Legacy).unwrap();
        let nf = integer(bytes, &mut cursor) as usize;
        let frequencies = values::<T>(bytes, &mut cursor, width, nf);
        let mapped_expected = values::<T>(bytes, &mut cursor, width, count * nf);
        let mapped = correlogram::to_frequency(&actual, 16000, &frequencies).unwrap();
        for (index, (&actual, &expected)) in actual
            .as_slice()
            .iter()
            .chain(mapped.as_slice())
            .zip(expected.iter().chain(&mapped_expected))
            .enumerate()
        {
            let error = (actual.into() - expected.into()).abs() / (1.0 + expected.into().abs());
            maximum_error = maximum_error.max(error);
            if width == 4 {
                assert_eq!(
                    actual.into().to_bits(),
                    expected.into().to_bits(),
                    "case={case} method={method:?} index={index} actual={} expected={}",
                    actual.into(),
                    expected.into()
                );
            } else {
                assert!(error <= 3e-15, "{} != {}", actual.into(), expected.into());
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("width={width}, maximum normalized correlogram error={maximum_error:e}");
}
#[test]
fn analytic_methods_boundaries_and_dc_conditioning() {
    let frame = [Frame {
        center: 2,
        length: 4,
    }];
    let ramp = [0.0_f64, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let absolute = correlogram::compute(&ramp, &frame, 5, Method::AbsoluteDifference).unwrap();
    assert_eq!(absolute.as_slice(), &[0.0, 4.0, 8.0, 12.0, 0.0]);
    let squared = correlogram::compute(&ramp, &frame, 5, Method::SquaredDifference).unwrap();
    assert_eq!(squared.as_slice(), &[0.0, 4.0, 16.0, 36.0, 0.0]);
    let yin = correlogram::compute(&ramp, &frame, 5, Method::Yin).unwrap();
    assert_eq!(yin.as_slice(), &[1.0, 1.0, 1.6, 108.0 / 56.0, 1.0]);
    let acf = correlogram::compute(&ramp, &frame, 5, Method::Autocorrelation).unwrap();
    for &value in &acf.as_slice()[..4] {
        assert!((value - 1.0).abs() < 3e-16);
    }
    let offset: Vec<_> = ramp.iter().map(|&x| 1e12 + x).collect();
    let legacy =
        correlogram::compute_with(&offset, &frame, 3, Method::Autocorrelation, Policy::Legacy)
            .unwrap();
    assert_eq!(*legacy.get(0, 0).unwrap(), 0.0);
    let offset = correlogram::compute(&offset, &frame, 3, Method::Autocorrelation).unwrap();
    for &value in offset.as_slice() {
        assert!((value - 1.0).abs() < 1e-7);
    }
    let alternating = [1.0_f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
    assert_eq!(
        *correlogram::compute(&alternating, &frame, 3, Method::Autocorrelation)
            .unwrap()
            .get(0, 1)
            .unwrap(),
        0.0
    );
    let huge = alternating.map(|x| x * f64::MAX);
    assert_eq!(
        *correlogram::compute(&huge, &frame, 3, Method::Autocorrelation)
            .unwrap()
            .get(0, 0)
            .unwrap(),
        1.0
    );
    let silence = correlogram::compute(&[0.0_f32; 8], &frame, 3, Method::Yin).unwrap();
    assert_eq!(silence.as_slice(), &[1.0, 1.0, 1.0]);
}
#[test]
fn frequency_mapping_and_invalid_inputs() {
    let periods = Array2::from_row_major(
        2,
        4,
        vec![0.0_f64, 10.0, 20.0, 30.0, 100.0, 110.0, 120.0, 130.0],
    )
    .unwrap();
    let mapped =
        correlogram::to_frequency(&periods, 100, &[100.0, 50.0, 40.0, 0.0, -100.0, -0.0]).unwrap();
    assert_eq!(
        mapped.as_slice(),
        &[
            10.0, 20.0, 25.0, 30.0, 0.0, 0.0, 110.0, 120.0, 125.0, 130.0, 100.0, 100.0
        ]
    );
    let single = Array2::from_row_major(1, 1, vec![3.0_f32]).unwrap();
    assert_eq!(
        correlogram::to_frequency(&single, 100, &[0.0, 1.0])
            .unwrap()
            .as_slice(),
        &[3.0, 3.0]
    );
    assert!(correlogram::to_frequency(&periods, 0, &[1.0]).is_err());
    assert!(correlogram::to_frequency(&periods, 100, &[f64::NAN]).is_err());
    assert!(
        correlogram::compute(
            &[1.0_f32],
            &[Frame {
                center: 0,
                length: 0
            }],
            3,
            Method::Yin
        )
        .is_err()
    );
    assert!(correlogram::compute(&[1.0_f32], &[], 0, Method::Yin).is_err());
    assert!(
        correlogram::compute::<f32>(
            &[],
            &[Frame {
                center: 0,
                length: 1
            }],
            3,
            Method::Yin
        )
        .is_err()
    );
    assert!(correlogram::compute(&[f64::NAN], &[], 3, Method::Yin).is_err());
    assert_eq!(
        correlogram::compute::<f32>(&[], &[], 3, Method::Yin)
            .unwrap()
            .rows(),
        0
    );
}
