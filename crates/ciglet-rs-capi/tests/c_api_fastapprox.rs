use ciglet_rs_capi::*;
use std::{collections::BTreeSet, ptr};
#[path = "support/fastapprox_c_api.rs"]
mod dispatch;

fn scalar(name: &str, value: f32, exponent: f32) -> Result<f32, u32> {
    let kernel = dispatch::SCALARS
        .iter()
        .find(|entry| entry.0 == name)
        .unwrap()
        .1;
    let mut output = 71_f32;
    // SAFETY: By-value inputs and an aligned exclusive initialized result slot.
    let status = unsafe { kernel.call(value, exponent, &mut output) };
    if status == 0 {
        Ok(output)
    } else {
        assert_eq!(status, 3, "{name}");
        assert_eq!(output.to_bits(), 71_f32.to_bits(), "{name}");
        Err(status)
    }
}
fn vector(name: &str, value: [f32; 4], exponent: [f32; 4]) -> Result<[f32; 4], u32> {
    let kernel = dispatch::VECTORS
        .iter()
        .find(|entry| entry.0 == name)
        .unwrap()
        .1;
    let mut output = CigletRsFloats4 { values: [71.; 4] };
    // SAFETY: By-value descriptors and an aligned exclusive initialized result slot.
    let status = unsafe { kernel.call(value, exponent, &mut output) };
    if status == 0 {
        Ok(output.values)
    } else {
        assert_eq!(status, 3, "{name}");
        assert_eq!(output.values, [71.; 4], "{name}");
        Err(status)
    }
}
fn bits(fields: &[&str]) -> [u32; 4] {
    std::array::from_fn(|i| u32::from_str_radix(fields[i], 16).unwrap())
}
fn hex(field: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(field, 16).unwrap())
}

#[test]
fn all_original_scalar_corpora_and_complete_coverage() {
    let mut covered = BTreeSet::new();
    let trig = include_str!("../../../tests/fixtures/c-trigonometric.txt");
    assert_eq!(trig.lines().count(), 456);
    for line in trig.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        covered.insert(f[0]);
        let actual = scalar(f[0], hex(f[1]), if f.len() == 4 { hex(f[2]) } else { 0. });
        let expected = hex(f.last().unwrap());
        if expected.is_finite() {
            assert_eq!(actual.unwrap().to_bits(), expected.to_bits(), "{line}");
        } else {
            assert!(actual.is_err(), "{line}");
        }
    }
    let special = include_str!("../../../tests/fixtures/c-special.txt");
    assert_eq!(special.lines().count(), 232);
    for line in special.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        covered.insert(f[0]);
        let actual = scalar(f[0], hex(f[1]), 0.).unwrap();
        let expected = hex(f[2]);
        // Unchanged approved scalar Lambert-exp correction criterion.
        if f[0].ends_with("lambertwexpx") && expected <= 0. {
            assert!(actual > 0., "{line}");
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits(), "{line}");
        }
    }
    let hyper = include_str!("../../../tests/fixtures/c-hyperbolic.txt");
    assert_eq!(hyper.lines().count(), 128);
    for line in hyper.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        covered.insert(f[0]);
        assert_eq!(
            scalar(f[0], f[1].parse().unwrap(), 0.).unwrap().to_bits(),
            f[2].parse::<f32>().unwrap().to_bits(),
            "{line}"
        );
    }
    // Basic native lane APIs delegate to source-order scalar kernels. Original
    // SSE2 bit records independently cover every corresponding scalar function.
    let basic = include_str!("../../../tests/fixtures/c-lanes.txt");
    assert_eq!(basic.lines().count(), 80);
    for line in basic.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        if !f[0].starts_with('v') {
            continue;
        }
        let name = &f[0][1..];
        covered.insert(name);
        let inputs = bits(&f[1..5]).map(f32::from_bits);
        let exponents = if f.len() == 13 {
            bits(&f[5..9]).map(f32::from_bits)
        } else {
            [0.; 4]
        };
        let expected = bits(&f[f.len() - 4..]);
        for i in 0..4 {
            let actual = scalar(name, inputs[i], exponents[i]);
            if f32::from_bits(expected[i]).is_finite() {
                assert_eq!(actual.unwrap().to_bits(), expected[i], "{line} lane {i}");
            } else {
                assert!(actual.is_err(), "{line} lane {i}");
            }
        }
    }
    assert_eq!(
        covered,
        dispatch::SCALARS.iter().map(|entry| entry.0).collect()
    );
}

#[test]
fn all_original_vector_corpora_and_complete_coverage() {
    let roots: std::collections::HashMap<_, _> =
        include_str!("../../../tests/fixtures/lambert-exp-corrected.txt")
            .lines()
            .map(|line| {
                let f: Vec<_> = line.split_whitespace().collect();
                (
                    u32::from_str_radix(f[0], 16).unwrap(),
                    u32::from_str_radix(f[1], 16).unwrap(),
                )
            })
            .collect();
    let mut covered = BTreeSet::new();
    for (fixture, count) in [
        (include_str!("../../../tests/fixtures/c-lanes.txt"), 80),
        (
            include_str!("../../../tests/fixtures/c-lanes-trigonometric.txt"),
            144,
        ),
        (
            include_str!("../../../tests/fixtures/c-lanes-special.txt"),
            67,
        ),
        (
            include_str!("../../../tests/fixtures/c-lanes-hyperbolic.txt"),
            48,
        ),
    ] {
        assert_eq!(fixture.lines().count(), count);
        for line in fixture.lines() {
            let f: Vec<_> = line.split_whitespace().collect();
            if !f[0].starts_with('v') {
                continue;
            }
            covered.insert(f[0]);
            let inputs = bits(&f[1..5]);
            let exponents = if f.len() == 13 {
                bits(&f[5..9]).map(f32::from_bits)
            } else {
                [0.; 4]
            };
            let expected = bits(&f[f.len() - 4..]);
            let actual = vector(f[0], inputs.map(f32::from_bits), exponents);
            if f[0].ends_with("lambertwexpx") {
                let actual = actual.unwrap();
                for i in 0..4 {
                    let original = f32::from_bits(expected[i]);
                    if !original.is_finite() || original <= 0. {
                        assert!(actual[i] >= 0. && actual[i].is_finite(), "{line}");
                        let root = roots.get(&inputs[i]).unwrap();
                        assert!(actual[i].to_bits().abs_diff(*root) <= 1, "{line}");
                        if *root == 0 {
                            assert_eq!(actual[i].to_bits(), 0);
                        }
                    } else {
                        assert_eq!(actual[i].to_bits(), expected[i], "{line}");
                    }
                }
            } else if expected
                .iter()
                .all(|&word| f32::from_bits(word).is_finite())
            {
                assert_eq!(actual.unwrap().map(f32::to_bits), expected, "{line}");
            } else {
                assert!(actual.is_err(), "{line}");
            }
        }
    }
    assert_eq!(
        covered,
        dispatch::VECTORS.iter().map(|entry| entry.0).collect()
    );
}

#[test]
fn every_kernel_retains_outputs_on_pointer_or_late_lane_error() {
    let mut result = 71_f32;
    let mut lanes = CigletRsFloats4 { values: [71.; 4] };
    // SAFETY: Invalid null/alignment pointers are rejected before access; other
    // destinations are initialized, aligned and exclusively writable.
    unsafe {
        for (name, kernel) in dispatch::SCALARS {
            assert_eq!(kernel.call(0.5, 0.25, ptr::null_mut()), 1, "{name}");
            let bad = (&mut result as *mut f32).cast::<u8>().add(1).cast::<f32>();
            assert_eq!(kernel.call(0.5, 0.25, bad), 1, "{name}");
            assert_eq!(kernel.call(f32::NAN, 0.25, &mut result), 3, "{name}");
            assert_eq!(result, 71.);
        }
        for (name, kernel) in dispatch::VECTORS {
            assert_eq!(
                kernel.call([0.5; 4], [0.25; 4], ptr::null_mut()),
                1,
                "{name}"
            );
            let bad = (&mut lanes as *mut CigletRsFloats4)
                .cast::<u8>()
                .add(1)
                .cast::<CigletRsFloats4>();
            assert_eq!(kernel.call([0.5; 4], [0.25; 4], bad), 1, "{name}");
            for index in 0..4 {
                let mut invalid = [0.5; 4];
                invalid[index] = f32::NAN;
                assert_eq!(
                    kernel.call(invalid, [0.25; 4], &mut lanes),
                    3,
                    "{name} lane {index}"
                );
                assert_eq!(lanes.values, [71.; 4]);
            }
        }
    }
}

#[test]
fn lane_conversion_helpers_preserve_bits_order_and_layout() {
    use std::mem::{align_of, size_of};
    assert_eq!(size_of::<CigletRsFloats4>(), 16);
    assert_eq!(align_of::<CigletRsFloats4>(), align_of::<f32>());
    assert_eq!(size_of::<CigletRsIntegers4>(), 16);
    assert_eq!(size_of::<CigletRsBits4>(), 16);
    assert_eq!(size_of::<CigletRsIntegers64x2>(), 16);
    assert_eq!(align_of::<CigletRsIntegers64x2>(), align_of::<i64>());
    let raw = [0x7fc00042, 0xffc00099, 0x80000000, 0x7f800000];
    let mut floats = CigletRsFloats4 { values: [71.; 4] };
    let mut words = CigletRsBits4 { values: [71; 4] };
    let mut integers = CigletRsIntegers4 { values: [71; 4] };
    // SAFETY: By-value descriptors and independent aligned exclusive output slots.
    unsafe {
        assert_eq!(
            ciglet_rs_lanes_from_bits(CigletRsBits4 { values: raw }, &mut floats),
            0
        );
        assert_eq!(floats.values.map(f32::to_bits), raw);
        assert_eq!(ciglet_rs_lanes_to_bits(floats, &mut words), 0);
        assert_eq!(words.values, raw);
        assert_eq!(ciglet_rs_lanes_absolute(floats, &mut floats), 0);
        assert_eq!(
            floats.values.map(f32::to_bits),
            raw.map(|x| x & 0x7fff_ffff)
        );
        assert_eq!(ciglet_rs_lanes_to_integers(floats, &mut integers), 0);
        assert_eq!(integers.values, [i32::MIN, i32::MIN, 0, i32::MIN]);
        assert_eq!(
            ciglet_rs_lanes_from_integers(
                CigletRsIntegers4 {
                    values: [1, -2, i32::MAX, i32::MIN]
                },
                &mut floats
            ),
            0
        );
        assert_eq!(floats.values, [1., -2., 2_147_483_648., -2_147_483_648.]);
        assert_eq!(
            ciglet_rs_lanes_from_integer64_lanes(
                CigletRsIntegers64x2 {
                    values: [0x12345678_9abcdef0, -1]
                },
                &mut integers
            ),
            0
        );
        assert_eq!(integers.values, [0x9abcdef0_u32 as i32, 0x12345678, -1, -1]);
        assert_eq!(ciglet_rs_lanes_to_integers(floats, ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_lanes_from_integers(integers, ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_lanes_to_bits(floats, ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_lanes_from_bits(words, ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_lanes_absolute(floats, ptr::null_mut()), 1);
        assert_eq!(
            ciglet_rs_lanes_from_integer64_lanes(
                CigletRsIntegers64x2 { values: [0; 2] },
                ptr::null_mut()
            ),
            1
        );
    }
}

#[test]
fn original_sse2_conversion_and_absolute_records() {
    let mut covered = BTreeSet::new();
    let fixture = include_str!("../../../tests/fixtures/c-lanes.txt");
    assert_eq!(fixture.lines().count(), 80);
    for line in fixture.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        if f[0].starts_with('v') {
            continue;
        }
        covered.insert(f[0]);
        let input = bits(&f[1..5]);
        let expected = bits(&f[f.len() - 4..]);
        let mut floats = CigletRsFloats4 { values: [71.; 4] };
        let mut integers = CigletRsIntegers4 { values: [71; 4] };
        // SAFETY: Copied inputs and independent aligned exclusive result slots.
        let actual = unsafe {
            match f[0] {
                "to_integers" => {
                    assert_eq!(
                        ciglet_rs_lanes_to_integers(
                            CigletRsFloats4 {
                                values: input.map(f32::from_bits)
                            },
                            &mut integers
                        ),
                        0
                    );
                    integers.values.map(|value| value as u32)
                }
                "from_integers" => {
                    assert_eq!(
                        ciglet_rs_lanes_from_integers(
                            CigletRsIntegers4 {
                                values: input.map(|value| value as i32)
                            },
                            &mut floats
                        ),
                        0
                    );
                    floats.values.map(f32::to_bits)
                }
                "absolute" => {
                    assert_eq!(
                        ciglet_rs_lanes_absolute(
                            CigletRsFloats4 {
                                values: input.map(f32::from_bits)
                            },
                            &mut floats
                        ),
                        0
                    );
                    floats.values.map(f32::to_bits)
                }
                _ => panic!("unknown original conversion record: {line}"),
            }
        };
        assert_eq!(actual, expected, "{line}");
    }
    assert_eq!(
        covered,
        ["to_integers", "from_integers", "absolute"]
            .into_iter()
            .collect()
    );
}

#[test]
fn vector_source_storage_reuse_and_independent_lane_exponents() {
    let expected = [0., 1., 2., 3.].map(|exponent| scalar("fastpow", 2., exponent).unwrap());
    let mut value = CigletRsFloats4 {
        values: [0., 1., 2., 3.],
    };
    // SAFETY: Input descriptor is copied before its exclusive result slot is written.
    unsafe {
        assert_eq!(
            ciglet_rs_fastapprox_exponential2_lanes_f32(value, &mut value),
            0
        );
        assert_eq!(value.values, [1., 2., 4., 8.]);
        assert_eq!(
            ciglet_rs_fastapprox_power_lanes_f32(
                CigletRsFloats4 { values: [2.; 4] },
                CigletRsFloats4 {
                    values: [0., 1., 2., 3.]
                },
                &mut value
            ),
            0
        );
        assert_eq!(value.values, expected);
    }
}
