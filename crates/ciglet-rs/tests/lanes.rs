use ciglet_rs::approximation::lanes;

fn words(fields: &[&str]) -> [u32; 4] {
    std::array::from_fn(|index| u32::from_str_radix(fields[index], 16).unwrap())
}

#[test]
fn ten_kernels_and_conversion_helpers_match_original_sse2_bits() {
    let fixture = include_str!("../../../tests/fixtures/c-lanes.txt");
    assert_eq!(fixture.lines().count(), 80);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let input = lanes::from_bits(words(&fields[1..5]));
        let expected = words(&fields[fields.len() - 4..]);
        let actual = match fields[0] {
            "vfastpow2" => lanes::exponential2(input),
            "vfasterpow2" => lanes::exponential2_fast(input),
            "vfastexp" => lanes::exponential(input),
            "vfasterexp" => lanes::exponential_fast(input),
            "vfastlog2" => lanes::logarithm2(input),
            "vfasterlog2" => lanes::logarithm2_fast(input),
            "vfastlog" => lanes::logarithm(input),
            "vfasterlog" => lanes::logarithm_fast(input),
            "vfastpow" => lanes::power(input, lanes::from_bits(words(&fields[5..9]))),
            "vfasterpow" => lanes::power_fast(input, lanes::from_bits(words(&fields[5..9]))),
            "to_integers" => {
                assert_eq!(
                    lanes::to_integers(input).map(|value| value as u32),
                    expected,
                    "{line}"
                );
                continue;
            }
            "from_integers" => Ok(lanes::from_integers(
                words(&fields[1..5]).map(|value| value as i32),
            )),
            "absolute" => Ok(lanes::absolute(input)),
            _ => panic!("unknown four-lane reference"),
        };
        if fields[0].starts_with('v')
            && expected
                .into_iter()
                .any(|bits| !f32::from_bits(bits).is_finite())
        {
            assert!(actual.is_err(), "source nonfinite result: {line}");
        } else {
            assert_eq!(lanes::to_bits(actual.unwrap()), expected, "{line}");
        }
    }
}

#[test]
fn invalid_lanes_abort_the_owned_result_at_any_position() {
    type Kernel = fn(lanes::Floats) -> Result<lanes::Floats, ciglet_rs::Error>;
    let exponentials: [Kernel; 4] = [
        lanes::exponential2,
        lanes::exponential2_fast,
        lanes::exponential,
        lanes::exponential_fast,
    ];
    let logarithms: [Kernel; 4] = [
        lanes::logarithm2,
        lanes::logarithm2_fast,
        lanes::logarithm,
        lanes::logarithm_fast,
    ];
    for index in 0..4 {
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut values = [1.0; 4];
            values[index] = invalid;
            for kernel in exponentials.into_iter().chain(logarithms) {
                assert!(kernel(values).is_err());
            }
        }
        for invalid in [0.0, -1.0] {
            let mut values = [1.0; 4];
            values[index] = invalid;
            for kernel in logarithms {
                assert!(kernel(values).is_err());
            }
            assert!(lanes::power(values, [1.0; 4]).is_err());
            assert!(lanes::power_fast(values, [1.0; 4]).is_err());
        }
        let mut exponents = [1.0; 4];
        exponents[index] = 128.0;
        assert!(lanes::exponential2(exponents).is_err());
        assert!(lanes::exponential2_fast(exponents).is_err());
        exponents[index] = 256.0;
        assert!(lanes::power([2.0; 4], exponents).is_err());
        assert!(lanes::power_fast([2.0; 4], exponents).is_err());
    }
}

#[test]
fn owned_arrays_replace_splats_indexers_and_bit_punning() {
    let payloads = [0x7fc12345, 0xffc12345, 0x80000000, 1];
    assert_eq!(lanes::to_bits(lanes::from_bits(payloads)), payloads);
    let splat: lanes::Floats = [-0.0; 4];
    assert_eq!(lanes::to_bits(splat), [0x80000000; 4]);
    let integers: lanes::Integers = [-2; 4];
    assert_eq!(integers[3], -2);
    assert_eq!(
        lanes::from_integer64_lanes([0x0123456789abcdef, 0xfedcba9876543210_u64 as i64]),
        [
            0x89abcdef_u32 as i32,
            0x01234567,
            0x76543210,
            0xfedcba98_u32 as i32
        ]
    );
}
