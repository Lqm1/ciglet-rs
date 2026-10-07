use ciglet_rs::approximation::special;

#[test]
fn all_special_functions_match_unchanged_bundled_c_bits() {
    let fixture = include_str!("fixtures/c-special.txt");
    assert_eq!(fixture.lines().count(), 232);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let value = f32::from_bits(u32::from_str_radix(fields[1], 16).unwrap());
        let expected = u32::from_str_radix(fields[2], 16).unwrap();
        let actual = match fields[0] {
            "fasterfc" => special::complementary_error(value),
            "fastererfc" => special::complementary_error_fast(value),
            "fasterf" => special::error(value),
            "fastererf" => special::error_fast(value),
            "fastinverseerf" => special::inverse_error(value),
            "fasterinverseerf" => special::inverse_error_fast(value),
            "fastlgamma" => special::log_gamma(value),
            "fasterlgamma" => special::log_gamma_fast(value),
            "fastdigamma" => special::digamma(value),
            "fasterdigamma" => special::digamma_fast(value),
            "fastlambertw" => special::lambert_w(value),
            "fasterlambertw" => special::lambert_w_fast(value),
            "fastlambertwexpx" => special::lambert_w_exp(value),
            "fasterlambertwexpx" => special::lambert_w_exp_fast(value),
            _ => panic!("unknown reference function"),
        }
        .unwrap();
        assert_eq!(actual.to_bits(), expected, "{line}");
    }
}

#[test]
fn error_function_domains_are_checked_without_saturation() {
    type Kernel = fn(f32) -> Result<f32, ciglet_rs::Error>;
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
    for kernel in kernels {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(kernel(value).is_err());
        }
    }
    for kernel in [special::inverse_error, special::inverse_error_fast] {
        for value in [-2.0, -1.0, 1.0, 2.0] {
            assert!(kernel(value).is_err());
        }
    }
    for kernel in [
        special::complementary_error,
        special::complementary_error_fast,
        special::error,
        special::error_fast,
    ] {
        assert!(kernel(40.0).is_err());
    }
    assert!(special::complementary_error(-f32::MAX).is_err());
    for kernel in [
        special::log_gamma,
        special::log_gamma_fast,
        special::digamma,
        special::digamma_fast,
    ] {
        for value in [-1.0, -0.0, 0.0] {
            assert!(kernel(value).is_err());
        }
    }
    for kernel in [special::lambert_w, special::lambert_w_fast] {
        assert!(kernel(-0.5).is_err());
    }
}
