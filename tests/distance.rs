use ciglet_rs::{
    distance::{self, Policy},
    statistics::Scalar,
};
fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"ISD1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    let cases = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    assert_eq!(cases, 53);
    let mut cursor = 12;
    for _ in 0..cases {
        let n = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;
        let mut values: Vec<T> = (0..2 * n + 1)
            .map(|_| {
                let value = if width == 4 {
                    f32::from_le_bytes(bytes[cursor..cursor + width].try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(bytes[cursor..cursor + width].try_into().unwrap())
                };
                cursor += width;
                T::from_f64(value).unwrap()
            })
            .collect();
        let expected = values.pop().unwrap();
        let actual =
            distance::log_itakura_saito_with(&values[..n], &values[n..], Policy::Legacy).unwrap();
        assert_eq!(actual.into().to_bits(), expected.into().to_bits());
    }
    assert_eq!(cursor, bytes.len());
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("fixtures/distance-float.bin"), 4);
    reference::<f64>(include_bytes!("fixtures/distance-double.bin"), 8);
}
#[test]
fn analytic_divergence_scale_and_near_identity() {
    for ratio in [0.125_f64, 0.5, 2.0, 8.0] {
        let expected = ratio - ratio.ln() - 1.0;
        let actual = distance::itakura_saito(&[ratio; 3], &[1.0; 3]).unwrap();
        assert!((actual - expected).abs() <= 2e-15 * expected);
        let log = distance::log_itakura_saito(&[ratio; 3], &[1.0; 3]).unwrap();
        assert!((log - expected.ln()).abs() < 1e-15);
        assert_eq!(
            actual,
            distance::itakura_saito(&[ratio * 0.25; 3], &[0.25; 3]).unwrap()
        );
    }
    let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
    let delta = adjacent - 1.0;
    let actual = distance::itakura_saito(&[adjacent], &[1.0]).unwrap();
    assert!((actual / (0.5 * delta * delta) - 1.0).abs() < 1e-14);
    assert_eq!(distance::itakura_saito(&[1.0_f64], &[1.0]).unwrap(), 0.0);
    assert_eq!(
        distance::log_itakura_saito(&[1.0_f64], &[1.0]).unwrap(),
        f64::NEG_INFINITY
    );
    let x = [0.5_f64, 2.0, 8.0];
    let mean = x.iter().map(|&value| value - value.ln() - 1.0).sum::<f64>() / 3.0;
    assert!((distance::itakura_saito(&x, &[1.0; 3]).unwrap() - mean).abs() < 2e-15);
    let adjacent32 = f32::from_bits(1.0_f32.to_bits() + 1);
    assert!(distance::itakura_saito(&[adjacent32], &[1.0_f32]).unwrap() > 0.0);
}
#[test]
fn extreme_ratios_zeros_and_validation() {
    for value in [0.0_f32, -1.0, f32::INFINITY, f32::NAN] {
        assert!(ciglet_rs::approximation::logarithm(value).is_err());
        assert!(ciglet_rs::approximation::logarithm2(value).is_err());
    }
    let tiny = f64::from_bits(1);
    let log_ratio = f64::MAX.ln() - tiny.ln();
    assert_eq!(
        distance::log_itakura_saito(&[f64::MAX], &[tiny]).unwrap(),
        log_ratio
    );
    assert_eq!(
        distance::itakura_saito(&[f64::MAX], &[tiny]).unwrap(),
        f64::INFINITY
    );
    assert!(
        (distance::itakura_saito(&[tiny], &[f64::MAX]).unwrap() - (log_ratio - 1.0)).abs() < 2e-12
    );
    for (a, b) in [(0.0_f64, 1.0), (1.0, 0.0)] {
        assert_eq!(distance::itakura_saito(&[a], &[b]).unwrap(), f64::INFINITY);
        assert_eq!(
            distance::log_itakura_saito(&[a], &[b]).unwrap(),
            f64::INFINITY
        );
    }
    assert!(distance::log_itakura_saito(&[0.0_f64], &[0.0]).is_err());
    assert!(distance::itakura_saito(&[0.0_f64, 0.0], &[1.0, 0.0]).is_err());
    for policy in [Policy::Stable, Policy::Legacy] {
        assert!(distance::log_itakura_saito_with::<f64>(&[], &[], policy).is_err());
        assert!(distance::log_itakura_saito_with(&[1.0_f64], &[], policy).is_err());
        assert!(distance::log_itakura_saito_with(&[-1.0_f64], &[1.0], policy).is_err());
        assert!(distance::log_itakura_saito_with(&[f64::NAN], &[1.0], policy).is_err());
    }
}
