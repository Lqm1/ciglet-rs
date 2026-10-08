#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
struct Reader<'a> {
    bytes: &'a [u8],
    width: usize,
}
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.bytes.split_at(n);
        self.bytes = b;
        a
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
fn frequency_f32_matches_original_c() {
    let mut r = Reader {
        bytes: include_bytes!("fixtures/c-frequency-f32.bin"),
        width: 4,
    };
    assert_eq!(r.take(4), b"FRQ1");
    assert_eq!(r.integer(), 4);
    assert_eq!(r.integer(), 12);
    for _ in 0..12 {
        let input = r.scalar() as f32;
        let mut mel = 71.0;
        let mut frequency = 71.0;
        let mut bark = 71.0;
        let mut inverse_bark = 71.0;
        let mut loudness = 71.0;
        // SAFETY: Every scalar output is a live aligned local with exclusive access.
        unsafe {
            assert_eq!(ciglet_rs_frequency_to_mel_f32(input, &mut mel), 0);
            assert_eq!(ciglet_rs_mel_to_frequency_f32(mel, &mut frequency), 0);
            assert_eq!(ciglet_rs_frequency_to_bark_f32(input, &mut bark), 0);
            assert_eq!(ciglet_rs_bark_to_frequency_f32(bark, &mut inverse_bark), 0);
            assert_eq!(ciglet_rs_equal_loudness_f32(input, &mut loudness), 0);
        }
        for value in [mel, frequency, bark, inverse_bark, loudness] {
            let expected = r.scalar();
            assert!(((value as f64) - expected).abs() / expected.abs().max(1.0) < 1e-7);
        }
    }
    assert_eq!(r.integer(), 5);
    for _ in 0..5 {
        let intervals = r.integer();
        let mut output = vec![71.0; intervals + 1];
        // SAFETY: Output has the explicitly provided matching interval count plus one.
        unsafe {
            assert_eq!(
                ciglet_rs_mel_space_f32(50.0, 8000.0, intervals, output.as_mut_ptr(), output.len()),
                0
            );
        }
        for value in output {
            assert_eq!((value as f64).to_bits(), r.scalar().to_bits());
        }
    }
    // The remaining sections cover approximation functions outside this adapter.
    assert_eq!(r.integer(), 12);
}
#[test]
fn legacy_distance_f32_matches_original_c_bits() {
    let mut r = Reader {
        bytes: include_bytes!("fixtures/distance-float.bin"),
        width: 4,
    };
    assert_eq!(r.take(4), b"ISD1");
    assert_eq!(r.integer(), 4);
    assert_eq!(r.integer(), 53);
    for _ in 0..53 {
        let n = r.integer();
        let source: Vec<f32> = (0..n).map(|_| r.scalar() as f32).collect();
        let reference: Vec<f32> = (0..n).map(|_| r.scalar() as f32).collect();
        let expected = r.scalar();
        let mut output = 71.0;
        // SAFETY: Initialized matching inputs and an independent writable scalar.
        unsafe {
            assert_eq!(
                ciglet_rs_log_itakura_saito_with_f32(
                    source.as_ptr(),
                    n,
                    reference.as_ptr(),
                    n,
                    1,
                    &mut output
                ),
                0
            );
        }
        assert_eq!((output as f64).to_bits(), expected.to_bits());
    }
    assert!(r.bytes.is_empty());
}

#[test]
fn frequency_f64_matches_original_c() {
    let mut r = Reader {
        bytes: include_bytes!("fixtures/c-frequency-f64.bin"),
        width: 8,
    };
    assert_eq!(r.take(4), b"FRQ1");
    assert_eq!(r.integer(), 8);
    assert_eq!(r.integer(), 12);
    for _ in 0..12 {
        let input = r.scalar();
        let mut mel = 71.0;
        let mut frequency = 71.0;
        let mut bark = 71.0;
        let mut inverse_bark = 71.0;
        let mut loudness = 71.0;
        // SAFETY: Every scalar output is a live aligned local with exclusive access.
        unsafe {
            assert_eq!(ciglet_rs_frequency_to_mel_f64(input, &mut mel), 0);
            assert_eq!(ciglet_rs_mel_to_frequency_f64(mel, &mut frequency), 0);
            assert_eq!(ciglet_rs_frequency_to_bark_f64(input, &mut bark), 0);
            assert_eq!(ciglet_rs_bark_to_frequency_f64(bark, &mut inverse_bark), 0);
            assert_eq!(ciglet_rs_equal_loudness_f64(input, &mut loudness), 0);
        }
        for value in [mel, frequency, bark, inverse_bark, loudness] {
            let expected = r.scalar();
            assert!((value - expected).abs() / expected.abs().max(1.0) < 1e-15);
        }
    }
    assert_eq!(r.integer(), 5);
    for _ in 0..5 {
        let intervals = r.integer();
        let mut output = vec![71.0; intervals + 1];
        // SAFETY: Output has the explicitly provided matching interval count plus one.
        unsafe {
            assert_eq!(
                ciglet_rs_mel_space_f64(50.0, 8000.0, intervals, output.as_mut_ptr(), output.len()),
                0
            );
        }
        for value in output {
            assert_eq!(value.to_bits(), r.scalar().to_bits());
        }
    }
    // The remaining sections cover approximation functions outside this adapter.
    assert_eq!(r.integer(), 12);
}
#[test]
fn legacy_distance_f64_matches_original_c_bits() {
    let mut r = Reader {
        bytes: include_bytes!("fixtures/distance-double.bin"),
        width: 8,
    };
    assert_eq!(r.take(4), b"ISD1");
    assert_eq!(r.integer(), 8);
    assert_eq!(r.integer(), 53);
    for _ in 0..53 {
        let n = r.integer();
        let source: Vec<f64> = (0..n).map(|_| r.scalar()).collect();
        let reference: Vec<f64> = (0..n).map(|_| r.scalar()).collect();
        let expected = r.scalar();
        let mut output = 71.0;
        // SAFETY: Initialized matching inputs and an independent writable scalar.
        unsafe {
            assert_eq!(
                ciglet_rs_log_itakura_saito_with_f64(
                    source.as_ptr(),
                    n,
                    reference.as_ptr(),
                    n,
                    1,
                    &mut output
                ),
                0
            );
        }
        assert_eq!(output.to_bits(), expected.to_bits());
    }
    assert!(r.bytes.is_empty());
}

#[test]
fn stable_distances_keep_analytic_results_zero_and_extreme_semantics() {
    let mut source = [2.0_f64; 3];
    let reference = [1.0_f64; 3];
    let expected = 1.0 - 2.0_f64.ln();
    let mut result = 71.0;
    // SAFETY: Live fixed-size buffers; scalar output may overlap input.
    unsafe {
        assert_eq!(
            ciglet_rs_itakura_saito_f64(
                source.as_ptr(),
                3,
                reference.as_ptr(),
                3,
                source.as_mut_ptr()
            ),
            0
        );
        assert!((source[0] - expected).abs() < 1e-15);
        assert_eq!(&source[1..], &[2.0, 2.0]);
        source[0] = 2.0;
        assert_eq!(
            ciglet_rs_log_itakura_saito_f64(source.as_ptr(), 3, reference.as_ptr(), 3, &mut result),
            0
        );
        assert!((result - expected.ln()).abs() < 1e-15);
        let mut explicit = 71.0;
        assert_eq!(
            ciglet_rs_log_itakura_saito_with_f64(
                source.as_ptr(),
                3,
                reference.as_ptr(),
                3,
                0,
                &mut explicit
            ),
            0
        );
        assert_eq!(explicit.to_bits(), result.to_bits());
        assert_eq!(
            ciglet_rs_itakura_saito_f64(reference.as_ptr(), 3, reference.as_ptr(), 3, &mut result),
            0
        );
        assert_eq!(result, 0.0);
        assert_eq!(
            ciglet_rs_log_itakura_saito_f64(
                reference.as_ptr(),
                3,
                reference.as_ptr(),
                3,
                &mut result
            ),
            0
        );
        assert_eq!(result, f64::NEG_INFINITY);
        let large = [1e300];
        let small = [1e-300];
        assert_eq!(
            ciglet_rs_log_itakura_saito_f64(large.as_ptr(), 1, small.as_ptr(), 1, &mut result),
            0
        );
        assert!((result - (1e300_f64.ln() - 1e-300_f64.ln())).abs() < 1e-12);
        let zero = [0.0];
        assert_eq!(
            ciglet_rs_itakura_saito_f64(zero.as_ptr(), 1, reference.as_ptr(), 1, &mut result),
            0
        );
        assert_eq!(result, f64::INFINITY);
    }
}
#[test]
fn rejected_frequency_and_distance_arguments_leave_outputs_unchanged() {
    let mut result = 71.0_f32;
    let mut grid = [71.0_f32; 3];
    let source = [2.0_f32, f32::NAN];
    let reference = [1.0_f32; 2];
    let zero = [0.0_f32];
    // SAFETY: Live buffers; empty null inputs are never dereferenced.
    unsafe {
        assert_eq!(ciglet_rs_frequency_to_mel_f32(-700.0, &mut result), 3);
        assert_eq!(ciglet_rs_bark_to_frequency_f32(f32::MAX, &mut result), 3);
        assert_eq!(
            ciglet_rs_mel_to_frequency_f32(f32::INFINITY, &mut result),
            3
        );
        assert_eq!(ciglet_rs_equal_loudness_f32(f32::MAX, &mut result), 3);
        assert_eq!(
            ciglet_rs_mel_space_f32(8000.0, 50.0, 2, grid.as_mut_ptr(), 3),
            3
        );
        assert_eq!(
            ciglet_rs_mel_space_f32(50.0, 8000.0, 0, grid.as_mut_ptr(), 1),
            3
        );
        assert_eq!(
            ciglet_rs_mel_space_f32(50.0, 8000.0, 2, grid.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            ciglet_rs_mel_space_f32(50.0, 8000.0, usize::MAX, grid.as_mut_ptr(), 3),
            2
        );
        assert_eq!(grid, [71.0; 3]);
        assert_eq!(
            ciglet_rs_itakura_saito_f32(source.as_ptr(), 2, reference.as_ptr(), 2, &mut result),
            3
        );
        assert_eq!(
            ciglet_rs_log_itakura_saito_with_f32(
                reference.as_ptr(),
                2,
                reference.as_ptr(),
                2,
                2,
                &mut result
            ),
            3
        );
        assert_eq!(
            ciglet_rs_log_itakura_saito_f32(zero.as_ptr(), 1, zero.as_ptr(), 1, &mut result),
            3
        );
        assert_eq!(
            ciglet_rs_itakura_saito_f32(reference.as_ptr(), 2, reference.as_ptr(), 1, &mut result),
            2
        );
        assert_eq!(
            ciglet_rs_itakura_saito_f32(std::ptr::null(), 0, std::ptr::null(), 0, &mut result),
            3
        );
        assert_eq!(result, 71.0);
    }
}
