use ciglet_rs::approximation::{self, Precision, trigonometric};

fn value(hex: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(hex, 16).unwrap())
}

#[test]
fn bundled_trigonometry_and_power_match_c_bits() {
    let fixture = include_str!("../../../tests/fixtures/c-trigonometric.txt");
    assert_eq!(fixture.lines().count(), 456);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let angle = value(fields[1]);
        let precision = if fields[0].starts_with("faster") {
            Precision::Fast
        } else {
            Precision::Approximate
        };
        let actual = match fields[0] {
            "fastsin" | "fastersin" => trigonometric::sine(angle, precision),
            "fastcos" | "fastercos" => trigonometric::cosine(angle, precision),
            "fasttan" | "fastertan" => trigonometric::tangent(angle, precision),
            "fasttanfull" | "fastertanfull" => trigonometric::tangent_full(angle, precision),
            "fastsinfull" => approximation::sine(angle),
            "fastersinfull" => approximation::sine_fast(angle),
            "fastcosfull" => approximation::cosine(angle),
            "fastercosfull" => approximation::cosine_fast(angle),
            "fastpow" => approximation::power(angle, value(fields[2])),
            "fasterpow" => approximation::power_fast(angle, value(fields[2])),
            _ => panic!("unknown reference kernel"),
        };
        let expected = value(fields.last().unwrap());
        if expected.is_finite() {
            assert_eq!(actual.unwrap().to_bits(), expected.to_bits(), "{line}");
        } else {
            assert!(actual.is_err(), "nonfinite source result: {line}");
        }
    }
}

#[test]
fn invalid_angles_and_power_domains_are_checked() {
    type Kernel = fn(f32, Precision) -> Result<f32, ciglet_rs::Error>;
    let kernels: [Kernel; 4] = [
        trigonometric::sine,
        trigonometric::cosine,
        trigonometric::tangent,
        trigonometric::tangent_full,
    ];
    for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
        for kernel in kernels {
            for angle in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                assert!(kernel(angle, precision).is_err());
            }
        }
    }
    for precision in [Precision::Fast, Precision::Approximate] {
        for angle in [f32::MAX, -f32::MAX, 2.0e10, -2.0e10] {
            assert!(trigonometric::tangent_full(angle, precision).is_err());
        }
    }
    for base in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(approximation::power_fast(base, 1.0).is_err());
    }
    for exponent in [f32::NAN, f32::INFINITY, 128.0] {
        assert!(approximation::power_fast(2.0, exponent).is_err());
    }
    assert_eq!(
        trigonometric::sine(-0.0, Precision::Exact)
            .unwrap()
            .to_bits(),
        (-0.0_f32).to_bits()
    );
    assert_eq!(trigonometric::cosine(0.0, Precision::Exact).unwrap(), 1.0);
    assert_eq!(
        trigonometric::tangent_full(0.0, Precision::Exact).unwrap(),
        0.0
    );
}
