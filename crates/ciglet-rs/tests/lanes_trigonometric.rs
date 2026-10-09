use ciglet_rs::approximation::{
    Precision,
    lanes::{self, trigonometric},
};

#[test]
fn twelve_vector_formulas_preserve_sse2_bits_or_reject_nonfinite_results() {
    let fixture = include_str!("../../../tests/fixtures/c-lanes-trigonometric.txt");
    assert_eq!(fixture.lines().count(), 144);
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
            "vfastsin" | "vfastersin" => trigonometric::sine(values, precision),
            "vfastcos" | "vfastercos" => trigonometric::cosine(values, precision),
            "vfasttan" | "vfastertan" => trigonometric::tangent(values, precision),
            "vfastsinfull" | "vfastersinfull" => trigonometric::sine_full(values, precision),
            "vfastcosfull" | "vfastercosfull" => trigonometric::cosine_full(values, precision),
            "vfasttanfull" | "vfastertanfull" => trigonometric::tangent_full(values, precision),
            _ => panic!("unknown vector trigonometric reference"),
        };
        if expected
            .into_iter()
            .all(|bits| f32::from_bits(bits).is_finite())
        {
            assert_eq!(lanes::to_bits(actual.unwrap()), expected, "{line}");
        } else {
            assert!(actual.is_err(), "source nonfinite result: {line}");
        }
    }
}

#[test]
fn vector_cosine_equality_branch_and_checked_lane_domains() {
    assert_eq!(
        lanes::to_bits(
            trigonometric::cosine([std::f32::consts::FRAC_PI_2; 4], Precision::Approximate)
                .unwrap()
        ),
        [0; 4]
    );
    type Kernel = fn(lanes::Floats, Precision) -> Result<lanes::Floats, ciglet_rs::Error>;
    let kernels: [Kernel; 6] = [
        trigonometric::sine,
        trigonometric::cosine,
        trigonometric::tangent,
        trigonometric::sine_full,
        trigonometric::cosine_full,
        trigonometric::tangent_full,
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
            if precision != Precision::Exact {
                for invalid in [f32::MAX, -f32::MAX, 2.0e10, -2.0e10] {
                    let mut values = [0.0; 4];
                    values[index] = invalid;
                    for kernel in [
                        trigonometric::sine_full,
                        trigonometric::cosine_full,
                        trigonometric::tangent_full,
                    ] {
                        assert!(kernel(values, precision).is_err());
                    }
                }
            }
        }
    }
    assert_eq!(
        lanes::to_bits(trigonometric::sine([-0.0; 4], Precision::Exact).unwrap()),
        [0x80000000; 4]
    );
    assert_eq!(
        trigonometric::cosine_full([0.0; 4], Precision::Exact).unwrap(),
        [1.0; 4]
    );
    assert_eq!(
        trigonometric::tangent_full([0.0; 4], Precision::Exact).unwrap(),
        [0.0; 4]
    );
}
