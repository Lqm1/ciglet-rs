use ciglet_rs::{
    sinc::{self, Policy},
    statistics::Scalar,
};
fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"SNC1");
    assert_eq!(
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize,
        width
    );
    let cases = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    assert_eq!(cases, 192);
    assert_eq!(bytes.len(), 12 + cases * 4 * width);
    let mut maximum = [0.0_f64; 2];
    for record in bytes[12..].chunks_exact(4 * width) {
        let fields: Vec<T> = record
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
        let actual = [
            sinc::aliased_with(fields[0], fields[1], Policy::Legacy).unwrap(),
            sinc::derivative_with(fields[0], fields[1], Policy::Legacy).unwrap(),
        ];
        for (field, (&a, &b)) in actual.iter().zip(&fields[2..]).enumerate() {
            if width == 4 || a == T::zero() || b == T::zero() {
                assert_eq!(a.into().to_bits(), b.into().to_bits());
            }
            let error = (a.into() - b.into()).abs() / (1.0 + b.into().abs());
            assert!(error.is_finite());
            maximum[field] = maximum[field].max(error);
        }
    }
    println!("{width}-byte Legacy sinc maximum errors {maximum:?}");
    let bounds = if width == 4 {
        [0.0, 0.0]
    } else {
        [2e-16, 5e-16]
    };
    for (field, (&actual, &bound)) in maximum.iter().zip(&bounds).enumerate() {
        assert!(actual <= bound, "field {field}: {actual} exceeds {bound}");
    }
}
#[test]
fn unchanged_c_reference() {
    reference::<f32>(include_bytes!("../../../tests/fixtures/sinc-float.bin"), 4);
    reference::<f64>(include_bytes!("../../../tests/fixtures/sinc-double.bin"), 8);
}
#[test]
fn analytic_integer_lengths_and_small_angles() {
    for angle in [
        -std::f64::consts::TAU,
        -1.0,
        -1e-4,
        -1e-8,
        0.0,
        1e-12,
        1e-8,
        1e-4,
        1.0,
        std::f64::consts::TAU,
        std::f64::consts::TAU + 1e-8,
        2.0 * std::f64::consts::TAU,
    ] {
        assert!((sinc::aliased(2.0_f64, angle).unwrap() - 2.0 * (angle * 0.5).cos()).abs() < 2e-15);
        assert!((sinc::derivative(2.0_f64, angle).unwrap() + (angle * 0.5).sin()).abs() < 2e-15);
        assert!((sinc::aliased(3.0_f64, angle).unwrap() - (1.0 + 2.0 * angle.cos())).abs() < 3e-15);
        assert!((sinc::derivative(3.0_f64, angle).unwrap() + 2.0 * angle.sin()).abs() < 3e-15);
        assert_eq!(sinc::aliased(1.0_f64, angle).unwrap(), 1.0);
        assert_eq!(sinc::derivative(-1.0_f64, angle).unwrap(), 0.0);
    }
    assert_eq!(sinc::aliased(2.0_f64, std::f64::consts::TAU).unwrap(), -2.0);
    assert_eq!(
        sinc::aliased_with(2.0_f64, std::f64::consts::TAU, Policy::Legacy).unwrap(),
        2.0
    );
    assert_eq!(
        sinc::derivative_with(3.0_f64, 1e-8, Policy::Legacy).unwrap(),
        0.0
    );
    assert!((sinc::derivative(3.0_f64, 1e-8).unwrap() + 2e-8).abs() < 1e-23);
    assert!((sinc::derivative(0.5_f64, 1e-8).unwrap() - 3.125e-10).abs() < 1e-25);
    for angle in [0.0_f32, 1e-8, 1e-4, 1.0, std::f32::consts::TAU] {
        let exact = -(f64::from(angle) * 0.5).sin();
        assert!((f64::from(sinc::derivative(2.0_f32, angle).unwrap()) - exact).abs() < 1e-7);
    }
}
#[test]
fn validation_and_removable_poles() {
    for policy in [Policy::Legacy, Policy::Stable] {
        assert!(sinc::aliased_with(f64::NAN, 0.0, policy).is_err());
        assert!(sinc::derivative_with(1.0_f64, f64::INFINITY, policy).is_err());
        assert_eq!(sinc::aliased_with(f64::MAX, 0.0, policy).unwrap(), f64::MAX);
        assert_eq!(sinc::derivative_with(f64::MAX, 0.0, policy).unwrap(), 0.0);
        assert!(sinc::aliased_with(f64::MAX, 4.0, policy).is_err());
    }
    assert_eq!(sinc::aliased(-2.0_f64, std::f64::consts::TAU).unwrap(), 2.0);
    assert_eq!(sinc::aliased(0.0_f64, 1.0).unwrap(), 0.0);
    assert_eq!(sinc::aliased(0.5_f64, 0.0).unwrap(), 0.5);
    assert_eq!(
        sinc::aliased(f64::MAX, std::f64::consts::TAU).unwrap(),
        -f64::MAX
    );
    assert_eq!(
        sinc::derivative(f64::MAX, std::f64::consts::TAU).unwrap(),
        0.0
    );
    assert!(sinc::aliased(0.5_f64, std::f64::consts::TAU).unwrap() > 1e15);
}
