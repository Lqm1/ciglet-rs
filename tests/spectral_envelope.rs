use ciglet_rs::{
    spectral_envelope::{NyquistPolicy, Plan},
    statistics::Scalar,
};
fn reference<T: Scalar>(bytes: &[u8], width: usize, tolerance: f64) {
    assert_eq!(&bytes[..4], b"ENV1");
    fn integer(bytes: &[u8], cursor: &mut usize) -> usize {
        let x = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        x as usize
    }
    fn values<T: Scalar>(bytes: &[u8], cursor: &mut usize, width: usize, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let x = if width == 4 {
                    f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap())
                };
                *cursor += width;
                T::from_f64(x).unwrap()
            })
            .collect()
    }
    let mut cursor = 4;
    assert_eq!(integer(bytes, &mut cursor), width);
    assert_eq!(integer(bytes, &mut cursor), 144);
    let mut maximum = [0.0_f64; 2];
    for case in 0..144 {
        let n = integer(bytes, &mut cursor);
        let count = integer(bytes, &mut cursor);
        let f0 = values::<T>(bytes, &mut cursor, width, 1)[0];
        let spectrum = values::<T>(bytes, &mut cursor, width, n / 2 + 1);
        let expected = values::<T>(bytes, &mut cursor, width, n);
        let expected_coeff = values::<T>(bytes, &mut cursor, width, n / 2 + 1);
        let mut plan = Plan::new(n).unwrap();
        let result = plan
            .estimate_with(
                &spectrum,
                f0,
                if count == 0 { None } else { Some(count) },
                NyquistPolicy::Legacy,
            )
            .unwrap();
        for (field, (actual, expected)) in [
            (result.log_spectrum.as_slice(), expected.as_slice()),
            (result.cepstrum.as_slice(), expected_coeff.as_slice()),
        ]
        .into_iter()
        .enumerate()
        {
            for (&a, &e) in actual.iter().zip(expected) {
                let error = (a.into() - e.into()).abs() / (1.0 + e.into().abs());
                maximum[field] = maximum[field].max(error);
                assert!(
                    error <= tolerance,
                    "case={case} field={field} {} != {} error={error:e}",
                    a.into(),
                    e.into()
                );
            }
        }
    }
    assert_eq!(cursor, bytes.len());
    eprintln!("envelope width={width} maximum normalized errors={maximum:?}");
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(
        include_bytes!("fixtures/spectral-envelope-float.bin"),
        4,
        3e-7,
    );
    reference::<f64>(
        include_bytes!("fixtures/spectral-envelope-double.bin"),
        8,
        5e-16,
    );
}
#[test]
fn constant_spectrum_and_reusable_plan() {
    let mut plan = Plan::<f64>::new(32).unwrap();
    for value in [0.0, 0.5, 2.0] {
        // Integer width avoids the source fractional endpoint artifact.
        let result = plan.estimate(&[value; 17], 0.1875, None).unwrap();
        let expected = ciglet_rs::approximation::logarithm((value + 1e-15) as f32).unwrap() as f64;
        for &log in &result.log_spectrum {
            assert!((log - expected).abs() < 2e-13);
        }
        assert!((result.cepstrum[0] - expected).abs() < 2e-13);
        for &coefficient in &result.cepstrum[1..] {
            assert!(coefficient.abs() < 2e-13);
        }
        assert_eq!(result.log_spectrum.len(), 32);
        assert_eq!(result.cepstrum.len(), 17);
    }
    let source: Vec<_> = (0..17).map(|i| 0.25 + i as f64 * 0.125).collect();
    let first = plan.estimate(&source, 0.125, Some(5)).unwrap();
    assert_eq!(first, plan.estimate(&source, 0.125, Some(5)).unwrap());
    for i in 17..32 {
        assert!((first.log_spectrum[i] - first.log_spectrum[32 - i]).abs() < 1e-14);
    }
}
#[test]
fn corrected_cutoff_nyquist_and_parameter_bounds() {
    let mut plan = Plan::<f64>::new(8).unwrap();
    let result = plan
        .estimate(&[0.5, 1.0, 2.0, 4.0, 8.0], 0.125, Some(2))
        .unwrap();
    for &value in &result.log_spectrum {
        assert!((value - result.log_spectrum[0]).abs() < 1e-14);
    }
    let preserve = plan
        .estimate(&[0.125, 0.25, 0.5, 1.0, 8.0], 0.25, None)
        .unwrap();
    let legacy = plan
        .estimate_with(
            &[0.125, 0.25, 0.5, 1.0, 8.0],
            0.25,
            None,
            NyquistPolicy::Legacy,
        )
        .unwrap();
    assert_ne!(preserve, legacy);
    assert_eq!(legacy.cepstrum[4], legacy.cepstrum[3]);
    for f0 in [0.0, -0.1, 0.5001, f64::NAN, f64::INFINITY] {
        assert!(plan.estimate(&[0.5; 5], f0, None).is_err());
    }
    for count in [0, 1, i32::MAX as usize + 1] {
        assert!(plan.estimate(&[0.5; 5], 0.125, Some(count)).is_err());
    }
    assert!(plan.estimate(&[0.5; 4], 0.125, None).is_err());
    assert!(plan.estimate(&[-1.0; 5], 0.125, None).is_err());
    assert!(plan.estimate(&[f64::INFINITY; 5], 0.125, None).is_err());
    assert!(Plan::<f32>::new(1).is_err());
    assert!(Plan::<f32>::new(7).is_err());
    let small = Plan::<f32>::new(2)
        .unwrap()
        .estimate(&[0.5, 1.0], 0.5, None)
        .unwrap();
    assert_eq!(small.log_spectrum[0], small.log_spectrum[1]);
    let mut tiny_plan = Plan::<f64>::new(2).unwrap();
    let tiny = tiny_plan.estimate(&[0.5, 1.0], 1e-10, None).unwrap();
    let explicit = tiny_plan.estimate(&[0.5, 1.0], 1e-10, Some(2)).unwrap();
    assert_eq!(tiny, explicit);
    assert_eq!(tiny.log_spectrum[0], tiny.log_spectrum[1]);
    let wide_count = plan.estimate(&[0.0; 5], 1e-12, None).unwrap();
    for &value in &wide_count.log_spectrum {
        assert!((value - wide_count.log_spectrum[0]).abs() < 1e-13);
    }
}
