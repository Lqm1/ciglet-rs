use ciglet_rs::approximation::{Precision, hyperbolic};

#[test]
fn all_eight_scalar_kernels_match_bundled_c_bits() {
    let fixture = include_str!("fixtures/c-hyperbolic.txt");
    assert_eq!(fixture.lines().count(), 128);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let value: f32 = fields[1].parse().unwrap();
        let expected: f32 = fields[2].parse().unwrap();
        let precision = if fields[0].starts_with("faster") {
            Precision::Fast
        } else {
            Precision::Approximate
        };
        let actual = match fields[0] {
            "fastsinh" | "fastersinh" => hyperbolic::sine(value, precision),
            "fastcosh" | "fastercosh" => hyperbolic::cosine(value, precision),
            "fasttanh" | "fastertanh" => hyperbolic::tangent(value, precision),
            "fastsigmoid" | "fastersigmoid" => hyperbolic::sigmoid(value, precision),
            _ => panic!("unknown reference kernel"),
        }
        .unwrap();
        assert_eq!(actual.to_bits(), expected.to_bits(), "{line}");
    }
}

#[test]
fn nonfinite_inputs_and_unrepresentable_source_exponentials_return_errors() {
    type Kernel = fn(f32, Precision) -> Result<f32, ciglet_rs::Error>;
    let kernels: [Kernel; 4] = [
        hyperbolic::sine,
        hyperbolic::cosine,
        hyperbolic::tangent,
        hyperbolic::sigmoid,
    ];
    for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
        for kernel in kernels {
            for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                assert!(kernel(value, precision).is_err());
            }
        }
        assert!(hyperbolic::sine(100.0, precision).is_err());
        assert!(hyperbolic::cosine(100.0, precision).is_err());
        assert!(hyperbolic::tangent(-100.0, precision).is_err());
        assert!(hyperbolic::sigmoid(-100.0, precision).is_err());
    }
    assert_eq!(hyperbolic::sine(0.0, Precision::Exact).unwrap(), 0.0);
    assert_eq!(hyperbolic::cosine(0.0, Precision::Exact).unwrap(), 1.0);
    assert_eq!(hyperbolic::tangent(0.0, Precision::Exact).unwrap(), 0.0);
    assert_eq!(hyperbolic::sigmoid(0.0, Precision::Exact).unwrap(), 0.5);
}
