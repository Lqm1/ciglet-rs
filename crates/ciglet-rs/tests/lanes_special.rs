use ciglet_rs::approximation::lanes::{self, special};

#[test]
fn fourteen_formulas_preserve_sse2_bits_and_repair_invalid_lambert_lanes() {
    let fixture = include_str!("../../../tests/fixtures/c-lanes-special.txt");
    assert_eq!(fixture.lines().count(), 67);
    let roots: std::collections::HashMap<u32, u32> =
        include_str!("../../../tests/fixtures/lambert-exp-corrected.txt")
            .lines()
            .map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                (
                    u32::from_str_radix(fields[0], 16).unwrap(),
                    u32::from_str_radix(fields[1], 16).unwrap(),
                )
            })
            .collect();
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let inputs: [u32; 4] =
            std::array::from_fn(|index| u32::from_str_radix(fields[index + 1], 16).unwrap());
        let expected: [u32; 4] =
            std::array::from_fn(|index| u32::from_str_radix(fields[index + 5], 16).unwrap());
        let values = lanes::from_bits(inputs);
        let actual = match fields[0] {
            "vfasterfc" => special::complementary_error(values),
            "vfastererfc" => special::complementary_error_fast(values),
            "vfasterf" => special::error(values),
            "vfastererf" => special::error_fast(values),
            "vfastinverseerf" => special::inverse_error(values),
            "vfasterinverseerf" => special::inverse_error_fast(values),
            "vfastlgamma" => special::log_gamma(values),
            "vfasterlgamma" => special::log_gamma_fast(values),
            "vfastdigamma" => special::digamma(values),
            "vfasterdigamma" => special::digamma_fast(values),
            "vfastlambertw" => special::lambert_w(values),
            "vfasterlambertw" => special::lambert_w_fast(values),
            "vfastlambertwexpx" => special::lambert_w_exp(values),
            "vfasterlambertwexpx" => special::lambert_w_exp_fast(values),
            _ => panic!("unknown vector special function"),
        }
        .unwrap();
        for index in 0..4 {
            let original = f32::from_bits(expected[index]);
            if fields[0].ends_with("lambertwexpx") && (!original.is_finite() || original <= 0.0) {
                assert!(actual[index] >= 0.0 && actual[index].is_finite(), "{line}");
                let root = roots
                    .get(&inputs[index])
                    .expect("independent corrected root");
                assert!(actual[index].to_bits().abs_diff(*root) <= 1, "{line}");
                if *root == 0 {
                    assert_eq!(actual[index].to_bits(), 0);
                }
            } else {
                assert_eq!(actual[index].to_bits(), expected[index], "{line}");
            }
        }
    }
}

#[test]
fn special_domains_are_checked_at_every_lane_and_negative_intervals_remain_supported() {
    type Kernel = fn(lanes::Floats) -> Result<lanes::Floats, ciglet_rs::Error>;
    let kernels: [Kernel; 14] = [
        special::complementary_error,
        special::complementary_error_fast,
        special::error,
        special::error_fast,
        special::inverse_error,
        special::inverse_error_fast,
        special::log_gamma,
        special::log_gamma_fast,
        special::digamma,
        special::digamma_fast,
        special::lambert_w,
        special::lambert_w_fast,
        special::lambert_w_exp,
        special::lambert_w_exp_fast,
    ];
    for index in 0..4 {
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut values = [0.5; 4];
            values[index] = invalid;
            for kernel in kernels {
                assert!(kernel(values).is_err());
            }
        }
        for invalid in [-1.0, 1.0] {
            let mut values = [0.5; 4];
            values[index] = invalid;
            assert!(special::inverse_error(values).is_err());
            assert!(special::inverse_error_fast(values).is_err());
        }
        let mut values = [0.5; 4];
        values[index] = 0.0;
        for kernel in [
            special::log_gamma,
            special::log_gamma_fast,
            special::digamma,
            special::digamma_fast,
        ] {
            assert!(kernel(values).is_err());
        }
        values[index] = -0.5;
        assert!(special::lambert_w(values).is_err());
        assert!(special::lambert_w_fast(values).is_err());
    }
    assert!(special::log_gamma([-1.75, -1.5, -1.25, 1.0]).is_ok());
    assert!(special::digamma([-1.75, -1.5, -0.75, -0.25]).is_ok());
    assert!(special::digamma_fast([-0.75, -0.5, -0.25, 1.0]).is_ok());
}
