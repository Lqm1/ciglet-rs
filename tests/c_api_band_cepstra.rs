#![cfg(feature = "c-api")]
use ciglet_rs::c_api::*;
use rustfft::FftNum;

type Single<T> = unsafe extern "C" fn(*const T, usize, usize, u32, *mut T, usize) -> u32;
type Matrix<T> = unsafe extern "C" fn(*const T, usize, usize, usize, u32, *mut T, usize) -> u32;
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self) -> f64 {
        if self.1 == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        }
    }
}
fn close<T: Copy + Into<f64>>(actual: &[T], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        let actual = actual.into();
        let difference = (actual - expected).abs() / expected.abs().max(1.0);
        assert!(
            difference < tolerance,
            "{actual} vs {expected}: {difference}"
        );
    }
}
fn compare<T: FftNum + Into<f64>>(
    bytes: &[u8],
    width: usize,
    selected: Single<T>,
    legacy: Single<T>,
    matrix: Matrix<T>,
) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), b"CEP1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 48);
    let tolerance = if width == 4 { 1e-7 } else { 1e-15 };
    for _ in 0..48 {
        let bands = reader.integer();
        let order = reader.integer();
        let include_dc = reader.integer() as u32;
        let count = order.min(bands - 1) + include_dc as usize;
        let input: Vec<T> = (0..3 * bands)
            .map(|_| T::from_f64(reader.scalar()).unwrap())
            .collect();
        let mut expected_matrix = Vec::new();
        let mut original_full = Vec::new();
        for frame in input.chunks_exact(bands) {
            let expected: Vec<f64> = (0..bands).map(|_| reader.scalar()).collect();
            let mut full = vec![T::zero(); bands];
            let mut prefix = vec![T::zero(); count];
            // SAFETY: Initialized readable frame and independent writable outputs.
            unsafe {
                assert_eq!(
                    legacy(
                        frame.as_ptr(),
                        bands,
                        order,
                        include_dc,
                        full.as_mut_ptr(),
                        bands
                    ),
                    0
                );
                assert_eq!(
                    selected(
                        frame.as_ptr(),
                        bands,
                        order,
                        include_dc,
                        prefix.as_mut_ptr(),
                        count
                    ),
                    0
                );
            }
            close(&full, &expected, tolerance);
            close(&prefix, &expected[..count], tolerance);
            expected_matrix.extend_from_slice(&expected[..count]);
            original_full.extend(expected);
        }
        // The original fixture repeats the complete legacy matrix after the
        // frame records; consume and verify every duplicate value.
        for expected in original_full {
            assert_eq!(reader.scalar(), expected);
        }
        let mut actual = vec![T::zero(); expected_matrix.len()];
        // SAFETY: Independent initialized input and correctly sized writable output.
        unsafe {
            assert_eq!(
                matrix(
                    input.as_ptr(),
                    input.len(),
                    bands,
                    order,
                    include_dc,
                    actual.as_mut_ptr(),
                    actual.len()
                ),
                0
            );
        }
        close(&actual, &expected_matrix, tolerance);
    }
    assert!(reader.0.is_empty());
}

#[test]
fn float_cepstra_match_all_original_records() {
    compare(
        include_bytes!("fixtures/c-cepstrum-f32.bin"),
        4,
        ciglet_rs_band_to_cepstrum_f32,
        ciglet_rs_band_to_cepstrum_legacy_f32,
        ciglet_rs_band_spectrogram_to_cepstra_f32,
    );
}
#[test]
fn double_cepstra_match_all_original_records() {
    compare(
        include_bytes!("fixtures/c-cepstrum-f64.bin"),
        8,
        ciglet_rs_band_to_cepstrum_f64,
        ciglet_rs_band_to_cepstrum_legacy_f64,
        ciglet_rs_band_spectrogram_to_cepstra_f64,
    );
}

#[test]
fn overlap_clamped_order_and_zero_sized_results() {
    let original = [1.0_f64, 2.0, 3.0, 4.0];
    let mut expected = [0.0; 4];
    let mut overlap = original;
    // SAFETY: In-place buffers are supported; output commit follows input borrowing.
    unsafe {
        assert_eq!(
            ciglet_rs_band_to_cepstrum_legacy_f64(
                original.as_ptr(),
                4,
                usize::MAX,
                1,
                expected.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f64(
                overlap.as_ptr(),
                4,
                usize::MAX,
                1,
                overlap.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(overlap, expected);
        overlap = original;
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f64(overlap.as_ptr(), 4, 1, 0, overlap.as_mut_ptr(), 1),
            0
        );
        assert_eq!(overlap[0], expected[0]);
        assert_eq!(overlap[1..], original[1..]);
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f64(original.as_ptr(), 4, 0, 0, std::ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            ciglet_rs_band_spectrogram_to_cepstra_f32(
                std::ptr::null(),
                0,
                2,
                1,
                1,
                std::ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            ciglet_rs_band_spectrogram_to_cepstra_f32(
                std::ptr::null(),
                0,
                0,
                1,
                1,
                std::ptr::null_mut(),
                0
            ),
            3
        );
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f32(std::ptr::null(), 0, 0, 0, std::ptr::null_mut(), 0),
            3
        );
    }
}

#[test]
fn invalid_options_shapes_and_later_frames_retain_output() {
    let valid = [1.0_f32, 2.0, 3.0, 4.0];
    let invalid = [1.0_f32, 2.0, f32::NAN, 4.0];
    let mut output = [71.0_f32; 4];
    // SAFETY: Initialized inputs and output, including buffers for rejected calls.
    unsafe {
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f32(valid.as_ptr(), 4, 1, 2, output.as_mut_ptr(), 1),
            3
        );
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f32(valid.as_ptr(), 4, 1, 1, output.as_mut_ptr(), 4),
            2
        );
        assert_eq!(
            ciglet_rs_band_to_cepstrum_legacy_f32(valid.as_ptr(), 4, 1, 1, output.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            ciglet_rs_band_spectrogram_to_cepstra_f32(
                valid.as_ptr(),
                4,
                3,
                1,
                0,
                output.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!(
            ciglet_rs_band_spectrogram_to_cepstra_f32(
                invalid.as_ptr(),
                4,
                2,
                1,
                1,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(
            ciglet_rs_band_to_cepstrum_f32(invalid.as_ptr(), 4, 0, 0, std::ptr::null_mut(), 0),
            3
        );
        assert_eq!(
            ciglet_rs_band_spectrogram_to_cepstra_f32(
                invalid.as_ptr(),
                4,
                2,
                0,
                0,
                std::ptr::null_mut(),
                0
            ),
            3
        );
        assert_eq!(output, [71.0; 4]);
        let invalid_double = [f64::INFINITY];
        let mut double_output = [71.0];
        assert_eq!(
            ciglet_rs_band_to_cepstrum_legacy_f64(
                invalid_double.as_ptr(),
                1,
                0,
                1,
                double_output.as_mut_ptr(),
                1
            ),
            3
        );
        assert_eq!(double_output, [71.0]);
    }
}
