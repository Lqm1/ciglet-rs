use ciglet_rs::{spectral_peak, statistics::Scalar};

fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"QIF1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    let count = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    assert_eq!(bytes.len(), 12 + count * 5 * width);
    for record in bytes[12..].chunks_exact(5 * width) {
        let values: Vec<T> = record
            .chunks_exact(width)
            .map(|value| {
                T::from_f64(if width == 4 {
                    f32::from_le_bytes(value.try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(value.try_into().unwrap())
                })
                .unwrap()
            })
            .collect();
        let peak = spectral_peak::interpolate(&values[..3], 1).unwrap();
        assert_eq!(peak.bin.into().to_bits(), values[3].into().to_bits());
        assert_eq!(peak.magnitude.into().to_bits(), values[4].into().to_bits());
    }
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("fixtures/spectral-peak-float.bin"), 4);
    reference::<f64>(include_bytes!("fixtures/spectral-peak-double.bin"), 8);
}
#[test]
fn analytic_peak_and_bounds() {
    let peak = spectral_peak::interpolate(&[0.0_f64, 1.0, 0.5], 1).unwrap();
    assert!((peak.bin - 7.0 / 6.0).abs() < 1e-15);
    assert!((peak.magnitude - 49.0 / 48.0).abs() < 1e-15);
    let capped = spectral_peak::interpolate(&[0.0_f64, 10.0, 10.0], 1).unwrap();
    assert_eq!(capped.bin, 1.5);
    assert_eq!(capped.magnitude, 10.2);
    assert!(spectral_peak::interpolate::<f64>(&[], 0).is_err());
    assert!(spectral_peak::interpolate(&[1.0_f64; 3], 0).is_err());
    assert!(spectral_peak::interpolate(&[1.0_f64; 3], 2).is_err());
    assert!(spectral_peak::interpolate(&[1.0_f64; 3], usize::MAX).is_err());
    assert!(spectral_peak::interpolate(&[0.0, f64::NAN, 0.0], 1).is_err());
    assert!(spectral_peak::interpolate(&[f64::MAX; 3], 1).is_err());
    // Values outside the three-bin neighborhood do not participate.
    assert_eq!(
        spectral_peak::interpolate(&[0.0, 1.0, 0.0, f64::NAN], 1)
            .unwrap()
            .magnitude,
        1.0
    );
}
