use ciglet_rs::approximation::special;

#[test]
fn invalid_source_results_are_corrected_and_valid_results_keep_original_bits() {
    let reference = include_str!("../../../tests/fixtures/lambert-exp-corrected.txt");
    let source = include_str!("../../../tests/fixtures/c-lambert-exp.txt");
    assert_eq!(reference.lines().count(), 481);
    assert_eq!(reference.lines().count(), source.lines().count());
    let mut corrected = 0;
    let mut preserved = 0;
    for (line, original) in reference.lines().zip(source.lines()) {
        let fields: Vec<_> = line.split_whitespace().collect();
        let source: Vec<_> = original.split_whitespace().collect();
        assert_eq!(fields[0], source[0]);
        let value = f32::from_bits(u32::from_str_radix(fields[0], 16).unwrap());
        let expected = u32::from_str_radix(fields[1], 16).unwrap();
        for (kernel, bits) in [special::lambert_w_exp, special::lambert_w_exp_fast]
            .into_iter()
            .zip(&source[1..])
        {
            let actual = kernel(value).unwrap();
            let original = u32::from_str_radix(bits, 16).unwrap();
            let previous = f32::from_bits(original);
            assert!(actual >= 0.0 && actual.is_finite(), "{line}");
            if previous.is_finite() && previous > 0.0 {
                assert_eq!(actual.to_bits(), original, "unchanged source: {line}");
                preserved += 1;
            } else {
                assert!(
                    actual.to_bits().abs_diff(expected) <= 1,
                    "corrected result: {line}"
                );
                if expected == 0 {
                    assert_eq!(actual.to_bits(), 0);
                }
                corrected += 1;
            }
        }
    }
    assert!(corrected > 800 && preserved > 0);
}

#[test]
fn subnormal_underflow_and_overflowing_source_inputs_remain_representable() {
    for kernel in [special::lambert_w_exp, special::lambert_w_exp_fast] {
        let tiny = kernel(-100.0).unwrap();
        assert!(tiny > 0.0 && tiny.is_subnormal());
        assert_eq!(kernel(-f32::MAX).unwrap().to_bits(), 0);
        assert_eq!(kernel(f32::MAX).unwrap(), f32::MAX);
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(kernel(invalid).is_err());
        }
    }
}
