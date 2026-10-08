#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
struct Reader<'a> {
    bytes: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn integer(&mut self) -> usize {
        let (value, rest) = self.bytes.split_at(4);
        self.bytes = rest;
        u32::from_le_bytes(value.try_into().unwrap()) as usize
    }
    fn scalar(&mut self) -> f64 {
        let (value, rest) = self.bytes.split_at(self.width);
        self.bytes = rest;
        if self.width == 4 {
            f32::from_le_bytes(value.try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(value.try_into().unwrap())
        }
    }
}

#[test]
fn phase_f32_matches_original_c() {
    let bytes = include_bytes!("fixtures/vector-phase-float.bin");
    assert_eq!(&bytes[..4], b"VPH1");
    let mut reader = Reader {
        bytes: &bytes[4..],
        width: 4,
    };
    assert_eq!(reader.integer(), 4);
    assert_eq!(reader.integer(), 24);
    for case in 0..24 {
        let n = reader.integer();
        let real: Vec<f32> = (0..n).map(|_| reader.scalar() as f32).collect();
        let imaginary: Vec<f32> = (0..n).map(|_| reader.scalar() as f32).collect();
        let angles: Vec<f32> = (0..n).map(|_| reader.scalar() as f32).collect();
        let expected: Vec<Vec<f32>> = (0..12)
            .map(|_| (0..n).map(|_| reader.scalar() as f32).collect())
            .collect();
        let mut output = vec![71.0; n];
        // SAFETY: All buffers are initialized live allocations of the stated length.
        unsafe {
            assert_eq!(
                ciglet_rs_phase_unwrap_f32(angles.as_ptr(), n, output.as_mut_ptr(), n),
                0
            );
            for (a, b) in output.iter().zip(&expected[3]) {
                assert_eq!(a.to_bits(), b.to_bits(), "unwrap {case}");
            }
            for i in 0..n {
                assert_eq!(ciglet_rs_phase_wrap_f32(angles[i], &mut output[i]), 0);
                assert_eq!(output[i].to_bits(), expected[4][i].to_bits());
                assert_eq!(
                    ciglet_rs_phase_difference_f32(angles[i], angles[(i + 3) % n], &mut output[i]),
                    0
                );
                assert_eq!(output[i].to_bits(), expected[5][i].to_bits());
            }
            for (function, field) in [
                (
                    ciglet_rs_magnitude_f32
                        as unsafe extern "C" fn(
                            *const f32,
                            usize,
                            *const f32,
                            usize,
                            *mut f32,
                            usize,
                        ) -> u32,
                    6,
                ),
                (ciglet_rs_argument_f32, 7),
            ] {
                assert_eq!(
                    function(
                        real.as_ptr(),
                        n,
                        imaginary.as_ptr(),
                        n,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[field]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "Cartesian {case}/{field}");
                }
            }
            for (function, field) in [
                (
                    ciglet_rs_polar_real_f32
                        as unsafe extern "C" fn(
                            *const f32,
                            usize,
                            *const f32,
                            usize,
                            *mut f32,
                            usize,
                        ) -> u32,
                    8,
                ),
                (ciglet_rs_polar_imaginary_f32, 9),
            ] {
                assert_eq!(
                    function(
                        expected[6].as_ptr(),
                        n,
                        angles.as_ptr(),
                        n,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[field]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "polar {case}/{field}");
                }
            }
            for symmetry in 0..2 {
                assert_eq!(
                    ciglet_rs_complete_spectrum_f32(
                        real.as_ptr(),
                        n,
                        symmetry,
                        1,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[10 + symmetry as usize]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "completion {case}/{symmetry}");
                }
            }
        }
    }
}

#[test]
fn dct_f32_matches_original_c() {
    let bytes = include_bytes!("fixtures/c-dct-f32.bin");
    assert_eq!(&bytes[..4], b"DCT1");
    let mut reader = Reader {
        bytes: &bytes[4..],
        width: 4,
    };
    assert_eq!(reader.integer(), 4);
    assert_eq!(reader.integer(), 32);
    for _ in 0..32 {
        let n = reader.integer();
        let scenario = reader.integer();
        let signal: Vec<f32> = (0..n)
            .map(|i| match scenario {
                0 => {
                    if i == n / 3 {
                        0.75
                    } else {
                        0.0
                    }
                }
                1 => ((i % 17) as i32 - 8) as f32 * 0.125,
                2 => 2048.0,
                3 => {
                    if i % 2 == 1 {
                        -0.5
                    } else {
                        0.5
                    }
                }
                _ => panic!("invalid fixture"),
            })
            .collect();
        let scale =
            signal.iter().map(|&v| (v as f64).abs()).fold(1.0, f64::max) * (n as f64).sqrt();
        let mut output = signal.clone();
        // SAFETY: One live initialized buffer; in-place operation is supported.
        unsafe {
            assert_eq!(
                ciglet_rs_dct_f32(output.as_ptr(), n, output.as_mut_ptr(), n),
                0
            );
        }
        for actual in output {
            assert!(((actual as f64) - reader.scalar()).abs() / scale <= 2e-7);
        }
    }
}

#[test]
fn phase_f64_matches_original_c() {
    let bytes = include_bytes!("fixtures/vector-phase-double.bin");
    assert_eq!(&bytes[..4], b"VPH1");
    let mut reader = Reader {
        bytes: &bytes[4..],
        width: 8,
    };
    assert_eq!(reader.integer(), 8);
    assert_eq!(reader.integer(), 24);
    for case in 0..24 {
        let n = reader.integer();
        let real: Vec<f64> = (0..n).map(|_| reader.scalar()).collect();
        let imaginary: Vec<f64> = (0..n).map(|_| reader.scalar()).collect();
        let angles: Vec<f64> = (0..n).map(|_| reader.scalar()).collect();
        let expected: Vec<Vec<f64>> = (0..12)
            .map(|_| (0..n).map(|_| reader.scalar()).collect())
            .collect();
        let mut output = vec![71.0; n];
        // SAFETY: All buffers are initialized live allocations of the stated length.
        unsafe {
            assert_eq!(
                ciglet_rs_phase_unwrap_f64(angles.as_ptr(), n, output.as_mut_ptr(), n),
                0
            );
            for (a, b) in output.iter().zip(&expected[3]) {
                assert_eq!(a.to_bits(), b.to_bits(), "unwrap {case}");
            }
            for i in 0..n {
                assert_eq!(ciglet_rs_phase_wrap_f64(angles[i], &mut output[i]), 0);
                assert_eq!(output[i].to_bits(), expected[4][i].to_bits());
                assert_eq!(
                    ciglet_rs_phase_difference_f64(angles[i], angles[(i + 3) % n], &mut output[i]),
                    0
                );
                assert_eq!(output[i].to_bits(), expected[5][i].to_bits());
            }
            for (function, field) in [
                (
                    ciglet_rs_magnitude_f64
                        as unsafe extern "C" fn(
                            *const f64,
                            usize,
                            *const f64,
                            usize,
                            *mut f64,
                            usize,
                        ) -> u32,
                    6,
                ),
                (ciglet_rs_argument_f64, 7),
            ] {
                assert_eq!(
                    function(
                        real.as_ptr(),
                        n,
                        imaginary.as_ptr(),
                        n,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[field]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "Cartesian {case}/{field}");
                }
            }
            for (function, field) in [
                (
                    ciglet_rs_polar_real_f64
                        as unsafe extern "C" fn(
                            *const f64,
                            usize,
                            *const f64,
                            usize,
                            *mut f64,
                            usize,
                        ) -> u32,
                    8,
                ),
                (ciglet_rs_polar_imaginary_f64, 9),
            ] {
                assert_eq!(
                    function(
                        expected[6].as_ptr(),
                        n,
                        angles.as_ptr(),
                        n,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[field]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "polar {case}/{field}");
                }
            }
            for symmetry in 0..2 {
                assert_eq!(
                    ciglet_rs_complete_spectrum_f64(
                        real.as_ptr(),
                        n,
                        symmetry,
                        1,
                        output.as_mut_ptr(),
                        n
                    ),
                    0
                );
                for (a, b) in output.iter().zip(&expected[10 + symmetry as usize]) {
                    assert_eq!(a.to_bits(), b.to_bits(), "completion {case}/{symmetry}");
                }
            }
        }
    }
}

#[test]
fn dct_f64_matches_original_c() {
    let bytes = include_bytes!("fixtures/c-dct-f64.bin");
    assert_eq!(&bytes[..4], b"DCT1");
    let mut reader = Reader {
        bytes: &bytes[4..],
        width: 8,
    };
    assert_eq!(reader.integer(), 8);
    assert_eq!(reader.integer(), 32);
    for _ in 0..32 {
        let n = reader.integer();
        let scenario = reader.integer();
        let signal: Vec<f64> = (0..n)
            .map(|i| match scenario {
                0 => {
                    if i == n / 3 {
                        0.75
                    } else {
                        0.0
                    }
                }
                1 => ((i % 17) as i32 - 8) as f64 * 0.125,
                2 => 2048.0,
                3 => {
                    if i % 2 == 1 {
                        -0.5
                    } else {
                        0.5
                    }
                }
                _ => panic!("invalid fixture"),
            })
            .collect();
        let scale = signal.iter().map(|&v| v.abs()).fold(1.0, f64::max) * (n as f64).sqrt();
        let mut output = signal.clone();
        // SAFETY: One live initialized buffer; in-place operation is supported.
        unsafe {
            assert_eq!(
                ciglet_rs_dct_f64(output.as_ptr(), n, output.as_mut_ptr(), n),
                0
            );
        }
        for actual in output {
            assert!((actual - reader.scalar()).abs() / scale <= 1e-15);
        }
    }
}

#[test]
fn phase_rejection_is_atomic_and_empty_inputs_are_supported() {
    let mut output = [71.0_f64; 6];
    let real = [3.0_f64, f64::NAN];
    let imaginary = [4.0_f64, 0.0];
    // SAFETY: Live buffers of the stated sizes; null inputs only use zero lengths.
    unsafe {
        assert_eq!(
            ciglet_rs_magnitude_f64(
                real.as_ptr(),
                2,
                imaginary.as_ptr(),
                2,
                output.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(output, [71.0; 6]);
        assert_eq!(
            ciglet_rs_argument_f64(
                real.as_ptr(),
                2,
                imaginary.as_ptr(),
                1,
                output.as_mut_ptr(),
                2
            ),
            2
        );
        assert_eq!(
            ciglet_rs_phase_wrap_f64(f64::INFINITY, output.as_mut_ptr()),
            3
        );
        assert_eq!(
            ciglet_rs_phase_difference_f64(-1e100, 1e100, output.as_mut_ptr()),
            3
        );
        assert_eq!(
            ciglet_rs_dct_f64(std::ptr::null(), 0, std::ptr::null_mut(), 0),
            3
        );
        assert_eq!(
            ciglet_rs_phase_unwrap_f64(std::ptr::null(), 0, std::ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            ciglet_rs_magnitude_f64(
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            ciglet_rs_complete_spectrum_f64(real.as_ptr(), 2, 2, 0, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(
            ciglet_rs_complete_spectrum_f64(real.as_ptr(), 2, 0, 2, output.as_mut_ptr(), 2),
            3
        );
        assert_eq!(output, [71.0; 6]);
    }
}

#[test]
fn completion_policies_and_cartesian_overlap_are_preserved() {
    let mut values = [1.0_f32, 2.0, 3.0, 4.0, 99.0, 99.0];
    let imaginary = [4.0_f32, 0.0, 0.0, 0.0, 0.0, 0.0];
    // SAFETY: Live initialized ranges; outputs may overlap inputs after computation.
    unsafe {
        assert_eq!(
            ciglet_rs_complete_spectrum_f32(values.as_ptr(), 6, 1, 0, values.as_mut_ptr(), 6),
            0
        );
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0, -3.0, -2.0]);
        assert_eq!(
            ciglet_rs_complete_spectrum_f32(values.as_ptr(), 6, 1, 1, values.as_mut_ptr(), 6),
            0
        );
        assert_eq!(values, [1.0, 2.0, 3.0, 3.0, -3.0, -2.0]);
        values[0] = 3.0;
        assert_eq!(
            ciglet_rs_magnitude_f32(
                values.as_ptr(),
                6,
                imaginary.as_ptr(),
                6,
                values.as_mut_ptr(),
                6
            ),
            0
        );
        assert_eq!(values[0], 5.0);
    }
}
