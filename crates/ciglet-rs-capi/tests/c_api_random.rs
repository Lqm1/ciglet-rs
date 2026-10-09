use ciglet_rs::random::{self, Generator, Policy};
use ciglet_rs_capi::*;
use std::ptr;

macro_rules! corpus {
    ($name:ident, $t:ty, $width:expr, $bytes:expr, $uniform:ident, $normal:ident, $white:ident, $buffer:ident) => {
        #[test]
        fn $name() {
            let bytes = $bytes;
            assert_eq!(&bytes[..4], b"RND1");
            assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), $width);
            assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 128);
            let mut cursor = 12;
            for case in 0..128 {
                let mut integers = [0; 3];
                for value in &mut integers {
                    *value = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
                    cursor += 4;
                }
                let mut values: [$t; 7] = [0.0; 7];
                for value in &mut values {
                    *value = <$t>::from_le_bytes(
                        bytes[cursor..cursor + $width as usize].try_into().unwrap(),
                    );
                    cursor += $width as usize;
                }
                let [maximum, first, second] = integers;
                let mut output: $t = 71.0;
                // SAFETY: Local aligned initialized buffers and exclusive outputs.
                unsafe {
                    let status = $uniform(first, maximum, &mut output);
                    assert_eq!(status, 0);
                    assert_eq!(output.to_bits(), values[3].to_bits());
                    let status = $normal(
                        values[0],
                        values[1],
                        first,
                        second,
                        maximum,
                        CIGLET_RS_RANDOM_LEGACY,
                        &mut output,
                    );
                    if values[4].is_finite() {
                        assert_eq!(status, 0, "case {case}");
                        assert_eq!(output.to_bits(), values[4].to_bits(), "case {case}");
                    } else {
                        assert_eq!(status, 3);
                    }
                    for (draw, expected) in [(first, values[5]), (second, values[6])] {
                        assert_eq!($white(values[2], draw, maximum, &mut output), 0);
                        assert_eq!(output.to_bits(), expected.to_bits());
                    }
                    let draws = [first, second];
                    let mut result = [71.0; 2];
                    assert_eq!(
                        $buffer(values[2], draws.as_ptr(), 2, maximum, result.as_mut_ptr()),
                        0
                    );
                    assert_eq!(
                        result.map(<$t>::to_bits),
                        [values[5].to_bits(), values[6].to_bits()]
                    );
                    let invalid = [first, maximum + 1];
                    result = [71.0; 2];
                    assert_eq!(
                        $buffer(values[2], invalid.as_ptr(), 2, maximum, result.as_mut_ptr()),
                        3
                    );
                    assert_eq!(result, [71.0; 2]);
                }
            }
            assert_eq!(cursor, bytes.len());
        }
    };
}
corpus!(
    original_float_draws,
    f32,
    4,
    include_bytes!("../../../tests/fixtures/random-float.bin"),
    ciglet_rs_random_uniform_from_draw_f32,
    ciglet_rs_random_normal_from_draws_f32,
    ciglet_rs_random_white_from_draw_f32,
    ciglet_rs_random_white_from_draws_f32
);
corpus!(
    original_double_draws,
    f64,
    8,
    include_bytes!("../../../tests/fixtures/random-double.bin"),
    ciglet_rs_random_uniform_from_draw_f64,
    ciglet_rs_random_normal_from_draws_f64,
    ciglet_rs_random_white_from_draw_f64,
    ciglet_rs_random_white_from_draws_f64
);

macro_rules! streams {
    ($name:ident,$t:ty,$uniform:ident,$normal:ident,$with:ident,$white:ident) => {
        #[test]
        fn $name() {
            let seed = [37u8; 32];
            let mut native =
                random::Sampler::new(Generator::from_seed(seed).into_rng(), 31).unwrap();
            let mut handle = ptr::null_mut();
            let mut clone = ptr::null_mut();
            let mut output: $t = 71.0;
            // SAFETY: Live owned handles, initialized slots and independent local outputs.
            unsafe {
                assert_eq!(
                    ciglet_rs_random_create_with_maximum(seed.as_ptr(), 32, 31, &mut handle),
                    0
                );
                let mut maximum = 0;
                assert_eq!(ciglet_rs_random_maximum(handle, &mut maximum), 0);
                assert_eq!(maximum, 31);
                for _ in 0..16 {
                    assert_eq!($uniform(handle, &mut output), 0);
                    assert_eq!(output.to_bits(), native.uniform::<$t>().unwrap().to_bits());
                }
                assert_eq!($normal(handle, 3.0, 0.0, &mut output), 0);
                assert_eq!(
                    output.to_bits(),
                    native.normal::<$t>(3.0, 0.0).unwrap().to_bits()
                );
                for mode in [CIGLET_RS_RANDOM_STABLE, CIGLET_RS_RANDOM_LEGACY] {
                    let policy = if mode == 0 {
                        Policy::Stable
                    } else {
                        Policy::Legacy
                    };
                    assert_eq!($with(handle, 1.0, 2.0, mode, &mut output), 0);
                    assert_eq!(
                        output.to_bits(),
                        native
                            .normal_with::<$t>(1.0, 2.0, policy)
                            .unwrap()
                            .to_bits()
                    );
                }
                let mut samples = [0.0; 7];
                assert_eq!($white(handle, -2.0, samples.as_mut_ptr(), 7), 0);
                assert_eq!(
                    samples.map(<$t>::to_bits),
                    native
                        .white_noise::<$t>(-2.0, 7)
                        .unwrap()
                        .try_into()
                        .map(|v: [$t; 7]| v.map(<$t>::to_bits))
                        .unwrap()
                );
                assert_eq!($white(handle, 0.0, ptr::null_mut(), 0), 0);
                assert_eq!($white(handle, <$t>::NAN, samples.as_mut_ptr(), 7), 3);
                assert_eq!($white(handle, 1.0, ptr::null_mut(), 1), 1);
                assert_eq!($white(handle, 1.0, samples.as_mut_ptr(), usize::MAX), 2);
                assert_eq!($normal(handle, 0.0, -1.0, &mut output), 3);
                assert_eq!($with(handle, 0.0, 1.0, 99, &mut output), 3);
                assert_eq!($uniform(handle, ptr::null_mut()), 1);
                assert_eq!(ciglet_rs_random_clone(handle, &mut clone), 0);
                assert_eq!(ciglet_rs_random_release(&mut handle), 0);
                assert!(handle.is_null());
                for _ in 0..32 {
                    let mut draw = 0;
                    assert_eq!(ciglet_rs_random_draw(clone, &mut draw), 0);
                    assert_eq!(draw, native.draw());
                }
                assert_eq!(ciglet_rs_random_release(&mut clone), 0);
                assert_eq!(ciglet_rs_random_release(&mut clone), 0);
            }
        }
    };
}
streams!(
    float_owned_stream,
    f32,
    ciglet_rs_random_uniform_f32,
    ciglet_rs_random_normal_f32,
    ciglet_rs_random_normal_with_f32,
    ciglet_rs_random_white_noise_f32
);
streams!(
    double_owned_stream,
    f64,
    ciglet_rs_random_uniform_f64,
    ciglet_rs_random_normal_f64,
    ciglet_rs_random_normal_with_f64,
    ciglet_rs_random_white_noise_f64
);

#[test]
fn seed_owner_and_pointer_validation() {
    let seed = [0u8; 32];
    let mut owner = ptr::null_mut();
    let mut output = 71u32;
    // SAFETY: Local ownership slots; invalid pointers are rejected before access.
    unsafe {
        assert_eq!(ciglet_rs_random_create(seed.as_ptr(), 31, &mut owner), 2);
        assert_eq!(ciglet_rs_random_create(ptr::null(), 32, &mut owner), 1);
        assert_eq!(
            ciglet_rs_random_create_with_maximum(seed.as_ptr(), 32, 0, &mut owner),
            3
        );
        assert!(owner.is_null());
        assert_eq!(ciglet_rs_random_create(seed.as_ptr(), 32, &mut owner), 0);
        assert_eq!(ciglet_rs_random_maximum(owner, &mut output), 0);
        assert_eq!(output, i32::MAX as u32);
        assert_eq!(ciglet_rs_random_clone(owner, ptr::null_mut()), 1);
        assert_eq!(
            ciglet_rs_random_draw(owner, ptr::without_provenance_mut(1)),
            1
        );
        assert_eq!(ciglet_rs_random_release(ptr::null_mut()), 1);
        let mut expected = Generator::from_seed(seed);
        assert_eq!(ciglet_rs_random_draw(owner, &mut output), 0);
        assert_eq!(output, expected.draw());
        assert_eq!(ciglet_rs_random_release(&mut owner), 0);
    }
}

#[test]
fn explicit_domains_alias_and_ieee_endpoints() {
    let mut output = 71.0f64;
    // SAFETY: Local aligned initialized storage, alias written after borrowed draws end.
    unsafe {
        assert_eq!(
            ciglet_rs_random_normal_from_draws_f64(3.0, 0.0, 0, 8, 8, 0, &mut output),
            0
        );
        assert_eq!(output, 3.0);
        output = 71.0;
        assert_eq!(
            ciglet_rs_random_normal_from_draws_f64(0.0, 1.0, 0, 8, 8, 0, &mut output),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            ciglet_rs_random_normal_from_draws_f64(0.0, 1.0, 0, 8, 8, 1, &mut output),
            0
        );
        assert!(output.is_finite());
        assert_eq!(
            ciglet_rs_random_white_from_draws_f64(1.0, ptr::null(), 0, 0, ptr::null_mut()),
            3
        );
        assert_eq!(
            ciglet_rs_random_white_from_draws_f64(1.0, ptr::null(), 0, 8, ptr::null_mut()),
            0
        );
        let mut storage = [0u32, 8];
        assert_eq!(
            ciglet_rs_random_white_from_draws_f32(
                -2.0,
                storage.as_ptr(),
                2,
                8,
                storage.as_mut_ptr().cast()
            ),
            0
        );
        assert_eq!(storage, [2.0f32.to_bits(), (-2.0f32).to_bits()]);
        for (draw, maximum) in [(0, 0), (9, 8), (0, u32::MAX)] {
            output = 71.0;
            assert_eq!(
                ciglet_rs_random_uniform_from_draw_f64(draw, maximum, &mut output),
                3
            );
            assert_eq!(output, 71.0);
        }
        assert_eq!(
            ciglet_rs_random_uniform_from_draw_f64(0, 8, ptr::null_mut()),
            1
        );
    }
}

#[test]
fn concurrent_draws_preserve_one_owned_stream() {
    let seed = [91u8; 32];
    let mut handle = ptr::null_mut();
    // SAFETY: Seed and output slot are live independent initialized storage.
    assert_eq!(
        unsafe { ciglet_rs_random_create(seed.as_ptr(), 32, &mut handle) },
        0
    );
    let address = handle as usize;
    let mut actual = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(move || {
                    (0..64)
                        .map(|_| {
                            let mut output = 0;
                            // SAFETY: Owner is retained until all workers join; Mutex serializes
                            // its state. Each worker uses an independent exclusive output slot.
                            assert_eq!(
                                unsafe {
                                    ciglet_rs_random_draw(
                                        address as *const CigletRsRandom,
                                        &mut output,
                                    )
                                },
                                0
                            );
                            output
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let mut native = Generator::from_seed(seed);
    let mut expected: Vec<_> = (0..256).map(|_| native.draw()).collect();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
    // SAFETY: Joined all workers before exclusive release of the owner.
    assert_eq!(unsafe { ciglet_rs_random_release(&mut handle) }, 0);
}
