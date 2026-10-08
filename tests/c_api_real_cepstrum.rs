#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, statistics::Scalar};
type Transform<T, H> = unsafe extern "C" fn(*const H, *const T, usize, u32, *mut T, usize) -> u32;
type Minimum<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, u32, u32, *mut T, usize) -> u32;
type ToCepstra<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, usize, u32, *mut T, usize) -> u32;
type ToSpectrum<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, usize, usize, u32, *mut T, usize) -> u32;
struct Api<T, H> {
    new: unsafe extern "C" fn(usize, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    shape: unsafe extern "C" fn(*const H, *mut usize, usize) -> u32,
    analyze: Transform<T, H>,
    reconstruct: Transform<T, H>,
    minimum: Minimum<T, H>,
    to_cepstra: ToCepstra<T, H>,
    to_spectrum: ToSpectrum<T, H>,
}
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
    fn vector<T: Scalar>(&mut self, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                let value = if self.1 == 4 {
                    f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
                } else {
                    f64::from_le_bytes(self.take(8).try_into().unwrap())
                };
                T::from_f64(value).unwrap()
            })
            .collect()
    }
}
fn compare<T: Scalar>(actual: &[T], expected: &[T], bound: f64) {
    assert_eq!(actual.len(), expected.len());
    for (&a, &b) in actual.iter().zip(expected) {
        let error = (a.into() - b.into()).abs() / (1.0 + b.into().abs());
        assert!(
            error.is_finite() && error <= bound,
            "{} vs {}: {error} > {bound}",
            a.into(),
            b.into()
        );
    }
}
fn reference<T: Scalar, H>(bytes: &[u8], width: usize, api: Api<T, H>) {
    let mut r = Reader(bytes, width);
    assert_eq!(r.take(4), b"RCP1");
    assert_eq!(r.integer(), width);
    assert_eq!(r.integer(), 72);
    let bounds = if width == 4 {
        [3e-8, 1.2e-7, 2e-7, 1.6e-7, 7e-9, 0.0, 2.5e-6]
    } else {
        [5e-17, 3e-16, 3e-16, 3e-16, 3e-17, 0.0, 0.0]
    };
    for _ in 0..72 {
        let n = r.integer();
        let count = r.integer();
        let bins = n / 2 + 1;
        let logs = r.vector::<T>(bins);
        let coefficients = r.vector::<T>(n);
        let inverse = r.vector::<T>(n);
        let phase = r.vector::<T>(n);
        let magnitudes = r.vector::<T>(3 * bins);
        let matrix_coefficients = r.vector::<T>(3 * count);
        let matrix_spectra = r.vector::<T>(3 * bins);
        let mut handle = std::ptr::null_mut();
        let mut actual = vec![T::zero(); n];
        let mut output = vec![T::zero(); n];
        // SAFETY: Locally owned typed plan and independent initialized buffers.
        unsafe {
            assert_eq!((api.new)(n, &mut handle), 0);
            let mut shape = [71; 2];
            assert_eq!((api.shape)(handle, shape.as_mut_ptr(), 2), 0);
            assert_eq!(shape, [n, bins]);
            assert_eq!(
                (api.analyze)(handle, logs.as_ptr(), bins, 1, actual.as_mut_ptr(), n),
                0
            );
            compare(&actual, &coefficients, bounds[0]);
            assert_eq!(
                (api.reconstruct)(
                    handle,
                    coefficients.as_ptr(),
                    count,
                    1,
                    output.as_mut_ptr(),
                    n
                ),
                0
            );
            compare(&output, &inverse, bounds[1]);
            assert_eq!(
                (api.reconstruct)(handle, actual.as_ptr(), count, 1, output.as_mut_ptr(), n),
                0
            );
            compare(&output, &inverse, bounds[2]);
            assert_eq!(
                (api.minimum)(handle, logs.as_ptr(), bins, 1, 1, output.as_mut_ptr(), n),
                0
            );
            compare(&output, &phase, bounds[3]);
            let mut actual_matrix = vec![T::zero(); 3 * count];
            assert_eq!(
                (api.to_cepstra)(
                    handle,
                    magnitudes.as_ptr(),
                    magnitudes.len(),
                    count,
                    1,
                    actual_matrix.as_mut_ptr(),
                    actual_matrix.len()
                ),
                0
            );
            compare(&actual_matrix, &matrix_coefficients, bounds[4]);
            let mut spectrum = vec![T::zero(); 3 * bins];
            assert_eq!(
                (api.to_spectrum)(
                    handle,
                    matrix_coefficients.as_ptr(),
                    matrix_coefficients.len(),
                    3,
                    count,
                    1,
                    spectrum.as_mut_ptr(),
                    spectrum.len()
                ),
                0
            );
            compare(&spectrum, &matrix_spectra, bounds[5]);
            assert_eq!(
                (api.to_spectrum)(
                    handle,
                    actual_matrix.as_ptr(),
                    actual_matrix.len(),
                    3,
                    count,
                    1,
                    spectrum.as_mut_ptr(),
                    spectrum.len()
                ),
                0
            );
            compare(&spectrum, &matrix_spectra, bounds[6]);
            assert_eq!((api.release)(&mut handle), 0);
            assert!(handle.is_null());
            assert_eq!((api.release)(&mut handle), 0);
        }
    }
    assert!(r.0.is_empty());
}

#[test]
fn all_original_f32_records_match_through_reused_plans() {
    reference(
        include_bytes!("fixtures/real-cepstrum-float.bin"),
        4,
        Api::<f32, CigletRsRealCepstrumF32> {
            new: ciglet_rs_real_cepstrum_new_f32,
            release: ciglet_rs_real_cepstrum_release_f32,
            shape: ciglet_rs_real_cepstrum_shape_f32,
            analyze: ciglet_rs_real_cepstrum_analyze_f32,
            reconstruct: ciglet_rs_real_cepstrum_reconstruct_f32,
            minimum: ciglet_rs_real_cepstrum_minimum_phase_f32,
            to_cepstra: ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f32,
            to_spectrum: ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f32,
        },
    );
}

#[test]
fn all_original_f64_records_match_through_reused_plans() {
    reference(
        include_bytes!("fixtures/real-cepstrum-double.bin"),
        8,
        Api::<f64, CigletRsRealCepstrumF64> {
            new: ciglet_rs_real_cepstrum_new_f64,
            release: ciglet_rs_real_cepstrum_release_f64,
            shape: ciglet_rs_real_cepstrum_shape_f64,
            analyze: ciglet_rs_real_cepstrum_analyze_f64,
            reconstruct: ciglet_rs_real_cepstrum_reconstruct_f64,
            minimum: ciglet_rs_real_cepstrum_minimum_phase_f64,
            to_cepstra: ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f64,
            to_spectrum: ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f64,
        },
    );
}

#[test]
fn modern_one_shots_match_analytic_round_trip_and_causal_phase() {
    let n = 16;
    let logs: Vec<f64> = (0..=n / 2)
        .map(|i| (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos())
        .collect();
    let mut coefficients = vec![71.0; n];
    let mut reconstructed = vec![71.0; n];
    let mut phase = vec![71.0; n];
    // SAFETY: Independent initialized vectors, correctly sized outputs.
    unsafe {
        assert_eq!(
            ciglet_rs_real_cepstrum_from_log_spectrum_f64(
                logs.as_ptr(),
                logs.len(),
                coefficients.as_mut_ptr(),
                n
            ),
            0
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_to_log_spectrum_f64(
                coefficients.as_ptr(),
                n,
                reconstructed.as_mut_ptr(),
                n
            ),
            0
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_minimum_phase_default_f64(
                logs.as_ptr(),
                logs.len(),
                phase.as_mut_ptr(),
                n
            ),
            0
        );
    }
    for i in 0..n {
        let expected = if i == 1 || i == n - 1 { 0.5 } else { 0.0 };
        assert!((coefficients[i] - expected).abs() < 1e-15);
        assert!(
            (reconstructed[i] - (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos()).abs()
                < 1e-15
        );
        assert!(
            (phase[i] + (2.0 * std::f64::consts::PI * i as f64 / n as f64).sin()).abs() < 1e-15
        );
    }
    let logs8 = [0.0, 1.0, 0.0, 0.0, 0.0];
    let mut handle = std::ptr::null_mut();
    let mut causal = [71.0; 8];
    let mut legacy = [71.0; 8];
    // SAFETY: Locally owned plan and independent initialized writable vectors.
    unsafe {
        assert_eq!(ciglet_rs_real_cepstrum_new_f64(8, &mut handle), 0);
        for nyquist in [0, 1] {
            assert_eq!(
                ciglet_rs_real_cepstrum_minimum_phase_f64(
                    handle,
                    logs8.as_ptr(),
                    5,
                    nyquist,
                    0,
                    causal.as_mut_ptr(),
                    8
                ),
                0
            );
            assert_eq!(
                ciglet_rs_real_cepstrum_minimum_phase_f64(
                    handle,
                    logs8.as_ptr(),
                    5,
                    nyquist,
                    1,
                    legacy.as_mut_ptr(),
                    8
                ),
                0
            );
            let coefficient5 = 0.25 * (5.0 * std::f64::consts::PI / 4.0).cos();
            for i in 0..8 {
                let difference =
                    coefficient5 * (-2.0 * std::f64::consts::PI * i as f64 * 5.0 / 8.0).sin();
                assert!((legacy[i] - causal[i] - difference).abs() < 1e-15);
            }
        }
        assert_eq!(ciglet_rs_real_cepstrum_release_f64(&mut handle), 0);
    }
}

#[test]
fn policies_errors_overlap_and_recovery_preserve_outputs() {
    let mut handle = std::ptr::null_mut();
    // SAFETY: Locally owned typed plan, stable buffers and supported input/output overlap.
    unsafe {
        assert_eq!(ciglet_rs_real_cepstrum_new_f64(4, &mut handle), 0);
        let original = handle;
        assert_eq!(ciglet_rs_real_cepstrum_new_f64(3, &mut handle), 3);
        assert_eq!(handle, original);
        let ignored = [0.0, 0.0, f64::NAN];
        let mut output = [71.0; 4];
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                ignored.as_ptr(),
                3,
                0,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(output, [71.0; 4]);
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                ignored.as_ptr(),
                3,
                1,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [0.0; 4]);
        output.fill(71.0);
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                ignored.as_ptr(),
                3,
                2,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_minimum_phase_f64(
                handle,
                ignored.as_ptr(),
                3,
                1,
                2,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(output, [71.0; 4]);
        let logs = [0.0, 0.0, 4.0];
        let mut overlap = [0.0, 0.0, 4.0, 71.0];
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                overlap.as_ptr(),
                3,
                0,
                overlap.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(overlap, [1.0, -1.0, 1.0, -1.0]);
        assert_eq!(
            ciglet_rs_real_cepstrum_reconstruct_f64(
                handle,
                overlap.as_ptr(),
                4,
                0,
                overlap.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(overlap, [0.0, 0.0, 4.0, 0.0]);
        let suffix = [0.0, 0.0, 0.0, f64::NAN];
        assert_eq!(
            ciglet_rs_real_cepstrum_reconstruct_f64(
                handle,
                suffix.as_ptr(),
                4,
                0,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [0.0; 4]);
        let magnitudes = [1.0, 1.0, 1.0, f64::NAN, 1.0, 1.0];
        let mut matrix = [71.0; 2];
        assert_eq!(
            ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f64(
                handle,
                magnitudes.as_ptr(),
                6,
                1,
                0,
                matrix.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(matrix, [71.0; 2]);
        let invalid = [f64::NAN, 1.0, 1.0];
        assert_eq!(
            ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f64(
                handle,
                invalid.as_ptr(),
                3,
                0,
                0,
                std::ptr::null_mut(),
                0
            ),
            3
        );
        let huge = [f64::MAX; 3];
        output.fill(71.0);
        assert_eq!(
            ciglet_rs_real_cepstrum_reconstruct_f64(
                handle,
                huge.as_ptr(),
                3,
                0,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(output, [71.0; 4]);
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                logs.as_ptr(),
                3,
                0,
                output.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(output, [1.0, -1.0, 1.0, -1.0]);
        output.fill(71.0);
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                logs.as_ptr(),
                2,
                0,
                output.as_mut_ptr(),
                4
            ),
            3
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f64(
                handle,
                logs.as_ptr(),
                3,
                0,
                output.as_mut_ptr(),
                3
            ),
            2
        );
        assert_eq!(output, [71.0; 4]);
        assert_eq!(ciglet_rs_real_cepstrum_release_f64(&mut handle), 0);
        assert!(handle.is_null());
    }
}

#[test]
fn singleton_zero_coefficients_empty_matrices_and_checked_counts() {
    let mut handle = std::ptr::null_mut();
    // SAFETY: Local typed handle and writable outputs; empty inputs follow null contracts.
    unsafe {
        assert_eq!(ciglet_rs_real_cepstrum_new_f32(1, &mut handle), 0);
        let input = [2.0];
        let mut output = [71.0];
        assert_eq!(
            ciglet_rs_real_cepstrum_analyze_f32(
                handle,
                input.as_ptr(),
                1,
                1,
                output.as_mut_ptr(),
                1
            ),
            0
        );
        assert_eq!(output, [2.0]);
        assert_eq!(
            ciglet_rs_real_cepstrum_minimum_phase_f32(
                handle,
                input.as_ptr(),
                1,
                1,
                1,
                output.as_mut_ptr(),
                1
            ),
            0
        );
        assert_eq!(output, [0.0]);
        let mut spectra = [71.0; 2];
        assert_eq!(
            ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f32(
                handle,
                std::ptr::null(),
                0,
                2,
                0,
                0,
                spectra.as_mut_ptr(),
                2
            ),
            0
        );
        let expected = ciglet_rs::approximation::exponential(0.0).unwrap();
        assert_eq!(spectra, [expected; 2]);
        assert_eq!(
            ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f32(
                handle,
                std::ptr::null(),
                0,
                usize::MAX,
                0,
                0,
                std::ptr::null_mut(),
                0
            ),
            2
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_cepstra_to_spectrogram_f32(
                handle,
                std::ptr::null(),
                0,
                0,
                1,
                0,
                std::ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(
            ciglet_rs_real_cepstrum_spectrogram_to_cepstra_f32(
                handle,
                std::ptr::null(),
                0,
                1,
                0,
                std::ptr::null_mut(),
                0
            ),
            0
        );
        assert_eq!(ciglet_rs_real_cepstrum_release_f32(&mut handle), 0);
    }
}

#[test]
fn concurrent_calls_serialize_shared_scratch() {
    let mut handle = std::ptr::null_mut();
    // SAFETY: Ownership is retained until scoped threads end; each owns its buffers.
    unsafe {
        assert_eq!(ciglet_rs_real_cepstrum_new_f64(4, &mut handle), 0);
    }
    let address = handle as usize;
    std::thread::scope(|scope| {
        let calls: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(move || {
                    let input = [0.0, 0.0, 4.0];
                    let mut output = [71.0; 4];
                    // SAFETY: The correctly typed plan remains live; Mutex serializes scratch.
                    unsafe {
                        assert_eq!(
                            ciglet_rs_real_cepstrum_analyze_f64(
                                address as *const CigletRsRealCepstrumF64,
                                input.as_ptr(),
                                3,
                                0,
                                output.as_mut_ptr(),
                                4
                            ),
                            0
                        );
                    }
                    assert_eq!(output, [1.0, -1.0, 1.0, -1.0]);
                })
            })
            .collect();
        for call in calls {
            call.join().unwrap();
        }
    });
    // SAFETY: Every call ended, and sole ownership is transferred exactly once.
    unsafe {
        assert_eq!(ciglet_rs_real_cepstrum_release_f64(&mut handle), 0);
    }
    assert!(handle.is_null());
}
