use ciglet_rs::approximation::{
    Precision,
    lanes::{self, hyperbolic},
};

#[test]
fn eight_mixed_lane_formulas_match_original_sse2_bits() {
    let fixture = include_str!("fixtures/c-lanes-hyperbolic.txt");
    assert_eq!(fixture.lines().count(), 48);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let values = lanes::from_bits(std::array::from_fn(|index| {
            u32::from_str_radix(fields[index + 1], 16).unwrap()
        }));
        let expected =
            std::array::from_fn(|index| u32::from_str_radix(fields[index + 5], 16).unwrap());
        let precision = if fields[0].starts_with("vfaster") {
            Precision::Fast
        } else {
            Precision::Approximate
        };
        let actual = match fields[0] {
            "vfastsinh" | "vfastersinh" => hyperbolic::sine(values, precision),
            "vfastcosh" | "vfastercosh" => hyperbolic::cosine(values, precision),
            "vfasttanh" | "vfastertanh" => hyperbolic::tangent(values, precision),
            "vfastsigmoid" | "vfastersigmoid" => hyperbolic::sigmoid(values, precision),
            _ => panic!("unknown four-lane hyperbolic function"),
        }
        .unwrap();
        assert_eq!(lanes::to_bits(actual), expected, "{line}");
    }
}

#[test]
fn invalid_inputs_abort_every_lane_and_exact_mode_keeps_formula_stages() {
    type Kernel = fn(lanes::Floats, Precision) -> Result<lanes::Floats, ciglet_rs::Error>;
    let kernels: [Kernel; 4] = [
        hyperbolic::sine,
        hyperbolic::cosine,
        hyperbolic::tangent,
        hyperbolic::sigmoid,
    ];
    for precision in [Precision::Fast, Precision::Approximate, Precision::Exact] {
        for index in 0..4 {
            for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut values = [0.0; 4];
                values[index] = invalid;
                for kernel in kernels {
                    assert!(kernel(values, precision).is_err());
                }
            }
            let mut values = [0.0; 4];
            values[index] = 100.0;
            assert!(hyperbolic::sine(values, precision).is_err());
            assert!(hyperbolic::cosine(values, precision).is_err());
            values[index] = -100.0;
            assert!(hyperbolic::tangent(values, precision).is_err());
            assert!(hyperbolic::sigmoid(values, precision).is_err());
        }
    }
    assert_eq!(
        hyperbolic::sine([0.0; 4], Precision::Exact).unwrap(),
        [0.0; 4]
    );
    assert_eq!(
        hyperbolic::cosine([0.0; 4], Precision::Exact).unwrap(),
        [1.0; 4]
    );
    assert_eq!(
        hyperbolic::tangent([0.0; 4], Precision::Exact).unwrap(),
        [0.0; 4]
    );
    assert_eq!(
        hyperbolic::sigmoid([0.0; 4], Precision::Exact).unwrap(),
        [0.5; 4]
    );
}
