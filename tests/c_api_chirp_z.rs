#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
struct Reader<'a> {
    bytes: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (value, rest) = self.bytes.split_at(n);
        self.bytes = rest;
        value
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self) -> f64 {
        if self.width == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        }
    }
}

#[test]
fn czt_f32_one_shot_and_plans_match_all_original_c_cases() {
    let mut reader = Reader {
        bytes: include_bytes!("fixtures/c-czt-f32.bin"),
        width: 4,
    };
    assert_eq!(reader.take(4), b"CZT1");
    assert_eq!(reader.integer(), 4);
    assert_eq!(reader.integer(), 1200);
    for case in 0..1200 {
        let n = reader.integer();
        let scenario = reader.integer();
        let mask = reader.integer();
        let inverse = reader.integer() as u32;
        let step = reader.scalar() as f32;
        let (real, imaginary): (Vec<f32>, Vec<f32>) = (0..n)
            .map(|i| {
                let values = match scenario {
                    0 => (
                        if i == n / 3 { 0.75 } else { 0.0 },
                        if i == n / 4 { -0.25 } else { 0.0 },
                    ),
                    1 => (
                        ((i % 17) as i32 - 8) as f64 * 0.125,
                        ((i % 11) as i32 - 5) as f64 * 0.0625,
                    ),
                    2 => (
                        2048.0 + (i % 7) as f64 * 0.125,
                        -1024.0 + (i % 5) as f64 * 0.0625,
                    ),
                    _ => panic!("invalid fixture scenario"),
                };
                (values.0 as f32, values.1 as f32)
            })
            .unzip();
        let real_pointer = if mask & 1 == 0 {
            std::ptr::null()
        } else {
            real.as_ptr()
        };
        let imaginary_pointer = if mask & 2 == 0 {
            std::ptr::null()
        } else {
            imaginary.as_ptr()
        };
        let scale = real
            .iter()
            .filter(|_| mask & 1 != 0)
            .chain(imaginary.iter().filter(|_| mask & 2 != 0))
            .map(|&v| (v as f64).abs())
            .fold(1.0, f64::max)
            * if inverse == 0 { n as f64 } else { 1.0 };
        let expected: Vec<_> = (0..n).map(|_| (reader.scalar(), reader.scalar())).collect();
        let mut re = vec![71.0; n];
        let mut im = vec![71.0; n];
        let mut plan = std::ptr::null_mut();
        // SAFETY: Live buffers and a locally owned correctly typed handle, released
        // after all transforms. Null components represent omitted inputs.
        unsafe {
            assert_eq!(
                ciglet_rs_czt_f32(
                    n,
                    step,
                    inverse,
                    real_pointer,
                    imaginary_pointer,
                    re.as_mut_ptr(),
                    im.as_mut_ptr()
                ),
                0
            );
            for (i, &(a, b)) in expected.iter().enumerate() {
                assert!(
                    ((re[i] as f64) - a).abs() / scale <= 5e-7,
                    "one-shot real {case}/{i}"
                );
                assert!(
                    ((im[i] as f64) - b).abs() / scale <= 5e-7,
                    "one-shot imaginary {case}/{i}"
                );
            }
            assert_eq!(ciglet_rs_czt_plan_new_f32(n, step, inverse, &mut plan), 0);
            for _ in 0..2 {
                assert_eq!(
                    ciglet_rs_czt_plan_transform_f32(
                        plan,
                        real_pointer,
                        imaginary_pointer,
                        n,
                        re.as_mut_ptr(),
                        im.as_mut_ptr()
                    ),
                    0
                );
                for (i, &(a, b)) in expected.iter().enumerate() {
                    assert!(
                        ((re[i] as f64) - a).abs() / scale <= 5e-7,
                        "plan real {case}/{i}"
                    );
                    assert!(
                        ((im[i] as f64) - b).abs() / scale <= 5e-7,
                        "plan imaginary {case}/{i}"
                    );
                }
            }
            assert_eq!(ciglet_rs_czt_plan_release_f32(&mut plan), 0);
            assert!(plan.is_null());
        }
    }
    assert!(reader.bytes.is_empty());
}

#[test]
fn czt_f64_one_shot_and_plans_match_all_original_c_cases() {
    let mut reader = Reader {
        bytes: include_bytes!("fixtures/c-czt-f64.bin"),
        width: 8,
    };
    assert_eq!(reader.take(4), b"CZT1");
    assert_eq!(reader.integer(), 8);
    assert_eq!(reader.integer(), 1200);
    for case in 0..1200 {
        let n = reader.integer();
        let scenario = reader.integer();
        let mask = reader.integer();
        let inverse = reader.integer() as u32;
        let step = reader.scalar();
        let (real, imaginary): (Vec<f64>, Vec<f64>) = (0..n)
            .map(|i| match scenario {
                0 => (
                    if i == n / 3 { 0.75 } else { 0.0 },
                    if i == n / 4 { -0.25 } else { 0.0 },
                ),
                1 => (
                    ((i % 17) as i32 - 8) as f64 * 0.125,
                    ((i % 11) as i32 - 5) as f64 * 0.0625,
                ),
                2 => (
                    2048.0 + (i % 7) as f64 * 0.125,
                    -1024.0 + (i % 5) as f64 * 0.0625,
                ),
                _ => panic!("invalid fixture scenario"),
            })
            .unzip();
        let real_pointer = if mask & 1 == 0 {
            std::ptr::null()
        } else {
            real.as_ptr()
        };
        let imaginary_pointer = if mask & 2 == 0 {
            std::ptr::null()
        } else {
            imaginary.as_ptr()
        };
        let scale = real
            .iter()
            .filter(|_| mask & 1 != 0)
            .chain(imaginary.iter().filter(|_| mask & 2 != 0))
            .map(|&v| v.abs())
            .fold(1.0, f64::max)
            * if inverse == 0 { n as f64 } else { 1.0 };
        let expected: Vec<_> = (0..n).map(|_| (reader.scalar(), reader.scalar())).collect();
        let mut re = vec![71.0; n];
        let mut im = vec![71.0; n];
        let mut plan = std::ptr::null_mut();
        // SAFETY: Live buffers and a locally owned correctly typed handle, released
        // after all transforms. Null components represent omitted inputs.
        unsafe {
            assert_eq!(
                ciglet_rs_czt_f64(
                    n,
                    step,
                    inverse,
                    real_pointer,
                    imaginary_pointer,
                    re.as_mut_ptr(),
                    im.as_mut_ptr()
                ),
                0
            );
            for (i, &(a, b)) in expected.iter().enumerate() {
                assert!(
                    (re[i] - a).abs() / scale <= 2e-15,
                    "one-shot real {case}/{i}"
                );
                assert!(
                    (im[i] - b).abs() / scale <= 2e-15,
                    "one-shot imaginary {case}/{i}"
                );
            }
            assert_eq!(ciglet_rs_czt_plan_new_f64(n, step, inverse, &mut plan), 0);
            for _ in 0..2 {
                assert_eq!(
                    ciglet_rs_czt_plan_transform_f64(
                        plan,
                        real_pointer,
                        imaginary_pointer,
                        n,
                        re.as_mut_ptr(),
                        im.as_mut_ptr()
                    ),
                    0
                );
                for (i, &(a, b)) in expected.iter().enumerate() {
                    assert!((re[i] - a).abs() / scale <= 2e-15, "plan real {case}/{i}");
                    assert!(
                        (im[i] - b).abs() / scale <= 2e-15,
                        "plan imaginary {case}/{i}"
                    );
                }
            }
            assert_eq!(ciglet_rs_czt_plan_release_f64(&mut plan), 0);
            assert!(plan.is_null());
        }
    }
    assert!(reader.bytes.is_empty());
}

#[test]
fn czt_errors_preserve_buffers_and_constructor_ownership() {
    let signal = [3.0_f32, 0.0, 0.0];
    let mut output = [71.0_f32; 3];
    let mut plan = std::ptr::null_mut();
    // SAFETY: Live fixed-size buffers and one handle, with documented overlap rules.
    unsafe {
        for (length, step, direction, expected) in [
            (3, 0.0, 2, 3),
            (3, f32::NAN, 0, 3),
            (3, f32::MAX, 0, 3),
            (0, 0.0, 0, 3),
            (usize::MAX, 0.0, 0, 2),
        ] {
            assert_eq!(
                ciglet_rs_czt_f32(
                    length,
                    step,
                    direction,
                    signal.as_ptr(),
                    std::ptr::null(),
                    output.as_mut_ptr(),
                    std::ptr::null_mut()
                ),
                expected
            );
            assert_eq!(output, [71.0; 3]);
        }
        assert_eq!(
            ciglet_rs_czt_f32(
                3,
                0.0,
                0,
                signal.as_ptr(),
                std::ptr::null(),
                output.as_mut_ptr(),
                output.as_mut_ptr()
            ),
            2
        );
        assert_eq!(output, [71.0; 3]);
        assert_eq!(ciglet_rs_czt_plan_new_f32(3, 0.0, 0, &mut plan), 0);
        let original = plan;
        assert_eq!(
            ciglet_rs_czt_plan_new_f32(3, f32::INFINITY, 0, &mut plan),
            3
        );
        assert_eq!(plan, original);
        assert_eq!(ciglet_rs_czt_plan_new_f32(3, 0.0, 2, &mut plan), 3);
        assert_eq!(plan, original);
        assert_eq!(
            ciglet_rs_czt_plan_transform_f32(
                plan,
                signal.as_ptr(),
                std::ptr::null(),
                2,
                output.as_mut_ptr(),
                std::ptr::null_mut()
            ),
            2
        );
        assert_eq!(output, [71.0; 3]);
        let mut n = 0;
        assert_eq!(ciglet_rs_czt_plan_length_f32(plan, &mut n), 0);
        assert_eq!(n, 3);
        assert_eq!(ciglet_rs_czt_plan_release_f32(&mut plan), 0);
        assert!(plan.is_null());
        assert_eq!(ciglet_rs_czt_plan_release_f32(&mut plan), 0);
    }
}
#[test]
fn orthogonal_grid_roundtrip_allows_in_place_planes() {
    let mut real = [1.0_f64, 2.0, 3.0, 4.0, 5.0];
    let mut imaginary = [0.0_f64; 5];
    let step = std::f64::consts::TAU / 5.0;
    // SAFETY: Initialized separate output planes; either may overlap its input.
    unsafe {
        assert_eq!(
            ciglet_rs_czt_f64(
                5,
                step,
                0,
                real.as_ptr(),
                std::ptr::null(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            0
        );
        assert_eq!(
            ciglet_rs_czt_f64(
                5,
                step,
                1,
                real.as_ptr(),
                imaginary.as_ptr(),
                real.as_mut_ptr(),
                imaginary.as_mut_ptr()
            ),
            0
        );
    }
    for (i, (&re, &im)) in real.iter().zip(&imaginary).enumerate() {
        assert!((re - (i + 1) as f64).abs() < 1e-12 && im.abs() < 1e-12);
    }
}
#[test]
fn shared_czt_plan_serializes_scratch_for_concurrent_callers() {
    let mut plan = std::ptr::null_mut();
    // SAFETY: One correctly typed plan stays live until every scoped caller joins.
    unsafe {
        assert_eq!(ciglet_rs_czt_plan_new_f64(3, 0.0, 0, &mut plan), 0);
        let shared = &*plan;
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(move || {
                    let signal = [3.0_f64, 0.0, 0.0];
                    let mut output = [71.0; 3];
                    assert_eq!(
                        ciglet_rs_czt_plan_transform_f64(
                            shared,
                            signal.as_ptr(),
                            std::ptr::null(),
                            3,
                            output.as_mut_ptr(),
                            std::ptr::null_mut()
                        ),
                        0
                    );
                    assert!(output.iter().all(|v| (v - 3.0).abs() < 1e-12));
                });
            }
        });
        assert_eq!(ciglet_rs_czt_plan_release_f64(&mut plan), 0);
        assert!(plan.is_null());
    }
}
